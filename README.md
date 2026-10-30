# intmc

A Rust-based Minecraft server hosting app for Android. Built with egui for the UI and designed to run natively on Android via the NDK.

## Features

- **Native Android App** - Runs directly on Android without containerization or virtualization
- **Minecraft Server Management** - Start, stop, and manage Java Edition servers
- **LAN Discovery** - Automatic local network server discovery
- **playit.gg Integration** - Easy tunneling for external access
- **Plugin Support** - Install and manage server plugins
- **Modern UI** - Built with egui/eframe for a responsive, native feel
- **SQLite Storage** - Persistent configuration and server data

## Architecture

- **Frontend**: egui/eframe (immediate mode GUI)
- **Backend**: Rust with tokio async runtime
- **Android Bridge**: android-activity + ndk crates
- **Storage**: SQLite via rusqlite
- **Networking**: socket2 for LAN discovery

## Build Instructions

### Prerequisites

- Rust toolchain (1.75+)
- Android NDK (r25+)
- Java 17+ (for running Minecraft servers)
- Android SDK with build-tools

### Setup Android Target

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

## Project Structure

```
intmc/
├── src/
│   ├── main.rs          # Entry point
│   ├── app/             # Main application logic
│   ├── server/          # Server management
│   ├── playit/          # playit.gg integration
│   ├── lan/             # LAN discovery
│   ├── plugins/         # Plugin management
│   ├── storage/         # SQLite persistence
│   └── ui/              # egui components
├── jni/                 # JNI bindings for Android
├── Cargo.toml
└── build.rs
```

## Configuration

Server configurations are stored in SQLite database. Default location:
- Desktop: `~/.config/intmc/`
- Android: App-specific storage directory

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo check` and `cargo test`
5. Submit a pull request