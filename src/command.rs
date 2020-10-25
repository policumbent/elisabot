pub use teloxide::utils::command::{BotCommand, ParseError};

pub use std::str::FromStr;

#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Queste sono le istruzioni che puoi darmi 😄:"
)]
pub enum Command {
    // comandi di default
    #[command(description = "off")]
    Start,
    #[command(description = "off")]
    Help,
    #[command(description = "off")]
    Cancel,

    #[command(
        description = "richiedi l'accesso alle api",
        parse_with = "accept_login"
    )]
    Login(Option<Login>),
    #[command(description = "display dei comandi degli admin")]
    Admin,
    #[command(description = "display i credits del bot")]
    Credits,
}
#[derive(Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn parse(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(s)
    }
}

impl FromStr for Login {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let res = s.split_ascii_whitespace().collect::<Vec<_>>();
        match res.len() {
            2 => Ok(Self {
                username: res.first().map(|s| s.to_string()).unwrap_or_default(),
                password: res.last().map(|s| s.to_string()).unwrap_or_default(),
            }),
            _ => Err(ParseError::IncorrectFormat("Invalid arguments".into())),
        }
    }
}

fn accept_login(input: String) -> Result<(Option<Login>,), ParseError> {
    match input.len() {
        0 => Ok((None,)),
        _ => Ok((Some(Login::from_str(&input)?),)),
    }
}

fn _accept_option(input: String) -> Result<(Option<String>,), ParseError> {
    match input.len() {
        0 => Ok((None,)),
        _ => Ok((Some(input),)),
    }
}
