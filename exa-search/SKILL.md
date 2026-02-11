---
name: exa-search
description: Performs a search using the Exa Answers API to retrieve high-quality, answer-focused results for a given query. Use this when the user asks a question that requires external information or specific answers from the web.
allowed-tools: Exa(answer:*)
---

# Exa Search Skill

This skill allows the agent to perform searches using the Exa Answers API. It is designed to find direct answers to questions rather than just a list of links.

## Usage

To use this skill, the agent should run the `search.ts` script located in the `scripts` directory.

### Prerequisites

- The Exa API key must be available via one of the following methods:
    1.  Stored in the system keychain (service: `exa-api-key`, account: `EXA_API_KEY`).
    2.  Set as the `EXA_API_KEY` environment variable.

### Instructions

1.  **Prepare the Query**: Formulate a clear and specific question or query.
2.  **Run the Script**: Execute the binary with the query as an argument.

```bash
# Run the search
./mist/skills/exa-search/scripts/exa-search "Your search query here"
```

### Script Arguments

- `query` (required): The search query string.

### Output

The script will output the search result, including the answer text, to stdout.

## Examples

**Example 1: Basic Search**

```bash
./mist/skills/exa-search/scripts/exa-search "What is the capital of France?"
```

**Output:**

```json
{
  "answer": "The capital of France is Paris.",
  "citations": [...]
}
```
