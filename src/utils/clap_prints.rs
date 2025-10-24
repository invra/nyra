use crate::utils::colorize::{
  Color,
  ColorExt,
};

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
