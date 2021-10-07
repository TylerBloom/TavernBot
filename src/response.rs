pub mod Response {

    use serenity::prelude::*;
    use serenity::model::channel::Embed;
    use serenity::framework::standard::CommandResult;
    use serenity::model::id::ChannelId;
    
    pub struct Response {
        message: Option<String>,
        embed: Option<Embed>,
    }

    pub fn new() -> Response {
        Response {
            message: None, embed: None,
        }
    }

    impl Response {
        pub fn set_message( &mut self, msg: String ) {
            self.message = Some(msg);
        }
        
        pub fn set_embed( &mut self, embed: Embed ) {
            self.embed = Some(embed);
        }
        
        async pub fn send( &self, channel: ChannelID ) -> CommandResult {
            if( self.message.is_none() && self.embed.is_none() ) {
                OK(())
            } else if( self.message.is_none() ) {
                OK(())
            } else {
                OK(())
            }
        }
    }
}
