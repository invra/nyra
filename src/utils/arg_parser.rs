use crate::utils::colorize::{
  Color,
  ColorExt,
};

#[derive(Debug, Default)]
pub struct Args {
  pub gui: bool,
  pub help: bool,
  pub version: bool,
  pub config: Option<String>,
}

pub fn get_args() -> Args {
  let raw_args: Vec<String> = std::env::args().collect();
  parse_args(&raw_args)
}

pub fn handle_common_args(args: &Args) -> bool {
  if args.help {
    print_help();
    true
  } else if args.version {
    print_version();
    true
  } else {
    false
  }
}

/// Internal: parse Vec<String> into an Args struct.
/// Exits if any invalid or unknown arguments are passed.
fn parse_args(raw_args: &Vec<String>) -> Args {
  let mut parsed = Args::default();
  let mut iter = raw_args.iter().skip(1);

  while let Some(arg) = iter.next() {
    match arg.as_str() {
      "-g" | "--gui" => parsed.gui = true,
      "-h" | "--help" => parsed.help = true,
      "-v" | "--version" => parsed.version = true,
      "-c" | "--config" => {
        if let Some(path) = iter.next() {
          parsed.config = Some(path.clone());
        } else {
          eprintln!(
            "{} Missing argument for flag {}",
            "[stderr/nyra]:".color(Color::Red).bold(),
            "--config".color(Color::Yellow)
          );
          std::process::exit(1);
        }
      }

      unknown if unknown.starts_with('-') => {
        eprintln!(
          "{} Unknown argument: {}",
          "[stderr/nyra]:".color(Color::Red).bold(),
          unknown.color(Color::Yellow)
        );
        eprintln!(
          "Try running {} for a list of valid options.",
          "--help".color(Color::Cyan)
        );
        std::process::exit(1);
      }

      _ => {}
    }
  }

  parsed
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

  println!("{}", help_msg);
}

pub fn print_version() {
  let version_msg = format!(
    "{} Version v{}",
    "[stdout/nyra]:".color(Color::Magenta).bold(),
    env!("CARGO_PKG_VERSION")
  );

  println!("{}", version_msg);
}
