#![allow(non_snake_case)]

mod commands;
use commands::{common::*, no_prefix::*, owner::*, role_applications::*};

use anyhow::Result;
use serde_derive::Deserialize;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::model::id::RoleId;
use serenity::model::{channel::Message, id::ChannelId};
use serenity::prelude::*;

use std::{collections::HashSet, sync::Arc};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
};
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Deserialize)]
struct Config {
    marker: String, // Marker string to recognise discord messages sent to the bot, defind in botconfig.toml.
    own_bot_token: String, // Own bot token used to connect with discord.
    channel_ids: Vec<ChannelId>, // Channels the bot listens on, definsed in botconfig.toml.
    default_roles: Vec<RoleId>, // Role to be added after accepting rules
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for Config {
    type Value = Config;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {

        // no_prefix(&ctx, &msg).await;

        // if msg.is_private() {
        //     println!("message received {}", msg.content);
        //     println!("message received {}", msg.timestamp);
        //     println!("message received {}", msg.author);

        // }
        no_prefix(&ctx, &msg).await;

        if msg.is_private() {
            println!("message received {}", msg.content);
            println!("message received {}", msg.timestamp);
            println!("message received {}", msg.author);
        }
        if msg.content == "!hello" {
            // The create message builder allows you to easily create embeds and messages
            // using a builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, three fields, and a footer.
       
        }
    }
    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<serenity::model::guild::Member>,
        mut new: serenity::model::guild::Member,
    ) {
        // This compares the old with the new to check if pending bool went from true to false
        // The bool being false means the user accepted the screening and a default role is added
        if let Some(old_member) = old_if_available {
            if old_member.pending && !new.pending {
                let data = ctx.data.read().await;
                if let Some(config) = data.get::<Config>() {
                    match new.add_roles(&ctx, &config.default_roles).await {
                        Ok(_) => {
                            let nickname = new.display_name();
                            // println!("User: {} has accepted screening. Added role(s).", nickname);
                        }
                        Err(err) => println!("Error occurred adding role: {}", err),
                    }
                }
            }
        }
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(quit, multiply, divide, howtohack, hacksplain, google, ping, hax, dunning, howtoask /*apply*/)]
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
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners)
                        .prefix(&config.marker)
                        .no_dm_prefix(true)
                    )
        // .normal_message(no_prefix)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&config.own_bot_token)
        .intents(GatewayIntents::all())
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
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
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
