use ::jenkins::{jenkins, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = jenkins::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.deploy().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-jenkins operational");
    Ok(())
}
