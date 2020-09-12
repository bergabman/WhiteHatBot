#![allow(non_snake_case)]
use anyhow::Result;
use serde_derive::Deserialize;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    marker: String,        // string to recognise discord messages
    own_bot_token: String, // own bot token used to make connection with discord server
    channel_id: u64,       // channel id we communicate on
}

impl TypeMapKey for Config {
    type Value = Config;
}

fn main() -> Result<()> {
    let config: Config = loadconfig().expect("Can't load configfile botconfig.toml, Please make sure you have one next to the executable and it's correct.");
    println!("Botconfig decoded {:?}", &config);
    let mut client = Client::new(&config.own_bot_token, Handler).expect("Error creating client");
    {
        let mut data = client.data.write(); //  adding config to context, so we can access it in the shard
        data.insert::<Config>(config);
    }

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
    Ok(())
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {}
}

fn loadconfig() -> Result<Config> {
    // loading bot config file
    let configtoml = std::fs::read_to_string("botconfig.toml")?;
    let decoded: Config = toml::from_str(&configtoml)?;
    Ok(decoded)
}
