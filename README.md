# Modbus Tool

<div align="center">

![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blueviolet)

**Modern Modbus Protocol Debug Tool**  
Built with Tauri 2.0 + React + TypeScript + Rust

[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [License](#license)

</div>

---

## ✨ Features

| Category | Feature | Description |
|----------|---------|-------------|
| 🌐 **Protocol** | Multi-Protocol | RTU, ASCII, TCP, UDP |
| 🔢 **Converter** | Data Types | INT, UINT, FLOAT, BCD, HEX, STRING |
| 📊 **Monitor** | Real-time | Register watch, live charts |
| 📜 **Automation** | Scripts | Simple trigger-action automation |
| 🔍 **Debug** | Trace | Message trace, error diagnosis |
| 💾 **Storage** | History | SQLite persistence |

---

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- Rust 1.70+
- npm or yarn

### Install & Run

```bash
# Clone repository
git clone https://github.com/YearsAlso/modbus-tool.git
cd modbus-tool

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

---

## 📁 Project Structure

```
modbus-tool/
├── src/                          # React Frontend
│   ├── components/               # UI Components
│   │   ├── Connection/          # Connection management
│   │   ├── Monitor/            # Data monitoring
│   │   ├── Converter/         # Data type converter
│   │   └── Trace/             # Message trace
│   ├── hooks/                  # Custom React hooks
│   ├── stores/                 # Zustand stores
│   └── utils/                  # Utility functions
│
├── src-tauri/                   # Rust Backend
│   └── src/
│       ├── modbus/             # Modbus protocol
│       │   ├── protocol.rs     # PDU encoding/decoding
│       │   ├── rtu.rs        # RTU frame assembler
│       │   └── tcp.rs         # TCP session manager
│       ├── serial/            # Serial communication
│       ├── storage/           # SQLite persistence
│       └── commands/          # Tauri IPC commands
│
├── tests/                       # Test files
├── docs/                        # Documentation
└── SPEC.md                      # Technical specification
```

---

## 🎯 Supported Protocols

| Protocol | Transport | Status |
|----------|-----------|--------|
| Modbus RTU | RS-232/RS-485 | ✅ |
| Modbus ASCII | RS-232/RS-485 | 📋 |
| Modbus TCP | TCP/IP (port 502) | ✅ |
| Modbus UDP | UDP/IP (port 502) | 📋 |

### Supported Function Codes

| Code | Name | Status |
|------|------|--------|
| 01 | Read Coils | ✅ |
| 02 | Read Discrete Inputs | ✅ |
| 03 | Read Holding Registers | ✅ |
| 04 | Read Input Registers | ✅ |
| 05 | Write Single Coil | ✅ |
| 06 | Write Single Register | ✅ |
| 15 | Write Multiple Coils | ✅ |
| 16 | Write Multiple Registers | ✅ |

---

## 📖 Documentation

- [SPEC.md](./SPEC.md) - Technical specification
- [docs/tasks.md](./docs/tasks.md) - Development tasks
- [docs/modbus-automation-script.md](./docs/modbus-automation-script.md) - Script automation design

---

## 🛠️ Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri 2.0 |
| Frontend | React 18 + TypeScript |
| Build Tool | Vite |
| UI Components | shadcn/ui + Tailwind CSS |
| State Management | Zustand |
| Backend | Rust + tokio |
| Serial Port | serialport-rs |
| Database | SQLite (rusqlite) |

---

## 📝 License

MIT © 2026 YearsAlso

---

<div align="center">

**Star the repo if you find it useful!** ⭐

</div>
