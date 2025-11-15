mod arg_parser;
mod nyra_main;
mod term_utils;

#[tokio::main]
async fn main() -> Result<(), ()> {
  nyra_main::main().await
}
