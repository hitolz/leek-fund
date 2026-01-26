# Research: 修复当日涨跌总金额计算中的正负号处理

**Date**: 2026-01-22
**Scope**: 技术选型和实现方法研究

## 测试框架选择

### Decision
选择 **Vitest + React Testing Library** 作为前端测试框架

### Rationale
- 项目已使用Vite，Vitest是天然的选择，零配置且性能优秀
- React Testing Library是React组件测试的黄金标准，专注于用户行为
- TypeScript和ESM原生支持，适合现代前端开发
- 支持覆盖率报告和UI测试界面

### Alternatives considered
- Jest: 虽然广泛使用但在TypeScript/ESM项目中配置复杂且性能较差
- Cypress: 主要用于E2E测试，不适合单元测试
- 原生Tauri测试: 功能有限，不足以覆盖前端逻辑测试需求

## 浮点数精度处理

### Decision
使用 **decimal.js** 库处理精确的金额计算

### Rationale
- JavaScript原生浮点数存在精度问题（如0.1+0.2≠0.3）
- 金融应用需要绝对精确的计算结果
- decimal.js提供任意精度的十进制算术运算
- 轻量级且API简洁易用

### Alternatives considered
- big.js: 功能相似但API较为复杂
- 原生Number.parseFloat(): 存在浮点数精度问题，不适合金融计算
- 整数运算（乘以100）: 需要大量的转换逻辑，容易出错

## 计算逻辑验证方法

### Decision
实施**多层测试策略**：单元测试 + 集成测试 + 边缘情况测试

### Rationale
- 单元测试：验证`parseChangePercent`、`formatSignedAmount`等独立函数
- 集成测试：验证完整的数据流：API → 计算 → 显示
- 边缘情况：零值、负值、极值、无效数据等
- 回归测试：确保修复不会引入新问题

### Alternatives considered
- 仅手动测试: 无法保证回归测试和覆盖所有边缘情况
- 仅E2E测试: 反馈慢，难以定位具体问题
- 过度测试: 成本高，维护困难

## 问题根因分析

### Decision
重点检查**数据解析和格式化逻辑**中的边缘情况

### Rationale
通过代码分析发现，当前的`parseChangePercent`和总金额计算逻辑在数学上是正确的，问题可能出现在：
1. 特定数据格式的解析异常（如包含特殊字符）
2. 显示格式的用户体验问题
3. 特定API数据源的异常情况

### Alternatives considered
- 重写整个计算逻辑: 当前逻辑基本正确，重写风险大且不必要
- 仅修改显示格式: 可能遗漏真实的计算问题
- 忽略边缘情况: 可能导致用户在特定场景下遇到错误

## 实现优先级

### Decision
采用**渐进式修复策略**：诊断 → 修复 → 验证 → 优化

### Rationale
1. **Phase 1**: 添加详细的测试用例，复现和确认问题
2. **Phase 2**: 修复确认的计算和显示问题
3. **Phase 3**: 优化用户体验和错误处理
4. **Phase 4**: 性能优化和边缘情况完善

### Alternatives considered
- 一次性重构: 风险高，可能引入新问题
- 仅针对用户反馈修复: 可能遗漏系统性问题
- 大规模架构调整: 超出当前问题的范围

## 技术债务处理

### Decision
建立**金融计算的最佳实践标准**

### Rationale
- 制定精确计算的编码规范
- 建立自动化测试覆盖金融计算逻辑
- 文档化浮点数处理的注意事项
- 为未来类似问题建立预防机制

### Alternatives considered
- 临时修复: 治标不治本，问题可能再次出现
- 外包计算逻辑: 增加依赖复杂度
- 避免浮点数计算: 在当前架构下实现困难