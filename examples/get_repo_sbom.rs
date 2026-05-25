use octocrab::{
    models::repos::sbom::{SbomDependencyGraph, SbomFetchResponse},
    Octocrab,
};

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let sbom_req = octocrab
        .repos("XAMPPRocky", "octocrab")
        .sbom()
        .generate_report()
        .await?;

    let the_graph: Box<SbomDependencyGraph>;
    loop {
        let sbom_graph = octocrab
            .repos("XAMPPRocky", "octocrab")
            .sbom()
            .fetch_report(sbom_req.clone())
            .await?;

        match sbom_graph {
            SbomFetchResponse::Ready { graph } => {
                the_graph = graph;
                break;
            }
            _ => {
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
    eprintln!("Here's the graph: {:#?}", the_graph);

    Ok(())
}
