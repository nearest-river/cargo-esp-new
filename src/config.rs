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

static TARGET: &str="xtensa-esp8266-none-elf";
static MAKEFILE: &str=include_str!("../assets/Makefile");
static CONFIG_TOML: &str=include_str!("../assets/config.toml");
static CARGO_TOML: &str=include_str!("../assets/Cargo-append.toml");


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

  Command::new("rustup")
  .args(["run","esp","cargo","fetch","--target",TARGET])
  .status()?
  .resolve();
  Ok(())
}




