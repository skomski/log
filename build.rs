//! This build script detects target platforms that lack proper support for
//! atomics and sets `cfg` flags accordingly.

use std::env;
use std::str;

#[cfg(feature = "kv_unstable")]
#[path = "src/kv/value/internal/cast/primitive.rs"]
mod primitive;

fn main() {
    let target = match rustc_target() {
        Some(target) => target,
        None => return,
    };

    if target_has_atomic_cas(&target) {
        println!("cargo:rustc-cfg=atomic_cas");
    }

    if target_has_atomics(&target) {
        println!("cargo:rustc-cfg=has_atomics");
    }

    // Generate sorted type id lookup
    #[cfg(feature = "kv_unstable")]
    primitive::generate();

    println!("cargo:rustc-cfg=srcbuild");
    println!("cargo:rerun-if-changed=build.rs");
}

fn target_has_atomic_cas(target: &str) -> bool {
  false
}

fn target_has_atomics(target: &str) -> bool {
   false
}

fn rustc_target() -> Option<String> {
    env::var("TARGET").ok()
}
