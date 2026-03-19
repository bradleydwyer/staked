use clap::Parser;
use staked::checker;
use staked::registry;
use staked::types::Availability;

#[derive(Parser)]
#[command(
    name = "staked",
    about = "Package registry name availability checker",
    version
)]
struct Cli {
    /// Package names to check
    names: Vec<String>,

    /// Comma-separated registry IDs
    #[arg(short, long)]
    registries: Option<String>,

    /// Filter by language
    #[arg(short, long)]
    languages: Option<String>,

    /// Check all 30 registries
    #[arg(short, long)]
    all: bool,

    /// Output results as JSON
    #[arg(short, long)]
    json: bool,

    /// Show per-registry detail
    #[arg(short, long)]
    verbose: bool,

    /// Show available registries
    #[arg(long)]
    list_registries: bool,
}

fn resolve_registries(cli: &Cli) -> Vec<&'static registry::Registry> {
    if cli.all {
        return registry::all_registries().iter().collect();
    }
    if let Some(ref ids) = cli.registries {
        let ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
        return registry::registries_by_ids(&ids);
    }
    if let Some(ref langs) = cli.languages {
        let langs: Vec<String> = langs.split(',').map(|s| s.trim().to_string()).collect();
        return registry::registries_by_languages(&langs);
    }
    registry::popular_registries()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list_registries {
        println!("{:<20} {:<25} {:<30} LANGUAGES", "ID", "NAME", "ECOSYSTEM");
        println!("{}", "-".repeat(95));
        for reg in registry::all_registries() {
            let popular = if reg.popular { " *" } else { "" };
            println!(
                "{:<20} {:<25} {:<30} {}{}",
                reg.id,
                reg.name,
                reg.ecosystem,
                reg.languages.join(", "),
                popular,
            );
        }
        println!();
        println!("* = included in default (popular) set");
        return Ok(());
    }

    if cli.names.is_empty() {
        eprintln!("Usage: staked [OPTIONS] <NAMES>...");
        eprintln!("       staked --list-registries");
        eprintln!();
        eprintln!("Run 'staked --help' for more information.");
        std::process::exit(1);
    }

    let registries = resolve_registries(&cli);
    if registries.is_empty() {
        eprintln!("No matching registries found.");
        std::process::exit(1);
    }

    let results = checker::check_packages(&cli.names, &registries).await;

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        for result in &results {
            println!("{}:", result.name);
            println!(
                "  {} available, {} taken, {} unknown ({}ms)",
                result.summary.available,
                result.summary.taken,
                result.summary.unknown,
                result.elapsed_ms,
            );

            if cli.verbose {
                for pkg in &result.results {
                    let symbol = match pkg.available {
                        Availability::Available => "[+]",
                        Availability::Taken => "[-]",
                        Availability::Unknown => "[?]",
                    };
                    println!(
                        "  {} {:<20} {:<12} ({}ms)",
                        symbol, pkg.registry_name, pkg.available, pkg.elapsed_ms,
                    );
                }
            } else {
                let available: Vec<&str> = result
                    .results
                    .iter()
                    .filter(|r| r.available == Availability::Available)
                    .map(|r| r.registry_name.as_str())
                    .collect();
                let taken: Vec<&str> = result
                    .results
                    .iter()
                    .filter(|r| r.available == Availability::Taken)
                    .map(|r| r.registry_name.as_str())
                    .collect();

                if !available.is_empty() {
                    println!("  available: {}", available.join(", "));
                }
                if !taken.is_empty() {
                    println!("  taken: {}", taken.join(", "));
                }
            }
            println!();
        }
    }

    Ok(())
}
