//                    GNU GENERAL PUBLIC LICENSE
//                       Version 2, June 1991
//
// Copyright (C) 1989, 1991 Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
// Everyone is permitted to copy and distribute verbatim copies
// of this license document, but changing it is not allowed.

use std::{
  fs,
  io,
  path::Path
};

static MAIN_RS: &str=include_str!("../assets/main.rs");

pub fn write_main<P: AsRef<Path>>(project_path: P)-> io::Result<()> {
  let path=project_path.as_ref().join("src/main.rs");
  fs::write(path,MAIN_RS)
}



