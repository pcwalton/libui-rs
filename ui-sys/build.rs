extern crate cmake;

use std::env::var;
use std::fs::rename;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=libui");

    if !Path::new("libui/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .expect("initializing submodule libui");
    }

    let profile = var("PROFILE").expect("reading environment variable PROFILE");
    let dst = cmake::Config::new("libui")
        .profile(profile.as_str())
        .build_target("")
        .build();

    let mut out_dir = dst.join(Path::new("build/out"));

    let target = var("TARGET").expect("reading environment variable TARGET");
    if target.contains("msvc") {
        out_dir = out_dir.join(Path::new(profile.as_str()));
        rename(out_dir.join("libui.lib"), out_dir.join("ui.lib")).expect("renaming file libui.lib");
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=ui");
}
