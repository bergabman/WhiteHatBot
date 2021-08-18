use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn no_prefix(_ctx: &Context, msg: &Message) {
    if msg.is_private() {
        println!("message received inner {}", msg.content);
        println!("message received inner {}", msg.timestamp);
        println!("message received inner {}", msg.author);
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
