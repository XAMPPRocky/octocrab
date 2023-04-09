use octocrab::Octocrab;

struct ProgramArguments {
    gist_id: String,
    star: bool,
}

/// Rudimentary CLI interface. Tries to be nice to the caller without too
/// much bloat on the example.
fn parse_argv_or_exit() -> ProgramArguments {
    const USAGE: &str = r#"Usage:
    <program> (--star | --unstar) GIST_ID"#;
    let star = if let Some(param) = std::env::args().nth(1) {
        if param == "--star" {
            true
        } else if param == "--unstar" {
            false
        } else {
            eprintln!("error: Need (--star | --unstar) as first argument.");
            eprintln!("{}", USAGE);
            std::process::exit(1);
        }
    } else {
        eprintln!("error: Need (--star | --unstar) as first argument.");
        eprintln!("{}", USAGE);
        std::process::exit(1);
    };

    let gist_id = if let Some(gist_id) = std::env::args().nth(2) {
        gist_id
    } else {
        eprintln!("error: Need GIST_ID as second argument.");
        eprintln!("{}", USAGE);
        std::process::exit(1);
    };
    ProgramArguments { gist_id, star }
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let args = parse_argv_or_exit();
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    if args.star {
        octocrab.gists().star(args.gist_id).await?;
    } else {
        octocrab.gists().unstar(args.gist_id).await?;
    }
    Ok(())
}
