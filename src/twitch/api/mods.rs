use color_eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::TWITCH_API_BASE_URL;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModQuery {
    user_id: String,
    broadcaster_id: String,
}

impl ModQuery {
    pub const fn new(user_id: String, broadcaster_id: String) -> Self {
        Self {
            user_id,
            broadcaster_id,
        }
    }
}

///TODO doc
pub async fn mod_twitch_user(client: &Client, query: ModQuery) -> Result<()> {
    let url = format!("{TWITCH_API_BASE_URL}/moderation/moderators");

    let mod_query = &[
        ("user_id", query.user_id),
        ("broadcaster_id", query.broadcaster_id),
    ];

    client
        .post(url)
        .query(mod_query)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

///TODO doc
pub async fn unmod_twitch_user(client: &Client, query: ModQuery) -> Result<()> {
    let url = format!("{TWITCH_API_BASE_URL}/moderation/moderators");

    let unmod_query = &[
        ("user_id", query.user_id),
        ("broadcaster_id", query.broadcaster_id),
    ];

    client
        .delete(url)
        .query(unmod_query)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
