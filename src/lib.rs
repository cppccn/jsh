//! # `jshell`
//! 
//! _A sandboxed shell that input and output JSON (which is handy)_
//! 
//! `jsh` intends to be secure by running everything inside a `chroot` and use a whitelist of authorized programs.
//! 
//! Built in the first place as a dependency of `pndr`, it should be able to be used through SSH!

use std::path::Path;

pub fn exec(_path: &Path, _cmd: &str) -> String {
    unimplemented!();
}
