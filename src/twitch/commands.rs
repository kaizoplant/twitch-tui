use std::str::FromStr;

use color_eyre::eyre::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum TwitchCommand {
    /// Clear the chat
    Clear,
    /// Ban username with an optional reason
    Ban(String, Option<String>),
    /// Timeout for username, duration in seconds, and optional reason
    Timeout(String, usize, Option<String>),
    /// Unban username
    Unban(String),
    /// Raid a username
    Raid(String),
    /// Cancel a raid
    Unraid,
    /// TODO doc
    Followers(Option<usize>),
    FollowersOff,
    Slow(usize),
    SlowOff,
    Subscribers,
    SubscribersOff,
    EmoteOnly,
    EmoteOnlyOff,

    Title(String),
    Category(String),
}

impl TwitchCommand {
    fn handle_ban_command(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [username] => Ok(Self::Ban((*username).to_string(), None)),
            [username, ban_reason @ ..] => {
                let reason = ban_reason.join(" ");

                Ok(Self::Ban((*username).to_string(), Some(reason)))
            }
            _ => bail!("Invalid ban command arguments"),
        }
    }

    fn handle_timeout_command(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [username, timeout_duration] => {
                let duration = timeout_duration.parse::<usize>()?;

                Ok(Self::Timeout((*username).to_string(), duration, None))
            }
            [username, timeout_duration, timeout_reason @ ..] => {
                let duration = timeout_duration.parse::<usize>()?;
                let reason = timeout_reason.join(" ");

                Ok(Self::Timeout(
                    (*username).to_string(),
                    duration,
                    Some(reason),
                ))
            }
            _ => bail!("Invalid timeout command arguments"),
        }
    }

    fn handle_unban_command(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [username] => Ok(Self::Unban((*username).to_string())),
            _ => bail!("Invalid unban command arguments"),
        }
    }

    fn handle_raid_command(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [username] => Ok(Self::Raid((*username).to_string())),
            _ => bail!("Invalid raid command arguments"),
        }
    }
    fn handle_followers_command(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [followed_duration] => {
                let duration = followed_duration.parse::<usize>()?;

                Ok(Self::Followers(Some(duration)))
            }
            [] => Ok(Self::Followers(None)),
            _ => bail!("Invalid followers command arguments"),
        }
    }
    fn handle_slow_commnad(args: &[&str]) -> Result<Self, Error> {
        match args.iter().as_slice() {
            [slow_duration] => {
                let duration = slow_duration.parse::<usize>()?;

                Ok(Self::Slow(duration))
            }
            _ => bail!("Invalid slow command arguments"),
        }
    }
    fn handle_title_command(args: &[&str]) -> Self {
        let title = args.join(" ");
        Self::Title(title)
    }
    fn handle_category_command(args: &[&str]) -> Self {
        let game_name = args.join(" ");
        Self::Category(game_name)
    }
}

impl FromStr for TwitchCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().to_lowercase();

        let cmd = match parts.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["clear"] => Self::Clear,
            ["ban", args @ ..] => Self::handle_ban_command(args)?,
            ["unban", args @ ..] => Self::handle_unban_command(args)?,
            ["timeout", args @ ..] => Self::handle_timeout_command(args)?,
            ["raid", args @ ..] => Self::handle_raid_command(args)?,
            ["unraid"] => Self::Unraid,
            ["followers", args @ ..] => Self::handle_followers_command(args)?,
            ["followersoff"] => Self::FollowersOff,
            ["slow", args @ ..] => Self::handle_slow_commnad(args)?,
            ["slowoff"] => Self::SlowOff,
            ["subscribers"] => Self::Subscribers,
            ["subscribersoff"] => Self::SubscribersOff,
            ["emoteonly"] => Self::EmoteOnly,
            ["emoteonlyoff"] => Self::EmoteOnlyOff,
            ["title", args @ ..] => Self::handle_title_command(args),
            ["category", args @ ..] => Self::handle_category_command(args),
            _ => bail!("Twitch command {} is not supported", s),
        };

        Ok(cmd)
    }
}
