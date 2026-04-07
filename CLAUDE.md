# CLAUDE.md - Modbus Tool

## 项目概述

现代化 Modbus 协议调试工具，基于 Tauri 2.0 + React + Rust。

## 技术栈

- **桌面框架**: Tauri 2.0
- **前端**: React 18 + TypeScript + Vite
- **UI**: shadcn/ui + Tailwind CSS
- **状态管理**: Zustand
- **后端**: Rust + tokio

## 目录结构

```
modbus-tool/
├── src/                    # React 前端
│   ├── components/          # UI 组件
│   │   ├── Connection/      # 连接管理
│   │   ├── Monitor/         # 数据监控
│   │   ├── Converter/       # 类型转换
│   │   └── Trace/           # 报文追踪
│   ├── hooks/              # 自定义 Hooks
│   ├── stores/             # Zustand stores
│   └── utils/              # 工具函数
├── src-tauri/             # Rust 后端
│   └── src/
│       ├── modbus_core/   # Modbus 协议栈
│       ├── serial_port/   # 串口通信
│       ├── data_store/    # 数据存储
│       └── commands/      # Tauri 命令
├── tests/                 # 测试
│   ├── unit/              # 单元测试
│   └── integration/        # 集成测试
└── docs/                  # 文档
```

## 开发命令

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 类型检查
npm run typecheck

# 测试
npm test

# 构建
npm run tauri build
```

## 关键规范

### Rust 代码风格
- 使用 `cargo clippy` 检查
- 遵循 Rust API guidelines
- 单元测试覆盖率 > 80%

### TypeScript 代码风格
- 遵循 ESLint + Prettier 配置
- 组件使用函数式 + Hooks
- 类型必须显式声明

### Git 提交规范
```
[type]: [描述]

[type]: feat | fix | docs | style | refactor | test | chore
```

## 任务管理

- 任务池: `TODO.md`
- 详细任务: `docs/tasks.md`
- Agent 调度: `AGENT_SCHEDULE.md`

## 相关文档

- [SPEC.md](./SPEC.md) - 技术规格
- [docs/tasks.md](./docs/tasks.md) - 详细任务分解
- [AGENT_SCHEDULE.md](./AGENT_SCHEDULE.md) - Agent 执行计划
