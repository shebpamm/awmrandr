mod awesome;

use crate::awesome::Awesome;
use anyhow::Result;
use zbus::Connection;

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    let awesome = Awesome::new(&connection).await?;

    let screens = awesome.get_screens().await?;
    for screen in screens {
        let tag_count = screen.get_tag_count().await?;

        println!("There are {} tags on screen {}", tag_count, screen.index);

        let tags = screen.get_tags().await?;

        for tag in tags {
            let clients = tag.get_clients().await?;

            for client in clients {
                println!("Client {} on tag {}", client.get_class().await?, tag.index);
            }
        }
    }

    Ok(())
}
