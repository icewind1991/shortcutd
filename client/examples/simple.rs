use futures::{pin_mut, StreamExt};
use shortcutd::{Shortcut, ShortcutClient};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShortcutClient::new().await?;

    let shortcut: Shortcut = "<Ctrl><Alt>-KeyO".parse()?;

    let stream = client.listen(shortcut).await?;

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{} {}", event.shortcut, event.state.as_str());
    }
    Ok(())
}
