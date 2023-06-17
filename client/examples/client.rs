use shortcutd::{ShortcutClient};
use std::error::Error;
use futures::pin_mut;
use futures::stream::iter;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShortcutClient::new().await?;

    let streams = [
        Box::pin(client.register("<Ctrl>-KeyM".parse()?).await?),
        Box::pin(client.register("<Ctrl><Alt>-KeyO".parse()?).await?),
    ];

    let stream = iter(streams).flatten_unordered(None);

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{} {}", event.shortcut, event.state.as_str());
    }
    Ok(())
}
