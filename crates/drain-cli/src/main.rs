use clap::{Parser, Subcommand};
use drain_score::DimensionScorer;

#[derive(Parser)]
#[command(name = "drain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Corpus {
        path: std::path::PathBuf,
    },
    Score {
        path: std::path::PathBuf,
    },
    TierSla {
        path: std::path::PathBuf,
    },
    Gap {
        #[arg(long)]
        scale: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Corpus { path } => {
            let content = std::fs::read_to_string(&path)?;
            let entry = drain_corpus::CorpusEntry::from_markdown(&content)?;
            println!("id: {}", entry.id);
            println!("validate: {:?}", entry.validate());
        }
        Commands::Score { path } => {
            let content = std::fs::read_to_string(&path)?;
            let entry = drain_corpus::CorpusEntry::from_markdown(&content)?;
            let scorer = drain_score::ProvisionalScorer::default();
            for dim in drain_score::Dimension::all() {
                println!("{} {}", dim.code(), scorer.score(&entry, dim).value());
            }
        }
        Commands::TierSla { path } => {
            let content = std::fs::read_to_string(&path)?;
            let entry = drain_corpus::CorpusEntry::from_markdown(&content)?;
            println!("tier: {:?}", drain_tier::classify(&entry));
            println!("sla_gap: {}", drain_tier::tier_sla_gap(&entry).is_some());
        }
        Commands::Gap { scale } => {
            let scale = drain_corpus::Scale::parse(&scale).ok_or("unknown scale")?;
            let rubric = drain_score::Rubric::v0();
            let result = drain_gap::find_gaps(&[], &rubric, scale, &[], false);
            println!("null_result: {}", result.null_result);
            println!("regions: {}", result.regions.len());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_gap() {
        assert!(Cli::try_parse_from(["drain", "gap", "--scale", "national"]).is_ok());
    }

    #[test]
    fn parses_corpus() {
        assert!(Cli::try_parse_from(["drain", "corpus", "some.md"]).is_ok());
    }
}
