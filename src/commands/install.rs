//
// install.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  path::PathBuf,
  env
};

use clap::{App};
use futures::FutureExt;

use crate::package::{self, Package};
use crate::manifest::Manifest;

const PACKAGE_DIR: &str = "packages";

pub struct Install {}

impl Install {
  pub fn app() -> App<'static> {
    App::new("init")
      .about("Initialize a new lug environemt")
  }


  fn package_dir() -> PathBuf {
    let pdir = env::current_dir().unwrap().join(PACKAGE_DIR);

    if pdir.exists() == false {
      std::fs::create_dir_all(&pdir).unwrap();
    }

    pdir
  }

  pub async fn run() {
    if !Manifest::exists() {
      console_error!("No .manifest found. Run `lug init` to create one.");
      return;
    }

    let dependencies = Manifest::load().unwrap().dependencies;

    for dependency in dependencies {
      if let [ident, url] = dependency.split("::").collect::<Vec<&str>>()[..] {

        match ident {
          "path" => {
            Self::from_path(url).await;
          },
          "registry" => {
            Self::from_registry(url).await;
          }
          _ => Self::from_github(ident, url).await.unwrap(),
        }
      } else {
        console_error!("Invalid dependency: {}", dependency);
        return;
      }
    }
  }

  async fn from_path(_path:&str) {

  }

  async fn from_registry(_path:&str) {

  }

  async fn from_github(repo:&str, url:&str) -> crate::Result<()> {
    if let [_, name] = repo.split("/").collect::<Vec<&str>>()[..] {
      let package = Package::new(name, Self::package_dir());
      let downloaded = package::download(name, url, &package.temp_dir).await.unwrap();

      package.unpack(downloaded).await.unwrap();

      Ok(())
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid repo").into())
    }
  }
}
