//
// mod.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  fs::File,
  sync::{
    Arc, 
    Mutex
  }
};

use flate2::read::GzDecoder;
use tar::Archive;
use surf;

use crate::Result;

const PKG_DIR:&str = ".packages";


pub struct Package(String);

impl Package {
  pub async fn download(&self) -> Result<()> {
   Ok(())
  }

  pub fn unpack(&self) -> Result<()> {
    unimplemented!()
  }
}