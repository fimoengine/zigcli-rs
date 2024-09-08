//! A build dependency for running `zig` to build a native library
//!
//! This crate provides some necessary boilerplate and shim support for running the system `zig`
//! command to build a native library. It will add appropriate flags and handle cross compilation.
//!
//! The builder-style configuration allows for various variables and such to be passed down into the
//! build as well.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! zigcli = "0.1"
//! ```
//!
//! ## Examples
//!
//! ```no_run
//! use zigcli;
//!
//! // Builds the project in the directory located in `libfoo`, installing it
//! // into $OUT_DIR
//! let dst = zigcli::build("libfoo");
//! let dst_lib = dst.join("lib");
//!
//! println!("cargo:rustc-link-search=native={}", dst_lib.display());
//! println!("cargo:rustc-link-lib=static=foo");
//! ```
//!
//! ```no_run
//! use zigcli::Build;
//!
//! let dst = Build::new("libfoo")
//!                  .option("-Dfoo=bar")
//!                  .target("aarch64-linux-gnu")
//!                  .build();
//! let dst_lib = dst.join("lib");
//! println!("cargo:rustc-link-search=native={}", dst_lib.display());
//! println!("cargo:rustc-link-lib=static=foo");
//! ```

mod build;

pub use build::*;
