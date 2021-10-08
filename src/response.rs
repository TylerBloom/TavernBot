pub mod Response {

    use serenity::prelude::*;
    use serenity::model::channel::Embed;
    use serenity::framework::standard::CommandResult;
    use serenity::model::channel::Message;
    use serenity::builder::CreateMessage;
    
    pub struct Response {
        content: Option<String>,
        embed: Option<Embed>,
    }

    pub fn new() -> Response {
        Response {
            content: None, embed: None,
        }
    }
    
    pub async fn send_message( res: Response, ctx: &Context, msg: &Message ) -> CommandResult {
        if( res.content.is_some() && res.embed.is_some() ) {
            // The message contains both content and an embed
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content(res.content());
                m.embed( |mut e| {
                    e.title(&res.embed_title());
                    e.fields( res.embed.unwrap().fields.iter().map( |f| { (f.name.clone(), f.value.clone(), f.inline) } ) );
                    
                    e
                } );
            
                m
            }).await?;
            Ok(())
        } else if( res.content.is_some() ) {
            // The response only contains some content
            msg.channel_id.say( &ctx.http, &res.content.unwrap() ).await?;
            Ok(())
        } else if( res.embed.is_some() ) {
            // The response only contains an embed
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed( |mut e| {
                    e.title(&res.embed_title());
                    e.fields( res.embed.unwrap().fields.iter().map( |f| { (f.name.clone(), f.value.clone(), f.inline) } ) );
                    
                    e
                } );
            
                m
            }).await?;
            Ok(())
        } else {
            // There is neither content nor an embed, so we will exit quietly
            Ok(())
        }
    }

    impl Response {
        pub fn content( &self ) -> String {
            match self.content.clone() {
                None => String::from(""),
                Some(c) => c.clone(),
            }
        }
        pub fn set_content( &mut self, msg: String ) {
            self.content = Some(msg);
        }
        
        pub fn embed_title( &self ) -> String {
            match self.embed.clone() {
                None => String::from(""),
                Some(e) => match e.title {
                    None => String::from(""),
                    Some(t) => t.clone(),
                }
            }
        }
        pub fn set_embed( &mut self, embed: Embed ) {
            self.embed = Some(embed);
        }
    }
}
