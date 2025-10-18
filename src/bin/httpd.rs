use zingat::data::AppDatabase;
use zingat::domain::maintenance::Maintenance;
use zingat::web::{renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;
use zingat::web::hitcounter::HitCounter;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpd", about = "Zingat HTTP server")]
struct Opt {
    #[structopt(env = "DATABASE_URL", default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), env = "TEMPLATE_DIR", default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    // Create database directory if it doesn't exist
    // Extract directory path from connection string (e.g., sqlite:/app/data/zingat.db -> /app/data)
    if let Some(db_path) = opt.connection_string.strip_prefix("sqlite:") {
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|e| eprintln!("Warning: Could not create database directory: {}", e));
        }
    }

    let rt = tokio::runtime::Runtime::new()
        .expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    let renderer = Renderer::new(opt.template_directory.clone());

    let database = rt.block_on(async move {AppDatabase::new(&opt.connection_string).await});

    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());

    let maintenance = Maintenance::spawn(database.get_pool().clone(), handle.clone());
    let config = zingat::RocketConfig{
        renderer,
        database,
        hit_counter,
        maintenance,
    };

    rt.block_on(async move {
        zingat::rocket(config)
            .launch()
            .await
            .expect("failed to launch Rocket server");
    })
}