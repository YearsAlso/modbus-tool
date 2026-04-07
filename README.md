# Modbus Tool

Modern Modbus Protocol Debug Tool built with Tauri 2.0 + React + Rust.

## Features

- 🌐 **Multi-Protocol Support**: RTU, ASCII, TCP, UDP
- 🔢 **Data Type Converter**: INT, UINT, FLOAT, BCD, HEX, STRING
- 📊 **Real-time Monitoring**: Register watch, live charts
- 📜 **Script Automation**: JavaScript scripting support
- 🔍 **Protocol Analysis**: Message trace, error diagnosis

## Tech Stack

- **Desktop Framework**: Tauri 2.0
- **Frontend**: React 18 + TypeScript + Vite
- **UI**: shadcn/ui + Tailwind CSS
- **State**: Zustand
- **Backend**: Rust + tokio

## Development

```bash
# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

```
modbus-tool/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/             # Custom React hooks
│   ├── stores/            # Zustand stores
│   └── utils/             # Utility functions
├── src-tauri/             # Rust backend
│   └── src/
│       ├── modbus_core/   # Modbus protocol
│       ├── serial_port/   # Serial communication
│       ├── data_store/    # Data persistence
│       └── commands/      # Tauri commands
├── tests/                 # Test files
└── docs/                  # Documentation
```

## License

MIT
