use diesel::{Insertable, Queryable};
use talk_loco_client::talk::chat::{Chat, ChatContent, ChatType, Chatlog};

use super::super::schema::chat;

#[derive(Debug, Insertable, Queryable, Clone, PartialEq, Eq)]
#[diesel(table_name = chat)]
pub struct ChatRow {
    pub log_id: i64,
    pub channel_id: i64,
    pub prev_log_id: Option<i64>,

    #[diesel(column_name = "type_")]
    pub chat_type: i32,

    pub message_id: i64,

    pub send_at: i64,

    pub author_id: i64,

    pub message: Option<String>,
    pub attachment: Option<String>,
    pub supplement: Option<String>,

    pub referer: Option<i32>,

    pub deleted_time: Option<i64>,
}

impl ChatRow {
    pub fn from_chatlog(log: Chatlog, deleted_time: Option<i64>) -> Self {
        Self {
            log_id: log.log_id,
            channel_id: log.channel_id,
            prev_log_id: log.prev_log_id,
            chat_type: log.chat.chat_type.0,
            message_id: log.chat.message_id,
            send_at: log.send_at,
            author_id: log.author_id,
            message: log.chat.content.message,
            attachment: log.chat.content.attachment,
            supplement: log.chat.content.supplement,
            referer: log.referer,
            deleted_time,
        }
    }
}

impl Into<Chatlog> for ChatRow {
    fn into(self) -> Chatlog {
        Chatlog {
            log_id: self.log_id,
            prev_log_id: self.prev_log_id,
            channel_id: self.channel_id,
            author_id: self.author_id,
            send_at: self.send_at,
            chat: Chat {
                chat_type: ChatType(self.chat_type),
                content: ChatContent {
                    message: self.message,
                    attachment: self.attachment,
                    supplement: self.supplement,
                },
                message_id: self.message_id,
            },
            referer: self.referer,
        }
    }
}
