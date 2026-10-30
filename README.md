# intmc - Minecraft Server Hosting for Android

A low-resource, mobile-first Minecraft server hosting app for Android. Run Java Edition Minecraft servers directly on your phone or tablet with minimal overhead.

## Features

- **Multiple Server Backends**: PumpkinMC, SteelMC, Rosewater, Glowstone-rs support
- **Playit.gg Tunnels**: Secure, zero-config tunneling for external access
- **LAN Discovery**: Automatic local network server discovery and announcement
- **Plugin Marketplace**: Browse, search, and install plugins for supported backends
- **Mobile-First UI**: Built with egui/eframe for responsive touch-friendly interface
- **SQLite Storage**: Persistent server configurations and metadata
- **Low Resource Usage**: Optimized for mobile devices with 2GB+ RAM

## Requirements

- **Android API 24+** (Android 7.0 Nougat)
- **2GB RAM** minimum (4GB+ recommended)
- **ARM64** architecture (aarch64-linux-android)
- **Storage**: 500MB+ for server files and world data

## Build Instructions

### Prerequisites

- Rust toolchain (1.75+)
- Android NDK (r25+)
- Java 17+ (for running Minecraft servers)
- Android SDK with build-tools

### Setup Android Targets

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

### Desktop Development Build

```bash
cargo run
```

### Android Build

```bash
# Build for all Android architectures
cargo build --target aarch64-linux-android
cargo build --target armv7-linux-androideabi
cargo build --target x86_64-linux-android
cargo build --target i686-linux-android

# Or use cargo-apk for APK generation
cargo install cargo-apk
cargo apk build
```

### Generate APK

```bash
cargo apk build --target aarch64-linux-android
# Output: target/aarch64-linux-android/debug/apk/intmc-debug.apk
```

## Usage Instructions

1. **Install the APK** on your Android device (API 24+)
2. **Grant storage permissions** when prompted
3. **Create a server** from the main screen:
   - Enter server name
   - Select backend (PumpkinMC, SteelMC, Rosewater, Glowstone-rs)
   - Set port (default: 25565)
4. **Start the server** - it will download the backend if needed
5. **Connect via LAN** - other devices on same network can join directly
6. **Enable Playit.gg tunnel** for external access (creates a secure tunnel)
7. **Install plugins** from the marketplace for supported backends

## Project Structure

```
intmc/
├── src/
│   ├── main.rs          # Entry point
│   ├── app/             # Main application logic
│   ├── server/          # Server management & backends
│   ├── playit/          # playit.gg tunnel integration
│   ├── lan/             # LAN discovery & announcement
│   ├── plugins/         # Plugin marketplace & management
│   ├── storage/         # SQLite persistence layer
│   └── ui/              # egui components & screens
├── jni/                 # JNI bindings for Android
├── Cargo.toml
├── build.rs
└── .gitignore
```

## Contributing Guidelines

1. **Fork** the repository
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** with clear, focused commits
4. **Run checks** before submitting:
   ```bash
   cargo check
   cargo clippy
   cargo fmt --check
   ```
5. **Test** on desktop and Android if possible
6. **Submit a pull request** with a clear description

### Code Style

- Follow Rust standard conventions (`rustfmt`)
- Use `clippy` lints
- Write docs for public APIs
- Keep functions small and focused

## License

MIT License - see [LICENSE](LICENSE) file for details.

Copyright (c) 2024 intmc contributors