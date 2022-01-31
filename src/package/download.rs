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
use flate2::read::GzDecoder;
use tar::Archive;

pub async fn download(name:&str, url:&str, destination:&PathBuf) -> Result<String, anyhow::Error> {
  let client = Client::new();

  let total_size = {
    let resp = client.head(url).send().await?;

    if resp.status().is_success() {
        resp.headers()
            .get(header::CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse().ok())
            .unwrap_or(0)
    } else {
        return Err(anyhow!(
            "Couldn't download URL: {}. Error: {:?}",
            url,
            resp.status(),
        ));
    }
  };

  let request = client.get(url);

  let template_start = format!("[{}] : ", name);
  let template_end = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";
  let template = string_combined!(template_start, template_end);

  let progress = ProgressBar::new(total_size);
  progress.set_style(
    ProgressStyle::default_bar()
    .template(template.as_str())
    .progress_chars("#>-")
  );

  let url = Url::parse(url).unwrap();

  let file = Path::new(
    url
    .path_segments()
    .and_then(|s| s.last())
    .and_then(|n| if n.is_empty() { None } else { Some(n) })
    .unwrap_or("tmp.bin")
  );

  
  let tmp_dir = Path::new(destination);
  let dest_path = tmp_dir.join(file);

  if dest_path.exists() {
    fs::remove_file(&dest_path).await?;
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

