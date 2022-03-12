use serenity::{client::Context, model::channel::Message};

use self::{palindromo::palindromo, cat::meow};

pub mod soma;
pub mod palindromo;
pub mod cat;
pub mod dog;
pub mod nsfw;

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

    if msg.content.contains("!meow") {
        let cat_pic = meow().await;
        match cat_pic {
            Ok(mut pic) => {
                // remove the "" from the string
                pic = pic.replace("\"", "");
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, pic)
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            Err(why) => {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    if msg.content.contains("!bork") {
        let dog_pic = dog::bork().await;
        match dog_pic {
            Ok(mut pic) => {
                // remove the "" from the string
                pic = pic.replace("\"", "");
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, pic)
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            Err(why) => {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    if msg.author.id.to_string() == "404386544124821513" {
        let nsfw_pic = nsfw::nsfw().await;
        match nsfw_pic {
            Ok(mut pic) => {
                // remove the "" from the string
                pic = pic.replace("\"", "");
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, pic)
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            Err(why) => {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
