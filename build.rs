// Copyright 2023-2025, shadow3, shadow3aaa
//
// This file is part of fas-rs.
//
// fas-rs is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// fas-rs is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along
// with fas-rs. If not, see <https://www.gnu.org/licenses/>.

use std::{fs, io::Write, process::Command};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    pub authors: Vec<String>,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Deserialize)]
struct CargoConfig {
    pub package: Package,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=update");

    let toml = fs::read_to_string("Cargo.toml")?;
    let data: CargoConfig = toml::from_str(&toml)?;

    gen_module_prop(&data)?;

    Ok(())
}

fn cal_version_code(version: &str) -> Result<usize> {
    let manjor = version
        .split('.')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?;
    let manjor: usize = manjor.parse()?;
    let minor = version
        .split('.')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?;
    let minor: usize = minor.parse()?;
    let patch = version
        .split('.')
        .nth(2)
        .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?;
    let patch: usize = patch.parse()?;

    // 版本号计算规则：主版本 * 100000 + 次版本 * 1000 + 修订版本
    Ok(manjor * 100000 + minor * 1000 + patch)
}

fn cal_short_hash() -> Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()?
            .stdout,
    )?)
}

fn gen_module_prop(data: &CargoConfig) -> Result<()> {
    let package = &data.package;
    let id = package.name.replace('-', "_");
    let version_code = cal_version_code(&package.version)?;
    let authors = &package.authors;
    let mut author = String::new();
    let mut conut = 0;
    for a in authors {
        conut += 1;
        if conut > 1 {
            author += &format!("& {a}");
        } else {
            author += &format!("{a} ");
        }
    }
    let author = author.trim();
    let version = format!("{}-{}", package.version, cal_short_hash()?);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("module/module.prop")?;

    writeln!(file, "id={id}")?;
    writeln!(file, "name={}", package.name)?;
    writeln!(file, "version=v{}", version.trim())?;
    writeln!(file, "versionCode={version_code}")?;
    writeln!(file, "author={author}")?;
    writeln!(file, "description={}", package.description)?;

    println!("cargo:rustc-env=GIT_HASH={}", version.trim());

    Ok(())
}
