use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "abaco", about = "AI-native calculator and unit converter for AGNOS")]
struct Cli {
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
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();

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
        None => {
            // Interactive REPL mode
            run_repl();
        }
    }
}

fn run_repl() {
    println!("Abaco — AI-native calculator for AGNOS");
    println!("Type expressions to evaluate, 'quit' to exit.");
    println!();

    let evaluator = abaco_eval::Evaluator::new();
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
                let msg = format!("{value} {from} -> {to} (live rates via hoosh — not yet connected)");
                println!("{msg}");
                history.push(input, &msg);
            }
            Ok(abaco_ai::ParsedQuery::Calculation(expr)) => {
                match evaluator.eval(&expr) {
                    Ok(value) => {
                        println!("{value}");
                        history.push(input, &value.to_string());
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
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
