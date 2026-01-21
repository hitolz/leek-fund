# Quickstart / 快速开始

## Goal / 目标

Verify the three-column layout, list selection, fund summaries, and detail panel behavior.  
验证三列布局、列表选择、基金摘要与详情面板行为。

## Prerequisites / 前置条件

- App can run in dev mode with existing data.  
- 应用可在开发模式启动并具备现有数据。

## Steps / 步骤

1. Launch the app in development mode.  
   启动应用的开发模式。

2. Confirm the UI shows three columns: lists (left), funds (middle), details (right).  
   确认界面显示三列：左侧列表、中间基金、右侧详情。

3. Click a list in the left column.  
   点击左侧任意列表。

4. Verify the middle column updates to show only funds from that list, each row showing code, name, and daily change with a timestamp.  
   验证中间列仅显示该列表基金，且每行包含代码、名称与当日涨跌并带时间戳。

5. Click a fund in the middle column.  
   点击中间列任意基金。

6. Verify the right column shows fund detail information and a trend chart.  
   验证右侧显示基金详情与走势图。

7. Select a list with no funds (if available) and verify the empty-state message.  
   选择一个无基金的列表（若有），验证空态提示。

8. Select a fund with no trend data (if available) and verify the no-data message.  
   选择一个无走势数据的基金（若有），验证无数据提示。

## Expected Results / 预期结果

- List selection updates the middle column within 1 second under normal data size.  
- 列表选择在正常数据量下 1 秒内更新中间列。

- Fund selection updates the right column with detail info and chart (or clear empty-state).  
- 基金选择后右侧更新详情与图表（或明确空态）。

- No blank panels appear without an explanatory message.  
- 不应出现无说明的空白面板。
