use anyhow::Result;
use chrono::{DateTime, Utc};
use iced::Length::Fill;
use iced::widget::scrollable::{Direction, Id, Scrollbar};
use iced::widget::{column, rich_text, scrollable, span};
use iced::{Element, Task, color};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::Mutex;
use youtube_chat::item::MessageItem;
use youtube_chat::live_chat::{Empty, LiveChatClient, LiveChatClientBuilder};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub id: String,
    pub author: String,
    pub message: String,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Tick,
}

pub struct App {
    messages: Arc<RwLock<Vec<ChatMessage>>>,
    client: Arc<Mutex<LiveChatClient<Empty, Empty, Empty, Empty>>>,
    scrollable_id: Id,
}
impl App {
    pub async fn try_new(url: impl AsRef<str>) -> Result<Self> {
        let messages = Arc::new(RwLock::new(Vec::<ChatMessage>::new()));
        let client = Arc::new(tokio::sync::Mutex::new({
            let mut client = LiveChatClientBuilder::new().url(url)?.build();
            client.start().await?;
            client
        }));
        let scrollable_id = Id::unique();

        Ok(Self {
            messages,
            client,
            scrollable_id,
        })
    }

    pub fn update(&mut self, msg: AppMessage) -> Task<AppMessage> {
        let client = self.client.clone();
        let messages_arc = self.messages.clone();
        let scrollable_id = self.scrollable_id.clone();

        match msg {
            AppMessage::Tick => Task::batch(vec![
                Task::future(async move {
                    let chats = {
                        let mut guard = client.lock().await;
                        match guard.fetch().await {
                            Ok(chats) => chats,
                            Err(err) => {
                                tracing::error!("{err}");
                                return AppMessage::Tick;
                            }
                        }
                    };

                    let chats: Vec<ChatMessage> = chats
                        .into_iter()
                        .map(|chat| {
                            let parts: Vec<String> = chat
                                .message
                                .into_iter()
                                .map(|v| match v {
                                    MessageItem::Emoji(emoji) => {
                                        emoji.emoji_text.unwrap_or(String::new())
                                    }
                                    MessageItem::Text(text) => text,
                                })
                                .collect();
                            ChatMessage {
                                id: chat.id,
                                author: chat.author.name.unwrap_or("Unknow".to_string()),
                                message: parts.join(" "),
                                timestamp: chat.timestamp,
                            }
                        })
                        .collect();

                    if !chats.is_empty() {
                        match messages_arc.write() {
                            Ok(mut guard) => {
                                guard.extend(chats);
                                if guard.len() > 50 {
                                    let overflow = guard.len() - 50;
                                    guard.drain(0..overflow);
                                }
                            }
                            Err(err) => {
                                tracing::error!("{err}");
                            }
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(500)).await;
                    AppMessage::Tick
                }),
                scrollable::snap_to(scrollable_id, scrollable::RelativeOffset::END),
            ]),
        }
    }

    pub fn view(&'_ self) -> Element<'_, AppMessage> {
        let messages_elements: Vec<Element<_>> = if let Ok(messages) = self.messages.read() {
            messages
                .iter()
                .map(|msg| {
                    rich_text![
                        span(format!("[{}]", msg.author)).color(color!(0xff907f)),
                        " ",
                        span(msg.message.clone()),
                    ]
                    .size(13)
                    .into()
                })
                .collect()
        } else {
            tracing::error!("Cannot read messages");
            Vec::new()
        };

        scrollable(column(messages_elements))
            .id(self.scrollable_id.clone())
            .width(Fill)
            .height(Fill)
            .spacing(10)
            .direction(Direction::Vertical(
                Scrollbar::new().width(0).scroller_width(0),
            ))
            .into()
    }
}
