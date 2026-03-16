# MCP Tools

Abaco exposes 5 MCP tools for integration with AGNOS agents via daimon.

## abaco_eval

Evaluate a mathematical expression.

**Parameters:**
- `expression` (string, required) — Math expression to evaluate

**Returns:** `{ "result": "14", "type": "Integer" }`

## abaco_convert

Convert a value between units.

**Parameters:**
- `value` (number, required) — Numeric value to convert
- `from` (string, required) — Source unit name or symbol
- `to` (string, required) — Target unit name or symbol

**Returns:** `{ "from_value": 5.0, "from_unit": "km", "to_value": 3.10686, "to_unit": "mi" }`

## abaco_currency

Convert between currencies using live rates from hoosh.

**Parameters:**
- `value` (number, required) — Amount to convert
- `from` (string, required) — Source currency code (e.g., "USD")
- `to` (string, required) — Target currency code (e.g., "EUR")

**Returns:** `{ "value": 100.0, "from": "USD", "to": "EUR", "result": 92.5, "rate": 0.925 }`

## abaco_history

Retrieve calculation history.

**Parameters:**
- `limit` (number, optional) — Max entries to return (default: 10)

**Returns:** `{ "entries": [{ "input": "2 + 3", "result": "5", "timestamp": "..." }] }`

## abaco_units

List available units, optionally filtered by category.

**Parameters:**
- `category` (string, optional) — Filter by category (e.g., "Length", "Mass", "Temperature")

**Returns:** `{ "units": [{ "name": "meter", "symbol": "m", "category": "Length" }] }`
