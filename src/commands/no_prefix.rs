use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn no_prefix(_ctx: &Context, msg: &Message) {
    if msg.is_private() {
        println!("message received inner {}", msg.content);
        println!("message received inner {}", msg.timestamp);
        println!("message received inner {}", msg.author);
    }
}
