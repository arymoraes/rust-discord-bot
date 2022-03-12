use serenity::{client::Context, model::channel::Message};

use self::palindromo::palindromo;

pub mod soma;
pub mod palindromo;

pub async fn boilimax(ctx: Context, msg: Message) {
    if msg.content == "boi" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "limax").await {
            println!("Error sending message: {:?}", why);
        }
    }
    if msg.content.contains("!soma:") {
        let total = soma::soma(&msg.content, "!soma:");

        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("Seu total eh: {}", total))
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    if msg.content.contains("!xamiliob") {
        let reversed_input = palindromo(&msg.content, "!xamiliob");
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("xamiliob: {}", reversed_input))
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}
