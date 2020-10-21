use dotenv;

use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::command::{BotCommand, ParseError},
};

#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Queste sono le istruzioni che puoi darmi 😄:"
)]
enum Command {
    #[command(description = "off")]
    Start,
    #[command(description = "display delle istruzioni")]
    Help,
    #[command(description = "presentati ad Elisa", parse_with = "accept_option")]
    Username(Option<String>),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}

fn accept_option(input: String) -> Result<(Option<String>,), ParseError> {
    match input.len() {
        0 => Ok((None,)),
        _ => Ok((Some(input),)),
    }
}

struct Msg;
impl<'a> Msg {
    // Messaggi quando viene dato il comando /start
    const WELCOME_1: &'a str = "Ciao, mi chiamo ElisaBot, ma puoi chiamarmi Elisa 😊";
    const WELCOME_MD: &'a str = r"Sono la migliore amica di [ALICE](https://www.policumbent.it/alice) e quando posso mi fa piacere darle una mano 😊";
    const WELCOME_2: &'a str = "Purtroppo sono un po' smemorata 😅, quindi ogni tanto di chiederò di ripresentarti.\n\nAd ogni modo, quale è il tuo /username? 😊";

    const IDLE: &'a str = "Che facciamo di bello? Ricorda che le istruzioni sono su /help ";
    const REFUSE_USER: &'a str =
        "Mi dispiace ma papà ha detto che non posso parlare con gli sconosciuti.\nAddio.";

    async fn welcome_msg(cx: &UpdateWithCx<Message>) -> ResponseResult<Message> {
        cx.answer_str(Self::WELCOME_1).await?;
        cx.answer(Self::WELCOME_MD)
            .parse_mode(ParseMode::MarkdownV2)
            .send()
            .await?;
        cx.answer_str(Self::WELCOME_2).await
    }
}

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting ElisaBot...");

    let bot = Bot::from_env();
    teloxide::commands_repl(bot, "ElisaBot", answer).await;
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Start => Msg::welcome_msg(&cx).await?,
        Command::Help => cx.answer_str(Command::descriptions()).await?,
        Command::Username(username) => match username {
            Some(u) => {
                let u = u.replace("@", "");

                if u == "gabelluardo" {
                    log::info!("New user added");
                    cx.answer_str(format!("È un piacere conoscerti @{} 😊", u))
                        .await?;
                    cx.answer_str(Msg::IDLE).await?
                } else {
                    cx.answer_str(Msg::REFUSE_USER).await?
                }
            }
            None => {
                cx.reply_to("Dovresti scriverlo dopo /username 😌")
                    .send()
                    .await?
            }
        },
        // Command::UsernameAndAge { username, age } => {
        //     cx.answer_str(format!(
        //         "Your username is @{} and age is {}.",
        //         username, age
        //     ))
        //     .await?
        // }
    };

    Ok(())
}
