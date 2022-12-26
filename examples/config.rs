use anyhow::Result;
use xdff::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/text.yml");
    let config = DiffConfig::from_yaml(content)?;
    
    println!("{:#?}", config);
    Ok(())
}