use serenity::{model::channel::Message, client::Context};

pub mod soma;

pub async fn boilimax(ctx: Context, msg: Message) {
    if msg.content == "boi" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "limax").await {
            println!("Error sending message: {:?}", why);
        }
    }
    if msg.content.contains("!soma:") {
        let total = soma::soma(&msg.content, "!soma:");

        if let Err(why) = msg.channel_id.say(&ctx.http, format!("Seu total eh: {}", total)).await {
            println!("Error sending message: {:?}", why);
        }
    }
}