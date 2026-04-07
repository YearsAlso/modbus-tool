# Modbus Tool - 技术规格说明书

**版本**: v1.0  
**日期**: 2026-04-07  
**状态**: 规划中

---

## 一、协议支持

### 1.1 支持的协议类型

| 协议 | 传输层 | 状态 |
|------|--------|------|
| Modbus RTU | RS-232/RS-485 | 📋 待开发 |
| Modbus ASCII | RS-232/RS-485 | 📋 待开发 |
| Modbus TCP | TCP/IP (port 502) | 📋 待开发 |
| Modbus UDP | UDP/IP (port 502) | 📋 待开发 |

### 1.2 支持的功能码

| 功能码 | 名称 | 状态 |
|--------|------|------|
| 01 | Read Coils | 📋 待开发 |
| 02 | Read Discrete Inputs | 📋 待开发 |
| 03 | Read Holding Registers | 📋 待开发 |
| 04 | Read Input Registers | 📋 待开发 |
| 05 | Write Single Coil | 📋 待开发 |
| 06 | Write Single Register | 📋 待开发 |
| 15 | Write Multiple Coils | 📋 待开发 |
| 16 | Write Multiple Registers | 📋 待开发 |
| 22 | Mask Write Register | 📋 待开发 |
| 23 | Read/Write Multiple Registers | 📋 待开发 |

---

## 二、功能规格

### 2.1 连接管理

**串口连接**
- [ ] 自动检测可用串口
- [ ] 波特率: 300~921600
- [ ] 数据位: 5/6/7/8
- [ ] 停止位: 1/1.5/2
- [ ] 校验: None/Odd/Even/Mark/Space
- [ ] RTS/CTS 流量控制

**网络连接**
- [ ] TCP 连接 (IP:Port)
- [ ] UDP 连接 (支持广播)
- [ ] 连接超时配置
- [ ] 自动重连

### 2.2 数据类型转换

| 数据类型 | 字节序支持 | 状态 |
|----------|------------|------|
| INT16 | - | 📋 |
| UINT16 | - | 📋 |
| INT32 | ABCD/DCBA/BADC/CDAB | 📋 |
| UINT32 | ABCD/DCBA/BADC/CDAB | 📋 |
| INT64 | ABCD/DCBA/BADC/CDAB | 📋 |
| UINT64 | ABCD/DCBA/BADC/CDAB | 📋 |
| FLOAT32 | ABCD/DCBA/BADC/CDAB | 📋 |
| FLOAT64 | ABCD/DCBA/BADC/CDAB | 📋 |
| BCD | - | 📋 |
| HEX | - | 📋 |
| STRING | ASCII/UTF-8 | 📋 |

### 2.3 数据监控

- [ ] 寄存器列表 (四区域显示)
- [ ] 实时值更新
- [ ] 变化高亮
- [ ] 实时曲线
- [ ] 报文追踪 (请求/响应)

### 2.4 批量操作

- [ ] 批量读取
- [ ] 批量写入
- [ ] CSV 导入
- [ ] 预设模板

### 2.5 脚本自动化

- [ ] JavaScript 脚本引擎
- [ ] 定时任务
- [ ] 条件触发

---

## 三、验收标准

### 3.1 功能验收

- [ ] 可成功连接 Modbus TCP 设备
- [ ] 可成功连接 Modbus RTU 串口
- [ ] 支持 03/04/06/16 功能码
- [ ] 数据类型转换精度误差 < 0.001
- [ ] 报文响应时间 < 100ms

### 3.2 性能验收

- [ ] 启动时间 < 3秒
- [ ] 内存占用 < 200MB
- [ ] 支持 100ms 轮询间隔

### 3.3 兼容性验收

- [ ] Windows 10+ 构建成功
- [ ] macOS 12+ 构建成功
- [ ] Linux 构建成功

---

## 四、技术约束

- Tauri 2.0+
- Rust 1.70+
- Node.js 18+
- React 18+
