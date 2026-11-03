use anyhow::Result;
use clap::Parser;
use jenkins::jenkins;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = jenkins::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = jenkins::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-jenkins deployed");
    Ok(())
}
