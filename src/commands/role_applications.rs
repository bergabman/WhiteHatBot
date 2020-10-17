use std::path::Path;
use crate::Config;
use crate::ShardManagerContainer;

use serenity::{
    prelude::*, 
    http::AttachmentType,
    utils::MessageBuilder,
};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn apply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first_arg = match args.single::<String>() {
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
        },
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

    msg.author.dm(&ctx, |m| {
        m.content("ssup");

        m
    }).await?;
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