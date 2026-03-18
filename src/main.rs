use clap::{Parser, Subcommand};
use serde_json::{Value as JsonValue, json};
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Parser)]
#[command(
    name = "abaco",
    about = "AI-native calculator and unit converter for AGNOS"
)]
struct Cli {
    /// Launch the desktop GUI
    #[arg(long)]
    gui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Evaluate a math expression
    Eval {
        /// Expression to evaluate (e.g., "2 + 3 * 4")
        expression: String,
    },
    /// Convert between units
    Convert {
        /// Conversion query (e.g., "5 km to miles")
        query: String,
    },
    /// List available units, optionally filtered by category
    List {
        /// Category to filter by (e.g., "length", "mass", "temperature")
        category: Option<String>,
    },
    /// Run as an MCP (Model Context Protocol) JSON-RPC tool server on stdio
    Mcp,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();

    if cli.gui {
        if let Err(e) = abaco_gui::run() {
            eprintln!("GUI error: {e}");
            std::process::exit(1);
        }
        return;
    }

    match cli.command {
        Some(Commands::Eval { expression }) => {
            let evaluator = abaco_eval::Evaluator::new();
            match evaluator.eval(&expression) {
                Ok(value) => println!("{value}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Convert { query }) => {
            let parser = abaco_ai::NlParser::new();
            let registry = abaco_units::UnitRegistry::new();

            match parser.parse_natural(&query) {
                Ok(abaco_ai::ParsedQuery::Conversion { value, from, to }) => {
                    match registry.convert(value, &from, &to) {
                        Ok(result) => println!("{result}"),
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    }
                }
                Ok(abaco_ai::ParsedQuery::CurrencyConversion { value, from, to }) => {
                    println!("Currency conversion: {value} {from} -> {to}");
                    println!("(Live rates not yet available — connect to hoosh for rates)");
                }
                Ok(other) => {
                    eprintln!("Expected conversion query, got: {other:?}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::List { category }) => {
            let registry = abaco_units::UnitRegistry::new();
            match category {
                Some(cat_str) => match cat_str.parse::<abaco_core::UnitCategory>() {
                    Ok(cat) => {
                        println!("{}:", cat);
                        for unit in registry.list_units(cat) {
                            println!("  {}", unit);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        eprintln!(
                            "Available categories: length, mass, temperature, time, datasize, speed, area, volume, energy, pressure"
                        );
                        std::process::exit(1);
                    }
                },
                None => {
                    for cat in abaco_core::UnitCategory::all_categories() {
                        println!("{}:", cat);
                        for unit in registry.list_units(*cat) {
                            println!("  {}", unit);
                        }
                        println!();
                    }
                }
            }
        }
        Some(Commands::Mcp) => {
            run_mcp_server();
        }
        None => {
            // Interactive REPL mode
            run_repl();
        }
    }
}

fn run_mcp_server() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let request: JsonValue = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let err_resp = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {e}") }
                });
                let _ = writeln!(writer, "{}", err_resp);
                let _ = writer.flush();
                continue;
            }
        };

        let method = request["method"].as_str().unwrap_or("");
        let id = request.get("id").cloned();

        // Notifications (no id) get no response
        if id.is_none() || id.as_ref().is_some_and(|v| v.is_null()) {
            // notifications/initialized is a notification — just ignore
            continue;
        }

        let id = id.unwrap();

        let result = match method {
            "initialize" => Ok(mcp_handle_initialize()),
            "tools/list" => Ok(mcp_handle_tools_list()),
            "tools/call" => mcp_handle_tools_call(&request["params"]),
            _ => Err(json!({
                "code": -32601,
                "message": format!("Method not found: {method}")
            })),
        };

        let response = match result {
            Ok(res) => json!({ "jsonrpc": "2.0", "id": id, "result": res }),
            Err(err) => json!({ "jsonrpc": "2.0", "id": id, "error": err }),
        };

        let _ = writeln!(writer, "{}", response);
        let _ = writer.flush();
    }
}

fn mcp_handle_initialize() -> JsonValue {
    json!({
        "protocolVersion": "2024-11-05",
        "capabilities": { "tools": {} },
        "serverInfo": { "name": "abaco", "version": "2026.3.18" }
    })
}

fn mcp_handle_tools_list() -> JsonValue {
    json!({
        "tools": [
            {
                "name": "abaco_eval",
                "description": "Evaluate a mathematical expression",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "Math expression to evaluate"
                        }
                    },
                    "required": ["expression"]
                }
            },
            {
                "name": "abaco_convert",
                "description": "Convert a value between units",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "value": { "type": "number", "description": "Numeric value to convert" },
                        "from": { "type": "string", "description": "Source unit name or symbol" },
                        "to": { "type": "string", "description": "Target unit name or symbol" }
                    },
                    "required": ["value", "from", "to"]
                }
            },
            {
                "name": "abaco_currency",
                "description": "Convert between currencies (stub — live rates not yet available)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "value": { "type": "number", "description": "Amount to convert" },
                        "from": { "type": "string", "description": "Source currency code (e.g. USD)" },
                        "to": { "type": "string", "description": "Target currency code (e.g. EUR)" }
                    },
                    "required": ["value", "from", "to"]
                }
            },
            {
                "name": "abaco_history",
                "description": "Return recent calculation history entries",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "limit": { "type": "number", "description": "Maximum number of entries to return" }
                    }
                }
            },
            {
                "name": "abaco_units",
                "description": "List available units, optionally filtered by category",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Category to filter by (e.g. length, mass, temperature)"
                        }
                    }
                }
            }
        ]
    })
}

fn mcp_handle_tools_call(params: &JsonValue) -> Result<JsonValue, JsonValue> {
    let tool_name = params["name"].as_str().unwrap_or("");
    let args = &params["arguments"];

    let result = match tool_name {
        "abaco_eval" => mcp_tool_eval(args),
        "abaco_convert" => mcp_tool_convert(args),
        "abaco_currency" => mcp_tool_currency(args),
        "abaco_history" => mcp_tool_history(args),
        "abaco_units" => mcp_tool_units(args),
        _ => Err(format!("Unknown tool: {tool_name}")),
    };

    match result {
        Ok(text) => Ok(json!({
            "content": [{ "type": "text", "text": text }]
        })),
        Err(msg) => Ok(json!({
            "content": [{ "type": "text", "text": format!("Error: {msg}") }],
            "isError": true
        })),
    }
}

fn mcp_tool_eval(args: &JsonValue) -> Result<String, String> {
    let expression = args["expression"]
        .as_str()
        .ok_or("Missing required parameter: expression")?;
    let evaluator = abaco_eval::Evaluator::new();
    evaluator
        .eval(expression)
        .map(|v| v.to_string())
        .map_err(|e| e.to_string())
}

fn mcp_tool_convert(args: &JsonValue) -> Result<String, String> {
    let value = args["value"]
        .as_f64()
        .ok_or("Missing required parameter: value")?;
    let from = args["from"]
        .as_str()
        .ok_or("Missing required parameter: from")?;
    let to = args["to"]
        .as_str()
        .ok_or("Missing required parameter: to")?;
    let registry = abaco_units::UnitRegistry::new();
    registry
        .convert(value, from, to)
        .map(|r| r.to_string())
        .map_err(|e| e.to_string())
}

fn mcp_tool_currency(args: &JsonValue) -> Result<String, String> {
    let value = args["value"]
        .as_f64()
        .ok_or("Missing required parameter: value")?;
    let from = args["from"]
        .as_str()
        .ok_or("Missing required parameter: from")?;
    let to = args["to"]
        .as_str()
        .ok_or("Missing required parameter: to")?;
    Ok(format!(
        "Currency conversion: {value} {from} -> {to} (live rates not yet available — connect to hoosh for rates)"
    ))
}

fn mcp_tool_history(args: &JsonValue) -> Result<String, String> {
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    // MCP server is stateless per invocation, so history is always empty
    let _ = limit;
    Ok("No history entries (MCP server is stateless per invocation)".to_string())
}

fn mcp_tool_units(args: &JsonValue) -> Result<String, String> {
    let registry = abaco_units::UnitRegistry::new();
    let category = args.get("category").and_then(|v| v.as_str());

    match category {
        Some(cat_str) => {
            let cat: abaco_core::UnitCategory = cat_str.parse().map_err(|e: String| e)?;
            let units = registry.list_units(cat);
            let mut out = format!("{}:\n", cat);
            for unit in units {
                out.push_str(&format!("  {}\n", unit));
            }
            Ok(out.trim_end().to_string())
        }
        None => {
            let mut out = String::new();
            for cat in abaco_core::UnitCategory::all_categories() {
                out.push_str(&format!("{}:\n", cat));
                for unit in registry.list_units(*cat) {
                    out.push_str(&format!("  {}\n", unit));
                }
                out.push('\n');
            }
            Ok(out.trim_end().to_string())
        }
    }
}

fn run_repl() {
    println!("Abaco — AI-native calculator for AGNOS");
    println!("Type expressions to evaluate, 'quit' to exit.");
    println!();

    let mut evaluator = abaco_eval::Evaluator::new();
    let nl_parser = abaco_ai::NlParser::new();
    let registry = abaco_units::UnitRegistry::new();
    let mut history = abaco_ai::CalculationHistory::new(100);

    let stdin = std::io::stdin();
    let mut line = String::new();

    loop {
        eprint!("abaco> ");
        line.clear();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if input == "quit" || input == "exit" {
            break;
        }
        if input == "history" {
            for entry in history.entries() {
                println!("  {} = {}", entry.input, entry.result);
            }
            continue;
        }

        // Check for variable assignment (e.g., "x = 5" or "x = 2 + 3")
        if let Some(eq_pos) = input.find('=') {
            let var_name = input[..eq_pos].trim();
            let is_valid_ident = !var_name.is_empty()
                && var_name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && var_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_');
            if is_valid_ident {
                let expr = input[eq_pos + 1..].trim();
                match evaluator.eval(expr) {
                    Ok(value) => {
                        if let Some(f) = value.as_f64() {
                            evaluator.set_variable(var_name, f);
                        }
                        println!("{value}");
                        history.push(input, &value.to_string());
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
                continue;
            }
        }

        // Try NL parsing first
        match nl_parser.parse_natural(input) {
            Ok(abaco_ai::ParsedQuery::Conversion { value, from, to }) => {
                match registry.convert(value, &from, &to) {
                    Ok(result) => {
                        println!("{result}");
                        history.push(input, &result.to_string());
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            Ok(abaco_ai::ParsedQuery::CurrencyConversion { value, from, to }) => {
                let msg =
                    format!("{value} {from} -> {to} (live rates via hoosh — not yet connected)");
                println!("{msg}");
                history.push(input, &msg);
            }
            Ok(abaco_ai::ParsedQuery::Calculation(expr)) => match evaluator.eval(&expr) {
                Ok(value) => {
                    println!("{value}");
                    history.push(input, &value.to_string());
                }
                Err(e) => eprintln!("Error: {e}"),
            },
            Err(_) => {
                // Fall back to direct eval
                match evaluator.eval(input) {
                    Ok(value) => {
                        println!("{value}");
                        history.push(input, &value.to_string());
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
    }
}
