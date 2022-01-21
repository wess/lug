//
// install.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

use clap::{App};
use futures::FutureExt;

use crate::package;
use crate::manifest::Manifest;
use crate::installer::Installer;

pub struct Install {}

impl Install {
  pub fn app() -> App<'static> {
    App::new("init")
      .about("Initialize a new lug environemt")
  }

  pub async fn run() {
    if !Manifest::exists() {
      console_error!("No .manifest found. Run `lug init` to create one.");
      return;
    }

    let dependencies = Manifest::load().unwrap().dependencies;

    for dependency in dependencies {
      if let [repo, url] = dependency.split("::").collect::<Vec<&str>>()[..] {
        if let [_, name] = repo.split("/").collect::<Vec<&str>>()[..] {
          let installer = Installer::new(name, url, "packages");
          installer.process().await.unwrap();
        } else {
          console_error!("Invalid dependency: {}", dependency);
          return;
        }
      } else {
        console_error!("Invalid dependency: {}", dependency);
        return;
      }
      
      // download(&dependency).await.unwrap();
      console_debug!("dep: {}", dependency);
    }
  }
}
