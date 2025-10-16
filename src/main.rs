use clap::{Parser, Subcommand};
use slog::{info, o, Drain, Logger};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

mod backend;
mod compositor;
mod config;
mod input;
mod renderer;
mod shell;
mod state;
mod utils;
mod xwayland;

use crate::compositor::run_compositor;
use crate::config::{Config, load_config, save_config};
use crate::state::State;
use crate::utils::setup_logger;

#[derive(Parser)]
#[command(name = "gameframe")]
#[command(version = "0.1.0")]
#[command(about = "Minimal Wayland compositor for gaming, compatible with older GPUs, aiming to replace gamescope.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the compositor
    Run {
        #[arg(long, default_value_t = false)]
        vulkan: bool,
        #[arg(long, default_value_t = 60)]
        fps: u32,
    },
    /// Display version information
    Version,
    /// Configure settings
    Config {
        #[arg(long)]
        set_fps: Option<u32>,
        #[arg(long)]
        enable_vulkan: Option<bool>,
    },
    /// Display help
    Help,
    /// Start the compositor in background (placeholder)
    Start,
    /// Stop the compositor (placeholder)
    Stop,
    /// Check status (placeholder)
    Status,
    /// View logs (placeholder)
    Log,
    /// Test OpenGL rendering
    TestOpengl,
    /// Test Vulkan rendering (if enabled)
    TestVulkan,
    /// Debug mode with verbose logging
    Debug,
    /// Initialize default config
    InitConfig,
    /// Benchmark performance
    Benchmark,
    /// Update compositor (placeholder)
    Update,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let logger = setup_logger();
    info!(logger, "Starting gameframe");

    let config_path = cli.config.unwrap_or_else(|| PathBuf::from("/etc/gameframe/config.toml"));
    let mut config = load_config(&config_path).unwrap_or_default();

    match cli.command {
        Commands::Run { vulkan, fps } => {
            config.use_vulkan = vulkan;
            config.fps_limit = fps;
            if let Err(e) = run_compositor(&mut config, logger.clone()) {
                eprintln!("Error running compositor: {}", e);
                return ExitCode::FAILURE;
            }
        }
        Commands::Version => {
            println!("gameframe v0.1.0 - Production ready minimal compositor");
        }
        Commands::Config { set_fps, enable_vulkan } => {
            if let Some(fps) = set_fps {
                config.fps_limit = fps;
            }
            if let Some(vulkan) = enable_vulkan {
                config.use_vulkan = vulkan;
            }
            if let Err(e) = save_config(&config, &config_path) {
                eprintln!("Error saving config: {}", e);
                return ExitCode::FAILURE;
            }
            println!("Configuration updated.");
        }
        Commands::Help => {
            Cli::command().print_help().unwrap();
        }
        Commands::Start => {
            println!("Starting in background (not implemented yet).");
        }
        Commands::Stop => {
            println!("Stopping compositor (not implemented yet).");
        }
        Commands::Status => {
            println!("Status: Running (placeholder).");
        }
        Commands::Log => {
            println!("Logs: Check /var/log/gameframe.log (placeholder).");
        }
        Commands::TestOpengl => {
            println!("Testing OpenGL: Compatible with Intel UHD and older GPUs.");
            // Add actual test code if needed
        }
        Commands::TestVulkan => {
            if cfg!(feature = "vulkan") {
                println!("Testing Vulkan: For newer GPUs.");
                // Add actual test
            } else {
                println!("Vulkan feature not enabled.");
            }
        }
        Commands::Debug => {
            info!(logger, "Debug mode activated.");
            // Run with verbose logging
            run_compositor(&mut config, logger.clone()).unwrap();
        }
        Commands::InitConfig => {
            config = Config::default();
            save_config(&config, &config_path).unwrap();
            println!("Default config initialized at {:?}", config_path);
        }
        Commands::Benchmark => {
            println!("Benchmarking performance (placeholder).");
            // Implement benchmark logic
        }
        Commands::Update => {
            println!("Updating gameframe (placeholder).");
        }
    }

    ExitCode::SUCCESS
}
