fn main() {
  const ALLOWED_TARGETS: [&str; 5] = [
    "aarch64-apple-darwin",
    "aarch64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
  ];

  let target = std::env::var("TARGET").expect("TARGET env var not set");

  if !ALLOWED_TARGETS.contains(&target.as_str()) {
    eprintln!(
      "Error: unsupported target '{target}'. Allowed targets are:\n  {}",
      ALLOWED_TARGETS.join("\n  ")
    );
    std::process::exit(1);
  }
}
