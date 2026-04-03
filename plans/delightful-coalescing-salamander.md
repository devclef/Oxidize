# Plan for Fixing OXI Tickets

## Context
This plan addresses three tickets for the OXI project:
- **OXI-5 (Robust Testing):** Implement a comprehensive testing framework for both backend and frontend, update GitHub Actions, and establish a TDD-oriented workflow.
- **OXI-7 (Update CLAUDE.md):** Update project documentation to reflect new testing standards and procedures.
- **OXI-8 (Summarize WOR tickets):** This ticket appears to be an external task related to a different project ("WOR"). Since there is no context for "WOR" in this repository, I will flag this as out-of-scope for this repository and suggest the user provides more context or handles it separately.

## Implementation Approach

### 1. Backend Testing (OXI-5)
- **Unit Tests:** Implement unit tests for core modules in `src/`:
    - `src/client/mod.rs`: Test `FireflyClient` methods with mocked responses.
    - `src/models/`: Test JSON serialization/deserialization.
    - `src/handlers/`: Test API endpoint logic.
- **Integration Tests:** Expand `tests/` with more comprehensive scenarios.
- **Mocking:** Use `mockito` or similar to mock the Firefly III API during testing.

### 2. Frontend Testing (OXI-5)
- **Setup:** Initialize a minimal Node.js environment with `package.json`.
- **Framework:** Implement `Vitest` (fast, modern) and `jsdom` for testing `static/app.js` logic.
- **Tests:** Write tests for:
    - `fetchAccounts()`
    - `fetchChartData()`
    - Data manipulation logic and `localStorage` interaction.

### 3. CI/CD Update (OXI-5)
- **GitHub Action:** Create `.github/workflows/test.yml`.
- **Pipeline:** The workflow will:
    1. Check out code.
    2. Set up Rust toolchain and run `cargo test`.
    3. Set up Node.js and run frontend tests via `npm test`.

### 4. Documentation Update (OXI-7)
- **CLAUDE.md:**
    - Add a **Testing** section.
    - Add commands for running backend and frontend tests.
    - Include guidance on the TDD approach and validation requirements.
    - Update **Editing Guidelines** to mandate testing.

### 5. OXI-8 Handling
- I will add a comment to the ticket (via the user or noting it in the final report) that it is an external task and requires context from the `WOR` project.

## Critical Files to be Modified
- `src/client/mod.rs` (and others in `src/`)
- `static/app.js`
- `package.json` (New)
- `.github/workflows/test.yml` (New)
- `CLAUDE.md`
- `Cargo.toml` (to add test dependencies like `mockito`)

## Verification Plan
- **Backend:** Run `cargo test` and ensure all new and existing tests pass.
- **Frontend:** Run `npm test` and ensure all new and existing tests pass.
- **CI Simulation:** Verify the GitHub Action workflow file is syntactically correct.
- **Documentation:** Manually verify `CLAUDE.md` reflects the new commands and guidelines.