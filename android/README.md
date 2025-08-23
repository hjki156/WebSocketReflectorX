# Android Configuration

This directory contains Android-specific configuration files for WebSocketReflectorX, following the same organizational pattern as the `windows/` and `macos/` directories.

## Directory Structure

```
android/
├── AndroidManifest.xml          # Android app manifest
├── build.sh                     # Android build script
├── build_config.rs              # Build configuration helpers
├── res/                         # Android resources
│   ├── mipmap-mdpi/
│   │   └── ic_launcher.png      # App icon (48x48)
│   ├── mipmap-hdpi/
│   │   └── ic_launcher.png      # App icon (72x72)
│   ├── mipmap-xhdpi/
│   │   └── ic_launcher.png      # App icon (96x96)
│   ├── mipmap-xxhdpi/
│   │   └── ic_launcher.png      # App icon (144x144)
│   └── mipmap-xxxhdpi/
│       └── ic_launcher.png      # App icon (192x192)
├── assets/                      # App assets (fonts, images, etc.)
│   ├── fonts/                   # UI fonts
│   ├── arts/                    # Graphics and artwork
│   └── README.md                # Assets documentation
└── README.md                    # This file
```

## Building

### Prerequisites

1. Install Android development tools:
   ```bash
   # Install cargo-apk
   cargo install cargo-apk
   
   # Add Android targets
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   ```

2. Set up Android SDK/NDK (required environment variables):
   ```bash
   export ANDROID_SDK_ROOT=/path/to/android/sdk
   export ANDROID_NDK_ROOT=/path/to/android/ndk
   ```

### Build Commands

From the project root:
```bash
# Quick build using the Android script
./android/build.sh

# Or manually from crates/desktop:
cd crates/desktop
cargo apk build --release --target aarch64-linux-android
```

### Installation

```bash
# Install on connected Android device
adb install path/to/generated.apk
```

## Configuration Files

### AndroidManifest.xml
Defines the app's package name, permissions, activities, and other metadata.

### Cargo.toml References
The desktop crate's `Cargo.toml` references files in this directory:
- `manifest = "../../android/AndroidManifest.xml"`
- `res = "../../android/res"`
- `assets = "../../android/assets"`

### Icons
App launcher icons are stored in `res/mipmap-*/` directories at different resolutions for various screen densities.

## Development Notes

- The Android configuration follows the same organizational pattern as `windows/` and `macos/` directories
- All Android-specific files are contained in this directory
- The desktop crate references these files through relative paths
- Icons are automatically generated from `arts/logo.png` during setup
