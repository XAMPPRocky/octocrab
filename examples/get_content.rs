use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let content = octocrab
        .repos("rust-lang", "rust")
        .get_content()
        .send()
        .await?;

    println!(
        "{} files/dirs in the repo root",
        content.items.into_iter().count()
    );

    let file_data = octocrab
        .repos("rust-lang", "rust")
        .get_content()
        .path("Cargo.toml")
        .send()
        .await?
        .file_data()?
        .unwrap();
    println!("Cargo.tmpl:\n{}", String::from_utf8(file_data).unwrap());

    Ok(())
}
