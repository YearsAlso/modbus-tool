# Modbus Tool - 开发任务详解

> 任务分配给各 Agent 定时执行

---

## Phase 1: 基础架构 (M1)

### T1: Tauri 项目初始化

**文件**: `tasks/T1-tauri-init.md`

```
Agent: software-engineer
优先级: P0
工时: 1d
依赖: 无
```

**任务内容**:
1. 初始化 Tauri 2.x + React + Vite + TypeScript 项目
2. 配置 shadcn/ui + Tailwind CSS
3. 配置 Cargo.toml 依赖 (tokio, serialport, serde_json, rusqlite, thiserror, tracing)
4. 配置 Tauri capabilities (serial port access)
5. 设置项目目录结构

**验收标准**:
- [ ] `npm run tauri dev` 成功启动
- [ ] 窗口正常显示
- [ ] 空白页面加载时间 < 3秒

---

### T2: Rust 项目结构规范

**文件**: `tasks/T2-rust-structure.md`

```
Agent: architect
优先级: P0
工时: 0.5d
依赖: 无 (可并行)
```

**任务内容**:
1. 定义 `src-tauri/src/` 模块划分
2. 创建 `mod.rs` 模块导出
3. 定义错误类型 (`thiserror`)
4. 配置日志 (`tracing` + `tracing-subscriber`)
5. 定义 Tauri IPC 接口契约

**验收标准**:
- [ ] 模块划分清晰，无循环依赖
- [ ] 错误类型统一
- [ ] 日志可正常输出

---

### T3: modbus_core 协议栈

**文件**: `tasks/T3-modbus-core.md`

```
Agent: software-engineer
优先级: P0
工时: 3d
依赖: T2 (Rust 项目结构)
```

**任务内容**:
1. 实现 Modbus RTU 协议
   - CRC16 计算
   - 报文组包/解包
   - 字节序处理

2. 实现 Modbus TCP 协议
   - MBAP 报文头
   - 事务ID管理
   - 连接管理

3. 实现功能码 (01/02/03/04/05/06/15/16)

**验收标准**:
- [ ] RTU CRC 计算正确 (参考 modbus.org 测试数据)
- [ ] TCP MBAP 解析正确
- [ ] 功能码 03/04/06/16 测试通过
- [ ] 单元测试覆盖率 > 80%

---

### T4: serial_port 串口模块

**文件**: `tasks/T4-serial-port.md`

```
Agent: software-engineer
优先级: P0
工时: 2d
依赖: T2 (Rust 项目结构)
```

**任务内容**:
1. 封装 `serialport-rs` crate
2. 实现串口列表获取
3. 实现串口打开/关闭
4. 实现串口读写 (异步)
5. 处理 Modbus RTU 帧边界

**验收标准**:
- [ ] 可列出系统所有串口
- [ ] 可成功打开/关闭串口
- [ ] 读写数据正常
- [ ] RTU 帧边界正确

---

### T5: Connection UI + IPC

**文件**: `tasks/T5-connection-ui.md`

```
Agent: ui-designer
优先级: P0
工时: 1d
依赖: T3 + T4
```

**任务内容**:
1. 创建连接管理面板
2. 串口连接表单
3. TCP/UDP 连接表单
4. Zustand connectionStore
5. Tauri IPC 命令绑定

**验收标准**:
- [ ] 可成功建立 TCP 连接
- [ ] 可成功打开串口
- [ ] 连接状态正确显示
- [ ] 断开连接正常

---

### T6: 基础读写功能 UI

**文件**: `tasks/T6-read-write-ui.md`

```
Agent: ui-designer
优先级: P0
工时: 1d
依赖: T5
```

**任务内容**:
1. 寄存器读取表单 (地址 + 数量)
2. 单寄存器写入表单
3. 多寄存器写入表单
4. useModbus Hook 封装
5. 错误处理和提示

**验收标准**:
- [ ] 可读取保持寄存器 (功能码 03)
- [ ] 可写入单个寄存器 (功能码 06)
- [ ] 可批量写入寄存器 (功能码 16)
- [ ] 错误信息清晰

---

### T7: 数据类型转换器

**文件**: `tasks/T7-data-converter.md`

```
Agent: software-engineer
优先级: P0
工时: 2d
依赖: T3
```

**任务内容**:
1. Rust 端实现数据类型转换
   - INT16/UINT16
   - INT32/UINT32 (4种字节序)
   - FLOAT32 (IEEE 754)
   - FLOAT64
   - BCD
   - HEX
   - STRING

2. React 端转换组件
3. 实时预览

**验收标准**:
- [ ] Float32 转换精度误差 < 0.001
- [ ] 4种字节序转换正确
- [ ] BCD 编码/解码正确
- [ ] 单元测试通过

---

### T8: 报文追踪面板

**文件**: `tasks/T8-message-trace.md`

```
Agent: ui-designer
优先级: P0
工时: 1d
依赖: T3
```

**任务内容**:
1. 报文列表组件
2. 请求/响应配对显示
3. 时间戳显示
4. 状态显示 (OK/Error)
5. 过滤/搜索功能
6. 日志导出

**验收标准**:
- [ ] 报文按时间顺序显示
- [ ] 请求/响应正确配对
- [ ] 可过滤功能码
- [ ] 可导出 CSV

---

## Phase 2: 增强功能 (M2-M3)

### T9: Modbus UDP 支持

```
Agent: software-engineer
优先级: P1
工时: 1d
依赖: T3
```

### T10: Modbus ASCII 支持

```
Agent: software-engineer
优先级: P1
工时: 1d
依赖: T3
```

### T11: 寄存器监视器

```
Agent: ui-designer
优先级: P1
工时: 2d
依赖: T6
```

### T12: 实时曲线

```
Agent: ui-designer
优先级: P1
工时: 2d
依赖: T11
```

### T13: 批量读写

```
Agent: ui-designer
优先级: P1
工时: 1.5d
依赖: T6
```

### T14: 数据存储 (SQLite)

```
Agent: software-engineer
优先级: P1
工时: 1.5d
依赖: T1
```

## Phase 3: 高级功能 (M4-M5)

### T15: 脚本自动化

```
Agent: software-engineer
优先级: P2
工时: 4d
依赖: M2 完成
```

### T16: 协议分析工具

```
Agent: software-engineer
优先级: P2
工时: 2d
依赖: T8
```

### T17: 协议一致性测试

```
Agent: tester
优先级: P2
工时: 3d
依赖: T16
```

### T18: 数据导出

```
Agent: ui-designer
优先级: P2
工时: 1d
依赖: T11
```

---

## 测试任务

### TEST-RTU: Modbus RTU 协议测试

```
Agent: tester
优先级: P0
工时: 1d
依赖: T3 + T4
```

**验收标准**:
- [ ] CRC 计算测试 (边界值)
- [ ] 功能码 03/04/06/16 测试
- [ ] 异常响应处理测试

### TEST-TCP: Modbus TCP 协议测试

```
Agent: tester
优先级: P0
工时: 1d
依赖: T3
```

**验收标准**:
- [ ] MBAP 解析测试
- [ ] 事务ID去重测试
- [ ] 超时测试

### TEST-CONVERTER: 数据类型转换测试

```
Agent: tester
优先级: P0
工时: 0.5d
依赖: T7
```

**验收标准**:
- [ ] Float32 精度测试
- [ ] 字节序转换测试
- [ ] BCD 边界测试

### TEST-BUILD: 跨平台构建测试

```
Agent: tester
优先级: P1
工时: 0.5d
依赖: T1
```

**验收标准**:
- [ ] macOS .app 生成
- [ ] Windows .exe 生成
- [ ] Linux AppImage 生成

---

## 任务依赖图

```
T1 ─┬─ T3 ─┬─ T5 ─┬─ T6 ─┬─ T11 ─┬─ T12
    │       │       │       │        │
    │       │       │       │        └─ T18
    │       │       │       │
    │       │       │       └─ T13
    │       │       │
    │       │       └─ T7 ── TEST-CONVERTER
    │       │
    │       └─ T4 ── TEST-RTU
    │
    └─ T2 ──┬─ T14
            │
            └─ TEST-BUILD

T3 ── T9 (UDP)
T3 ── T10 (ASCII)
T3 ── TEST-TCP
T8 ── T16 ── T17
T11 ── T12
```

---

*最后更新: 2026-04-07*
