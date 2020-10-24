#![allow(non_snake_case)]

mod commands;
use commands::{
    owner::*,
    common::*,
    role_applications::*,
};

use anyhow::Result;
use serde_derive::Deserialize;
use serenity::model::{channel::Message, id::ChannelId};
use serenity::{prelude::*, utils::Colour};

use std::{
    collections::HashSet,
    sync::Arc,
};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

#[derive(Debug, Deserialize)]
struct Config {
    marker: String, // Marker string to recognise discord messages sent to the bot, defind in botconfig.toml.
    own_bot_token: String, // Own bot token used to connect with discord.
    channel_ids: Vec<ChannelId>, // Channels the bot listens on, definsed in botconfig.toml.
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for Config {
    type Value = Config;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(quit, multiply, divide, howtohack, hacksplain, google, ping, apply)]
struct General;

#[tokio::main]
async fn main() {
    // Load botconfig.toml 
    let config: Config = loadconfig().expect("Can't load config file: botconfig.toml. Please make sure you have one next to the executable and it's correct.");
    info!("Botconfig loaded {:?}", &config);

    // Initialize the logger to use environment variables. `RUST_LOG`
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start logging");

    let http = Http::new_with_token(&config.own_bot_token);

    // Fetch bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix(&config.marker))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&config.own_bot_token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Add config data to bot context so it becomes accessible throughout the bot.
    // It is in a separate scope to drop the mutable reference right after we add the config.
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<Config>(config);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        // error!("Client error: {:?}", why);
        println!("Client error: {:?}", why);
    }
}

// Loading bot config file.
fn loadconfig() -> Result<Config> {
    let configtoml = std::fs::read_to_string("botconfig.toml")?;
    let decoded: Config = toml::from_str(&configtoml)?;
    Ok(decoded)
}
