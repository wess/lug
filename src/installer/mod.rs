//
// mod.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/21/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{env, path::{Path, PathBuf}};
use anyhow::anyhow;
use tokio::{fs, io::AsyncWriteExt};
use reqwest::{Client, Url, header};
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::{self, TempDir};
use flate2::read::GzDecoder;
use tar::Archive;

pub struct Installer {
  name:String,
  url:Url,
  destination:String,
}

impl Installer {
  pub fn new(name:&str, url:&str, destination:&str) -> Self {
    let dest = string!(format!("{}/{}/{}", env::current_dir().unwrap().display(), destination, name));

    Self {
      name: name.to_string(),
      url: Url::parse(url).unwrap(),
      destination: dest,
    }
  }

  pub async fn process(&self) -> Result<(), anyhow::Error> {
    let path = Path::new(self.destination.as_str());

    if path.exists() == false {
      fs::create_dir_all(&path).await.unwrap();
    }

    let downloaded = self.download().await?;

    self.unpack(downloaded).await
  }

  pub async fn download(&self) -> Result<String, anyhow::Error> {
    let client = Client::new();
    
    let total_size = {
      let resp = client.head(self.url.as_str()).send().await?;
  
      if resp.status().is_success() {
          resp.headers()
              .get(header::CONTENT_LENGTH)
              .and_then(|ct_len| ct_len.to_str().ok())
              .and_then(|ct_len| ct_len.parse().ok())
              .unwrap_or(0)
      } else {
          return Err(anyhow!(
              "Couldn't download URL: {}. Error: {:?}",
              self.url.as_str(),
              resp.status(),
          ));
      }
    };
  
    let mut request = client.get(self.url.as_str());
  
    let template_start = format!("[{}] : ", self.name);
    let template_end = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";
    let template = string_combined!(template_start, template_end);
  
    let progress = ProgressBar::new(total_size);
    progress.set_style(
      ProgressStyle::default_bar()
      .template(template.as_str())
      .progress_chars("#>-")
    );
  
    let file = Path::new(
      self.url
      .path_segments()
      .and_then(|s| s.last())
      .and_then(|n| if n.is_empty() { None } else { Some(n) })
      .unwrap_or("tmp.bin")
    );
  
    let tmp_dir = self.tmp_dir();
    let dest_path = tmp_dir.join(file);

    if dest_path.exists() {
      let size = file.metadata()?.len().saturating_sub(1);
  
      request =
        request.header(header::RANGE, format!("bytes={}-", size));
  
        progress.inc(size);
    }
  
    let mut response = request.send().await?;
    let mut dest = 
      fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(&dest_path).await?;
  
    while let Some(chunk) = response.chunk().await? {
      dest.write_all(&chunk).await?;
      progress.inc(chunk.len() as u64);
    }
  
    Ok(dest_path.to_str().unwrap().to_string())
  }

  pub async fn unpack(&self, path:String) -> Result<(), anyhow::Error> {
    let tarball = std::fs::File::open(&path).unwrap();
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);

    archive.unpack(self.destination.as_str()).unwrap();

    std::fs::remove_dir_all(self.tmp_parent()).unwrap();

    Ok(())
  }

  fn tmp_parent(&self) -> PathBuf {
    env::current_dir()
    .unwrap()
    .join(".tmp")
  }

  fn tmp_dir(&self) -> PathBuf {
    let path = self.tmp_parent().join(self.name.as_str());

    if path.exists() == false {
      std::fs::create_dir_all(&path).unwrap();
    }

    path
  }
}

