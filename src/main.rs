use octocrab::Octocrab;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let octocrab = Octocrab::default();

    println!(
        "{:?}",
        octocrab.pulls("rust-lang", "triagebot").list().send().await?
    );

    Ok(())
}
