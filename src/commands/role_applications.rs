use crate::Config;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::{prelude::*, utils::MessageBuilder};

#[command]
pub async fn apply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data
        .get::<Config>()
        .expect("Expected Config in SharedMap, Please check your botconfig.toml");
    if !config.channel_ids.contains(&msg.channel_id) {
        return Ok(());
    }

    let mut level = match args.single::<String>() {
        Ok(level) => level,
        Err(_) => {
            apply_command_help(&ctx, &msg).await?;
            return Ok(());
        },
    };
    let mut haxor_level = "";

    if level == "g" || level == "good" {
        haxor_level = "Good";
    } else if level == "e" || level == "experienced" {
        haxor_level = "Experienced";
    } else if level == "m" || level == "master"{
        haxor_level = "Master";
    } else {
        apply_command_help(&ctx, &msg).await?;
        return Ok(());
    }

<<<<<<< HEAD
=======
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
pub async fn m(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
>>>>>>> 910c09d0773b7720b1d98858b8362351ab29e652
    msg.author
        .dm(&ctx, |m| {
            m.embed(|e| {
<<<<<<< HEAD
                e.title(format!("You have applied for the {} hacker role.", &haxor_level));
                e.description("You can find fictional cases that you have to solve here: https://discordapp.com/channels/429657740562923521/661628899175694336/661629917443915796");
                e.field("When you are ready with your answer, just dm it to the bot and it will be forwarded to the staff.", "_ _", false);
                e.footer(|f| {
                    f.text("WhiteHat Hacking https://discord.gg/whAx4qh");
                    f
                });
=======
                e.title("This is a title");
                e.description("This is a description");
                e.image("attachment://ferris_eyes.png");
                e.fields(vec![
                    ("This is the first argument", "This is a field body", true),
                    (
                        "This is the second field",
                        "Both of these fields are inline",
                        true,
                    ),
                ]);
                e.field("First argument", &args.message(), false);
                e.footer(|f| {
                    f.text("This is a footer");

                    f
                });

>>>>>>> 910c09d0773b7720b1d98858b8362351ab29e652
                e
            });
            m
        })
        .await?;

    Ok(())
}

async fn create_channel(ctx: &Context,) {

}

async fn apply_command_help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Apply for roles.");
            e.description("You can apply for 3 different roles, Good, Experienced and Master hacker.");
            e.field("Good hacker `--apply g`", "https://discordapp.com/channels/429657740562923521/661628899175694336/661629917443915796", false);
            e.field("Experienced hacker `--apply e`", "https://discordapp.com/channels/429657740562923521/661628899175694336/661630071274209352", false);
            e.field("Master hacker `--apply m`", "https://discordapp.com/channels/429657740562923521/661628899175694336/661630182104498177", false);
            e.field("After typing the right command, expect a dm from the bot to start the application.", "WhiteHat Hacking https://discord.gg/whAx4qh", false);
            e
        });
        m
    }).await?;
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
