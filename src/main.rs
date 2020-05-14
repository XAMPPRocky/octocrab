use octocrab::{models, Octocrab };

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder()
        // .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;

    let user: octocrab::Result<bool> = octocrab.orgs("rust-lang").check_membership("XAMPPRocky").await;

    match user {
        Err(error) => panic!("{}", error),
        Ok(pull) => println!("{:#?}", pull),
    }

    Ok(())
}
