#!/bin/bash
# 创建所有 GitHub Issues

cd ~/Project/modbus-tool

# T1: Tauri 项目初始化
gh issue create --title "[T1] Tauri 项目初始化" --body "## 任务描述
初始化 Tauri 2.x + React + Vite + TypeScript 项目

## 验收标准
- [ ] npm run tauri dev 成功启动
- [ ] 窗口正常显示
- [ ] 空白页面加载时间 < 3秒

## Agent: software-engineer | P0 | 1d" --label "P0,phase-1" 2>&1

# T2: Rust 项目结构规范
gh issue create --title "[T2] Rust 项目结构规范" --body "## 任务描述
定义 src-tauri/src/ 模块划分，错误类型，日志规范

## 验收标准
- [ ] 模块划分清晰，无循环依赖
- [ ] 错误类型统一
- [ ] 日志可正常输出

## Agent: architect | P0 | 0.5d" --label "P0,phase-1" 2>&1

# T3: modbus_core 协议栈
gh issue create --title "[T3] modbus_core 协议栈" --body "## 任务描述
实现 Modbus RTU 和 Modbus TCP 协议栈，支持功能码 01/02/03/04/05/06/15/16

## 验收标准
- [ ] RTU CRC 计算正确
- [ ] TCP MBAP 解析正确
- [ ] 功能码测试通过
- [ ] 单元测试覆盖率 > 80%

## Agent: software-engineer | P0 | 3d" --label "P0,phase-1" 2>&1

# T4: serial_port 串口模块
gh issue create --title "[T4] serial_port 串口模块" --body "## 任务描述
封装 serialport-rs，实现跨平台串口通信

## 验收标准
- [ ] 可列出系统所有串口
- [ ] 可成功打开/关闭串口
- [ ] 读写数据正常

## Agent: software-engineer | P0 | 2d" --label "P0,phase-1" 2>&1

# T5: Connection UI + IPC
gh issue create --title "[T5] Connection UI + IPC" --body "## 任务描述
连接管理面板：串口/TCP/UDP 选择，连接/断开，配置保存

## 验收标准
- [ ] 可成功建立 TCP 连接
- [ ] 可成功打开串口
- [ ] 连接状态正确显示

## Agent: ui-designer | P0 | 1d" --label "P0,phase-1" 2>&1

# T6: 基础读写功能 UI
gh issue create --title "[T6] 基础读写功能 UI" --body "## 任务描述
寄存器读取/写入面板，支持单寄存器读写

## 验收标准
- [ ] 可读取保持寄存器 (功能码 03)
- [ ] 可写入单个寄存器 (功能码 06)
- [ ] 可批量写入寄存器 (功能码 16)

## Agent: ui-designer | P0 | 1d" --label "P0,phase-1" 2>&1

# T7: 数据类型转换器
gh issue create --title "[T7] 数据类型转换器" --body "## 任务描述
数据类型转换组件：INT16/32/64、UINT、FLOAT32/64、BCD、HEX、STRING，4种字节序

## 验收标准
- [ ] Float32 转换精度误差 < 0.001
- [ ] 4种字节序转换正确
- [ ] 单元测试通过

## Agent: software-engineer | P0 | 2d" --label "P0,phase-1" 2>&1

# T8: 报文追踪面板
gh issue create --title "[T8] 报文追踪面板" --body "## 任务描述
请求/响应报文显示，时间戳，状态，过滤搜索

## 验收标准
- [ ] 报文按时间顺序显示
- [ ] 请求/响应正确配对
- [ ] 可导出 CSV

## Agent: ui-designer | P0 | 1d" --label "P0,phase-1" 2>&1

echo "Issues created successfully!"
