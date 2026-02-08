# Using wash CLI to Initialize a Provider

This document explains how to properly initialize a wasmCloud provider using the `wash` CLI tool.

## Installation

### Option 1: Using Official Installation Script (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/wasmcloud/wash/refs/heads/main/install.sh | bash
```

Add wash to your PATH:
```bash
export PATH="$PATH:$HOME/.wash/bin"
```

### Option 2: Using Cargo

```bash
cargo install wash-cli
```

### Option 3: Pre-built Binaries

Download from [GitHub Releases](https://github.com/wasmCloud/wash/releases)

## Creating a New Provider

### Initialize a Provider Project

```bash
wash new provider wasmcloud-websocket-provider --template-name custom-template-rust
cd wasmcloud-websocket-provider
```

This creates a properly structured provider with:
- Correct SDK version and dependencies
- WIT interface definitions
- Proper project structure
- Example implementation

### Project Structure

```
wasmcloud-websocket-provider/
├── Cargo.toml           # Project configuration
├── src/
│   ├── main.rs         # Entry point
│   ├── provider.rs     # Provider implementation
│   └── config.rs       # Configuration handling
├── wit/
│   └── world.wit       # WebAssembly Interface definitions
├── tests/              # Integration tests
└── README.md
```

## Building the Provider

### Development Build
```bash
wash build
```

### Release Build
```bash
wash build --release
```

## Running Locally

### Start wasmCloud Host
```bash
wash up
```

### Start the Provider
```bash
wash start provider ./target/wasm32-unknown-unknown/debug/wasmcloud_websocket_provider.wasm
```

## Testing

### Run Unit Tests
```bash
cargo test
```

### Run with wash
```bash
wash test
```

## Current Implementation Note

**Important**: Due to network restrictions in the current environment, this project was initialized with `cargo init` instead of `wash new`. The implementation follows wasmCloud provider best practices and uses the correct SDK (v0.17), but lacks the wash-generated scaffolding.

### To Properly Re-initialize with wash

When you have wash CLI available, you can regenerate the project structure:

1. Backup current implementation files:
   ```bash
   mkdir -p ../backup
   cp -r src ../backup/
   ```

2. Re-initialize with wash:
   ```bash
   cd ..
   rm -rf wasmcloud-websocket-provider
   wash new provider wasmcloud-websocket-provider --template-name custom-template-rust
   ```

3. Integrate the custom implementation:
   - Copy `src/websocket.rs` and `src/config.rs` to the new project
   - Update `src/provider.rs` or `src/main.rs` with the WebSocket logic
   - Update `Cargo.toml` with additional dependencies (tokio-tungstenite, etc.)

## Deployment

### Build Provider Archive
```bash
wash par create \
  --name wasmcloud-websocket-provider \
  --vendor yourname \
  --version 0.1.0 \
  --binary ./target/release/wasmcloud_websocket_provider
```

### Push to Registry
```bash
wash par push <registry-url>/wasmcloud-websocket-provider:0.1.0
```

## Additional Resources

- [wasmCloud Provider Documentation](https://wasmcloud.com/docs/developer/languages/rust/providers/)
- [wash CLI Documentation](https://wasmcloud.com/docs/wash/)
- [Provider SDK Documentation](https://docs.rs/wasmcloud-provider-sdk/latest/)
