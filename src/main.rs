use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::default();

    match octocrab.pulls("rust-lang", "rust").is_merged(71692).await {
        Err(error) => panic!("{}", error),
        Ok(pull) => println!("{:#?}", pull),
    }

    Ok(())
}
