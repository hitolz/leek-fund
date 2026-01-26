# Research: Fund Detail UI Alignment

## Decisions

### Decision 1: Use backend-provided sort fields
- Decision: Holding amount and daily change amount used for sorting are returned
  from the backend; the UI does not compute them.
- Rationale: Ensures deterministic calculations and aligns with the constitution
  rule that Rust owns data and business logic.
- Alternatives considered: Compute values on the frontend (rejected for
  consistency and data integrity concerns).

### Decision 2: Missing values sort to end
- Decision: Any missing sort field values are placed at the end of the list for
  both ascending and descending order.
- Rationale: Prevents missing data from skewing visible rankings.
- Alternatives considered: Treat missing as zero; disable sorting when missing
  values exist.

### Decision 3: Cost price zero-share handling
- Decision: When holding shares are zero, cost price displays `--` and an explicit
  message indicates zero shares.
- Rationale: Avoids divide-by-zero and prevents misleading numeric output.
- Alternatives considered: Display 0; block saving until corrected.

### Decision 4: Input precision for holding values
- Decision: Holding amount supports 2 decimal places; holding shares supports 2
  decimal places.
- Rationale: Matches common fund entry conventions and keeps input simple.
- Alternatives considered: Integer-only; 4-decimal shares.

### Decision 5: Sort state scope
- Decision: Sort state is global within the session and persists across list
  switches (not across app restarts).
- Rationale: Predictable behavior with minimal complexity.
- Alternatives considered: Per-list sort state; reset on each list change;
  persistent sort across sessions.

### Decision 6: macOS refresh controls
- Decision: Refresh controls live in the macOS menu bar with a checkmark to
  indicate the selected option.
- Rationale: Matches platform conventions and user request.
- Alternatives considered: In-window refresh controls only.

## References

- `/Users/hitol/code/ai/leek-fund/specs/004-holding-amount/spec.md`
- `/Users/hitol/code/ai/leek-fund/specs/005-fix-amount-calculation/spec.md`
- `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`
