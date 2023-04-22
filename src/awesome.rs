use anyhow::Result;
use std::num::ParseIntError;
use thiserror::Error;
use zbus::{dbus_proxy, Connection};

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Failed to parse screen count")]
    CountParseError(#[from] ParseIntError),
    #[error("DBUS connection failure")]
    DBusConnectionError(#[from] zbus::Error),
}

#[dbus_proxy(
    interface = "org.awesomewm.awful.Remote",
    default_service = "org.awesomewm.awful",
    default_path = "/"
)]
trait Commander {
    async fn eval(&self, code: &str) -> Result<String, EvalError>;
}

#[derive(Debug)]
pub struct Awesome<'a> {
    proxy: CommanderProxy<'a>,
}

#[derive(Debug)]
pub struct AwesomeScreen<'a> {
    pub index: u32,
    instance: &'a Awesome<'a>,
}

#[derive(Debug)]
pub struct AwesomeTag<'a> {
    pub index: u32,
    screen: &'a AwesomeScreen<'a>,
}

#[derive(Debug)]
pub struct AwesomeClient<'a> {
    pub index: u32,
    tag: &'a AwesomeTag<'a>,
}

impl Awesome<'_> {
    pub async fn new(connection: &Connection) -> Result<Awesome, EvalError> {
        let proxy = CommanderProxy::new(connection).await?;
        return Ok(Awesome { proxy });
    }
    async fn execute(&self, code: &str) -> Result<String, EvalError> {
        let formatted_query = format!("return tostring({})", code);
        return self.proxy.eval(&formatted_query).await;
    }

    pub async fn get_screen_count(&self) -> Result<u32, EvalError> {
        let screen_count = self.execute("screen:count()").await?;
        return screen_count
            .parse()
            .map_err(EvalError::CountParseError);
    }

    pub async fn get_screens(&self) -> Result<Vec<AwesomeScreen>, EvalError> {
        let screen_count = self.get_screen_count().await?;

        let mut screens = Vec::new();

        for i in 0..screen_count {
            screens.push(AwesomeScreen { index: i, instance: self });
        }

        return Ok(screens);
    }
}

impl AwesomeScreen<'_> {
    pub async fn get_tag_count(&self) -> Result<u32, EvalError> {
        let tag_count = self
            .instance
            .execute(&format!("#screen[{}].tags", self.index + 1))
            .await?;
        return tag_count
            .parse()
            .map_err(EvalError::CountParseError);
    }

    pub async fn get_tags(&self) -> Result<Vec<AwesomeTag>, EvalError> {
        let tag_count = self.get_tag_count().await?;

        let mut tags = Vec::new();

        for i in 0..tag_count {
            tags.push(AwesomeTag { index: i, screen: self });
        }

        return Ok(tags);
    }
}

impl AwesomeTag<'_> {
    pub async fn get_name(&self) -> Result<String, EvalError> {
        let tag_name = self
            .screen
            .instance
            .execute(&format!(
                "screen[{}].tags[{}].name",
                self.screen.index + 1,
                self.index + 1
            ))
            .await?;
        return Ok(tag_name);
    }
    pub async fn get_clients(&self) -> Result<Vec<AwesomeClient>, EvalError> {
        let client_count = self
            .screen
            .instance
            .execute(&format!(
                "#screen[{}].tags[{}]:clients()",
                self.screen.index + 1,
                self.index + 1
            ))
            .await?;
        let client_count = client_count
            .parse()
            .map_err(EvalError::CountParseError)?;

        let mut clients = Vec::new();

        for i in 0..client_count {
            clients.push(AwesomeClient { index: i + 1, tag: self });
        }

        return Ok(clients);
    }
}

impl AwesomeClient<'_> {
    pub async fn get_name(&self) -> Result<String, EvalError> {
        let client_name = self
            .tag
            .screen
            .instance
            .execute(&format!(
                "screen[{}].tags[{}]:clients()[{}].name",
                self.tag.screen.index + 1,
                self.tag.index + 1,
                self.index
            ))
            .await?;
        return Ok(client_name);
    }
    
    pub async fn get_x_window_id(&self) -> Result<u32, EvalError> {
        let client_name = self
            .tag
            .screen
            .instance
            .execute(&format!(
                "screen[{}].tags[{}]:clients()[{}].window",
                self.tag.screen.index + 1,
                self.tag.index + 1,
                self.index
            ))
            .await?;
        return client_name
            .parse()
            .map_err(EvalError::CountParseError);
    }

    pub async fn get_class(&self) -> Result<String, EvalError> {
        let client_name = self
            .tag
            .screen
            .instance
            .execute(&format!(
                "screen[{}].tags[{}]:clients()[{}].class",
                self.tag.screen.index + 1,
                self.tag.index + 1,
                self.index
            ))
            .await?;
        return Ok(client_name);
    }
}
