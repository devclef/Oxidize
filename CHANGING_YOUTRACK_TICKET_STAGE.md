# How to Change a YouTrack Ticket Stage

## Overview

To change a YouTrack ticket's stage (e.g., from "Backlog" to "Develop"), use the `update_issue` MCP tool with the `customFields` parameter.

## Steps

1. **Get the issue fields schema** for the project to find valid stage values:
   ```
   Tool: get_issue_fields_schema
   Parameters: { "projectKey": "PROJECT_KEY" }
   ```
   This returns all custom fields and their valid enum values.

2. **Update the issue** with the new stage:
   ```
   Tool: update_issue
   Parameters: {
     "issueId": "PROJECT-123",
     "customFields": { "Stage": "Develop" }
   }
   ```

## Example

Change ticket `DUM-1` from "Backlog" to "Develop":

```json
{
  "issueId": "DUM-1",
  "customFields": {
    "Stage": "Develop"
  }
}
```

## Common Stage Values

Typical stages include: `Backlog`, `Develop`, `Review`, `Test`, `Staging`, `Done`

Check the schema for your specific project to confirm available values.
