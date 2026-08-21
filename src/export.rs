//! Renders `assets/template.html` (with data already injected) to a
//! screenshot via headless Chrome, then wraps the PNG in an `<svg>` wrapper
//! so the committed file stays a `.svg` — this is a port of `export_svg.ts`,
//! simplified by dropping the local `serve.ts` HTTP server: chromiumoxide's
//! `Page::set_content` can load the HTML directly, no localhost hop needed.

use std::path::Path;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;

/// Reads width/height straight out of the PNG's IHDR chunk (bytes 16..24),
/// instead of pulling in an image-decoding crate just for two integers.
fn png_dimensions(png: &[u8]) -> Result<(u32, u32)> {
    const SIG: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if png.len() < 24 || &png[0..8] != SIG {
        bail!("screenshot bytes did not start with a PNG signature");
    }
    let width = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
    let height = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
    Ok((width, height))
}

pub async fn render_html_to_svg(html: &str, output_path: &Path) -> Result<()> {
    let mut builder = BrowserConfig::builder()
        // CI runners execute as root, and Chrome refuses to run as root
        // without this flag.
        .no_sandbox();

    // Optional override — GitHub Actions' `browser-actions/setup-chrome`
    // exports the binary path here; chromiumoxide otherwise tries to
    // auto-detect a system Chrome/Chromium install.
    if let Ok(path) = std::env::var("CHROME_PATH") {
        builder = builder.chrome_executable(path);
    }

    let config = builder.build().map_err(|e| anyhow::anyhow!(e))?;

    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .context("failed to launch headless Chrome — is it installed? set CHROME_PATH if it's not on PATH")?;

    // chromiumoxide's `Handler` drives the CDP websocket in the background;
    // it has to be continuously polled or nothing else will make progress.
    let handler_task = tokio::spawn(async move {
        while let Some(event) = handler.next().await {
            if event.is_err() {
                break;
            }
        }
    });

    let page = browser
        .new_page("about:blank")
        .await
        .context("failed to open a new page")?;

    page.set_content(html)
        .await
        .context("failed to set page content")?;

    // `set_content` (unlike a real navigation) has no "network idle" signal
    // to wait on, and the template's `render()` populates the DOM and draws
    // the contribution chart on a `DOMContentLoaded` handler. A short fixed
    // delay is the simplest reliable way to let fonts + that JS settle
    // before capturing.
    tokio::time::sleep(Duration::from_millis(800)).await;

    let png = page
        .screenshot(
            ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(true)
                .build(),
        )
        .await
        .context("screenshot capture failed")?;

    let _ = browser.close().await;
    handler_task.abort();

    let (width, height) = png_dimensions(&png)?;
    let encoded = STANDARD.encode(&png);

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{width}" height="{height}" viewBox="0 0 {width} {height}"><foreignObject width="100%" height="100%"><img xmlns="http://www.w3.org/1999/xhtml" src="data:image/png;base64,{encoded}" width="{width}" height="{height}"/></foreignObject></svg>"#
    );

    std::fs::write(output_path, svg)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(())
}
