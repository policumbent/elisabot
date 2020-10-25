// mod callback;
mod command;
mod utils;

use derive_more::From;
use dotenv;
use teloxide::prelude::*;
use teloxide_macros::{teloxide, Transition};

use crate::command::{BotCommand, Command, Login};

type Cx = UpdateWithCx<Message>;
type In = DialogueWithCx<Message, Dialogue, std::convert::Infallible>;
type Out = TransitionOut<Dialogue>;

// struct _1State;
// struct _2State;
// struct _3State;

// #[teloxide(subtransition)]
// async fn _1_transition(_state: _1State, _cx: TransitionIn, _ans: String) -> Out {
//     _cx.answer_str("state 1").await?;
//     next(_2State)
// }

// #[teloxide(subtransition)]
// async fn _2_transition(_state: _2State, _cx: TransitionIn, _ans: String) -> Out {
//     _cx.answer_str("state 2").await?;
//     next(_3State)
// }

// #[teloxide(subtransition)]
// async fn _3_transition(_state: _3State, _cx: TransitionIn, _ans: String) -> Out {
//     _cx.answer_str("state 3").await?;
//     exit()
// }

#[teloxide(subtransition)]
async fn login_transition(_state: LoginState, cx: TransitionIn, ans: String) -> Out {
    let l = Login::parse(&ans).ok();
    cx.answer_str(format!("cred: {:#?}", l)).await?;

    answer_autentication(cx, l).await?;
    exit()
}

async fn answer_autentication(cx: Cx, l: Option<Login>) -> ResponseResult<Message> {
    match l {
        None => cx.answer_str("Credenziali non valide").await,
        Some(l) => match utils::autenticate(l).await {
            Ok(_) => cx.answer_str("Adesso sei autenticato!").await,
            _ => cx.answer_str("Impossibile autenticarsi").await,
        },
    }
}

#[teloxide(subtransition)]
async fn idle_transision(_state: IdleState, cx: TransitionIn, _ans: String) -> Out {
    cx.answer_str(Command::descriptions()).await?;
    exit()
}

struct IdleState;
struct LoginState;

#[derive(Transition, From)]
enum Dialogue {
    IdleState(IdleState),
    LoginState(LoginState),
    // _1(_1State),
    // _2(_2State),
    // _3(_3State),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::IdleState(IdleState)
    }
}

impl Dialogue {
    fn login_start() -> Self {
        Self::LoginState(LoginState)
    }
}

#[tokio::main]
async fn main() {
    run().await
}

// .messages_handler(|rx: DispatcherHandlerRx<Message>| {
//     rx.for_each_concurrent(None, |cx| async move {
//         handle_message(cx)
//             .await
//             .expect("Something wrong with the bot!");
//     })
// })
// .callback_queries_handler(|rx: DispatcherHandlerRx<CallbackQuery>| {
//     rx.for_each_concurrent(None, |cx| async move {
//         callback::handle_message(cx)
//             .await
//             .expect("Something wrong with the bot!");
//     })
// })

async fn run() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting ElisaBot...");

    let bot = Bot::from_env();
    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |DialogueWithCx { cx, dialogue }: In| async move {
                handle_message(cx, dialogue.unwrap())
                    .await
                    .expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}

/* Quando viene passato un nuovo comando si esce dal
 * dialogo in corso e si esegue il comando.
 * Se il comando necessita di un dialogo, inizia.
 *
 * `/cancel` per uscire dal dialogo in qualsiasi momento
 */
async fn handle_message(cx: Cx, dialogue: Dialogue) -> Out {
    match Command::parse(
        cx.update.text().unwrap(),
        dotenv::var("BOT_NAME").expect("BOT_NAME undefined"),
    ) {
        // viene selezionato un comando
        Ok(c) => match c {
            Command::Cancel => {
                cx.answer_str("Operazione annullata").await?;
                exit()
            }
            Command::Start => {
                utils::Msg::welcome_msg(cx).await?;
                exit()
            }
            Command::Help => {
                cx.answer_str(Command::descriptions()).await?;
                exit()
            }
            Command::Login(login) => match login {
                Some(_) => {
                    answer_autentication(cx, login).await?;
                    exit()
                }
                None => {
                    cx.answer_str("Credenziali").await?;
                    next(Dialogue::login_start())
                }
            },
            Command::Admin => {
                // TODO: Aggiungere i comandi degli admin
                cx.answer_str("todo: comandi degli admin").await?;
                exit()
            }
            Command::Credits => {
                // TODO: Aggiungere i credits
                cx.answer_str("todo: chi ha lavorato al bot").await?;
                exit()
            }
        },

        // continua un dialogo in corso
        _ => {
            let ans = cx.update.text_owned().unwrap();
            dialogue.react(cx, ans).await
        }
    }
}

// let btn = InlineKeyboardButton::callback(
//     "Login".to_string(),
//     "Ok mandami le credenziali".to_string(),
// );
// let keyboard = InlineKeyboardMarkup::default().append_row(vec![btn]);

// cx.answer("testo messaggio")
//     .reply_markup(keyboard)
//     .send()
//     .await?

// Command::Username(username) => match username {
//     Some(u) => {
//         let u = u.replace("@", "");

//         if u == "gabelluardo" {
//             log::info!("New user added");
//             cx.answer_str(format!("È un piacere conoscerti @{} 😊", u))
//                 .await?;
//             cx.answer_str(Msg::IDLE).await?
//         } else {
//             cx.answer_str(Msg::REFUSE_USER).await?
//         }
//     }
//     None => {
//         let login_url = LoginUrl::new("google.com".to_string());

//         let button = InlineKeyboardButton::new(
//             "text",
//             InlineKeyboardButtonKind::LoginUrl(login_url),
//         );
//         let keyboard = InlineKeyboardMarkup::default().append_row(vec![button]);
//         cx.answer("prova").reply_markup(keyboard).send().await?
//     }
// },
