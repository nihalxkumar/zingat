# Zingat

![Rust](https://img.shields.io/badge/Rust-1.56%2B-orange?logo=rust)
![License](https://img.shields.io/github/license/nihalxkumar/zingat)
![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-brightgreen)

Pastebin like app in rust

Securely share texts on the internet with password protection and expiry date.

![browser-screenshot.png](browser-screenshot.png "Window screenshot of a browser with zingat localhost")

## Local Installation

- Install [Rust](https://www.rust-lang.org/tools/install)
- [Clone](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) the Repository
- Setup Database
  - Install SQLX with `cargo install sqlx-cli`
  - `sqlx database setup`
- For Web Interface
  - `cargo run --bin httpd`
- For Command Line Interface
  - `cargo run --bin clipclient`

## Tech Stack

- Rust
- SQLX
- Rocket
- Tokio

### Nuggets for devs

- Instead of logging each view (hit) directly into the database, we defer these operations into separate threads. This allows us to batch multiple hits into a single database commit, significantly improving performance and reducing contention on the database.
- 