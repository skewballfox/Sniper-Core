use directories::BaseDirs;
use std::path::PathBuf;
use std::io::fs::PathExtensions;
use std::fs::create_dir;


// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    if let Some(base_dirs)=BaseDirs::new(){
        let sniper_home=base_dirs.config_dir().join(PathBuf::from(&"sniper"));
        if (!sniper_home.exists()){
            create_dir(sniper_home);

        }
    }
}
