use std::sync::Arc;

use domain::{
    ChangeEvent, ChangelogRepository, ControlMethod, ControlledContentRepository,
    ControlledContentType, RepositoryError, UserId,
};
use teloxide::{prelude::*, types::ChatJoinRequest, utils::command::BotCommands};

/// Shared handles the dispatcher injects into every handler.
type ContentRepo = Arc<dyn ControlledContentRepository + Send + Sync>;
type Changelog = Arc<dyn ChangelogRepository + Send + Sync>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "Catshark wiki information commands."
)]
pub enum Command {
    #[command(
        description = "add to the ban list: /add_ban <artist|kink|tag|character> <name> [reason]"
    )]
    AddBan(String),
    #[command(
        description = "add to the spoiler list: /add_spoiler <artist|kink|tag|character> <name> [reason]"
    )]
    AddSpoiler(String),
    #[command(description = "check whether something is controlled: /check <name>")]
    Check(String),
}

/// Build and run the dispatcher until the process is stopped.
pub async fn run(bot: Bot, content_repo: ContentRepo, changelog: Changelog) {
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(answer),
        )
        .branch(Update::filter_chat_join_request().endpoint(greet_join_request));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![content_repo, changelog])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    content_repo: ContentRepo,
    changelog: Changelog,
) -> ResponseResult<()> {
    let reply = match cmd {
        Command::AddBan(args) => {
            control(
                &bot,
                &msg,
                &content_repo,
                &changelog,
                ControlMethod::Banned,
                &args,
            )
            .await
        }
        Command::AddSpoiler(args) => {
            control(
                &bot,
                &msg,
                &content_repo,
                &changelog,
                ControlMethod::Spoilered,
                &args,
            )
            .await
        }
        Command::Check(query) => check(&content_repo, &query).await,
    };

    bot.send_message(msg.chat.id, reply).await?;
    Ok(())
}

/// Handle a `chat_join_request` update: DM the requester the joining checklist.
///
/// Telegram lets the bot message `user_chat_id` for ~5 minutes after the
/// request (until it is processed), which is exactly the window where the
/// checklist helps. Delivery requires the bot to be an admin of a chat with
/// "Approve New Members" enabled. The request itself is left untouched;
/// age verification is a human's call, so approval stays with the admins.
async fn greet_join_request(bot: Bot, request: ChatJoinRequest) -> ResponseResult<()> {
    let text = welcome_message(
        &request.from.first_name,
        request.chat.title().unwrap_or("the network"),
        &wiki_url(),
    );

    // Best-effort: the DM window may already be gone (request processed or
    // user privacy settings); that must not take the dispatcher down.
    if let Err(error) = bot.send_message(request.user_chat_id, text).await {
        eprintln!("Could not DM join requester {}: {error}", request.from.id.0);
    }
    Ok(())
}

/// Where the wiki lives, for links in bot messages. Overridable so deploys
/// can point at the real site once it has a domain.
fn wiki_url() -> String {
    std::env::var("WIKI_URL")
        .unwrap_or_else(|_| "https://github.com/CatShark-TreeHouse/wiki".to_owned())
}

/// The checklist DM sent to someone requesting to join. Self-contained on
/// purpose: the steps are spelled out inline so the message stays useful
/// even if the reader never opens the link.
fn welcome_message(first_name: &str, chat_title: &str, wiki_url: &str) -> String {
    format!(
        "Hi {first_name}! You've requested to join {chat_title}, welcome to the \
CatShark TreeHouse network!\n\
\n\
While an admin reviews your request, here's what you need to know:\n\
\n\
1. Every space in the network is strictly 18+, SFW and NSFW alike.\n\
2. Read the network rules first; admins are instructed not to reply unless \
you acknowledge them.\n\
3. To be let in, you'll verify your age with an admin using an ID that shows \
your date of birth (a driving license or national ID). We never store it.\n\
4. Once verified, you're set for every chat in the network, no need to do \
this again.\n\
\n\
Rules, the staff list, and the full joining guide: {wiki_url}\n\
\n\
See you in the TreeHouse!"
    )
}

/// Handle `/add_ban` and `/add_spoiler`: admin-gate, control the content, emit a changelog event.
async fn control(
    bot: &Bot,
    msg: &Message,
    content_repo: &ContentRepo,
    changelog: &Changelog,
    method: ControlMethod,
    args: &str,
) -> String {
    let Some(user) = msg.from.as_ref() else {
        return "Could not identify the sender.".to_owned();
    };

    if !is_admin(bot, msg.chat.id, user.id).await {
        return "Only chat admins can control content.".to_owned();
    }

    let (content_type, name, reason) = match parse_control_args(args) {
        Ok(parsed) => parsed,
        Err(usage) => return usage,
    };

    let verb = match method {
        ControlMethod::Banned => "banned",
        ControlMethod::Spoilered => "spoilered",
    };

    match content_repo
        .control(name.clone(), content_type, method, reason)
        .await
    {
        Ok(content) => {
            // Best-effort audit trail; a changelog failure should not undo the control.
            let _ = changelog
                .emit(ChangeEvent::Added, content.id, UserId::new(user.id.0))
                .await;
            format!("{verb} {} ({}).", name, type_label(content_type))
        }
        Err(RepositoryError::Conflict(alias)) => {
            format!("{alias} is already controlled.")
        }
        Err(_) => "Something went wrong while controlling that.".to_owned(),
    }
}

/// Handle `/check`: report whether the alias is banned, spoilered, or uncontrolled.
async fn check(content_repo: &ContentRepo, query: &str) -> String {
    let alias = query.trim();
    if alias.is_empty() {
        return "Usage: /check <name>".to_owned();
    }

    match content_repo.find_controlled(alias.to_owned()).await {
        Ok(Some(content)) => {
            let status = match content.control_method {
                ControlMethod::Banned => "BANNED",
                ControlMethod::Spoilered => "SPOILERED",
            };
            let mut out = format!(
                "{alias} is {status} ({}).",
                type_label(content.content_type)
            );
            if let Some(reason) = content.reason {
                out.push_str(&format!(" Reason: {reason}"));
            }
            out
        }
        Ok(None) => format!("{alias} is not controlled."),
        Err(_) => "Something went wrong while checking that.".to_owned(),
    }
}

/// Split off the first whitespace-delimited word, returning it and the remainder
/// with leading whitespace trimmed. Tolerates runs of whitespace between tokens.
fn split_first_word(input: &str) -> (&str, &str) {
    let input = input.trim_start();
    match input.find(char::is_whitespace) {
        Some(index) => (&input[..index], input[index..].trim_start()),
        None => (input, ""),
    }
}

/// `<type> <name> [reason...]`: type and name are single tokens, the rest is the reason.
/// Extra whitespace between tokens is tolerated; whitespace inside the reason is preserved.
fn parse_control_args(
    args: &str,
) -> Result<(ControlledContentType, String, Option<String>), String> {
    const USAGE: &str = "Usage: <artist|kink|tag|character> <name> [reason]";

    let (type_token, rest) = split_first_word(args);
    if type_token.is_empty() {
        return Err(USAGE.to_owned());
    }
    let content_type = parse_content_type(type_token).ok_or_else(|| USAGE.to_owned())?;

    let (name, rest) = split_first_word(rest);
    if name.is_empty() {
        return Err(USAGE.to_owned());
    }

    let reason = match rest.trim() {
        "" => None,
        reason => Some(reason.to_owned()),
    };

    Ok((content_type, name.to_owned(), reason))
}

fn parse_content_type(token: &str) -> Option<ControlledContentType> {
    match token.to_lowercase().as_str() {
        "artist" => Some(ControlledContentType::Artist),
        // "kink" is the vocabulary moderators use for controlled tags.
        "tag" | "kink" => Some(ControlledContentType::Tag),
        "character" => Some(ControlledContentType::Character),
        _ => None,
    }
}

fn type_label(content_type: ControlledContentType) -> &'static str {
    match content_type {
        ControlledContentType::Artist => "artist",
        ControlledContentType::Tag => "tag",
        ControlledContentType::Character => "character",
    }
}

/// True if `user` is an administrator (or creator) of `chat`.
async fn is_admin(bot: &Bot, chat: ChatId, user: teloxide::types::UserId) -> bool {
    match bot.get_chat_administrators(chat).await {
        Ok(admins) => admins.iter().any(|member| member.user.id == user),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use persistence::in_memory::InMemoryControlledContentRepository;

    fn repo() -> ContentRepo {
        Arc::new(InMemoryControlledContentRepository::new())
    }

    #[test]
    fn parses_type_name_and_reason() {
        let (ty, name, reason) = parse_control_args("artist Zaush draws underage").unwrap();
        assert_eq!(ty, ControlledContentType::Artist);
        assert_eq!(name, "Zaush");
        assert_eq!(reason.as_deref(), Some("draws underage"));
    }

    #[test]
    fn reason_is_optional() {
        let (ty, name, reason) = parse_control_args("tag vore").unwrap();
        assert_eq!(ty, ControlledContentType::Tag);
        assert_eq!(name, "vore");
        assert_eq!(reason, None);
    }

    #[test]
    fn content_type_is_case_insensitive() {
        let (ty, _, _) = parse_control_args("CHARACTER GearFox").unwrap();
        assert_eq!(ty, ControlledContentType::Character);
    }

    #[test]
    fn rejects_unknown_type() {
        assert!(parse_control_args("artistt foo").is_err());
    }

    #[test]
    fn rejects_missing_name() {
        assert!(parse_control_args("artist").is_err());
        assert!(parse_control_args("   ").is_err());
    }

    #[test]
    fn reason_whitespace_is_trimmed_and_collapsed() {
        let (_, name, reason) = parse_control_args("tag  vore    a long   reason").unwrap();
        assert_eq!(name, "vore");
        assert_eq!(reason.as_deref(), Some("a long   reason"));
    }

    #[test]
    fn parse_content_type_variants() {
        assert!(matches!(
            parse_content_type("artist"),
            Some(ControlledContentType::Artist)
        ));
        assert!(matches!(
            parse_content_type("TAG"),
            Some(ControlledContentType::Tag)
        ));
        assert!(matches!(
            parse_content_type("Character"),
            Some(ControlledContentType::Character)
        ));
        assert!(matches!(
            parse_content_type("kink"),
            Some(ControlledContentType::Tag)
        ));
        assert!(parse_content_type("kinks").is_none());
    }

    #[test]
    fn kink_parses_as_tag_in_full_command_args() {
        let (ty, name, reason) = parse_control_args("kink Vore rare kink").unwrap();
        assert_eq!(ty, ControlledContentType::Tag);
        assert_eq!(name, "Vore");
        assert_eq!(reason.as_deref(), Some("rare kink"));
    }

    #[test]
    fn type_label_roundtrips_through_parse() {
        for ty in [
            ControlledContentType::Artist,
            ControlledContentType::Tag,
            ControlledContentType::Character,
        ] {
            assert_eq!(parse_content_type(type_label(ty)), Some(ty));
        }
    }

    #[test]
    fn welcome_message_contains_the_essentials() {
        let msg = welcome_message("Fen", "Zuri Cat Tree", "https://wiki.example");
        assert!(msg.contains("Hi Fen!"), "greets by first name: {msg}");
        assert!(msg.contains("Zuri Cat Tree"), "names the chat: {msg}");
        assert!(msg.contains("18+"), "states the age policy: {msg}");
        assert!(msg.contains("date of birth"), "explains ID check: {msg}");
        assert!(
            msg.contains("https://wiki.example"),
            "links the wiki: {msg}"
        );
    }

    #[test]
    fn wiki_url_defaults_to_the_repo() {
        // WIKI_URL is not set in the test environment.
        assert!(wiki_url().starts_with("https://github.com/CatShark-TreeHouse"));
    }

    #[tokio::test]
    async fn check_reports_not_controlled() {
        let repo = repo();
        assert!(check(&repo, "anything").await.contains("not controlled"));
    }

    #[tokio::test]
    async fn check_empty_query_shows_usage() {
        let repo = repo();
        assert!(check(&repo, "   ").await.contains("Usage"));
    }

    #[tokio::test]
    async fn check_reports_banned_with_type_and_reason() {
        let repo = repo();
        repo.control(
            "Zaush".into(),
            ControlledContentType::Artist,
            ControlMethod::Banned,
            Some("cub art".into()),
        )
        .await
        .unwrap();

        let msg = check(&repo, "Zaush").await;
        assert!(msg.contains("BANNED"), "got: {msg}");
        assert!(msg.contains("artist"), "got: {msg}");
        assert!(msg.contains("cub art"), "got: {msg}");
    }

    #[tokio::test]
    async fn check_reports_spoilered() {
        let repo = repo();
        repo.control(
            "Vore".into(),
            ControlledContentType::Tag,
            ControlMethod::Spoilered,
            None,
        )
        .await
        .unwrap();
        assert!(check(&repo, "Vore").await.contains("SPOILERED"));
    }
}
