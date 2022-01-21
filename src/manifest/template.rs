//
// file.rs
// lug
// 
// Author: wess (me@wess.io)
// Created: 01/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

use super::Manifest;

pub const MANIFEST_HEADER: &str = r#"
local manifest = {}

local function package(name, version)
  return "package:" .. name .. "@" .. version
end

local function github(repo, opts)
  if opts == nil then
    return repo .. "::https://github.com/" .. repo .. "/archive/refs/heads/master.tar.gz"
  elseif opts.branch then
    return repo .. "::https://github.com/" .. repo .. "/archive/refs/heads/" .. opts.branch .. ".tar.gz"
  elseif opts.tag then
    return repo .. "::https://github.com/" .. repo .. "/archive/refs/tags/" .. opts.tag .. ".tar.gz"
  end
  
end


"#;

pub const MANIFEST_BODY: &str = r#"
name = "{{name}}"
version = "{{version}}"
license = "{{license}}"
{% if author %}
author = "{{author}}"
{% endif %}
{% if homepage %}
homepage = "{{homepage}}"
{% endif %}
{% if description %}
description = "{{decription}}"
{% endif %}
dependencies = {
}

"#;

pub const MANIFEST_FOOTER: &str = r#"
manifest.name = name
manifest.version = version
manifest.author = author
manifest.homepage = homepage
manifest.description = description
manifest.license = license
manifest.dependencies = dependencies

return manifest
"#;

