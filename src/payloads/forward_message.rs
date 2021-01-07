// This file is auto generated by `cg` <https://github.com/teloxide/cg> (be02d84).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, Message};

impl_payload! {
    /// Use this method to forward messages of any kind. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub ForwardMessage (ForwardMessageSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
            pub from_chat_id: ChatId [into],
            /// Message identifier in the chat specified in _from\_chat\_id_
            pub message_id: i32,
        }
        optional {
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
        }
    }
}
