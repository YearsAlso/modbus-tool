# Modbus Tool - 开发任务清单

## 项目信息
- **项目名称**: Modbus Tool
- **类型**: Modbus 协议调试工具
- **技术栈**: Tauri 2.0 + React 18 + TypeScript + Vite + Rust
- **状态**: Rust 后端基础框架完成 ✅

---

## Rust 后端基础设施 (T1.3.x)

### T1.3.1 - 设计 Rust 目录结构 ✅ 已完成 (2026-04-07)
- [x] 创建 `src/modbus/` - Modbus 协议相关模块
- [x] 创建 `src/serial/` - 串口通信模块
- [x] 创建 `src/storage/` - 数据存储模块
- [x] 创建 `src/commands/` - Tauri 命令模块
- [x] 在 lib.rs 中引入这些模块
- [x] Cargo.toml 添加 rusqlite 和 bytes 依赖

### T1.3.2 - 实现错误类型定义 ✅ 已完成 (2026-04-07)
- [x] 创建 `src/error.rs` - ModbusToolError 错误枚举
- [x] 包含串口错误 (1000-1999)、TCP错误 (2000-2999)、Modbus错误 (3000-3999)
- [x] 实现 `.code()` 方法返回错误码供前端显示
- [x] 实现 `From<serialport::Error>` 和 `From<std::num::ParseIntError>` trait

### T1.3.3 - 配置 Tokio 异步运行时 ✅ 已完成 (2026-04-07)
- [x] Cargo.toml 配置 `tokio = { version = "1", features = ["full"] }`
- [x] 支持异步串口通信 (tokio-serial)
- [x] 支持异步TCP通信 (tokio::net)

### T1.3.4 - 实现日志模块 ✅ 已完成 (2026-04-07)
- [x] 创建 `src/logging.rs` - 使用 tracing + tracing-subscriber
- [x] 支持控制台日志输出
- [x] 支持文件日志滚动 (tracing-appender)
- [x] 可配置日志目录和日志级别

### T1.3.5 - 创建 Tauri 命令基础框架 ✅ 已完成 (2026-04-07)
- [x] 创建 `src/lib.rs` - 库入口，导出 error、logging、commands 模块
- [x] 创建 `src/main.rs` - Tauri 应用入口，注册命令处理器
- [x] 创建 `src/commands/mod.rs` - 命令响应封装 `CommandResponse<T>`
- [x] 创建 `src/commands/serial.rs` - 串口命令
- [x] 创建 `src/commands/tcp.rs` - TCP命令

---

## 串口通信模块 (T1.4.x)

### T1.4.1 - 实现串口列表获取 ✅ 已完成 (2026-04-07)
- [x] `serial_list_ports()` - 列出可用串口
- [x] 返回 `SerialPortInfo` 结构 (name, port_type, manufacturer, serial_number, product)
- [x] 使用 serialport crate

### T1.4.2 - 实现串口打开/关闭 ✅ 已完成 (2026-04-07)
- [x] `serial_open_port(config)` - 打开串口，返回连接ID
- [x] `serial_close_port(id)` - 关闭串口
- [x] 使用 serialport crate 的阻塞式 API

### T1.4.3 - 实现串口配置结构 ✅ 已完成 (2026-04-07)
- [x] `SerialConfig` 结构包含:
  - path (串口路径)
  - baud_rate (波特率)
  - data_bits (数据位: 5/6/7/8)
  - stop_bits (停止位: 1/1.5/2)
  - parity (校验位: none/odd/even/mark/space)
  - flow_control (流控: none/hardware/software)
  - timeout_ms (超时时间)

### T1.4.4 - 实现串口读取/写入 ✅ 已完成 (2026-04-07)
- [x] `serial_read(id, len)` - 从串口读取数据
- [x] `serial_write(id, data)` - 向串口写入数据
- [x] 返回 `Vec<u8>` 类型数据

### T1.4.5 - 注册 Tauri 串口命令 ✅ 已完成 (2026-04-07)
- [x] `#[tauri::command]` 属性暴露以下接口:
  - `serial_list_ports` -> `CommandResponse<Vec<SerialPortInfo>>`
  - `serial_open_port` -> `CommandResponse<String>` (返回连接ID)
  - `serial_close_port` -> `CommandResponse<()>`
  - `serial_read` -> `CommandResponse<Vec<u8>>`
  - `serial_write` -> `CommandResponse<usize>`

---

## TCP通信模块 (T1.5.x)

### T1.5.1 - 实现 TCP 连接管理 ✅ 已完成 (2026-04-07)
- [x] `tcp_connect(config)` - 连接到 TCP Modbus 服务器
- [x] `tcp_disconnect(id)` - 断开 TCP 连接
- [x] `tcp_is_connected(id)` - 检查连接状态
- [x] 使用 tokio::net::TcpStream 异步连接

### T1.5.2 - 实现 TCP 读取/写入 ✅ 已完成 (2026-04-07)
- [x] `tcp_read(id, len)` - 从 TCP 读取数据
- [x] `tcp_write(id, data)` - 向 TCP 写入数据
- [x] 返回 `Vec<u8>` 类型数据

### T1.5.3 - 实现连接池管理 ✅ 已完成 (2026-04-07)
- [x] `TcpManager` 结构管理多个 TCP 连接
- [x] 使用 `parking_lot::Mutex` 保护连接 Map
- [x] 每个连接有唯一 UUID 作为 ID

### T1.5.4 - 注册 Tauri TCP 命令 ✅ 已完成 (2026-04-07)
- [x] `#[tauri::command]` 属性暴露以下接口:
  - `tcp_connect` -> `CommandResponse<TcpConnectionInfo>`
  - `tcp_disconnect` -> `CommandResponse<()>`
  - `tcp_is_connected` -> `CommandResponse<bool>`
  - `tcp_read` -> `CommandResponse<Vec<u8>>`
  - `tcp_write` -> `CommandResponse<usize>`

---

## 前端基础架构 (T1.2.x)

### T1.2.1 - 设计前端目录结构 ✅ 已完成 (2026-04-07)
### T1.2.2 - 配置 Tailwind CSS + shadcn/ui ✅ 已完成 (2026-04-07)
### T1.2.3 - 创建基础布局组件 ✅ 已完成 (2026-04-07)
### T1.2.4 - 配置 Zustand 状态管理 ✅ 已完成 (2026-04-07)
### T1.2.5 - 创建 Tauri IPC 命令封装 ✅ 已完成 (2026-04-07)
### T1.2.6 - 配置前端路由与页面结构 ✅ 已完成 (2026-04-07)
### T1.2.7 - 创建全局类型定义 ✅ 已完成 (2026-04-07)
### T1.2.8 - 构建验证 ✅ 已完成 (2026-04-07)

---

## Tauri 配置 (T1.1.x)

### T1.1.1 - 创建 Tauri + React + TypeScript 项目 ✅ 已完成 (2026-04-07)
- [x] 使用 `npm create tauri-app@latest` 创建项目
- [x] 基础项目结构已存在

### T1.1.2 - 配置 Git 仓库与 .gitignore ✅ 已完成 (2026-04-07)
- [x] 初始化 Git 仓库
- [x] 创建 .gitignore (包含 node_modules/, src-tauri/target/, dist/, .env)

### T1.1.3 - 配置前端开发环境 ✅ 已完成 (2026-04-07)
- [x] npm install 完成
- [x] 安装 zustand, recharts, @tauri-apps/api
- [x] 安装 ESLint, Prettier 等开发依赖

### T1.1.7 - 配置 Tauri 权限与安全策略 ✅ 已完成 (2026-04-07)

---

## 构建状态

### ✅ Rust 编译通过
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 32.79s
```

### ⚠️ 警告
- 未使用的变量 (不影响功能)

---

## 自动化脚本 (T1.8.x - T2.1.x)

> 需求文档: `docs/modbus-automation-script.md`
> 核心理念: 像搭积木一样简单的 "当...就..." 自动化规则

### T1.8.x - 自动化核心数据结构

| 任务ID | 描述 | 优先级 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| T1.8.1 | Rust: 定义 `Script`, `Trigger`, `Action` 数据结构（参照文档7.1） | P0 | 2h | T1.3.1 | ✅ 已完成 (2026-04-08) |
| T1.8.2 | Rust: 实现 `Trigger` 枚举的 `check_trigger()` 方法（数值比较 > < = ≠ ≥ ≤） | P0 | 3h | T1.8.1 | ⬜ |
| T1.8.3 | Rust: 实现 `Action` 枚举的 `execute()` 方法（写寄存器/通知/日志） | P0 | 3h | T1.8.1 | ✅ 已完成 (2026-04-09) |
| T1.8.4 | Rust: 实现 `ScriptStore` 持久化存储（JSON 文件，位于 app_data/scripts/） | P0 | 2h | T1.8.1 | ✅ 已完成 (2026-04-09) |

### T1.9.x - 脚本执行引擎

| 任务ID | 描述 | 优先级 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| T1.9.1 | Rust: 实现 `ScriptEngine` 结构体（管理所有脚本 + running_scripts + last_values） | P0 | 3h | T1.8.x | ✅ 已完成 (2026-04-11) |
| T1.9.2 | Rust: 实现 `evaluate()` 方法（每个扫描周期检测触发条件） | P0 | 3h | T1.9.1 | ✅ 已完成 (2026-04-11) |
| T1.9.3 | Rust: 注册 Tauri 命令：`script_list`, `script_save`, `script_delete`, `script_run`, `script_stop` | P0 | 2h | T1.9.1 | ✅ 已完成 (2026-04-11) |
| T1.9.4 | Rust: 实现变化检测（BecameOn/BecameOff/Changed/StableN） | P1 | 3h | T1.9.2 | ⬜ |

### T2.0.x - 自动化前端 UI

| 任务ID | 描述 | 优先级 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| T2.0.1 | 前端: 创建 `pages/Script` 目录结构（index.tsx / data.ts / service.ts） | P0 | 1h | T1.2.x | ✅ 已完成 (2026-04-11) |
| T2.0.2 | 前端: 实现脚本列表页（显示脚本名、状态、启停按钮） | P0 | 2h | T2.0.1 | ✅ 已完成 (2026-04-11) |
| T2.0.3 | 前端: 创建 Trigger 编辑器组件（寄存器选择 + 操作符下拉 + 数值输入） | P0 | 3h | T2.0.1 | ✅ 已完成 (2026-04-11) |
| T2.0.4 | 前端: 创建 Action 编辑器组件（动作类型选择 + 参数配置） | P0 | 3h | T2.0.1 | ✅ 已完成 (2026-04-11) |
| T2.0.5 | 前端: 实现脚本编辑页（积木式布局：触发条件 + 执行动作列表 + 保存/测试） | P0 | 3h | T2.0.3 + T2.0.4 | ✅ 已完成 (2026-04-11) |
| T2.0.6 | 前端: 实现弹窗通知 `ShowNotification`（系统桌面通知） | P0 | 1h | T2.0.4 | ✅ 已完成 (2026-04-11) |
| T2.0.7 | 前端: 调用 Tauri 命令封装（`script_list`/`save`/`delete`/`run`/`stop`） | P0 | 2h | T1.9.3 | ✅ 已完成 (2026-04-11) |

### T2.1.x - 自动化增强功能

| 任务ID | 描述 | 优先级 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| T2.1.1 | Rust: 支持 `AND` 组合条件（多个 Trigger 的 AND 组合） | P1 | 3h | T1.9.4 | ✅ 已完成 (2026-04-11) |
| T2.1.2 | 前端: 变化检测 Trigger UI（BecameOn/BecameOff/Changed/StableN 选项卡） | P1 | 2h | T2.0.3 | ✅ 已完成 (2026-04-11) |
| T2.1.3 | 前端: 日志记录面板（显示脚本触发历史，含时间戳） | P1 | 2h | T2.0.2 | ✅ 已完成 (2026-04-11) |
| T2.1.4 | Rust: 脚本联动 `RunScript` / `StopScript` 动作 | P2 | 3h | T1.9.x | ⬜ |
| T2.1.5 | Rust: 延时动作 `Delay(N)` 支持（等待 N 秒后执行） | P2 | 2h | T1.9.x | ⬜ |
| T2.1.6 | 前端: 声音提醒 UI（Alert/Warning/Info 三种提示音） | P2 | 2h | T2.0.4 | ⬜ |
| T2.1.7 | 前端: 脚本导入/导出 JSON 功能（预留 UI） | P2 | 2h | T2.0.5 | ⬜ |

### Phase 里程碑

| Phase | 目标 | 包含任务 |
|-------|------|----------|
| Phase 1 (MVP) | 数值比较触发 + 写入 + 弹窗 + 启停 | T1.8.1~T1.8.4, T1.9.1~T1.9.3, T2.0.1~T2.0.7 | ✅ 全部完成 |
| Phase 2 | 变化检测 + 组合条件 + 日志 | T1.9.4, T2.1.1~T2.1.3 |
| Phase 3 | 脚本联动 + 延时 + 声音 | T2.1.4~T2.1.7 |

---

## 下一步任务

### T1.6.x - Modbus 协议实现
- [ ] 实现 Modbus RTU/ASCII 协议解析
- [ ] 实现 Modbus TCP 协议解析
- [ ] 实现 CRC/LRC 校验
- [ ] 实现读线圈 (Read Coils 0x01)
- [ ] 实现读离散输入 (Read Discrete Inputs 0x02)
- [ ] 实现读保持寄存器 (Read Holding Registers 0x03)
- [ ] 实现读输入寄存器 (Read Input Registers 0x04)
- [ ] 实现写单个线圈 (Write Single Coil 0x05)
- [ ] 实现写单个寄存器 (Write Single Register 0x06)
- [ ] 实现写多个线圈 (Write Multiple Coils 0x0F)
- [ ] 实现写多个寄存器 (Write Multiple Registers 0x10)

### T1.7.x - UI 优化
- [ ] 添加数据转换面板
- [ ] 添加寄存器批量读取/写入界面
- [ ] 添加消息追踪面板
- [ ] 优化移动端适配

---

## 技术债务
- [ ] Rust clippy 检查通过
- [ ] TypeScript strict mode
- [ ] 单元测试覆盖
- [ ] E2E 测试
- [ ] Chunk 大小优化

---

## 文档
- [x] README.md 基础说明
- [ ] API 文档
- [ ] 用户使用指南

---

_Last Updated: 2026-04-11 10:41_
