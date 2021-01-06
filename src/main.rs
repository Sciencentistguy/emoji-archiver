use log::*;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use structopt::StructOpt;

const PATH_PREFIX: &str = "/dev/shm/";

#[tokio::main]
async fn main() {
    // Set default log level to info unless otherwise specified.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "emoji_archiver=info");
    }
    pretty_env_logger::init();
    let opts = Opt::from_args();
    let token = if opts.token.is_some() {
        opts.token.unwrap()
    } else if opts.token_filename.is_some() {
        std::fs::read_to_string(opts.token_filename.unwrap()).expect("File does not exist")
    } else {
        env::var("DISCORD_TOKEN")
            .expect("Expected either --token, --token-filename, or a token in the environment")
    };

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!archive" {
            let guild_name = msg.guild(&ctx.cache).await.unwrap().name;
            info!(
                "Archive started by {} in server '{}'",
                msg.author.name, guild_name
            );
            let server_id = msg.guild_id.unwrap();
            let emojis = server_id.emojis(&ctx.http).await.unwrap();
            let download_subdirectory = format!(
                "{}-{}",
                guild_name.replace(" ", "-").to_lowercase(),
                chrono::Utc::now().format("%Y-%m-%dT%H-%M-%S")
            );
            for emoji in emojis {
                let url = emoji.url();
                let name = &emoji.name;
                let ext = &url[url
                    .rfind(".")
                    .expect("Emoji url does not have a file extension")..];
                let path = format!("{}/{}/{}{}", PATH_PREFIX, download_subdirectory, name, ext);
                info!(
                    "Downloading emoji {} from url {} to location {}...",
                    name, url, path
                );
                download_url(&url, &path).await.unwrap();
            }
            msg.reply(&ctx, "Archive complete.")
                .await
                .expect("Error sending message");
            info!("Archive complete.");
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

async fn download_url<P: AsRef<Path>>(
    url: &str,
    destination: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await.unwrap();
    let mut dest = {
        let destdir = destination
            .as_ref()
            .parent()
            .expect("Destination path did not have a parent");
        if !destdir.is_dir() {
            std::fs::create_dir(destdir)?;
        };
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("<unnamed>");
        trace!("file to download: '{}'", fname);
        trace!("Will be stored in '{:?}'", destination.as_ref());
        File::create(destination.as_ref())?
    };

    let mut content = io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut dest)?;
    info!("Download complete");

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "emoji-archiver",
    about = "Discord bot to export emoji from a server. Provide the token with either --token, --token-filename, or as the environment variable DISCORD_TOKEN, in order of decreasing priority."
)]
struct Opt {
    /// Provide the token
    #[structopt(short, long)]
    token: Option<String>,
    /// Provide the name of a file containing the token
    #[structopt(short = "f", long)]
    token_filename: Option<String>,
}
