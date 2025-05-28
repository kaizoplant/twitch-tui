use std::str::FromStr;

use color_eyre::Result;

use crate::twitch::commands::TwitchCommand;

#[test]
fn test_clear_command_enum_from_str() -> Result<()> {
    let raw_command = "clear";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(enum_command, TwitchCommand::Clear);

    Ok(())
}

#[test]
#[should_panic(expected = "No username in ban command")]
fn test_ban_command_from_str_no_username() {
    let raw_command = "ban";
    TwitchCommand::from_str(raw_command).expect("No username in ban command");
}

#[test]
fn test_ban_command_from_str_no_reason() -> Result<()> {
    let raw_command = "ban asdf";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(enum_command, TwitchCommand::Ban("asdf".to_string(), None));

    Ok(())
}

#[test]
fn test_ban_command_from_str_some_reason() -> Result<()> {
    let raw_command = "ban asdf user was being very not good";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(
        enum_command,
        TwitchCommand::Ban(
            "asdf".to_string(),
            Some("user was being very not good".to_string())
        )
    );

    Ok(())
}

#[test]
#[should_panic(expected = "No username in timeout command")]
fn test_timeout_command_from_str_no_username() {
    let raw_command = "timeout";
    TwitchCommand::from_str(raw_command).expect("No username in timeout command");
}

#[test]
#[should_panic(expected = "Timeout duration could not be parsed")]
fn test_timeout_command_from_str_invalid_timeout_duration() {
    let raw_command = "timeout asdf asdf";
    TwitchCommand::from_str(raw_command).expect("Timeout duration could not be parsed");
}

#[test]
fn test_timeout_command_from_str_no_reason() -> Result<()> {
    let raw_command = "timeout asdf 60";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(
        enum_command,
        TwitchCommand::Timeout("asdf".to_string(), 60, None)
    );

    Ok(())
}

#[test]
fn test_timeout_command_from_str_some_reason() -> Result<()> {
    let raw_command = "timeout asdf 60 not a good user";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(
        enum_command,
        TwitchCommand::Timeout("asdf".to_string(), 60, Some("not a good user".to_string()))
    );

    Ok(())
}

#[test]
#[should_panic(expected = "No username in unban command")]
fn test_unban_command_from_str_no_username() {
    let raw_command = "unban";
    TwitchCommand::from_str(raw_command).expect("No username in unban command");
}

#[test]
fn test_unban_command_from_str() -> Result<()> {
    let raw_command = "unban asdf";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(enum_command, TwitchCommand::Unban("asdf".to_string()));

    Ok(())
}

#[test]
#[should_panic(expected = "No username in raid command")]
fn test_raid_command_from_str_no_username() {
    let raw_command = "raid";
    TwitchCommand::from_str(raw_command).expect("No username in raid command");
}

#[test]
fn test_raid_command_from_str() -> Result<()> {
    let raw_command = "raid asdf";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(enum_command, TwitchCommand::Raid("asdf".to_string()));

    Ok(())
}

#[test]
fn test_unraid_command_enum_from_str() -> Result<()> {
    let raw_command = "unraid";
    let enum_command = TwitchCommand::from_str(raw_command)?;

    assert_eq!(enum_command, TwitchCommand::Unraid);

    Ok(())
}

//TODO more commands to test

#[test]
#[should_panic(expected = "Command is not supported")]
fn test_unsupported_command() {
    let raw_command = "unsupported_command";
    TwitchCommand::from_str(raw_command).expect("Command is not supported");
}
