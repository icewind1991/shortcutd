use clap::Parser;
use evdev_shortcut::Shortcut;
use futures::pin_mut;
use futures::stream::iter;
use futures::StreamExt;
use shortcutd::ShortcutClient;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Shortcut to listen to
    shortcuts: Vec<Shortcut>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = ShortcutClient::new().await?;

    let stream = iter(args.shortcuts)
        .then(|shortcut| async { Box::pin(client.listen(shortcut).await.unwrap()) })
        .flatten_unordered(None);

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{} {}", event.shortcut, event.state.as_str());
    }
    Ok(())
}
