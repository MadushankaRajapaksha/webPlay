# WebPlay

A lightweight, web-based video player server built with Rust. Stream videos from URLs to any device on your network with a beautiful, responsive interface.

![WebPlay Logo](https://img.shields.io/badge/WebPlay-1.0.0-red?style=for-the-badge&logo=rust)
![Rust](https://img.shields.io/badge/Rust-2021-black?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)

## ✨ Features

- 🚀 **Network Streaming**: Access from any device on your network
- 🎬 **Universal Video Support**: MP4, WebM, OGV, HLS streams
- 📱 **Mobile Friendly**: Touch controls and responsive design
- 🎮 **Rich Controls**: Play/pause, seek, volume, fullscreen, picture-in-picture
- ⌨️ **Keyboard Shortcuts**: Full keyboard navigation support
- 🌐 **Cross-Platform**: Runs on Windows, macOS, and Linux
- ⚡ **Fast & Lightweight**: Built with Rust for optimal performance

## 📦 Installation

### Download Pre-built Binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/MadushankaRajapaksha/webPlay/releases).

- **Windows**: `webPlay-windows.exe`
- **macOS**: `webPlay-macos`
- **Linux**: `webPlay-linux`

### Build from Source

```bash
# Clone the repository
git clone https://github.com/MadushankaRajapaksha/webPlay.git
cd webPlay

# Build release binary
cargo build --release

# Run the server
./target/release/webPlay
```

## 🚀 Quick Start

1. **Download** the executable for your platform
2. **Run** the executable:
   ```bash
   # Windows
   webPlay-windows.exe

   # macOS/Linux
   ./webPlay-macos  # or ./webPlay-linux
   ```
3. **Open** any of the displayed URLs in your browser
4. **Enter** a video URL and click "Play Video"

## 📖 Usage

### Command Line Options

```bash
webPlay [OPTIONS]

OPTIONS:
    -p, --port <PORT>    Port to run the server on (default: 3000)
    -h, --help           Print help information
    -V, --version        Print version information
```

### Examples

```bash
# Start on default port 3000
webPlay

# Start on custom port
webPlay --port 8080

# Show help
webPlay --help
```

### How to Use

1. **Start the Server**: Run `webPlay` in your terminal
2. **Access the Interface**: Open the displayed URL in any web browser
3. **Enter Video URL**: Paste any direct video URL into the input field
4. **Play**: Click the play button to start streaming

### Supported Video Sources

- Direct MP4/MP3/WebM/OGV links
- HLS streams (m3u8)
- Any video URL that works in modern browsers

## 🎮 Controls

### Desktop Controls
- **Spacebar**: Play/Pause
- **F**: Toggle fullscreen
- **M**: Mute/Unmute
- **←/→**: Seek backward/forward 5 seconds
- **↑/↓**: Volume up/down
- **Click**: Play/Pause
- **Drag progress bar**: Seek to position

### Mobile Controls
- **Tap video**: Play/Pause
- **Swipe left/right**: Seek
- **Touch controls**: All buttons work on touch

### Player Features
- **Picture-in-Picture**: Click the PiP button
- **Fullscreen**: Click fullscreen button or press F
- **Volume Control**: Drag slider or use arrow keys
- **Progress Bar**: Click or drag to seek

## 🔧 Development

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git

### Development Setup

```bash
# Clone repository
git clone https://github.com/MadushankaRajapaksha/webPlay.git
cd webPlay

# Install dependencies
cargo build

# Run in development mode
cargo run

# Run tests
cargo test
```

### Project Structure

```
webPlay/
├── src/
│   └── main.rs          # Main application logic
├── static/
│   ├── player.html      # Input interface
│   └── video.html       # Video player interface
├── .github/
│   └── workflows/
│       └── release.yml  # GitHub Actions CI/CD
├── Cargo.toml           # Rust dependencies
└── README.md           # This file
```

### Building for Release

```bash
# Build optimized binary
cargo build --release

# The binary will be in target/release/webPlay
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Video player interface inspired by modern streaming services
- Icons from various open source projects

## 🐛 Troubleshooting

### Can't connect from other devices?
- Ensure all devices are on the same WiFi network
- Check firewall settings
- Try disabling VPN if active

### Video won't play?
- Verify the video URL is direct and accessible
- Check browser console for errors
- Ensure video format is supported (MP4, WebM, OGV, HLS)

### Server won't start?
- Make sure the port (default 3000) is not in use
- Try a different port with `--port` option
- Check for permission issues

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/MadushankaRajapaksha/webPlay/issues) page
2. Create a new issue with detailed information
3. Include your OS, browser, and error messages

---

**Enjoy streaming with WebPlay! 🎬**