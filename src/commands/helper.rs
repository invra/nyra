use poise::Command;

#[derive(Debug)]
pub(crate) struct Data;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(dead_code)]
pub(crate) struct MyCommand(fn() -> Command<Data, Error>);
