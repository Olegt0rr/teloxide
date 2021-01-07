// This file is auto generated by `cg` <https://github.com/teloxide/cg> (be02d84).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{InputFile, MaskPosition, True};

impl_payload! {
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You must use exactly one of the fields _png\_sticker_ or _tgs\_sticker_. Returns _True_ on success.
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub CreateNewStickerSet (CreateNewStickerSetSetters) => True {
        required {
            /// User identifier of sticker file owner
            pub user_id: i32,
            /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., _animals_). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in _“\_by\_<bot username>”. <bot\_username>_ is case insensitive. 1-64 characters.
            pub name: String [into],
            /// Sticker set title, 1-64 characters
            pub title: String [into],
            /// One or more emoji corresponding to the sticker
            pub emojis: String [into],
        }
        optional {
            /// **PNG** image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a _file\_id_ as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub png_sticker: InputFile,
            /// **TGS** animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
            pub tgs_sticker: InputFile,
            /// Pass _True_, if a set of mask stickers should be created
            pub contains_masks: bool,
            /// A JSON-serialized object for position where the mask should be placed on faces
            pub mask_position: MaskPosition,
        }
    }
}
