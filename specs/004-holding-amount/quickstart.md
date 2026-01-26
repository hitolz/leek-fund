# Quickstart / 快速开始

## Goal / 目标

Verify holding amount setup in the right detail panel, scoped by group + fund, and daily change amount display.  
验证右侧详情面板的持仓设置（按分组+基金），以及当日涨跌金额展示。

## Prerequisites / 前置条件

- App can run in dev mode with at least one list and one fund.  
  应用可在开发模式启动，且至少有一个列表与一个基金。
- Fund daily change data is available for the selected fund.  
  所选基金有可用的当日涨跌数据。

## Steps / 步骤

1. Launch the app in development mode.  
   启动应用的开发模式。

2. Select a list in the left column and a fund in the middle column.  
   在左侧选择一个列表，在中间选择一个基金。

3. In the right detail panel, enter holding shares and cost price, then save.  
   在右侧详情面板输入持仓份额与成本价并保存。

4. Verify the holding amount appears and is associated with the selected group and fund.  
   验证持仓金额显示，并且与当前分组和基金绑定。

5. Verify the daily change amount is shown based on the current daily change data.  
   验证当日涨跌金额基于当前涨跌数据展示。

6. Switch to another list that contains the same fund (if available) and verify holdings are independent.  
   切换到包含同一基金的另一个列表（若有），验证持仓独立。

7. Clear the holding info and confirm the placeholder is shown.  
   清空持仓信息并确认显示占位文本。

## Expected Results / 预期结果

- Holding info persists for each group-fund pair and reloads on selection.  
  持仓信息按分组-基金持久化并在选择时加载。

- Daily change amount displays when daily change data is available; otherwise, an unavailable state is shown.  
  当日涨跌数据可用时显示当日涨跌金额；不可用时显示不可用状态。

- Clearing holding info removes the displayed holding amount for that group-fund pair.  
  清空后该分组-基金的持仓金额不再显示。
