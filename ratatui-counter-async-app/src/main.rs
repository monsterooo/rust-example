use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()>{
    startup()?;
    
    let result = run();

    shutdown()?;

    result?;

    Ok(())
}
