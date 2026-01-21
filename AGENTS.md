# leek-fund Development Guidelines / leek-fund 开发指南

Auto-generated from all feature plans. Last updated: 2026-01-21  
由所有功能计划自动生成。最后更新：2026-01-21

## Active Technologies / 当前技术栈

- Rust 1.70+, TypeScript (React 18), Tauri 1.5, Vite 5  
  Rust 1.70+、TypeScript（React 18）、Tauri 1.5、Vite 5
- React 18, @tauri-apps/api, serde/serde_json, reqwes  
  React 18、@tauri-apps/api、serde/serde_json、reqwes
- Local JSON file in Tauri app data directory  
  Tauri 应用数据目录中的本地 JSON 文件

## Project Structure / 项目结构

```text
src/
src-tauri/
tests/
```

## Commands / 命令

cargo test, cargo clippy, npm run tauri:dev, npm run tauri:build  
需要时使用：cargo test、cargo clippy、npm run tauri:dev、npm run tauri:build

## Code Style / 代码风格

Rust 1.70+, TypeScript (React 18): Follow standard conventions  
Rust 1.70+、TypeScript（React 18）：遵循标准约定

## Recent Changes / 最近变更

- 002-fund-detail-layout: Added three-column fund detail layout planning artifacts.  
  002-fund-detail-layout：新增三列基金详情布局的规划文档。
- 001-fund-tracker-client: Added Rust + Tauri + React client stack and initial specs.  
  001-fund-tracker-client：新增 Rust + Tauri + React 客户端技术栈及初始规格。

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->


生成的文档如果是英文的，需要按段落翻译为中文，段落对应