mod client;
mod util;

use client::*;
use util::*;

use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::dialogue::{serializer::Json, ErasedStorage, SqliteStorage, Storage},
    prelude::*,
};

const DB_FILE: &str = "db.sqlite";

type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type MyDialogue = Dialogue<State, ErasedStorage<State>>;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    ReceiveToken,
    ReceiveArticleUrl {
        token: String,
    },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env();

    let storage: MyStorage = SqliteStorage::open(DB_FILE, Json).await.unwrap().erase();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(dptree::case![State::ReceiveToken].endpoint(receive_token))
        .branch(dptree::case![State::ReceiveArticleUrl { token }].endpoint(receive_article));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Привет! Для использования бота требуется указать OAuth токен. \
        Его можно сгенерировать на сайте: https://300.ya.ru/.",
    )
    .await?;
    dialogue.update(State::ReceiveToken).await?;

    Ok(())
}

async fn receive_token(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        if !is_valid_token(text) {
            bot.send_message(msg.chat.id, "Указанный токен не валиден.")
                .await?;
            dialogue.update(State::ReceiveToken).await?;
            return Ok(());
        }
        bot.send_message(msg.chat.id, "Ок.").await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        dialogue
            .update(State::ReceiveArticleUrl { token: text.into() })
            .await?;
    } else {
        bot.send_message(msg.chat.id, "Пожалуйста, укажи валидный OAuth токен.")
            .await?;
    }

    Ok(())
}

async fn receive_article(bot: Bot, _: MyDialogue, token: String, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        if !is_valid_url(text) {
            bot.send_message(msg.chat.id, "Пожалуйста, укажи валидный URL.")
                .await?;
            return Ok(());
        }
        let summary = Client::from_token(token.as_str()).get_summary(text).await;
        match summary {
            Ok(summary) => {
                let message = summary.format();
                bot.send_message(msg.chat.id, message).await?
            }
            Err(err) => {
                let error_message = format!("Произошла ошибка: {}. Попробуй другую статью.", err);
                bot.send_message(msg.chat.id, error_message).await?
            }
        };
    } else {
        bot.send_message(msg.chat.id, "Попробуй еще раз.").await?;
    }

    Ok(())
}
