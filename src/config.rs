//                    GNU GENERAL PUBLIC LICENSE
//                       Version 2, June 1991
//
// Copyright (C) 1989, 1991 Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
// Everyone is permitted to copy and distribute verbatim copies
// of this license document, but changing it is not allowed.
//
// This configuration template is highly inspired by `https://github.com/andreas-mausch/blinky-esp8266-rust`, `https://andreas-mausch.de/blog/2024-10-20-esp01s/`
// written by `https://github.com/andreas-mausch`



use crate::StatusExt;
use std::{
  env,
  path::Path,
  process::Command,
  fs::{
    self,
    OpenOptions
  },
  io::{
    self,
    Write
  }
};

static CONFIG_TOML: &str=r#"[target.xtensa-esp8266-none-elf]
runner = "espflash flash --monitor"

[env]
ESP_LOG="info"

[build]
# rustflags = ["-C", "link-arg=-nostartfiles", "-C", "link-arg=-Wl,-Tlink.x"]
# target = "xtensa-esp8266-none-elf"

[unstable]
build-std = ["core"]
"#;

static MAKEFILE: &str=r#"
TARGET=xtensa-esp8266-none-elf
RUSTFLAGS:=-C link-arg=-nostartfiles -C link-arg=-Wl,-Tlink.x

build:
  @RUSTFLAGS="$(RUSTFLAGS)" rustup run esp cargo build --target $(TARGET)
flash:
  @RUSTFLAGS="$(RUSTFLAGS)" ustup run esp cargo espflash flash --release --target $(TARGET)
run:
  @RUSTFLAGS="$(RUSTFLAGS)" rustup run esp cargo run --release --target $(TARGET)
"#;

static CARGO_TOML: &str=r#"

[profile.dev]
# Rust debug is too slow.
# For debug builds always builds with some optimization
opt-level = "s"

[profile.release]
codegen-units = 1 # LLVM can perform better optimizations using a single thread
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 's'
overflow-checks = false
"#;

static DEPS: [Dependency<'static>;7]=[
  ("log",&[]),
  ("xtensa-lx-rt",&["esp8266"]),
  ("xtensa-lx",&["esp8266"]),
  ("panic-halt",&[]),
  ("esp8266-hal",&[]),
  ("esp8266",&[]),
  ("embedded-hal",&["unproven"])
];



pub fn sync_config<P: AsRef<Path>>(project_path: P)-> io::Result<()> {
  env::set_current_dir(project_path)?;

  fs::create_dir_all(".cargo")?;
  fs::write(".cargo/config.toml",CONFIG_TOML)?;
  fs::write("Makefile",MAKEFILE)?;

  let mut file=OpenOptions::new()
  .write(true)
  .append(true)
  .open("Cargo.toml")?;
  write!(file,"{}",CARGO_TOML)?;
  drop(file); // dropping it early so that other processes can access it freely.


  DEPS.into_iter()
  .map(add_dependency)
  .collect::<Result<Vec<_>,_>>()?;
  Ok(())
}

/// name and features pair
type Dependency<'a>=(&'a str,&'a [&'a str]);

fn add_dependency(dep: Dependency<'_>)-> io::Result<()> {
  let mut process=Command::new("cargo");
  process.arg("add")
  .arg(dep.0)
  .args(dep.1)
  .status()?
  .resolve();

  Ok(())
}




