//! # The `about` Module
//!
//! This module implements the `about` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            EmbedFieldBuilder,
            ImageSource
        },
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            interaction::Interaction
        }

    },
    error::{
        HarTexError,
        HarTexResult
    },
    HARTEX_BUILD
};

use hartex_dbmani::whitelist::GetWhitelistedGuilds;

use hartex_utils::FutureRetType;

/// # Struct `About`
///
/// The `about` command.
pub struct About;

impl Command for About {
    fn name(&self) -> String {
        String::from("about")
    }

    fn description(&self) -> String {
        String::from("GlobalPlugin.AboutCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_about_command(ctx))
    }
}

/// # Asynchronous Function `execute_about_command`
///
/// Executes the `about` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_about_command(ctx: CommandContext) -> HarTexResult<()> {
    let whitelists = GetWhitelistedGuilds::default().await?.len();
    let interaction = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    let embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new()
            .name("HarTex")
            .icon_url(ImageSource::url("https://cdn.discordapp.com/attachments/795539269925601341/862616114239897610/275a4a2ecfb5380a45c393c81838c14b.png")?)
        )
        .description("HarTex is a Discord bot that is built and optimized for efficient Discord moderation and administration, maintained by the HarTex Development Team members.")
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new("Bot Version", HARTEX_BUILD))
        .field(EmbedFieldBuilder::new("Whitelisted Guilds", whitelists.to_string()).inline().build())
        .build()?;

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: None,
                    embeds: vec![embed],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await?;

    Ok(())
}
