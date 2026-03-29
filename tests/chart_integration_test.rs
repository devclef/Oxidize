#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_chart_data_with_earned_spent() {
        // Simulate Firefly III API response with earned/spent data
        let json_response = r#"
        [
            {
                "label": "earned",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "0",
                    "2026-02-27T00:00:00+00:00": "0",
                    "2026-02-28T00:00:00+00:00": "2500.00"
                }
            },
            {
                "label": "spent",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "-1000",
                    "2026-02-27T00:00:00+00:00": "-2000",
                    "2026-02-28T00:00:00+00:00": "-3000"
                }
            },
            {
                "label": "SEB Checking",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "10000",
                    "2026-02-27T00:00:00+00:00": "10500",
                    "2026-02-28T00:00:00+00:00": "11000"
                }
            },
            {
                "label": "Klarna Card",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "-5000",
                    "2026-02-27T00:00:00+00:00": "-5200",
                    "2026-02-28T00:00:00+00:00": "-5500"
                }
            }
        ]
        "#;

        // Parse the JSON
        let chart_line: serde_json::Value = serde_json::from_str(json_response).unwrap();

        // Simulate filtering logic
        let mut filtered_data: Vec<serde_json::Value> = Vec::new();
        let mut seen_labels = std::collections::HashSet::new();

        for dataset in chart_line.as_array().unwrap() {
            let label = dataset.get("label").unwrap().as_str().unwrap();

            // Skip aggregated "earned"/"spent" labels
            if label == "earned" || label == "spent" {
                continue;
            }

            // Only include datasets we haven't seen yet
            if seen_labels.insert(label.to_string()) {
                filtered_data.push(dataset.clone());
            }
        }

        // Verify filtering worked
        assert_eq!(filtered_data.len(), 2, "Should have 2 account datasets (not earned/spent)");

        let labels: Vec<String> = filtered_data.iter()
            .map(|d| d.get("label").unwrap().as_str().unwrap().to_string())
            .collect();

        assert!(labels.contains(&"SEB Checking".to_string()));
        assert!(labels.contains(&"Klarna Card".to_string()));
    }

    #[test]
    fn test_parse_chart_data_with_account_names() {
        // Test with account names that match what Firefly III might return
        let json_response = r#"
        [
            {
                "label": "SEB Checking",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "10000",
                    "2026-02-27T00:00:00+00:00": "10500",
                    "2026-02-28T00:00:00+00:00": "11000"
                }
            },
            {
                "label": "SEB Savings",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "50000",
                    "2026-02-27T00:00:00+00:00": "50000",
                    "2026-02-28T00:00:00+00:00": "51000"
                }
            }
        ]
        "#;

        let chart_line: serde_json::Value = serde_json::from_str(json_response).unwrap();

        let labels: Vec<String> = chart_line.as_array().unwrap()
            .iter()
            .map(|d| d.get("label").unwrap().as_str().unwrap().to_string())
            .collect();

        assert_eq!(labels.len(), 2);
        assert!(labels.contains(&"SEB Checking".to_string()));
        assert!(labels.contains(&"SEB Savings".to_string()));
    }

    #[test]
    fn test_parse_empty_chart_data() {
        // Test with empty response
        let json_response = "[]";

        let chart_line: serde_json::Value = serde_json::from_str(json_response).unwrap();
        assert!(chart_line.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_parse_chart_data_with_array_entries() {
        // Test with array format entries (different Firefly III response format)
        let json_response = r#"
        [
            {
                "label": "SEB Checking",
                "currency_symbol": "kr",
                "currency_code": "SEK",
                "entries": [
                    {"key": "2026-02-26", "value": "10000"},
                    {"key": "2026-02-27", "value": "10500"},
                    {"key": "2026-02-28", "value": "11000"}
                ]
            }
        ]
        "#;

        let chart_line: serde_json::Value = serde_json::from_str(json_response).unwrap();
        let array_entries = chart_line[0]["entries"].as_array().unwrap();

        assert_eq!(array_entries.len(), 3);
        assert_eq!(array_entries[0]["key"], "2026-02-26");
        assert_eq!(array_entries[0]["value"], "10000");
    }

    #[test]
    fn test_account_matching() {
        // Simulate account names from frontend
        let account_names = vec!["SEB Checking".to_string(), "Klarna Card".to_string()];

        // Simulate dataset labels from backend
        let dataset_labels = vec![
            "SEB Checking".to_string(),
            "Klarna Card".to_string(),
            "earned".to_string(),
            "spent".to_string(),
        ];

        // Filter out earned/spent
        let filtered_labels: Vec<String> = dataset_labels.iter()
            .filter(|l| l != &&"earned".to_string() && l != &&"spent".to_string())
            .cloned()
            .collect();

        // Match accounts to datasets
        let mut matched_accounts = Vec::new();
        for account_name in &account_names {
            if filtered_labels.contains(account_name) {
                matched_accounts.push(account_name.clone());
            }
        }

        assert_eq!(matched_accounts.len(), 2);
        assert!(matched_accounts.contains(&"SEB Checking".to_string()));
        assert!(matched_accounts.contains(&"Klarna Card".to_string()));
    }

    #[test]
    fn test_date_parsing() {
        // Test that dates in entries are properly handled
        let json_response = r#"
        [
            {
                "label": "Test Account",
                "entries": {
                    "2026-02-26T00:00:00+00:00": "100",
                    "2026-02-27T00:00:00+00:00": "200",
                    "2026-02-28T00:00:00+00:00": "300"
                }
            }
        ]
        "#;

        let chart_line: serde_json::Value = serde_json::from_str(json_response).unwrap();
        let entries = &chart_line[0]["entries"];

        assert!(entries.as_object().unwrap().len() == 3);
        assert!(entries.get("2026-02-26T00:00:00+00:00").is_some());
        assert!(entries.get("2026-02-27T00:00:00+00:00").is_some());
        assert!(entries.get("2026-02-28T00:00:00+00:00").is_some());
    }
}
