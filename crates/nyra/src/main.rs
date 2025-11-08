/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/

use nyra_utils::arg_parser::get_args;

#[tokio::main]
async fn main() {
  let args = get_args();
  if !nyra_utils::arg_parser::handle_common_args(&get_args()) {
    nyra_core::BotLauncher::init_instance(args.config.clone());

    #[cfg(feature = "only-gui")]
    {
      nyra_gui::init_gui();
      return;
    }

    #[cfg(all(feature = "gui", not(feature = "only-gui")))]
    if args.gui {
      nyra_gui::init_gui();
      return;
    }

    #[allow(unreachable_code)]
    nyra_core::BotLauncher::start().await;
  }
}
