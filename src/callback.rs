use teloxide::{
    prelude::*,
    types::{
        CallbackQuery, ChatId, ChatOrInlineMessage, InlineKeyboardButton, InlineKeyboardMarkup,
    },
};

#[derive(Copy, Clone)]
enum CallbackCommand {
    UnknownCommand,
}

pub async fn handle_message(cx: UpdateWithCx<CallbackQuery>) -> ResponseResult<()> {
    let query = &cx.update;
    let query_id = &query.id;
    let chat_id = query.message.clone().unwrap().chat_id();

    // let user_id = query.from.id;

    match &query.data {
        Some(data) => {
            let _ = cx
                .bot
                .answer_callback_query(query_id)
                .text(format!("callback data: {}", data.to_string()))
                .send()
                .await;

            let _ = cx
                .bot
                .send_message(chat_id, "manda le credenziali")
                .send()
                .await;

            Ok(())
        }
        None => Ok(()),
    }
}
