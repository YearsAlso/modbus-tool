#!/bin/bash
# Modbus Tool - Agent 定时执行脚本
# 由 launchd 或 cron 定时调用

PROJECT_DIR="$HOME/Project/modbus-tool"
LOG_FILE="$PROJECT_DIR/.agent-execution.log"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

log "=== Agent Workflow Started ==="

# 1. 更新本地仓库
cd "$PROJECT_DIR"
git pull origin main 2>&1 | tee -a "$LOG_FILE"

# 2. 获取待认领的 P0 任务
TASK=$(gh issue list --repo YearsAlso/modbus-tool --state open --label "P0" --json number,title --jq '.[0]')

if [ -n "$TASK" ]; then
    log "Found P0 task: $TASK"
    # 这里可以调用 subagent 来执行任务
    # 目前仅记录日志
else
    log "No pending P0 tasks"
fi

# 3. 检查 CI 状态
CI_STATUS=$(gh run list --repo YearsAlso/modbus-tool --limit 1 --json status,conclusion --jq '.[0]')
log "CI Status: $CI_STATUS"

# 4. 生成进度报告
gh issue list --repo YearsAlso/modbus-tool --state open --json number,title,labels --jq '.[] | "\(.number) \(.title)"' >> "$LOG_FILE"

log "=== Agent Workflow Completed ==="
