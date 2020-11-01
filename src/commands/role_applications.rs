use crate::Config;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::{prelude::*, utils::MessageBuilder};
use serenity::model::channel::GuildChannel;

#[command]
pub async fn apply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data
        .get::<Config>()
        .expect("Expected Config in SharedMap, Please check your botconfig.toml");
    if !config.channel_ids.contains(&msg.channel_id) {
        return Ok(());
    }
    let _first_arg = match args.single::<String>() {
        Ok(first_arg) => first_arg,
        Err(_) => {
            msg.reply(&ctx.http, "You what?").await?;
            return Ok(());
        }
    };

    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return Ok(());
        }
    };
    let channel_type = msg.channel(&ctx.cache).await.unwrap();
    let value = "&";
    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push("UserID ")
        .push_bold_safe(&msg.author.id)
        .push("channelType ")
        .push_bold_safe(channel_type)
        .push(" used the 'apply' command in the ")
        .mention(&channel)
        .push(" channel")
        .push_mono(value)
        .build();

    msg.author
        .dm(&ctx, |m| {
            m.content("ssup");

            m
        })
        .await?;
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    // let msg = msg.channel_id.send_message(&ctx.http, |m| {
    //     m.content("Hello, World!");
    //     m.embed(|e| {
    //         e.title("This is a title");
    //         e.description("This is a description");
    //         e.image("attachment://ferris_eyes.png");
    //         e.fields(vec![
    //             ("This is the first argument", "This is a field body", true),
    //             ("This is the second field", "Both of these fields are inline", true),
    //         ]);
    //         e.field("First argument", &first_arg, false);
    //         e.footer(|f| {
    //             f.text("This is a footer");

    //             f
    //         });

    //         e
    //     });
    //     m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
    //     m
    // }).await?;

    // msg.channel_id.say(&ctx.http, "ok").await?;

    Ok(())
}

#[command]
#[only_in(dm)]
pub async fn m(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    msg.author
        .dm(&ctx, |m| {
            m.content("ssup");
            m.embed(|e| {
                e.title("This is a title");
                e.description("This is a description");
                e.image("attachment://ferris_eyes.png");
                e.fields(vec![
                    ("This is the first argument", "This is a field body", true),
                    ("This is the second field", "Both of these fields are inline", true),
                ]);
                e.field("First argument", &args.message(), false);
                e.footer(|f| {
                    f.text("This is a footer");
    
                    f
                });
    
                e
            });
            m
        })
        .await?;

    msg.is_private();
    Ok(())
}

// pub fn create_hidden_channel(name: String, guild_id: GuildId) -> Result<GuildChannel> {

//     // let main_channel: GuildId = 771849934286618685;
//     let new_channel = GuildId(771849934286618685).create_channel(|channel| channel.name(name).kind(ChannelType::Text))
//         .chain_err(|| "failed to create new channel")?;

//     // block the channel for everyone who hasn't clicked the checkmark. see process_checkmark().
//     hide_channel(
//         &new_channel,
//         from_role_id(everyone_role(guild_id).chain_err(|| "failed to find @everyone RoleId")?),
//     ).chain_err(|| "failed to configure new channel")?;

//     Ok(new_channel)
// }