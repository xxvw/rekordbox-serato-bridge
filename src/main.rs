use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rsb", version, about = "Bridge DJ library exports between rekordbox and Serato")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, global = true, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert a library from one format to another.
    Convert {
        #[arg(long, value_enum)]
        from: Format,

        #[arg(long, value_enum)]
        to: Format,

        /// Input file (for rekordbox-xml) or directory (for rekordbox-usb / serato).
        #[arg(long)]
        input: PathBuf,

        /// Output file or directory. Format dependent.
        #[arg(long)]
        output: PathBuf,

        /// Print what would be written without touching the filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Inspect an input library and print a summary (no write).
    Inspect {
        #[arg(long, value_enum)]
        format: Format,

        #[arg(long)]
        input: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Format {
    /// rekordbox.app XML export (DJ_PLAYLISTS root).
    RekordboxXml,
    /// CDJ-compatible USB directory (PIONEER/ANLZ + export.pdb).
    RekordboxUsb,
    /// Serato (ID3 GEOB frames inside audio files).
    Serato,
}

fn init_tracing(level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_new(level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(&cli.log_level);

    match cli.command {
        Command::Convert {
            from,
            to,
            input,
            output,
            dry_run,
        } => {
            tracing::info!(
                ?from,
                ?to,
                input = %input.display(),
                output = %output.display(),
                dry_run,
                "convert requested"
            );
            anyhow::bail!("convert is not implemented yet — scaffolding stage");
        }
        Command::Inspect { format, input } => {
            tracing::info!(?format, input = %input.display(), "inspect requested");
            anyhow::bail!("inspect is not implemented yet — scaffolding stage");
        }
    }
}
