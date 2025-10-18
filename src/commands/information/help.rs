/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/help.rs
  Authors: Invra, Hiten-Tandon
*/

use {
  crate::commands::{
    self,
    helper::{
      Context,
      Error,
      MyCommand,
    },
  },
  poise::{
    CreateReply,
    builtins::{
      self,
      HelpConfiguration,
    },
    command,
    serenity_prelude::{
      self,
      CreateActionRow,
      CreateEmbed,
      CreateSelectMenu,
      CreateSelectMenuKind,
      CreateSelectMenuOption,
      EditMessage,
    },
  },
};

/// Show a better help menu
#[command(prefix_command, track_edits, slash_command, category = "information")]
pub async fn new_help(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: chrono::DateTime<chrono::Utc> = chrono::offset::Utc::now();
  let reply = CreateReply::default()
    .embed(
      CreateEmbed::new()
        .title("help")
        .colour(serenity_prelude::Colour::PURPLE)
        .timestamp(timestamp),
    )
    .components(vec![CreateActionRow::SelectMenu(CreateSelectMenu::new(
      "Select category",
      CreateSelectMenuKind::String {
        options: vec![
          CreateSelectMenuOption::new("Information", "information")
            .description("The commands related to getting information about something."),
          CreateSelectMenuOption::new("Moderation", "Moderation")
            .description("The commands related to server moderation."),
        ],
      },
    ))]);

  let v = ctx.send(reply.clone()).await?;
  let temp = v.message().await?;
  loop {
    let Some(mut ci) = temp
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

    match ci.data.kind {
      serenity_prelude::ComponentInteractionDataKind::StringSelect { values } => {
        let new_data = commands::all()
          .into_iter()
          .filter(|x| x.category.as_ref() == values.get(0))
          .map(|x| format!("{}\n  {}", x.name, x.description.unwrap_or_default()))
          .fold(String::new(), |acc, x| acc + "\n\n" + &x);
        ci.message
          .edit(ctx, EditMessage::new().content(new_data))
          .await?;
      }
      _ => todo!(),
    }
  }
}
inventory::submit! {MyCommand(new_help)}

/// Show this menu
#[command(prefix_command, track_edits, slash_command, category = "information")]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
  builtins::help(
    ctx,
    command.as_deref(),
    HelpConfiguration {
      ephemeral: true,
      show_subcommands: true,
      show_context_menu_commands: true,
      ..Default::default()
    },
  )
  .await
  .map_err(Into::into)
}
inventory::submit! { MyCommand(help) }
