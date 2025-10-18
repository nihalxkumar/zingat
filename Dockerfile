FROM rust:1.80-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src/bin src/lib/data src/lib/domain/clip/field src/lib/service/ask src/lib/web/api && \
    echo "fn main() {}" > src/bin/httpd.rs && \
    echo "fn main() {}" > src/bin/clipclient.rs && \
    echo "pub mod data; pub mod domain; pub mod service; pub mod web; \
pub struct RocketConfig { pub renderer: (), pub database: (), pub hit_counter: (), pub maintenance: () } \
pub fn rocket(_config: RocketConfig) -> () { () } \
pub type Clip = ();" > src/lib/mod.rs && \
    echo "pub type AppDatabase = (); pub type DataError = ();" > src/lib/data/mod.rs && \
    echo "pub mod clip; pub type Clip = ();" > src/lib/domain/mod.rs && \
    echo "pub mod field; pub type Clip = ();" > src/lib/domain/clip/mod.rs && \
    echo "pub type Content = (); pub type Expires = (); pub type Password = (); pub type ShortCode = (); pub type Title = ();" > src/lib/domain/clip/field/mod.rs && \
    echo "pub mod ask;" > src/lib/service/mod.rs && \
    echo "pub type GetClip = (); pub type NewClip = (); pub type UpdateClip = ();" > src/lib/service/ask/mod.rs && \
    echo "pub mod api;" > src/lib/web/mod.rs && \
    echo "pub type ApiKey = (); pub const API_KEY_HEADER: &str = \"\";" > src/lib/web/api/mod.rs

ENV CARGO_PROFILE_RELEASE_LTO=true \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
    DATABASE_URL=sqlite:///tmp/dummy.db

RUN touch /tmp/dummy.db && \
    sqlite3 /tmp/dummy.db "CREATE TABLE IF NOT EXISTS clips (clip_id TEXT PRIMARY KEY NOT NULL, shortcode TEXT UNIQUE NOT NULL, content TEXT NOT NULL, title TEXT, posted DATETIME NOT NULL, expires DATETIME, password TEXT, hits BIGINT NOT NULL); CREATE TABLE IF NOT EXISTS api_keys (api_key BLOB PRIMARY KEY);" && \
    cargo build --release && \
    rm -rf src

COPY src ./src
COPY templates ./templates
COPY static ./static
COPY migrations ./migrations

RUN cargo build --release && \
    strip target/release/httpd

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    sqlite3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 zingat

WORKDIR /app

COPY --from=builder /app/target/release/httpd /app/httpd
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
COPY --from=builder /app/migrations /app/migrations

RUN mkdir -p /app/data && chown -R zingat:zingat /app
USER zingat

EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

CMD ["./httpd"]
