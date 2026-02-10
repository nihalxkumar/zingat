# Zingat

![Rust](https://img.shields.io/badge/Rust-1.56%2B-orange?logo=rust)
![License](https://img.shields.io/github/license/nihalxkumar/zingat)
![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-brightgreen)

Pastebin like app in rust

Securely share texts on the internet with password protection and expiry date.

![browser-screenshot.png](./assets/browser-screenshot.png "Window screenshot of a browser with zingat localhost")

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
  - ```
    Zingat API Client

    USAGE:
        clipclient --api-key <api-key> [addr] <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --api-key <api-key>

    ARGS:
        <addr>     [env: ZINGAT_ADDR=]  [default: http://127.0.0.1:8000]

    SUBCOMMANDS:
        get
        help      Prints this message or the help of the given subcommand(s)
        new
        update
      ```

## Tech Stack

- Rust
- SQLX
- Rocket
- Tokio

### Nuggets for devs

- Instead of logging each view (hit) directly into the database, we defer these operations into separate threads. This allows us to batch multiple hits into a single database commit, significantly improving performance and reducing contention on the database.
-

![diagram](./assets/zingatdiagram.png)
