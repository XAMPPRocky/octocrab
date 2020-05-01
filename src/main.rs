use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::default();

    match octocrab
        .issues("xampprocky", "tokei")
        .check_assignee("xampprocky")
        .await
    {
        Err(error) => panic!("{}", error),
        Ok(pull) => println!("{:#?}", pull),
    }

    Ok(())
}
