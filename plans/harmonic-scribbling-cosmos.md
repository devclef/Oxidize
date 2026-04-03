# Fix earned vs spent chart account filtering bug

## Context
The "earned vs spent" chart in the Oxidize dashboard currently shows data from all accounts instead of only the accounts selected by the user. The user wants the chart to strictly respect the selected account IDs.

## Problem
While the frontend and backend API handlers appear to pass the `accounts[]` parameter correctly to the `FireflyClient`, there is a suspicion that the data processing logic in `get_earned_spent` or the way transactions are fetched/filtered might be causing the issue. Specifically, if the Firefly III API returns transactions that belong to multiple accounts, the current logic might be including them even if they aren't the ones requested, or the filtering within the `transactions` sub-array might be insufficient.

## Proposed Approach

1.  **Verify API Request**: Confirm that `fetch_all_transactions` in `src/client/mod.rs` is correctly passing the `accounts[]` query parameters to the Firefly III `/v1/transactions` endpoint. (Initial investigation suggests it is).
2.  **Inspect Transaction Data Structure**: Carefully examine how the `transactions` array within the Firefly III transaction object is being used to determine if a transaction is a "deposit" or "withdrawal".
3.  **Refine Filtering Logic**: 
    - Ensure that when `account_ids` are provided, the transactions we process are indeed associated with those accounts. 
    - If the Firefly API's `accounts[]` filter is not sufficient (e.g., it returns transactions that touch *any* of the accounts, but some sub-transactions might belong to other accounts), we may need to add an explicit check within `get_earned_spent` or `fetch_all_transactions` to ensure the transaction actually belongs to the requested account(s).
4.  **Fix `get_earned_spent`**: Update the filtering logic in `src/client/mod.rs` to ensure it correctly identifies earned/spent transactions *only* for the requested accounts.
5.  **Add Test Coverage**: 
    - Create a new integration test (or update `tests/chart_integration_test.rs`) that mocks a Firefly III response containing transactions from both requested and non-requested accounts.
    - Verify that the resulting `ChartLine` only contains data derived from the requested accounts.

## Critical Files
- `src/client/mod.rs`: Core logic for fetching and processing transactions.
- `src/handlers/account.rs`: API handler for the earned-spent endpoint.
- `static/app.js`: Frontend logic for sending the request.
- `tests/chart_integration_test.rs`: For adding regression tests.

## Verification Plan
1.  **Run existing tests**: `cargo test` to ensure no regressions.
2.  **New Integration Test**: Run the newly added test case that specifically targets account filtering in the earned/spent flow.
3.  **Manual Verification (if possible)**: Since I don't have a live Firefly III instance, I will rely heavily on robust unit/integration tests with mocked JSON responses that mimic the problematic scenario.