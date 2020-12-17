//! `build.rs`:
fn main() {
    println!("cargo:rustc-rerun-if-changed=build.rs"); // Don't rerun if files under `src/` change.
    let info = os_info::get();
    use os_info::{Type, Version};
    match (info.os_type(), info.version()) {
        (Type::Macos, &Version::Semantic(11, x, _)) if x >= 1 => {
            println!("cargo:rustc-cfg=big_sur_update")
        }
        _ => { /* … */ }
    }
}
