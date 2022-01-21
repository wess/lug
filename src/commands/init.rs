//
// init.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/14/2022
// 
// Copywrite (c) 2022 Wess.io
//

use clap::{App};

use crate::manifest::Manifest;

pub struct Init {}

impl Init {
  pub fn app() -> App<'static> {
    App::new("init")
      .about("Initialize a new lug environemt")
  }

  pub fn run() {
    if Manifest::exists() {
      console_error!("A manifest already exists in this directory.");
      return;
    }

    let manifest = Manifest::default();
    manifest.write().unwrap();

    println!();
    console_success!("Manifest created.");
  }
}
