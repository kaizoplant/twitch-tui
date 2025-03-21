use std::{
    convert::Into,
    fmt::Display,
    string::{String, ToString},
    vec::Vec,
};

use color_eyre::Result;
use reqwest::Client;
use serde::Deserialize;

use super::oauth::{get_twitch_client, get_twitch_client_id};
use crate::{
    handlers::config::{CompleteConfig, TwitchConfig},
    ui::components::utils::SearchItemGetter,
};

const FOLLOWER_COUNT: usize = 100;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FollowingUser {
    pub broadcaster_login: String,
    // broadcaster_id: String,
    // broadcaster_name: String,
    // followed_at: String,
}

impl Display for FollowingUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.broadcaster_login)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StreamingUser {
    pub user_login: String,
    pub game_name: String,
    pub title: String,
}

impl Display for StreamingUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            user_login,
            game_name,
            title,
        } = self;
        let fmt_game = format!("[{game_name:.22}]");
        write!(f, "{user_login:<16.16}: {fmt_game:<24} {title}",)
    }
}

impl From<StreamingUser> for FollowingUser {
    fn from(value: StreamingUser) -> Self {
        Self {
            broadcaster_login: value.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
#[allow(dead_code)]
struct Pagination {
    cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct FollowingChannelList {
    pub total: u64,
    pub data: Vec<FollowingUser>,
    pagination: Pagination,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[allow(dead_code)]
pub struct LiveChannelList {
    pub data: Vec<StreamingUser>,
    pagination: Pagination,
}

impl From<LiveChannelList> for FollowingChannelList {
    fn from(val: LiveChannelList) -> Self {
        Self {
            total: val.data.len() as u64,
            data: val.data.into_iter().map(Into::into).collect(),
            pagination: val.pagination,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Following {
    // TODO: Don't re-create client on new requests
    // client: &Client,
    config: CompleteConfig,
    list: FollowingChannelList,
}

// https://dev.twitch.tv/docs/api/reference/#get-followed-channels
pub async fn get_user_following(
    client: &Client,
    user_id: &str,
    live: bool,
) -> Result<FollowingChannelList> {
    let channels = if live {
        let url = format!(
            "https://api.twitch.tv/helix/streams/followed?user_id={user_id}&first={FOLLOWER_COUNT}",
        );

        client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<LiveChannelList>()
            .await?
            .into()
    } else {
        let url = format!(
            "https://api.twitch.tv/helix/channels/followed?user_id={user_id}&first={FOLLOWER_COUNT}",
        );

        client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<FollowingChannelList>()
            .await?
    };

    Ok(channels)
}

pub async fn get_following(
    twitch_config: &TwitchConfig,
    live: bool,
) -> Result<FollowingChannelList> {
    let client = get_twitch_client(twitch_config.token.as_deref()).await?;
    let user_id = &get_twitch_client_id(None).await?.user_id;

    get_user_following(&client, user_id, live).await
}

impl Following {
    pub fn new(config: CompleteConfig) -> Self {
        Self {
            config,
            list: FollowingChannelList::default(),
        }
    }
}

impl SearchItemGetter<String> for Following {
    async fn get_items(&mut self) -> Result<Vec<String>> {
        let following = get_following(
            &self.config.twitch,
            self.config.frontend.only_get_live_followed_channels,
        )
        .await;

        following.map(|v| {
            v.data
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        })
    }
}
