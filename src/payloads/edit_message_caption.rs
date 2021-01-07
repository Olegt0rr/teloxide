// This file is auto generated by `cg` <https://github.com/teloxide/cg> (be02d84).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InlineKeyboardMarkup, Message, ParseMode};

impl_payload! {
    /// Use this method to edit captions of messages. On success, the edited Message is returned.
    ///
    /// See also: [`EditMessageCaptionInline`](crate::payloads::EditMessageCaptionInline)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageCaption (EditMessageCaptionSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: ChatId [into],
            /// Identifier of the message to edit
            pub message_id: i32,
        }
        optional {
            /// New caption of the message, 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
