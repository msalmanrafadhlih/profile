# profile-svg

Rust port of [linuxmobile/linuxmobile](https://github.com/linuxmobile/linuxmobile)'s
GitHub-profile-README generator. Fetches your GitHub stats via GraphQL,
renders a styled HTML dashboard through headless Chrome, screenshots it, and
wraps the PNG in an `.svg` so it can be embedded in a profile `README.md`
(GitHub READMEs can't execute JS/CSS live, so this pre-renders once per day
via CI instead).

## Before first run

1. Edit `src/config.rs` — replace the 6 placeholder `pinned` repo names with
   your own **public** repo slugs (the template needs all 6 to render every
   card slot), and adjust the `stack` chip list.
2. Edit `assets/template.html` — replace the placeholder email near the
   "By Moch (@msalmanrafadhlih)" line with your real contact.
3. Rebuild after any `assets/template.html` change — it's embedded into the
   binary at compile time via `include_str!`, so edits only take effect
   after `cargo build`.

## Local run

```sh
export GITHUB_TOKEN=ghp_xxx   # needs `repo` read scope for GraphQL
cargo run --release
```

Needs a local Chrome/Chromium install. If it's not on `PATH`, point at it
explicitly:

```sh
export CHROME_PATH=/usr/bin/chromium
```

Writes `profile.svg` to the current directory.

## CI

`.github/workflows/generate_profile.yml` runs daily (`cron`) and on manual
dispatch: builds the Rust binary, installs Chrome via
`browser-actions/setup-chrome`, runs the binary, and commits `profile.svg`
back to the repo. Uses the default `secrets.GITHUB_TOKEN` — no extra secret
needed. Reference it from your profile README with:

```md
<img src="profile.svg" alt="Profile" width="100%" />
```

## Architecture notes

- No local HTTP server (unlike the original TS version's `serve.ts`) —
  `chromiumoxide`'s `Page::set_content` loads the HTML directly.
- The GitHub fetch is a blocking `ureq` call, not async `reqwest` — this
  script does one fetch, then one browser session, in a straight line, so a
  sync HTTP client keeps the dependency tree (and MSRV) much lighter.

<div align="center">
  <img src="profile.svg" alt="Journal Profile" width="100%" />
</div>
