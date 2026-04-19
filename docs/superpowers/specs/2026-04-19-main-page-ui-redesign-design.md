# Main Page UI Redesign Design Spec

**Date:** 2026-04-19
**Scope:** Main page (index.html, style.css) — clean & minimal aesthetic

## Goal

Improve the main page UI to feel modern and polished, following clean & minimal design principles (Linear, Vercel, Notion). Focus on two pain points: account selection UX and typography/spacing consistency.

## Design Decisions

### 1. Navigation & Header
- Nav bar: reduced padding, border-bottom instead of shadow, active state uses underline indicator
- Theme toggle: 36px circle, stays fixed top-right
- Page title: left-aligned, 1.5rem, lighter weight

### 2. Account Section
- **Type filter:** Replace multi-select `<select>` with horizontal pills — click to toggle, active pills filled, inactive outlined
- **Search input:** Clean search bar with magnifying glass icon for filtering accounts by name
- **Account cards:** Reduced padding, thinner left accent bar, smaller type tags, monospace balance font
- **Select All/Deselect All:** Text links instead of buttons
- **Account count badge:** Shows count next to section heading

### 3. Chart Controls
- **Primary row:** Date range, interval selector, "Update Graph" button on one compact row
- **Advanced options:** Collapsible section ("More options" toggle) containing combined/split mode, % change, comparison toggle, comparison date inputs
- **Save as widget:** Moved below chart, compact input + button

### 4. Typography & Spacing
- 8px spacing grid system (4px, 8px, 12px, 16px, 24px, 32px, 48px)
- Font sizes: 1.5rem (h1), 1.25rem (h2), 1rem (h3)
- Line heights: 1.5 body, 1.3 headings
- Border radius: 8px cards, 6px inputs, 4px small elements
- Shadows: reduced intensity (0 1px 2px rgba(0,0,0,0.05))

## Constraints
- No functional changes — all existing JS event handlers and API calls must work unchanged
- Dark mode must continue to work with new design
- Mobile responsive behavior must be preserved or improved
- CSS variables for theming must be maintained
