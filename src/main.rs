
mod src;
mod config;
use std::{
  io,
  env,
  process::{
    self,
    Command,
    ExitStatus
  }
};


fn main()-> io::Result<()> {
  let args=env::args().skip(2).collect::<Vec<_>>();
  let mut process=Command::new("cargo");
  process.arg("new")
  .args(&args)
  .status()?
  .resolve();

  let project_path=&args[0];
  config::sync_config(project_path)?;
  src::write_main(project_path)?;

  Ok(())
}


pub trait StatusExt {
  fn resolve(self);
}

impl StatusExt for ExitStatus {
  fn resolve(self) {
    match self.code() {
      Some(0)=> (),
      Some(code)=> process::exit(code),
      _=> ()
    }
  }
}


