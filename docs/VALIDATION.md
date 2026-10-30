# Validation — 2026-10-30

- `npm run build`: passed TypeScript and Vite production build.
- `cargo test --manifest-path src-tauri/Cargo.toml`: 10 passed. Includes a real temporary Git repository, changed-file recovery, HEAD conflict, invalid task state, offline/non-Git folder, cache exclusion, lease expiry, existing handoff lock detection, SQLite audit and lock uniqueness, and local provider round trip. No automated GUI or real multi-device test.
- Windows: Tauri development window and release executable launched. The NSIS bundle was built; installation on a clean computer has not been tested.
- DEMO acceptance: initialized a separate demo Git repo, registered it in the GUI, created `DEMO - Task A`, changed a file and checkpointed, acquired a Codex lock, changed the file again without a checkpoint, closed the app and opened the release executable. Dashboard showed `RECOVERY_REQUIRED`. Inspected Git diff and snapshot. To simulate passage of the lease period without an eight-hour wait, only the demo lock's expiration was set in local SQLite to a past time; confirmed the simulated agent was stopped, took the lock as `dsh`, recorded a recovery checkpoint, resumed, modified a file, checkpointed again and finished. Read back task status `completed`, agent `dsh`, three checkpoints, audit events and an empty lock table. This is a same-device simulation, not a real DSH process or cross-device test.
- After this scenario, added extra conflict/lease checks and reran automated tests; the full GUI scenario has not been repeated on the final binary.
- The demo workspace and audit records remain in the local app database and are labelled DEMO. The demo Git repository is under ignored `work/demo-workspace`.

Known gaps: local leases do not protect other computers or parallel sessions using the same identity; no heartbeat or automatic interrupted detection; existing agent-handoff YAML is read-only and its locks/checkpoints are not integrated; live sync and NAS are unimplemented; app creation/checkpoint/finish operations are not all transactional; Windows install/uninstall, missing Git executable, corrupted database, and clean-machine execution have not been tested. Conflict mode currently requires external resolution before a new checkpoint and has no in-app override.

Resume and stale-lock takeover refuse a workspace containing an existing `.agent-state/locks/*.lock`; detection is conservative and does not replace integration with that protocol. Git index content changes that preserve the same status and working-file bytes are not fingerprinted separately.

Final release executable was launched after the last rebuild; its dashboard displayed the completed DEMO task and local audit feed. The full recovery sequence was performed on the preceding release build, not rerun after the final read-only scan and lock-guard changes.

## 中文部署验证（2026-10-30）

- 中文界面、状态标签和常见错误提示已加入；`npm run build` 和 10 项 Rust 测试通过。
- NSIS 中文安装包构建成功，并以静默模式安装，安装进程返回 0。安装程序未经过交互式安装和干净电脑验证。
- 安装后的中文总览、已有 DEMO 记录和审计活动已实际打开检查。由于本机 Codex 打包环境对默认应用数据目录存在重定向，另将已安装程序副本放在固定目录，并将本机桌面和开始菜单快捷方式指向该副本；此步骤不是安装器在普通机器上的通用行为。
