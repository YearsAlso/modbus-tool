# Modbus Tool - 开发任务清单

## 项目信息
- **项目名称**: Modbus Tool
- **类型**: Modbus 协议调试工具
- **技术栈**: Tauri 2.0 + React 18 + TypeScript + Vite
- **状态**: 开发中

---

## 前端基础架构 (T1.2.x)

### T1.2.1 - 设计前端目录结构 ✅ 已完成 (2026-04-07)
- [x] 在 src/ 下创建 components/、pages/、hooks/、stores/、utils/、types/ 目录
- [x] 创建 index.ts 导出文件
- [x] 创建 lib/ 目录 (工具函数和 IPC 封装)

### T1.2.2 - 配置 Tailwind CSS + shadcn/ui ✅ 已完成 (2026-04-07)
- [x] 创建 package.json 并配置依赖
- [x] 安装 Tailwind CSS
- [x] 安装并配置 shadcn/ui 组件库 (Button, Card, Input, Label, Select)
- [x] 配置 tailwind.config.js
- [x] 配置 postcss.config.js
- [x] 配置 vite.config.ts
- [x] 配置 tsconfig.json

### T1.2.3 - 创建基础布局组件 ✅ 已完成 (2026-04-07)
- [x] 创建 AppLayout 组件（侧边栏+主内容区）
- [x] 创建 ConnectionPanel 组件（串口/TCP 连接区）
- [x] 创建基础 UI 组件 (Button, Card, Input, Label, Select)

### T1.2.4 - 配置 Zustand 状态管理 ✅ 已完成 (2026-04-07)
- [x] 安装 zustand
- [x] 创建 connection store（连接状态、消息记录）
- [x] 创建 settings store（配置状态、持久化）
- [x] 创建 monitor store（监控点、实时数据）

### T1.2.5 - 创建 Tauri IPC 命令封装 ✅ 已完成 (2026-04-07)
- [x] 在 src/lib/ipc.ts 创建 IPC 封装
- [x] 定义 invoke 调用的类型安全封装
- [x] 实现 connect/disconnect/modbus 操作命令

### T1.2.6 - 配置前端路由与页面结构 ✅ 已完成 (2026-04-07)
- [x] 安装 react-router-dom
- [x] 配置路由：/connection、/monitor、/settings
- [x] 创建 ConnectionPage（连接管理）
- [x] 创建 MonitorPage（实时监控）
- [x] 创建 SettingsPage（设置页面）

### T1.2.7 - 创建全局类型定义 ✅ 已完成 (2026-04-07)
- [x] 在 types/ 下创建 Modbus 相关类型
- [x] 定义 ModbusRequest、ModbusResponse、ConnectionConfig
- [x] 定义 ConnectionStatus、MonitorPoint 等

---

## 下一步任务

### T1.3.x - 核心功能开发
- [ ] 实现 Modbus 读/写功能 (readRegisters, writeRegister)
- [ ] 实现数据格式转换 (INT, UINT, FLOAT, BCD, HEX, STRING)
- [ ] 实现消息日志功能
- [ ] 实现寄存器列表管理

### T1.4.x - UI 优化
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

---

## 文档
- [x] README.md 基础说明
- [ ] API 文档
- [ ] 用户使用指南

---

_Last Updated: 2026-04-07 10:30_
