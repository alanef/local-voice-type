# Local Voice Type

A cross-platform voice-to-text typing tool. Hold a hotkey, speak, and your words are typed into any application.

**100% local and private** - Your voice never leaves your machine.

**I built this for myself.** After decades of living in the terminal, my typing still sucks. AI CLI tools like Claude Code, aider, and Copilot CLI are amazing - but only if you can get your thoughts into them fast enough. I couldn't. Now I just talk.

If you're a developer who loves the command line but your fingers can't keep up with your brain, this is for you.

## Architecture

```
┌─────────────────┐         ┌─────────────────┐
│  Rust Client    │  HTTP   │  Python Server  │
│  (cross-platform)│ ──────► │  (Docker)       │
│                 │         │                 │
│  - Hotkey       │         │  - Whisper AI   │
│  - Audio capture│         │  - Transcription│
│  - Typing       │         │  - REST API     │
└─────────────────┘         └─────────────────┘
```

## Components

- **server/** - Dockerized Python FastAPI server running OpenAI Whisper for speech-to-text
- **client/** - Rust client that captures audio, sends to server, and types the result

## Quick Start

### Server

```bash
cd server
docker compose up -d
```

### Client

```bash
cd client
cargo build --release
./target/release/voice-type
```

## Usage

1. Start the server (Docker)
2. Run the client
3. Hold **Super+C** to record
4. Release to transcribe and type

## Configuration

Client config: `~/.config/voice-type/config.toml`

```toml
api_url = "http://localhost:8000"
api_token = "changeme"
hotkey = "super+c"
language = "en"
```

Server config via environment variables in `docker-compose.yml`:
- `API_TOKEN` - Authentication token
- `WHISPER_MODEL` - Model size: tiny, base, small, medium, large

## Requirements

### Server
- Docker

### Client (Runtime)
- Linux: xdotool
- macOS: None (uses built-in frameworks)
- Windows: None (uses built-in APIs)

### Client (Build)
- Rust toolchain
- Linux: libx11-dev, libxi-dev, libxtst-dev, libasound2-dev, libxdo-dev
- macOS: Xcode command line tools
- Windows: None

## Support

If you find this useful, consider buying me a coffee:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/wpalan)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT - see [LICENSE](LICENSE)