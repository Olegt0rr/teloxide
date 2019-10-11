use async_trait::async_trait;
use serde::Serialize;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

#[derive(Debug, Clone, Serialize)]
/// Use this method to send point on the map. On success, the sent [`Message`]
/// is returned.
pub struct SendLocation<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Latitude of the location
    latitude: f64,
    /// Longitude of the location
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Period in seconds for which the location will be updated
    /// (see [Live Locations](https://telegram.org/blog/live-locations)),
    /// should be between 60 and 86400.
    live_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sends the message silently. Users will receive a notification with
    /// no sound.
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendLocation<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendLocation<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendLocation",
            &self,
        )
        .await
    }
}

impl<'a> SendLocation<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn latitude<T: Into<f64>>(mut self, latitude: T) -> Self {
        self.latitude = latitude.into();
        self
    }

    pub fn longitude<T: Into<f64>>(mut self, longitude: T) -> Self {
        self.longitude = longitude.into();
        self
    }

    pub fn live_period<T: Into<i32>>(mut self, live_period: T) -> Self {
        self.live_period = Some(live_period.into());
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i32>>(mut self, val: T) -> Self {
        self.reply_to_message_id = Some(val.into());
        self
    }
}
