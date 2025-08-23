// Android build configuration for WebSocketReflectorX
// This file contains the build logic for Android-specific setup

use std::env;
use std::path::Path;

pub fn setup_android_build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let android_dir = Path::new("../../android");
    
    // Tell cargo to rerun if Android files change
    println!("cargo:rerun-if-changed=../../android");
    println!("cargo:rerun-if-changed=../../android/AndroidManifest.xml");
    println!("cargo:rerun-if-changed=../../android/res");
    println!("cargo:rerun-if-changed=../../android/assets");
    
    // Set Android-specific environment variables
    if cfg!(target_os = "android") {
        println!("cargo:rustc-env=ANDROID_MANIFEST_PATH=../../android/AndroidManifest.xml");
        println!("cargo:rustc-env=ANDROID_RES_PATH=../../android/res");
        println!("cargo:rustc-env=ANDROID_ASSETS_PATH=../../android/assets");
    }
}
