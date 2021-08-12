use serenity::model::prelude::Message;
use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandResult;

// #[hook]
pub async fn no_prefix(ctx: &Context, msg: &Message) {

    if msg.content.starts_with("+") {
        match surf::get("http://white-hat-mail-1xnauai.glitch.me/").await {
            Ok(mut res) => {
                println!("WhiteHatMail was waken up by {}\nSurf result : {}", msg.author.name, res.body_string().await.unwrap() );
                return;
            },
            Err(e) => {
                println!("Failed to wake WhiteHatMail {}", e.to_string());
                return;
            },
        };
        // println!("message received normal {}", res.body_string().await.unwrap());
    }

    
    // if !msg.is_private() {
    //     println!("message received normal {}", msg.content);
    //     return
    // }
    // println!("message received inner {}", msg.content);
    // println!("message received inner {}", msg.timestamp);
    // println!("message received inner {}", msg.author);

    // let msg = msg.channel_id.send_message(&ctx.http, |m| {
    //     m.content("Hello, World for all!");
    //     m.embed(|e| {
    //         e.title("This is a title");
    //         e.description("This is a description");
    //         e.image("attachment://ferris_eyes.png");
    //         e.fields(vec![
    //             ("This is the first field", "This is a field body", true),
    //             ("This is the second field", "Both of these fields are inline", true),
    //         ]);
    //         e.field("This is the third field", "This is not an inline field", false);
    //         e.footer(|f| {
    //             f.text("This is a footer");

    //             f
    //         });

    //         e
    //     });
    //     // m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
    //     m
    // }).await;

    // if let Err(why) = msg {
    //     println!("Error sending message: {:?}", why);
    // }
}

