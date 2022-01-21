//
// mani.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/17/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  env, 
  path::Path,
  fs::{
    File
  },
  io::{
    self,
    Write,
    prelude::*,
  },
  collections::HashMap
};

use serde::{
  self,
  Deserialize,
  Serialize,
};

use serde_with::skip_serializing_none;
use serde_yaml::Value;

use mlua::prelude::*;
use mlua::{
  Value as LuaValue,
  UserData
};

use tera::{self, Tera};
use regex::Regex;

mod template;
use template::{
  MANIFEST_HEADER,
  MANIFEST_BODY,
  MANIFEST_FOOTER
};



#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
  pub name: String,
  pub version: String,
  pub author: Option<String>,
  pub homepage: Option<String>,
  pub description: Option<String>,
  pub license: String,
  pub dependencies: Vec<String>,
}

impl UserData for Manifest {}

impl Manifest {
  pub fn exists() -> bool {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join(".manifest");

    path.exists()
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let cwd = string!(env::current_dir().unwrap().to_str().unwrap()); 
    let _manifest = string!(self.build_init().unwrap());
    let write_path = format!("{}/.manifest", cwd);
    let path = Path::new(write_path.as_str());

    let mut file = match File::create(&path) {
      Err(why) => console_panic!("Unable to create ._manifest : {}", why),
      Ok(file) => file,
    };

    file.write_all(_manifest.as_bytes())
  }

  pub fn load() -> Result<Self, io::Error> {
    Self::read()
  }

  fn read() -> Result<Self, io::Error> {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join(".manifest");
    let body = file_read!(path.to_str().unwrap());

    let mut content = String::new();
    content.push_str(MANIFEST_HEADER);
    content.push_str(body.as_str());
    content.push_str(MANIFEST_FOOTER);

    let lua = Lua::new();
    let result:Manifest = lua.from_value(
      lua.load(&content).eval().unwrap()
    ).unwrap();

    Ok(result.clone())
  }

  fn build_init(&self) -> crate::Result<String> {
    let mut tpl = Tera::default();
    tpl.add_raw_template(
      "manifest",
      &MANIFEST_BODY.to_string()
    ).unwrap();

    let result = tpl.render("manifest", &tera::Context::from_serialize(self).unwrap()).unwrap();
    
    Ok(result.replace("\n\n", "").clone())
  }
}

impl Default for Manifest {
  fn default() -> Self {
    let cwd = std::env::current_dir().unwrap();
    let display = cwd.file_name().unwrap().to_str().unwrap_or("project");
    
    Self {
      name: display.to_string(),
      version: "0.0.1".to_string(),
      author: None,
      homepage: None,
      description: None,
      license: "MIT".to_string(),
      dependencies: vec![],
    }
  }
}