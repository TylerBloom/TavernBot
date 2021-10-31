pub mod Response {

    use std::collections::HashMap;

    use serenity::builder::CreateMessage;
    use serenity::framework::standard::CommandResult;
    use serenity::model::channel::Embed;
    use serenity::model::channel::Message;
    use serenity::prelude::*;
    use serenity::utils::Colour;

    // Models a serenity Embed.  Unlike in discord.py, embeds can't really be
    // created, modified, and then sent.  The send_message method takes a
    // FnOnce of a CreateEmbed, so they have to be created when they are sent.
    // To cope with this, EmbedSpoof models that bare minimum for an embed
    // while putting a few restrictions on its use.  All EmbedSpoofs must have
    // a title and a color.
    // TODO: Consider using webhooks
    #[derive(Clone)]
    pub struct EmbedSpoof {
        pub title: String,
        pub colour: Colour,
        pub fields: Vec<(String, String, bool)>,
    }

    pub struct Response {
        content: Option<String>,
        embed: Option<EmbedSpoof>,
    }

    pub fn new() -> Response {
        Response {
            content: None,
            embed: None,
        }
    }

    pub async fn send_message(res: Response, ctx: &Context, msg: &Message) -> CommandResult {
        if (res.content.is_some() && res.embed.is_some()) {
            // The message contains both content and an embed
            let embed = res.embed.unwrap().clone();
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|mut e| {
                        e.title(embed.title);
                        e.colour(embed.colour);
                        e.fields(embed.fields);

                        e
                    });

                    m
                })
                .await?;
            Ok(())
        } else if (res.content.is_some()) {
            // The response only contains some content
            msg.channel_id.say(&ctx.http, &res.content.unwrap()).await?;
            Ok(())
        } else if (res.embed.is_some()) {
            // The response only contains an embed
            let embed = res.embed.unwrap().clone();
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|mut e| {
                        e.title(embed.title);
                        e.colour(embed.colour);
                        e.fields(embed.fields);

                        e
                    });

                    m
                })
                .await?;
            Ok(())
        } else {
            // There is neither content nor an embed, so we will exit quietly
            Ok(())
        }
    }

    impl Response {
        pub fn content(&self) -> String {
            match self.content.clone() {
                None => String::from(""),
                Some(c) => c.clone(),
            }
        }
        pub fn set_content(&mut self, msg: String) {
            self.content = Some(msg);
        }

        pub fn embed_title(&self) -> String {
            match self.embed.clone() {
                None => String::from(""),
                Some(e) => e.title,
            }
        }
        pub fn set_embed(&mut self, embed: EmbedSpoof) {
            self.embed = Some(embed);
        }
    }
}
