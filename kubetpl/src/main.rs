use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select};

mod kubectl;
use kubectl::nodes;

/// A tool to pregenerate some commands for k8s
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Composes node ssh command
    Node { ssh: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Node { .. } => {
            let nodes = nodes::get_node_list()?;
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select node to ssh to")
                .default(0)
                .items(&nodes)
                .interact()?
                .to_string()
                .parse::<usize>()?;

            let node_id = nodes[selection].node_id.clone();
            let region_trimmed = nodes[selection].region.clone();
            println!("aws ssm start-session --region {region_trimmed} --target {node_id} ");
        }
    };
    Ok(())
}
