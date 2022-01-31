//
// mod.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/21/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  env::{self, temp_dir},
  path::{Path, PathBuf},
  fs::{self, File},
  io::{
    Write,
    Read,
    Seek,
    SeekFrom,
  }
};

use fs_extra::file::{
  move_file,
  CopyOptions,
};

use flate2::read::GzDecoder;
use tar::{Archive, Entry};
use glob::glob;

mod download;
pub(crate) use download::download;
use tokio::io::copy;

use crate::manifest::Manifest;

pub struct Package {
  pub name:String,
  pub temp_dir:PathBuf,
  pub destination:PathBuf,
}

impl Package {
  pub fn new(name:&str, destination:PathBuf) -> Self {
    let destination = destination.join(name);

    let temp_dir = temp_dir().join("lug-package");

    if temp_dir.exists() == false {
      std::fs::create_dir_all(&temp_dir).unwrap();
    }

    if destination.exists() == false {
      std::fs::create_dir_all(&destination).unwrap();
    }

    Self {
      name: string!(name),
      temp_dir,
      destination
    }
  }

  pub async fn unpack(&self, path:String) -> Result<(), anyhow::Error> {
    let tmp_path = self.temp_dir.join(path);

    let tarball = std::fs::File::open(&tmp_path).unwrap();
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);

    let unpacked = self.temp_dir.join(&self.name).join("unpacked");

    archive.unpack(&unpacked).unwrap();

    self.process(unpacked).await
  }

  pub async fn process(&self, tmp_path:PathBuf) -> Result<(), anyhow::Error> {

    for entry in glob(&format!("{}/**/.manifest", tmp_path.display())).unwrap() {
      match entry {
        Ok(p) => {
          let manifest = Manifest::read(Some(&p)).unwrap();
          let parent = p.parent().unwrap();

          for src in manifest.source.iter() {
            let tmp_src = parent.join(src);
            
            for src_entry in glob(tmp_src.display().to_string().as_str()).unwrap() {
              match src_entry {
                Ok(s) => {
                  let dest_src_path = self.destination.join(s.strip_prefix(parent).unwrap());

                  if s.is_dir() && dest_src_path.exists() == false {
                    std::fs::create_dir_all(&dest_src_path).unwrap();
                  }

                  if s.is_file() {
                    let options = CopyOptions::new();

                    move_file(&s, &dest_src_path, &options).unwrap();
                  }
                },
                Err(_) => continue
              }
            }
          }
        },
        Err(e) => println!("{:?}", e),
      }
    }

    Ok(())
  }
}