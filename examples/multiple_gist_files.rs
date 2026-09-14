use octocrab::Octocrab;

/// This example demonstrates how to update multiple files in an existing gist
/// using both:
/// 1. The batch `.files(...)` method with an iterator of `(filename, content)` tuples.
/// 2. The `.build()` method on `UpdateGistFileBuilder` to update files in a loop.
#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let gist_id = std::env::args().nth(1).unwrap_or_else(|| {
        std::env::var("GIST_ID").unwrap_or_else(|_| {
            eprintln!("Usage: multiple_gist_files <GIST_ID>");
            eprintln!("Or set GIST_ID environment variable.");
            std::process::exit(1);
        })
    });

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    // 1. Updating multiple files at once using the batch `.files(...)` method:
    println!("Updating gist {gist_id} with multiple files using .files()...");
    let batch_files = vec![
        (
            "file1.rs",
            "fn main() {\n    println!(\"Hello from file 1!\");\n}",
        ),
        (
            "file2.rs",
            "fn main() {\n    println!(\"Hello from file 2!\");\n}",
        ),
    ];

    let gist = octocrab
        .gists()
        .update(&gist_id)
        .description("Updated via batch .files() method")
        .files(batch_files)
        .send()
        .await?;

    println!("Updated gist successfully: {url}", url = gist.html_url);

    // 2. Updating multiple files dynamically in a loop using `.file(...).build()`:
    println!("Updating gist {gist_id} in a loop using .build()...");
    let dynamic_updates = vec![
        ("file1.rs", "// Updated in loop\nfn main() {}"),
        ("file3.rs", "// Newly added file\npub fn helper() {}"),
    ];

    let mut builder = octocrab.gists().update(&gist_id);
    for (filename, content) in dynamic_updates {
        builder = builder.file(filename).with_content(content).build();
    }

    let gist = builder.send().await?;
    println!(
        "Updated gist in loop successfully: {url}",
        url = gist.html_url
    );

    Ok(())
}
