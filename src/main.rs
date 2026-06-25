use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

mod codec;
mod subcmd;

#[derive(Parser)]
#[command(name = "silk-convert")]
#[command(version, about = "Convert WeChat SILK to MP3/WAV/Opus/AAC and back")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Decode SILK → WAV/PCM
    Decode {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Output format (wav, pcm)
        #[arg(long, default_value = "wav")]
        to: String,
    },
    /// Encode WAV/MP3/Opus → SILK
    Encode {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// SILK sample rate (8000 or 16000 / 24000)
        #[arg(long, default_value = "24000")]
        sample_rate: u32,
    },
    /// Convert between audio formats (auto-detect)
    Convert {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Target format (wav, mp3, opus, aac, silk, ogg, flac)
        #[arg(long)]
        to: String,
    },
    /// Detect audio format
    Detect {
        input: PathBuf,
    },
    /// Show file metadata
    Info {
        input: PathBuf,
    },
    /// Batch process a directory
    Batch {
        input_dir: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Target format
        #[arg(long)]
        to: String,
        /// File pattern (glob)
        #[arg(long, default_value = "*")]
        pattern: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Decode { input, output, to } => {
            subcmd::decode::run(&input, output.as_deref(), &to)?;
        }
        Command::Encode { input, output, sample_rate } => {
            subcmd::encode::run(&input, output.as_deref(), sample_rate)?;
        }
        Command::Convert { input, output, to } => {
            subcmd::convert::run(&input, output.as_deref(), &to)?;
        }
        Command::Detect { input } => {
            subcmd::detect::run(&input)?;
        }
        Command::Info { input } => {
            subcmd::info::run(&input)?;
        }
        Command::Batch { input_dir, output, to, pattern } => {
            subcmd::batch::run(&input_dir, output.as_deref(), &to, &pattern)?;
        }
    }

    Ok(())
}
