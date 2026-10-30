# LKMT Agent Hub

面向开发团队的 Windows 本地工作区检查与智能体任务记录工具。当前版本为 **0.1.0-alpha**，界面为中文。

> 当前版本适合本机试用。锁和任务状态仅保存在本机，尚不能用于保证多台电脑之间的互斥。

## 功能

- 登记本地工作区，检查 Git 分支、HEAD、修改文件、差异、远程地址和最近提交。
- 读取 `AGENTS.md`、已有 agent-handoff 任务与锁；支持常见 Unreal、Unity、Godot 目录识别。
- 创建本机任务和检查点，比较工作区与检查点，提示冲突或需要恢复。
- 记录本机任务交接、锁和审计日志。过期锁接管需要人工检查确认。
- 显示「已核实」「报告」「推断」，避免将 Agent 自述当作已验证事实。

截图：待补充。

## 安装和使用

运行 Windows x64 安装包，或直接运行构建出的可执行文件。详细步骤和恢复说明见 [使用教程](docs/使用教程.md)。安装包目前未签名，也未在干净电脑上验证。

## 开发

需要 Windows、Node.js、Rust、Microsoft C++ Build Tools 和 WebView2：

```powershell
npm install
npm run tauri dev
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

## 架构与安全

界面采用 React 和 TypeScript，桌面端采用 Tauri 2 与 Rust，本机数据保存在 SQLite。Git 操作以只读为主；不会自动执行 reset、push、删除项目文件或读取私人 AI 聊天记录。大文件缓存目录不作为文件指纹的重要证据。

本机数据库包含工作区路径和任务说明，不应提交到 Git。接管时必须检查实际文件、Git 和项目规则；租约过期不证明原 Agent 已停止。Git 暂存区的部分变化尚未单独指纹化。

## Agent 协议

Hub 任务和检查点使用本机 SQLite 中的 JSON 数据。已有 `.agent-state/active/*.yaml` 和 `.agent-state/locks/*.lock` 只读展示；发现原协议锁时会保守阻止 Hub 恢复或接管。Hub 尚不能替代项目原有 agent-handoff 命令。同步和存储接口见 [providers.md](docs/providers.md)。

## 计划

- 与现有 agent-handoff 协议双向集成。
- 使用版本化状态文件实现 GitHub 同步与跨设备锁，并进行真实双机测试。
- 提供设备心跳、自动中断识别和检查点历史。
- 后续研究 NAS 资产存储；当前没有 NAS 管理功能。

## 参与贡献

请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。本项目采用 [MIT 许可证](LICENSE)。验证结果与已知限制见 [VALIDATION.md](docs/VALIDATION.md)。
