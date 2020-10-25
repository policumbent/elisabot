use std::collections::HashMap;
use teloxide::{prelude::*, types::ParseMode};

pub struct Msg;
impl<'a> Msg {
    // Messaggi quando viene dato il comando /start
    const WELCOME_1: &'a str = "Ciao, mi chiamo ElisaBot, ma puoi chiamarmi Elisa 😊";
    const WELCOME_MD: &'a str = r"Sono la migliore amica di [ALICE](https://www.policumbent.it/alice) e quando posso mi fa piacere darle una mano 😊";
    const WELCOME_2: &'a str = "Purtroppo sono un po' smemorata 😅, quindi ogni tanto di chiederò di ripresentarti.\n\nAd ogni modo, ti sei già collegato ad Alice con /login? 😊";

    // const IDLE: &'a str = "Che facciamo di bello? Ricorda che le istruzioni sono su /help ";
    // const REFUSE_USER: &'a str =
    //     "Mi dispiace ma papà ha detto che non posso parlare con gli sconosciuti.\nAddio.";

    pub async fn welcome_msg(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
        cx.answer_str(Self::WELCOME_1).await?;
        cx.answer(Self::WELCOME_MD)
            .parse_mode(ParseMode::MarkdownV2)
            .disable_web_page_preview(true)
            .send()
            .await?;
        cx.answer_str(Self::WELCOME_2).await
    }
}

pub async fn autenticate(l: crate::command::Login) -> ResponseResult<reqwest::Response> {
    let mut map = HashMap::new();
    map.insert("username", l.username);
    map.insert("password", l.password);

    // "alice_url.xx/authenticate"
    let url = dotenv::var("SERVER_URL").expect("SERVER_URL undefined");

    reqwest::Client::new()
        .post(&url)
        .json(&map)
        .send()
        .await
        .map_err(|e| RequestError::NetworkError(e))
}
