# Quickstart: Fund Detail UI Alignment

## Prerequisites

- Node.js 18+
- Rust 1.70+

## Run

```bash
npm run tauri:dev
```

## Manual Verification Checklist

1. Open the app and select a list with funds.
2. Confirm the three-column layout matches the reference image.
3. In the middle list, verify sort controls support:
   - Fields: daily change amount, daily change percent, holding amount
   - Orders: descending, ascending, no sort
4. Confirm missing sort values appear at the end of the list.
5. In the detail panel, enter holding amount and holding shares:
   - Cost price updates as amount / shares
   - When shares are zero, cost price shows `--` with a clear message
6. Save and clear holding inputs; verify the summary updates.
7. On macOS, open the menu bar refresh options and confirm the selected option
   shows a checkmark.
