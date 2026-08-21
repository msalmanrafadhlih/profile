mod config;
mod export;
mod stats;

use anyhow::{Context, Result};

/// The template is embedded into the binary at compile time (`include_str!`)
/// so the program has no runtime dependency on being run from a particular
/// working directory — only on `assets/template.html` existing next to
/// `src/` at *build* time.
const TEMPLATE: &str = include_str!("../assets/template.html");

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("[1/3] fetching stats from the GitHub GraphQL API...");
    let stats = stats::generate_stats().context("failed to generate stats")?;

    eprintln!("[2/3] rendering HTML...");
    let payload = serde_json::to_string(&stats).context("failed to serialize stats to JSON")?;
    let html = TEMPLATE.replacen(
        "</head>",
        &format!("<script>window.profileData = {payload};</script>\n</head>"),
        1,
    );
    if html == TEMPLATE {
        anyhow::bail!("could not find `</head>` in assets/template.html to inject data into");
    }

    eprintln!("[3/3] launching headless Chrome and capturing screenshot...");
    let output_path = std::path::Path::new("profile.svg");
    export::render_html_to_svg(&html, output_path).await?;

    eprintln!("done — wrote {}", output_path.display());
    Ok(())
}
