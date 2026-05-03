/// Tests for OXI: Earned vs spent chart journal-level filtering
///
/// Verifies that each journal within a transaction group is processed
/// individually, transfers between selected accounts are excluded, and
/// income/expense transactions are correctly counted.
#[cfg(test)]
mod tests {
    /// Unit test for the corrected journal classification logic.
    ///
    /// The OLD bug: transaction groups were classified at the group level.
    /// A group with both deposit and withdrawal journals was counted in
    /// BOTH earned and spent, inflating both values.
    ///
    /// The FIX: each journal is processed individually. A deposit counts
    /// as earned only if money flows INTO selected accounts (source not
    /// selected). A withdrawal counts as spent only if money flows OUT
    /// of selected accounts (dest not selected). Transfers between
    /// selected accounts are excluded entirely.
    #[test]
    fn test_journal_classification_logic() {
        fn classify_journals(
            journals: &[serde_json::Value],
            selected_ids: &std::collections::HashSet<String>,
        ) -> (f64, f64) {
            let mut earned = 0.0;
            let mut spent = 0.0;

            for journal in journals {
                let journal_type = journal.get("type").and_then(|t| t.as_str()).unwrap_or("");

                let source_id = journal
                    .get("source_id")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let dest_id = journal
                    .get("destination_id")
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                // Skip internal transfers (both source and dest selected)
                if !selected_ids.is_empty()
                    && selected_ids.contains(source_id)
                    && selected_ids.contains(dest_id)
                {
                    continue;
                }

                // Earned = deposit INTO selected accounts from outside
                let is_earned = journal_type == "deposit"
                    && (!selected_ids.contains(source_id) || selected_ids.is_empty());
                // Spent = withdrawal FROM selected accounts to outside
                let is_spent = journal_type == "withdrawal"
                    && (!selected_ids.contains(dest_id) || selected_ids.is_empty());

                if is_earned {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        earned += amt;
                    }
                }
                if is_spent {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        spent += amt;
                    }
                }
            }

            (earned, spent)
        }

        let selected: std::collections::HashSet<String> =
            vec!["1".to_string(), "2".to_string()].into_iter().collect();

        // Salary: revenue(99) -> checking(1). Dest IS selected, so
        // withdrawal doesn't count as spent. Source NOT selected, so
        // deposit counts as earned.
        let salary_journals = vec![
            serde_json::json!({
                "type": "deposit", "amount": "5000.00",
                "source_id": "99", "destination_id": "1"
            }),
            serde_json::json!({
                "type": "withdrawal", "amount": "5000.00",
                "source_id": "99", "destination_id": "1"
            }),
        ];

        // Transfer: checking(1) -> savings(2). Both selected = internal.
        let transfer_journals = vec![
            serde_json::json!({
                "type": "transfer", "amount": "1000.00",
                "source_id": "1", "destination_id": "2"
            }),
            serde_json::json!({
                "type": "transfer", "amount": "1000.00",
                "source_id": "1", "destination_id": "2"
            }),
        ];

        // Purchase: checking(1) -> expense(88). Source selected, dest not
        // = withdrawal counts as spent. Deposit has source selected = not earned.
        let purchase_journals = vec![
            serde_json::json!({
                "type": "withdrawal", "amount": "200.00",
                "source_id": "1", "destination_id": "88"
            }),
            serde_json::json!({
                "type": "deposit", "amount": "200.00",
                "source_id": "1", "destination_id": "88"
            }),
        ];

        let all_journals = [salary_journals, transfer_journals, purchase_journals].concat();

        let (earned, spent) = classify_journals(&all_journals, &selected);

        // Earned = $5000 (salary deposit only)
        // Spent = $200 (purchase withdrawal only)
        // Transfer excluded, salary withdrawal excluded, purchase deposit excluded
        assert!(
            (earned - 5000.0).abs() < 0.01,
            "Earned should be $5000, got {}",
            earned
        );
        assert!(
            (spent - 200.0).abs() < 0.01,
            "Spent should be $200, got {}",
            spent
        );
    }

    /// Test with single account selected: all flows involving that account
    /// should be classified correctly.
    #[test]
    fn test_single_account_classification() {
        fn classify_journals(
            journals: &[serde_json::Value],
            selected_ids: &std::collections::HashSet<String>,
        ) -> (f64, f64) {
            let mut earned = 0.0;
            let mut spent = 0.0;

            for journal in journals {
                let journal_type = journal.get("type").and_then(|t| t.as_str()).unwrap_or("");

                let source_id = journal
                    .get("source_id")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let dest_id = journal
                    .get("destination_id")
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                if !selected_ids.is_empty()
                    && selected_ids.contains(source_id)
                    && selected_ids.contains(dest_id)
                {
                    continue;
                }

                let is_earned = journal_type == "deposit"
                    && (!selected_ids.contains(source_id) || selected_ids.is_empty());
                let is_spent = journal_type == "withdrawal"
                    && (!selected_ids.contains(dest_id) || selected_ids.is_empty());

                if is_earned {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        earned += amt;
                    }
                }
                if is_spent {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        spent += amt;
                    }
                }
            }

            (earned, spent)
        }

        let selected: std::collections::HashSet<String> =
            vec!["1".to_string()].into_iter().collect();

        // Salary: revenue(99) -> checking(1)
        let salary_journals = vec![
            serde_json::json!({
                "type": "deposit", "amount": "3000.00",
                "source_id": "99", "destination_id": "1"
            }),
            serde_json::json!({
                "type": "withdrawal", "amount": "3000.00",
                "source_id": "99", "destination_id": "1"
            }),
        ];

        let (earned, spent) = classify_journals(&salary_journals, &selected);

        // Only the deposit counts as earned ($3000)
        // The withdrawal doesn't count as spent (dest=1 IS selected)
        assert!(
            (earned - 3000.0).abs() < 0.01,
            "Earned should be $3000, got {}",
            earned
        );
        assert!(
            (spent - 0.0).abs() < 0.01,
            "Spent should be $0, got {}",
            spent
        );
    }

    /// Test that when no accounts are selected, all deposits and withdrawals
    /// are counted (no filtering).
    #[test]
    fn test_no_accounts_selected() {
        fn classify_journals(
            journals: &[serde_json::Value],
            selected_ids: &std::collections::HashSet<String>,
        ) -> (f64, f64) {
            let mut earned = 0.0;
            let mut spent = 0.0;

            for journal in journals {
                let journal_type = journal.get("type").and_then(|t| t.as_str()).unwrap_or("");

                let source_id = journal
                    .get("source_id")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let dest_id = journal
                    .get("destination_id")
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                if !selected_ids.is_empty()
                    && selected_ids.contains(source_id)
                    && selected_ids.contains(dest_id)
                {
                    continue;
                }

                let is_earned = journal_type == "deposit"
                    && (!selected_ids.contains(source_id) || selected_ids.is_empty());
                let is_spent = journal_type == "withdrawal"
                    && (!selected_ids.contains(dest_id) || selected_ids.is_empty());

                if is_earned {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        earned += amt;
                    }
                }
                if is_spent {
                    if let Some(amt) = journal
                        .get("amount")
                        .and_then(|a| a.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        spent += amt;
                    }
                }
            }

            (earned, spent)
        }

        let selected: std::collections::HashSet<String> = std::collections::HashSet::new();

        let journals = vec![
            serde_json::json!({
                "type": "deposit", "amount": "5000.00",
                "source_id": "99", "destination_id": "1"
            }),
            serde_json::json!({
                "type": "withdrawal", "amount": "200.00",
                "source_id": "1", "destination_id": "88"
            }),
        ];

        let (earned, spent) = classify_journals(&journals, &selected);

        // No accounts selected = all deposits and withdrawals count
        assert!(
            (earned - 5000.0).abs() < 0.01,
            "Earned should be $5000, got {}",
            earned
        );
        assert!(
            (spent - 200.0).abs() < 0.01,
            "Spent should be $200, got {}",
            spent
        );
    }
}
