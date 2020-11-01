use serenity::model::prelude::Message;
use serenity::client::Context;

pub async fn no_prefix(ctx: &Context, msg: &Message) {
    if msg.is_private() {
        println!("message received inner {}", msg.content);
        println!("message received inner {}", msg.timestamp);
        println!("message received inner {}", msg.author);

    }
}

