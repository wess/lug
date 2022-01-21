//
// lib.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/14/2022
// 
// Copywrite (c) 2022 Wess.io
//

#![allow(dead_code)]
#![allow(unused_imports)]

use clap::{
  App,
  AppSettings,
};

#[macro_use]
extern crate oxide;

pub mod result;
pub use result::Result;

pub mod installer;
pub mod package;

mod manifest;
use manifest::Manifest;

mod commands;
use commands::{
  Init,
  Install,
};

pub async fn run() -> Result<()> {
  let mut app = 
    App::new("Lug")
    .version(env!("CARGO_PKG_VERSION"))
    .about("A nice Lua setup for you.")
    .before_help("\n")
    .setting(AppSettings::AllowExternalSubcommands)
    .setting(AppSettings::ArgRequiredElseHelp)
    .subcommand(Init::app())
    .subcommand(Install::app());


  let mut help = vec![];
  app.write_help(&mut help).unwrap();

  let matches = app.get_matches();
  match matches.subcommand_name() {
    Some("init") => Init::run(),
    Some("install") => Install::run().await,
    Some(cmd) => {
      println!();
      println!();
      console_error!("Unknown command: {}", cmd);
      println!("{}", String::from_utf8_lossy(&help));
    },
    None => {},
  }

  println!();
  Ok(())
}