use tracing::info;

pub fn read() -> anyhow::Result<()> {
    info!("read token");
    Ok(())
}

pub fn update() -> anyhow::Result<()> {
    info!("update token");
    Ok(())
}

pub fn create() -> anyhow::Result<()> {
    info!("create token");
    Ok(())
}

pub fn delete() -> anyhow::Result<()> {
    info!("delete token");
    Ok(())
}
