# Snow

Snow is a heavily asynchronous server software built in Rust. It was designed from the ground up with **scalability** and **simplicity** in mind, aiming to provide a high-performance foundation for Minecraft: Bedrock Edition servers.

# Getting Started
Running Snow requires at least **Rust 1.90** and **Cargo**.

For a user-friendly start, use the provided wrapper scripts, which handle compilation and execution automatically.

## 1. Clone the Repository

```
git clone https://github.com/Vantorio/snow.git
cd snow
```

## 2. Run the Server (Recommended)

Use the script for your operating system. These scripts automatically check for Rust, compile the server in optimized **Release Mode** if needed, and run the binary.

### Windows (PowerShell)
```
.\start.ps1
```

### Linux (Bash)
```
chmod +x start.sh
./start.sh
```

## 3. Manual Compilation and Run

To manually compile and run in the highest performance mode:
```
# Compile the optimized release binary
cargo build --release

# Run the compiled binary
./target/release/snow
```