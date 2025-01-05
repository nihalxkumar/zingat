use zingat::data::AppDatabase;
use zingat::web::{renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpd", about = "Zingat HTTP server")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new()
        .expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = zingat::RocketConfig{
            renderer,
            database,
        };

        zingat::rocket(config)
            .launch()
            .await
            .expect("failed to launch Rocket server");
    })
}