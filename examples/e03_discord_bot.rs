use poise::serenity_prelude as serenity;
use cleverbotfreeapi::{Cleverbot, CleverbotBuilder};
use reqwest;
use dotenvy::dotenv;

// so that the cleverbot instance in data accessible in commands and the event handler too
struct Data {
    cleverbot: Cleverbot
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = reqwest::Client::new();

    // setting up the cleverbot instance
    let cleverbot = CleverbotBuilder::default()
        .with_client(client)  // can be useful if the client is reused for other things like fetching from an api
        .build()
        .await
        .expect("should never fail");

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![history()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                mention_as_prefix: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // storing the cleverbot instance in data accessible in commands
                Ok(Data { cleverbot })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}


async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {

            // if mentioned, we want the bot to respond
            if new_message.mentions_me(ctx).await? {
                let response = data.cleverbot.get_response(&new_message.content).await?;
                new_message.reply(ctx, &response).await?;
            }

        }
        _ => {}
    }
    Ok(())
}


/// Displays the chat history (you shouldnt have such a command becuase of privacy, oh well, too bad!)
#[poise::command(slash_command, prefix_command)]
async fn history(ctx: Context<'_>) -> Result<(), Error> {
    let history = ctx.data().cleverbot.get_history();
    let history_str = format!("{:?}", history);

    // 1800 so that the msg isn't too long to be sent in discord
    let truncated_history_str = if history_str.len() > 1800 {
        history_str.chars().take(1800).collect::<String>() + "..."
    } else {
        history_str
    };

    ctx.say(format!("Messaging history: \n```{}```", truncated_history_str)).await?;
    Ok(())
}