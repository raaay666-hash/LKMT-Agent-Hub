# Validation — 2026-10-30

- `npm run build`: passed TypeScript and Vite production build.
- `cargo test --manifest-path src-tauri/Cargo.toml`: 10 passed. Includes a real temporary Git repository, changed-file recovery, HEAD conflict, invalid task state, offline/non-Git folder, cache exclusion, lease expiry, existing handoff lock detection, SQLite audit and lock uniqueness, and local provider round trip. No automated GUI or real multi-device test.
- Windows: Tauri development window and release executable launched. The NSIS bundle was built; installation on a clean computer has not been tested.
- DEMO acceptance: initialized a separate demo Git repo, registered it in the GUI, created `DEMO - Task A`, changed a file and checkpointed, acquired a Codex lock, changed the file again without a checkpoint, closed the app and opened the release executable. Dashboard showed `RECOVERY_REQUIRED`. Inspected Git diff and snapshot. To simulate passage of the lease period without an eight-hour wait, only the demo lock's expiration was set in local SQLite to a past time; confirmed the simulated agent was stopped, took the lock as `dsh`, recorded a recovery checkpoint, resumed, modified a file, checkpointed again and finished. Read back task status `completed`, agent `dsh`, three checkpoints, audit events and an empty lock table. This is a same-device simulation, not a real DSH process or cross-device test.
- After this scenario, added extra conflict/lease checks and reran automated tests; the full GUI scenario has not been repeated on the final binary.
- The demo workspace and audit records remain in the local app database and are labelled DEMO. The demo Git repository is under ignored `work/demo-workspace`.

Known gaps: local leases do not protect other computers or parallel sessions using the same identity; no heartbeat or automatic interrupted detection; existing agent-handoff YAML is read-only and its locks/checkpoints are not integrated; live sync and NAS are unimplemented; app creation/checkpoint/finish operations are not all transactional; Windows install/uninstall, missing Git executable, corrupted database, and clean-machine execution have not been tested. 原版本中 HEAD 变化需要外部处理；0.1.1 加入人工审查后的同分支快进检查点更新，仍拒绝未审查、脏工作区和历史分叉。

Resume and stale-lock takeover refuse a workspace containing an existing `.agent-state/locks/*.lock`; detection is conservative and does not replace integration with that protocol. Git index content changes that preserve the same status and working-file bytes are not fingerprinted separately.

Final release executable was launched after the last rebuild; its dashboard displayed the completed DEMO task and local audit feed. The full recovery sequence was performed on the preceding release build, not rerun after the final read-only scan and lock-guard changes.

## 中文部署验证（2026-10-30）

- 中文界面、状态标签和常见错误提示已加入；`npm run build` 和 10 项 Rust 测试通过。
- NSIS 中文安装包构建成功，并以静默模式安装，安装进程返回 0。安装程序未经过交互式安装和干净电脑验证。
- 安装后的中文总览、已有 DEMO 记录和审计活动已实际打开检查。由于本机 Codex 打包环境对默认应用数据目录存在重定向，另将已安装程序副本放在固定目录，并将本机桌面和开始菜单快捷方式指向该副本；此步骤不是安装器在普通机器上的通用行为。

## 0.1.1-alpha 人工交接验证

- `cargo test --manifest-path src-tauri/Cargo.toml`：13 项通过，覆盖交接文件创建且不覆盖、损坏文件、仓库地址规范化和凭据拒绝，以及真实临时 Git 仓库中的目标仓库、分支、HEAD、脏工作区、外部锁和已审查快进检查。
- `npm run build`、Windows x64 NSIS 构建通过。发布版窗口已启动，任务页面已显示人工交接入口和安全提示。
- 尚未在两台实体电脑上完成导出、传输、预览、导入与恢复的完整 GUI 流程；也没有联网同步或自动分布式锁测试。版本仍为 alpha。
- JSON 未签名，仅包含任务报告和 Git 标识。没有传输代码，Git 忽略文件、LFS 对象和是否推送均需人工验证。源与目标检查发生在单机上，不能阻止另一台电脑同时修改。
