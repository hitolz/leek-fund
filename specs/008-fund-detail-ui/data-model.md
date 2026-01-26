# Data Model: Fund Detail UI Alignment

## Entities

### FundList
- id: string
- name: string
- fund_codes: string[] (6-digit codes)

### FundSummary
- code: string (6-digit)
- name: string
- daily_change: number (net change value)
- daily_change_percent: number (percent)
- daily_change_amount: number | null (computed from holding when available)
- holding_amount: number | null
- as_of: datetime

### FundDetail
- code: string (6-digit)
- name: string
- net_value: number
- net_value_date: date
- daily_change: number
- daily_change_percent: number
- daily_change_amount: number | null
- as_of: datetime
- holding_amount: number | null
- holding_shares: number | null
- cost_price: number | null (computed as holding_amount / holding_shares)

### Holding
- list_id: string
- fund_code: string (6-digit)
- holding_amount: number (2 decimal places)
- holding_shares: number (2 decimal places)
- updated_at: datetime

### SortState
- field: enum { daily_change_amount, daily_change_percent, holding_amount }
- order: enum { asc, desc, none }
- scope: enum { session_global }

### RefreshOption
- id: string
- label: string
- selected: boolean
- platform: enum { macos }

## Relationships

- FundList 1..* FundSummary (via list_id + fund_code)
- Holding belongs to FundList + Fund (composite key)
- FundDetail references Holding for the selected list and fund

## Validation Rules

- Fund codes must match ^[0-9]{6}$
- holding_amount and holding_shares support 2 decimal places
- cost_price = holding_amount / holding_shares
- If holding_shares == 0, cost_price is null and UI shows `--`
- Missing sort field values are always placed at list end

## State Transitions

- Holding: unset -> saved -> updated -> cleared
- SortState: none -> asc/desc -> none
