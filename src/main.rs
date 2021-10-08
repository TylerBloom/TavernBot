//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```

mod response;
mod card;
mod card_db;
mod card_entry;
mod tradelist;
mod utils;

use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Write,
    sync::{Arc, RwLock},
};

use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::{GatewayIntents, ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, Delimiter, DispatchError, HelpOptions,
        Reason, StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message, Embed, EmbedField},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions, Colour},
};
use tokio::sync::Mutex;
use dashmap::DashMap;

pub use response::Response;
pub use card::Card;
pub use card_db::CardDB;
pub use card_entry::CardEntry;
pub use tradelist::Tradelist;
pub use utils::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(printings, tradelist)]
struct General;

// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~\n\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip_in_{dm, guild}` aren't specified.
// If you pass in a value, it will be displayed instead.
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
    // You may want to handle a Discord rate limit if this fails.
    let _ = msg.react(ctx, '⏱').await;
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}

// You can construct a hook without the use of a macro, too.
// This requires some boilerplate though and the following additional import.
use serenity::{futures::future::BoxFuture, FutureExt};
fn _dispatch_error_no_macro<'fut>(
    ctx: &'fut mut Context,
    msg: &'fut Message,
    error: DispatchError,
) -> BoxFuture<'fut, ()> {
    async move {
        if let DispatchError::Ratelimited(info) = error {
            if info.is_first_try {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", info.as_secs()),
                    )
                    .await;
            }
        };
    }
    .boxed()
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("!")
                // In this case, if "," would be first, a message would never
                // be delimited at ", ", forcing you to trim your arguments if you
                // want to avoid whitespaces at the start of each.
                .delimiters(vec![", ", ","])
                // Sets the bot's owners. These will be used for commands that
                // are owners only.
                .owners(owners)
        })
        // Set a function to be called prior to each command execution. This
        // provides the context of the command, the message that was received,
        // and the full name of the command that will be called.
        //
        // Avoid using this to determine whether a specific command should be
        // executed. Instead, prefer using the `#[check]` macro which
        // gives you this functionality.
        //
        // **Note**: Async closures are unstable, you may use them in your
        // application if you are fine using nightly Rust.
        // If not, we need to provide the function identifiers to the
        // hook-functions (before, after, normal, ...).
        .before(before)
        // Similar to `before`, except will be called directly _after_
        // command execution.
        .after(after)
        // Set a function that's called whenever an attempted command-call's
        // command could not be found.
        .unrecognised_command(unknown_command)
        // Set a function that's called whenever a message is not a command.
        .normal_message(normal_message)
        // Set a function that's called whenever a command's execution didn't complete for one
        // reason or another. For example, when a user has exceeded a rate-limit or a command
        // can only be performed by the bot owner.
        .on_dispatch_error(dispatch_error)
        // Can't be used more than once per 5 seconds:
        .bucket("emoji", |b| b.delay(5))
        .await
        // Can't be used more than 2 times per 30 seconds, with a 5 second delay applying per channel.
        // Optionally `await_ratelimits` will delay until the command can be executed instead of
        // cancelling the command invocation.
        .bucket("complicated", |b| {
            b.limit(2)
                .time_span(30)
                .delay(5)
                // The target each bucket will apply to.
                .limit_for(LimitedFor::Channel)
                // The maximum amount of command invocations that can be delayed per target.
                // Setting this to 0 (default) will never await/delay commands and cancel the invocation.
                .await_ratelimits(1)
                // A function to call when a rate limit leads to a delay.
                .delay_action(delay_action)
        })
        .await
        // The `#[group]` macro generates `static` instances of the options set for the group.
        // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
        // #name is turned all uppercase
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        // For this example to run properly, the "Presence Intent" and "Server Members Intent"
        // options need to be enabled.
        // These are needed so the `required_permissions` macro works on the commands that need to
        // use it.
        // You will need to enable these 2 options on the bot application, and possibly wait up to 5
        // minutes.
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CardDB::CardDB>(CardDB::create(String::from("AtomicCards.json")));
        data.insert::<Tradelist::Tradelist>(DashMap::new());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command("printings")]
async fn printings(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg_card = args.single::<String>();

    let data = ctx.data.read().await;
    let db = data.get::<CardDB::CardDB>().unwrap();

    if arg_card.is_err() {
        msg.channel_id
            .say(
                &ctx.http,
                &String::from("You need to specify a card to see it's printings."),
            )
            .await?;
    } else {
        let card_name = arg_card.unwrap();
        //let db = CardDB::create( String::from( "AtomicCards.json" ) );
        let card = db.get_card(&card_name);

        if card.is_none() {
            msg.channel_id
                .say(
                    &ctx.http,
                    &String::from("The card that you specified could not be found."),
                )
                .await?;
        } else {
            let mut content = format!("{} was printed the following sets: ", &card_name);
            for p in &card.unwrap().printings {
                content += &format!("[{}], ", p);
            }
            msg.channel_id.say(&ctx.http, &content).await?;
        }
    }
    Ok(())
}

fn create_entries(db: &CardDB::CardDB, args: &str) -> Vec<CardEntry::CardEntry> {
    let mut digest = Vec::new();
    let mut entries: Args = Args::new(args, &[Delimiter::Single('\n')]);

    let mut curr_entry = entries.single::<String>();
    let mut entry_args: Args;
    let mut entry_quantity: Types::CardCount;
    let mut entry_name: String;
    let mut entry_card: Option<&Card::AtomicCard>;

    while !curr_entry.is_err() {
        entry_args = Args::new(&curr_entry.unwrap(), &[Delimiter::Single(' ')]);
        entry_quantity = entry_args.single::<Types::CardCount>().unwrap();
        entry_name = String::from(entry_args.rest());
        entry_card = db.get_card(&entry_name);
        if entry_card.is_none() {
            ();
        } else {
            digest.push(CardEntry::new(
                entry_quantity,
                Card::new(entry_card.unwrap().clone(), String::from("")),
            ));
        }
        curr_entry = entries.single::<String>();
    }
    digest
}

async fn view_tradelist(ctx: &Context, msg: &Message) -> Response::Response {
    let mut digest: Response::Response = Response::new();
    let data = ctx.data.read().await;
    let tradelists = data.get::<Tradelist::Tradelist>().unwrap();

    if let Some(list) = tradelists.get(&msg.author.id) {
        println!( "{}", list.to_string() );
        let temp_embed = Embed::fake( |mut e| { 
            e.title(String::from("Your Tradelist: "));
            e.field(String::from("Cards"), list.to_string(), true );
            e
            });
        digest.set_embed( temp_embed );
    } else {
        digest.set_content( String::from("You don't have a tradelist. Use '!tradelist add' to add some cards first.") );
    }
    
    digest
}

async fn add_to_tradelist(ctx: &Context, msg: &Message, args: Args) -> Response::Response {
    println!("Adding cards to the tradelist.");
    let mut digest: Response::Response = Response::new();
    let data = ctx.data.write().await;
    let tradelists = data.get::<Tradelist::Tradelist>().unwrap();
    let db = data.get::<CardDB::CardDB>().unwrap();
    let entries = create_entries(db, args.rest());

    if let Some(list) = tradelists.get_mut(&msg.author.id) {
        digest.set_content( String::from("Your tradelist has been updated. Use '!tradelist view' to see it.") );
    } else {
        tradelists
            .insert(msg.author.id, Tradelist::new());
        digest.set_content( String::from("You have added a tradelist with some cards. To see it, use the command '!tradelist view'.") );
    }

    for entry in entries {
        tradelists
            .get_mut(&msg.author.id)
            .unwrap()
            .add_card(entry);
    }
    digest
}

async fn remove_from_tradelist(ctx: &Context, msg: &Message, mut args: Args) -> Response::Response {
    let mut digest: Response::Response = Response::new();
    let data = ctx.data.read().await;
    let list = data.get::<Tradelist::Tradelist>().unwrap();
    digest
}

async fn make_public_tradelist(ctx: &Context, msg: &Message) -> Response::Response {
    println!("Making the tradelist public.");
    let mut digest: Response::Response = Response::new();
    let data = ctx.data.read().await;
    let tradelists = data.get::<Tradelist::Tradelist>().unwrap();
    if let Some(mut list) = tradelists.get_mut(&msg.author.id) {
        list.set_public();
        digest.set_content( String::from("Your tradelist has been set to public. Your tradelist **will** be found during tradelist searches.") );
    } else {
        digest.set_content( String::from("You do not have a tradelist. To add one, just use the the command '!tradelist add' followed by a quantity and card name.") );
    }
    digest
}

async fn make_private_tradelist(ctx: &Context, msg: &Message) -> Response::Response {
    println!("Making the tradelist private.");
    let mut digest: Response::Response = Response::new();
    let data = ctx.data.read().await;
    let tradelists = data.get::<Tradelist::Tradelist>().unwrap();
    if let Some(mut list) = tradelists.get_mut(&msg.author.id) {
        list.set_private();
        digest.set_content( String::from("Your tradelist has been set to private. Your tradelist **will not** be found during tradelist searches." ) );
    } else {
        digest.set_content( String::from( "You do not have a tradelist. To add one, just use the the command '!tradelist add' followed by a quantity and card name." ) );
    }
    digest
}

#[command("tradelist")]
async fn tradelist(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut digest: Response::Response = Response::new();
    let mut new_args: Args = Args::new(args.rest(), &[Delimiter::Single(' ')]);
    if let Ok(task) = new_args.single::<String>() {
        println!("Task found: {}", task);
        if task.as_str() == "" {
        } else if task.as_str() == "view" {
            digest = view_tradelist( ctx, msg ).await;
        } else if task.as_str() == "add" {
            digest = add_to_tradelist(ctx, msg, new_args).await;
        } else if task.as_str() == "remove" {
            //"remove" => remove_from_tradelist( ctx, msg, new_args ).await;
        } else if task.as_str() == "public" {
            digest = make_public_tradelist(ctx, msg).await;
        } else if task.as_str() == "private" {
            digest = make_private_tradelist(ctx, msg).await;
        } else {
            digest.set_content( String::from("You need to specify what you want to do with your tradelist.") );
        }
    } else {
        digest.set_content(String::from("You need to specify what you want to do with your tradelist.") );
    }
    Response::send_message( digest, ctx, msg ).await
}
