use {
  crate::utils::colorize::{
    Color,
    ColorExt,
  },
  clap::Parser,
};

#[derive(Parser, Debug)]
#[command(author, version, disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
  #[arg(short, long)]
  pub gui: bool,
  #[arg(short, long)]
  pub help: bool,
  #[arg(short, long)]
  pub version: bool,
  #[arg(short, long)]
  pub config: Option<String>,
}

/// Literally just returns the Arguments as
/// I didn't want to have to importa a whole bunch of
/// stuff to just get the args (in effort to make main file cleaner)
pub fn get_args() -> Args {
  Args::parse()
}

/// Executes the arg which is passed, and returns true
/// for if a arg *was* passed
pub fn handle_common_args(args: &Args) -> bool {
  args.help.then(print_help).is_some() || args.version.then(print_version).is_some()
}

pub fn print_help() {
  let help_msg = r#"
Nyra Help
Nyra is a Discord bot written in Rust!
Upstream Git repo: https://gitlab.com/invra/nyra

  {•} -[-g]ui        {→}  Opens Nyra with a GUI
  {•} -[-h]elp       {→}  Shows this help message
  {•} -[-v]ersion    {→}  Shows this package's version
  {•} -[-c]onfig     {→}  Change the location to load Nyra's config
  "#
  .replace("{•}", &"•".color(Color::Cyan).bold())
  .replace("{→}", &"→".color(Color::Blue).bold());

  print!("{}", help_msg);
}

pub fn print_version() {
  let version_msg = "{version_line}".replace(
    "{version_line}",
    &format!(
      "{} Version v{}",
      "[stdout/nyra]:".color(Color::Magenta).bold(),
      env!("CARGO_PKG_VERSION")
    ),
  );

  print!("{}", version_msg);
}
