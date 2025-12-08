/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: information/help.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  poise::{
    CreateReply,
    command,
    serenity_prelude::{
      self,
      Colour,
      CreateActionRow,
      CreateEmbed,
      CreateEmbedAuthor,
      CreateEmbedFooter,
      CreateSelectMenu,
      CreateSelectMenuKind,
      CreateSelectMenuOption,
      EditMessage,
    },
  },
  std::ops::Not,
};

/// Shows this menu
#[command(prefix_command, track_edits, slash_command, category = "Information")]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp = chrono::Utc::now();

  let embed = CreateEmbed::new()
    .title("Help")
    .description("Select a category below to view its commands.")
    .colour(Colour::PURPLE)
    .timestamp(timestamp)
    .footer(CreateEmbedFooter::new("Nyra Help Menu"))
    .author(CreateEmbedAuthor::new("Nyra Bot"));

  let select_menu = CreateActionRow::SelectMenu(
    CreateSelectMenu::new(
      "select_category",
      CreateSelectMenuKind::String {
        options: vec![
          CreateSelectMenuOption::new("📖 Information", "Information")
            .description("Commands related to information and stats."),
          CreateSelectMenuOption::new("📌 Moderation", "Moderation")
            .description("Commands related to moderation and management."),
          CreateSelectMenuOption::new("🔨 Utilities", "Utilities").description("Debuggers &c."),
          CreateSelectMenuOption::new("1️⃣  Mathematics", "Maths")
            .description("Mathematical checks and expressions"),
        ],
      },
    )
    .placeholder("📚 Command Categories"),
  );

  let reply = CreateReply::default()
    .embed(embed)
    .components(vec![select_menu]);
  let sent_msg = ctx.send(reply).await?;
  let msg = sent_msg.message().await?;

  loop {
    let Some(mut ci) = msg
      .await_component_interaction(ctx.serenity_context())
      .await
    else {
      continue;
    };

    ci.create_response(
      ctx,
      serenity_prelude::CreateInteractionResponse::Acknowledge,
    )
    .await?;

    if let serenity_prelude::ComponentInteractionDataKind::StringSelect { values } = &ci.data.kind {
      let selected = values.first().cloned().unwrap_or_default();

      let cmds = crate::all()
        .into_iter()
        .filter(|x| x.category.as_ref() == Some(&selected))
        .collect::<Vec<_>>();

      let mut new_embed = CreateEmbed::new()
        .title(format!("{selected} Commands"))
        .colour(Colour::PURPLE)
        .timestamp(timestamp);

      if cmds.is_empty() {
        new_embed = new_embed.description("No commands found in this category.");
      } else {
        for cmd in cmds {
          new_embed = new_embed.field(
            cmd.name,
            format!(
              "{}{}",
              cmd
                .description
                .unwrap_or_else(|| "No description available.".into()),
              cmd
                .parameters
                .iter()
                .map(|x| x
                  .description
                  .clone()
                  .unwrap_or("No description".into())
                  .to_lowercase()
                  .replace(" ", "_"))
                .fold(
                  cmd
                    .parameters
                    .is_empty()
                    .not()
                    .then_some(" -".into())
                    .unwrap_or_default(),
                  |acc, x| format!("{acc} `{x}`")
                )
            ),
            false,
          );
        }
      }

      ci.message
        .edit(ctx, EditMessage::new().embed(new_embed))
        .await?;
    }
  }
}

inventory::submit! { MyCommand(help) }
