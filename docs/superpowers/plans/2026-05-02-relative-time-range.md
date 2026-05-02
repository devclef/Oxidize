# Relative Time Range Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a configurable relative time range dropdown to the main page UI so users can quickly select date ranges (7d, 30d, 3m, 6m, 1y, ytd) and custom ranges (e.g., "4 months") without manually typing dates.

**Architecture:** Backend parses env vars and injects config into `OXIDIZE_CONFIG`. Frontend builds a dropdown from the config, wires it to existing `date-utils.js` functions, and adds custom range inputs + round-end date controls.

**Tech Stack:** Rust (actix-web, serde_json), vanilla JS (Vitest + jsdom for tests), SQLite

---

## Task 1: Backend — Add config fields for TIME_RANGES and DEFAULT_TIME_RANGE

**Files:**
- Modify: `src/config.rs:27-82`

- [ ] **Step 1: Add fields to Config struct**

Add two new fields to the `Config` struct:

```rust
pub time_ranges: Vec<String>,
pub default_time_range: String,
```

Update the struct at line 27-35:

```rust
#[derive(Clone, Debug)]
pub struct Config {
    pub firefly_url: FireflyUrl,
    pub firefly_token: String,
    pub host: String,
    pub port: u16,
    pub account_types: Vec<String>,
    pub auto_fetch_accounts: bool,
    pub data_dir: String,
    pub time_ranges: Vec<String>,
    pub default_time_range: String,
}
```

- [ ] **Step 2: Parse env vars in `from_env()`**

Add parsing logic after the `data_dir` parsing (after line 70, before the `Self {` block):

```rust
// Parse TIME_RANGES: comma-separated list of relative time range presets
let time_ranges = env::var("TIME_RANGES")
    .unwrap_or_else(|_| "7d,30d,3m,6m,1y,ytd".to_string())
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect();

// Parse DEFAULT_TIME_RANGE: which preset to pre-select (default: 30d)
let default_time_range = env::var("DEFAULT_TIME_RANGE")
    .unwrap_or_else(|_| "30d".to_string());
```

- [ ] **Step 3: Add fields to Self construction**

Update the `Self { ... }` block (lines 72-80) to include the new fields:

```rust
Self {
    firefly_url,
    firefly_token,
    host,
    port,
    account_types,
    auto_fetch_accounts,
    data_dir,
    time_ranges,
    default_time_range,
}
```

- [ ] **Step 4: Run backend tests**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add src/config.rs
git commit -m "feat: add TIME_RANGES and DEFAULT_TIME_RANGE config fields

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 2: Backend — Inject config into index.html

**Files:**
- Modify: `src/handlers/index.rs:1-37`

- [ ] **Step 1: Add config fields to OXIDIZE_CONFIG**

Update the `index` function to inject `timeRanges` and `defaultTimeRange`:

Replace the `config_script` block (lines 9-20) with:

```rust
let config_script = format!(
    r#"
    <script>
        window.OXIDIZE_CONFIG = {{
            accountTypes: {},
            autoFetchAccounts: {},
            timeRanges: {},
            defaultTimeRange: "{}"
        }};
    </script>
    "#,
    serde_json::to_string(&config.account_types).unwrap_or_else(|_| "[]".to_string()),
    config.auto_fetch_accounts,
    serde_json::to_string(&config.time_ranges).unwrap_or_else(|_| "[]".to_string()),
    config.default_time_range
);
```

- [ ] **Step 2: Run backend tests**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
git add src/handlers/index.rs
git commit -m "feat: inject timeRanges and defaultTimeRange into OXIDIZE_CONFIG

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 3: Frontend — Add date-utils functions

**Files:**
- Modify: `static/date-utils.js:1-57`

- [ ] **Step 1: Add `calculateRelativeDatesFromCustom` function**

Add this new function after the existing `calculateRelativeDates` function (after line 38):

```javascript
export function calculateRelativeDatesFromCustom(count, unit) {
    const endDate = new Date();
    const startDate = new Date();
    const num = parseInt(count, 10);

    switch (unit) {
        case 'days':
            startDate.setDate(startDate.getDate() - num);
            break;
        case 'weeks':
            startDate.setDate(startDate.getDate() - (num * 7));
            break;
        case 'months':
            startDate.setMonth(startDate.getMonth() - num);
            break;
        case 'years':
            startDate.setFullYear(startDate.getFullYear() - num);
            break;
        default:
            return null;
    }

    return {
        start: startDate.toISOString().split('T')[0],
        end: endDate.toISOString().split('T')[0]
    };
}
```

- [ ] **Step 2: Add `roundEndDate` function**

Add this function after `calculateRelativeDatesFromCustom`:

```javascript
export function roundEndDate(dateStr, mode) {
    const date = new Date(dateStr);
    const now = new Date();

    switch (mode) {
        case 'start_of_current_month':
            date.setDate(1);
            date.setHours(0, 0, 0, 0);
            break;
        case 'end_of_current_month':
            date.setMonth(date.getMonth() + 1, 0);
            date.setHours(23, 59, 59, 999);
            break;
        case 'start_of_next_month':
            date.setMonth(date.getMonth() + 1, 1);
            date.setHours(0, 0, 0, 0);
            break;
        default:
            return dateStr;
    }

    return date.toISOString().split('T')[0];
}
```

- [ ] **Step 3: Verify file**

Run: `cat static/date-utils.js`
Expected: Both new functions are present after the existing ones

- [ ] **Step 4: Commit**

```bash
git add static/date-utils.js
git commit -m "feat: add calculateRelativeDatesFromCustom and roundEndDate to date-utils

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 4: Frontend — Add UI elements to index.html

**Files:**
- Modify: `static/index.html:76-102`

- [ ] **Step 1: Add time range controls section**

Replace the existing `chart-controls` div (lines 77-102) with:

```html
<div class="chart-controls">
    <!-- Time range presets -->
    <div class="time-range-controls">
        <label for="time-range-select">Time Range:</label>
        <select id="time-range-select">
            <option value="__none__">Custom</option>
        </select>
        <button id="apply-time-range-btn">Apply</button>
    </div>

    <!-- Custom relative range inputs (hidden by default) -->
    <div id="custom-range-controls" class="custom-range-controls" style="display: none;">
        <label for="custom-range-count">Last</label>
        <input type="number" id="custom-range-count" value="1" min="1" max="999" style="width:60px">
        <label for="custom-range-unit">
            <select id="custom-range-unit">
                <option value="days">days</option>
                <option value="weeks">weeks</option>
                <option value="months" selected>months</option>
                <option value="years">years</option>
            </select>
        </label>
        <button id="apply-custom-range-btn">Apply</button>
    </div>

    <!-- Round end date controls (hidden by default) -->
    <div id="round-end-controls" class="round-end-controls" style="display: none;">
        <label>
            <input type="checkbox" id="round-end-checkbox">
            Round end date to month boundary
        </label>
        <select id="round-end-mode" style="display: none;">
            <option value="start_of_current_month">Start of current month</option>
            <option value="end_of_current_month" selected>End of current month</option>
            <option value="start_of_next_month">Start of next month</option>
        </select>
    </div>

    <!-- Existing date inputs -->
    <div class="date-inputs-row">
        <label for="start-date">Start:</label>
        <input type="date" id="start-date">
        <label for="end-date">End:</label>
        <input type="date" id="end-date">
    </div>

    <label for="interval-select">Interval:</label>
    <select id="interval-select">
        <option value="auto">Auto</option>
        <option value="1D">Day</option>
        <option value="1W">Week</option>
        <option value="1M">Month</option>
        <option value="1Q">Quarter</option>
        <option value="1Y">Year</option>
    </select>
    <div class="chart-mode-toggle">
        <label class="mode-label">
            <input type="radio" name="chart-mode" value="combined" checked>
            Combined
        </label>
        <label class="mode-label">
            <input type="radio" name="chart-mode" value="split">
            Split
        </label>
    </div>
    <button id="update-chart-btn">Update Graph</button>
</div>
```

- [ ] **Step 2: Verify file**

Run: `cat static/index.html`
Expected: New time range controls are present before the date inputs

- [ ] **Step 3: Commit**

```bash
git add static/index.html
git commit -m "feat: add time range dropdown, custom range inputs, and round-end controls to UI

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 5: Frontend — Wire up UI interactions in app.js

**Files:**
- Modify: `static/app.js:2290-2550` (DOMContentLoaded section)

- [ ] **Step 1: Add time range setup code**

Add this code inside `DOMContentLoaded` (after the theme initialization around line 2300, before the fetchAccountsBtn setup):

```javascript
    // Time range setup
    const timeRangeSelect = document.getElementById('time-range-select');
    const applyTimeRangeBtn = document.getElementById('apply-time-range-btn');
    const customRangeControls = document.getElementById('custom-range-controls');
    const customRangeCount = document.getElementById('custom-range-count');
    const customRangeUnit = document.getElementById('custom-range-unit');
    const applyCustomRangeBtn = document.getElementById('apply-custom-range-btn');
    const roundEndControls = document.getElementById('round-end-controls');
    const roundEndCheckbox = document.getElementById('round-end-checkbox');
    const roundEndMode = document.getElementById('round-end-mode');
    const ROLL_END_MODE_KEY = 'oxidize_round_end_mode';

    // Build time range dropdown from config
    const timeRanges = CONFIG.timeRanges || ['7d', '30d', '3m', '6m', '1y', 'ytd'];
    const defaultTimeRange = CONFIG.defaultTimeRange || '30d';

    timeRanges.forEach(key => {
        const option = document.createElement('option');
        option.value = key;
        // Convert key to readable label: '7d' -> '7 Days', '30d' -> '30 Days', 'ytd' -> 'YTD'
        const match = key.match(/^(\d+)([dmwy])$/);
        if (match) {
            const num = match[1];
            const unit = match[2];
            const unitLabels = { d: 'Days', w: 'Weeks', m: 'Months', y: 'Years' };
            option.textContent = `${num} ${unitLabels[unit]}`;
        } else {
            option.textContent = key.toUpperCase();
        }
        if (key === defaultTimeRange) option.selected = true;
        timeRangeSelect.appendChild(option);
    });

    // Apply selected time range preset
    function applyPresetTimeRange(key) {
        const dates = calculateRelativeDates(key);
        if (!dates) return;
        document.getElementById('start-date').value = dates.start;
        document.getElementById('end-date').value = dates.end;

        // Update comparison dates if enabled
        if (typeof enableComparison !== 'undefined' && enableComparison) {
            const durationMs = new Date(dates.end) - new Date(dates.start);
            const comparisonEndDate = new Date(new Date(dates.start).getTime() - durationMs);
            const comparisonStart = new Date(comparisonEndDate.getTime() - durationMs);
            document.getElementById('comparison-start-date').value = comparisonStart.toISOString().split('T')[0];
            document.getElementById('comparison-end-date').value = comparisonEndDate.toISOString().split('T')[0];
        }

        // Apply round end date if enabled
        if (roundEndCheckbox && roundEndCheckbox.checked) {
            const roundedEnd = roundEndDate(dates.end, roundEndMode.value);
            document.getElementById('end-date').value = roundedEnd;
        }

        fetchChartData();
    }

    // Apply custom relative range
    function applyCustomRange() {
        const count = parseInt(customRangeCount.value, 10) || 1;
        const unit = customRangeUnit.value;
        const dates = calculateRelativeDatesFromCustom(count, unit);
        if (!dates) return;
        document.getElementById('start-date').value = dates.start;
        document.getElementById('end-date').value = dates.end;

        // Apply round end date if enabled
        if (roundEndCheckbox && roundEndCheckbox.checked) {
            const roundedEnd = roundEndDate(dates.end, roundEndMode.value);
            document.getElementById('end-date').value = roundedEnd;
        }

        fetchChartData();
    }

    // Time range select handler
    timeRangeSelect.addEventListener('change', () => {
        const value = timeRangeSelect.value;
        if (value === '__none__') {
            // Show custom range inputs
            customRangeControls.style.display = 'inline-flex';
            applyTimeRangeBtn.style.display = 'none';
        } else {
            // Hide custom range inputs
            customRangeControls.style.display = 'none';
            applyTimeRangeBtn.style.display = 'inline-block';
            applyPresetTimeRange(value);
        }
    });

    // Apply button for preset
    if (applyTimeRangeBtn) {
        applyTimeRangeBtn.addEventListener('click', () => {
            applyPresetTimeRange(timeRangeSelect.value);
        });
    }

    // Apply button for custom range
    if (applyCustomRangeBtn) {
        applyCustomRangeBtn.addEventListener('click', applyCustomRange);
    }

    // Round end date checkbox handler
    if (roundEndCheckbox) {
        const savedRoundMode = localStorage.getItem(ROLL_END_MODE_KEY) || 'end_of_current_month';
        roundEndMode.value = savedRoundMode;

        roundEndCheckbox.addEventListener('change', () => {
            roundEndControls.querySelector('#round-end-mode').style.display = roundEndCheckbox.checked ? 'inline-block' : 'none';
            if (roundEndCheckbox.checked) {
                // Re-apply current range with rounding
                const currentValue = timeRangeSelect.value;
                if (currentValue !== '__none__') {
                    applyPresetTimeRange(currentValue);
                } else {
                    applyCustomRange();
                }
            } else {
                // Re-apply without rounding
                const currentValue = timeRangeSelect.value;
                if (currentValue !== '__none__') {
                    applyPresetTimeRange(currentValue);
                } else {
                    applyCustomRange();
                }
            }
        });
    }

    // Round end mode change handler
    if (roundEndMode) {
        roundEndMode.addEventListener('change', () => {
            localStorage.setItem(ROLL_END_MODE_KEY, roundEndMode.value);
            // Re-apply current range with new rounding
            const currentValue = timeRangeSelect.value;
            if (currentValue !== '__none__') {
                applyPresetTimeRange(currentValue);
            } else {
                applyCustomRange();
            }
        });
    }
```

- [ ] **Step 2: Import date-utils**

Add import at the top of `app.js` (before the existing code, around line 1):

```javascript
import { calculateRelativeDates, calculateRelativeDatesFromCustom, roundEndDate } from './date-utils.js';
```

Wait — `app.js` is not a module, it's loaded as a regular script in the HTML. Check if `date-utils.js` is loaded before `app.js` in the HTML. Looking at `index.html`, it's NOT currently imported. We need to add a script tag for it.

- [ ] **Step 3: Add script tag for date-utils.js in index.html**

In `index.html`, add a script tag for `date-utils.js` before the `app.js` script tag (around line 178):

Replace:
```html
    <script src="/static/app.js"></script>
```

With:
```html
    <script src="/static/date-utils.js"></script>
    <script src="/static/app.js"></script>
```

- [ ] **Step 4: Remove import statement from app.js**

Since we're using a script tag instead of ES modules, remove the import line we added in Step 2. The functions will be available globally from `date-utils.js`.

- [ ] **Step 5: Run frontend tests**

Run: `npm test`
Expected: All existing tests pass

- [ ] **Step 6: Commit**

```bash
git add static/app.js static/index.html
git commit -m "feat: wire up time range dropdown, custom range, and round-end controls in app.js

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 6: Add frontend tests for date-utils functions

**Files:**
- Modify: `static/app.test.js`

- [ ] **Step 1: Add tests for `calculateRelativeDatesFromCustom`**

Add a new describe block at the end of the test file:

```javascript
describe('Relative Time Range', () => {
    it('should calculate dates from custom range (months)', () => {
        const dates = calculateRelativeDatesFromCustom(3, 'months');
        expect(dates).toBeDefined();
        expect(dates).toHaveProperty('start');
        expect(dates).toHaveProperty('end');
        // End date should be today
        const today = new Date().toISOString().split('T')[0];
        expect(dates.end).toBe(today);
        // Start should be ~3 months ago
        const start = new Date(dates.start);
        const end = new Date(dates.end);
        const diffMonths = (end.getFullYear() - start.getFullYear()) * 12 + (end.getMonth() - start.getMonth());
        expect(diffMonths).toBe(3);
    });

    it('should calculate dates from custom range (days)', () => {
        const dates = calculateRelativeDatesFromCustom(7, 'days');
        expect(dates).toBeDefined();
        const start = new Date(dates.start);
        const end = new Date(dates.end);
        const diffDays = Math.floor((end - start) / (1000 * 60 * 60 * 24));
        expect(diffDays).toBe(7);
    });

    it('should calculate dates from custom range (weeks)', () => {
        const dates = calculateRelativeDatesFromCustom(2, 'weeks');
        expect(dates).toBeDefined();
        const start = new Date(dates.start);
        const end = new Date(dates.end);
        const diffDays = Math.floor((end - start) / (1000 * 60 * 60 * 24));
        expect(diffDays).toBe(14);
    });

    it('should calculate dates from custom range (years)', () => {
        const dates = calculateRelativeDatesFromCustom(1, 'years');
        expect(dates).toBeDefined();
        const start = new Date(dates.start);
        const end = new Date(dates.end);
        const diffYears = end.getFullYear() - start.getFullYear();
        expect(diffYears).toBe(1);
    });

    it('should return null for invalid unit', () => {
        const dates = calculateRelativeDatesFromCustom(5, 'invalid');
        expect(dates).toBeNull();
    });
});

describe('Round End Date', () => {
    it('should round to start of current month', () => {
        const result = roundEndDate('2026-05-02', 'start_of_current_month');
        expect(result).toBe('2026-05-01');
    });

    it('should round to end of current month', () => {
        // May has 31 days
        const result = roundEndDate('2026-05-02', 'end_of_current_month');
        expect(result).toBe('2026-05-31');
    });

    it('should round to start of next month', () => {
        const result = roundEndDate('2026-05-02', 'start_of_next_month');
        expect(result).toBe('2026-06-01');
    });

    it('should return unchanged date for invalid mode', () => {
        const result = roundEndDate('2026-05-02', 'invalid');
        expect(result).toBe('2026-05-02');
    });
});
```

- [ ] **Step 2: Run frontend tests**

Run: `npm test`
Expected: All tests pass including new ones

- [ ] **Step 3: Commit**

```bash
git add static/app.test.js
git commit -m "test: add tests for relative time range and round-end date functions

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```

## Task 7: Verify end-to-end

- [ ] **Step 1: Run all tests**

Run: `cargo test` and `npm test`
Expected: All tests pass

- [ ] **Step 2: Start dev server and verify**

Run: `cargo run`
Expected: Server starts on port 8080

Open `http://localhost:8080/` in a browser and verify:
- Time range dropdown shows preset options (7 Days, 30 Days, 3 Months, etc.)
- Selecting a preset fills the date inputs and updates the chart
- "Custom" shows inline number + unit inputs
- Round end date checkbox reveals the month-boundary dropdown
- Manual date editing still works

- [ ] **Step 3: Final commit**

```bash
git add -A
git commit -m "verify: all tests pass for relative time range feature

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
"
```
