use serenity::async_trait;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{
    client::{Client, Context, EventHandler},
    prelude::TypeMapKey,
};
use std::collections::HashSet;
use std::env;

#[group]
#[commands(list_games, add_game)]
struct General;

struct Handler;

struct PendingGames;

impl TypeMapKey for PendingGames {
    type Value = HashSet<String>;
}

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let mut client =
        Client::builder(env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN has been set"))
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

    let mut set = HashSet::new();
    set.insert(String::from("World of Warcraft"));
    set.insert(String::from("Red Dead Online"));

    let mut guard = client.data.write().await;
    guard.insert::<PendingGames>(set);
    drop(guard);

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn list_games(ctx: &Context, msg: &Message) -> CommandResult {
    let guard = ctx.data.read().await;

    let result = if let Some(games) = guard.get::<PendingGames>() {
        games
            .into_iter()
            .map(String::to_owned)
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        String::from("There are currently no games in the pipeline")
    };

    msg.reply(ctx, result.as_str()).await?;

    Ok(())
}

#[command]
async fn add_game(ctx: &Context, msg: &Message) -> CommandResult {
    // let guard = ctx.data.write().await;
    let contents = msg.content.as_str();
    if contents.is_empty() {
        return Ok(());
    }
    let contents = contents[9..].trim();
    if contents.is_empty() {
        return Ok(());
    }

    let mut guard = ctx.data.write().await;
    if let Some(games) = guard.get_mut::<PendingGames>() {
        games.insert(String::from(contents));
    } else {
        let mut set = HashSet::new();
        set.insert(String::from(contents));
        guard.insert::<PendingGames>(set);
    };

    msg.reply(ctx, format!("Added {} to the game pipeline", contents))
        .await?;
    Ok(())
}
