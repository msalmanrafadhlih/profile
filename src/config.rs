//! Static configuration, ported from `linuxmobile-main`'s `config.ts`.
//!
//! Edit the values below to match your own GitHub account and the repos you
//! want featured. `pinned` repo names must be repos you actually own (the
//! GraphQL query below fetches them by `owner/name`), otherwise they'll
//! silently be skipped in the output.

pub struct PinnedRepo {
    pub name: &'static str,
    pub topic: &'static str,
}

pub struct Config {
    pub username: &'static str,
    pub pinned: &'static [PinnedRepo],
    pub stack: &'static [&'static str],
}

// NOTE: `assets/template.html`'s render() JS only fills in the six "feature"
// card slots (feat, sec1, sec2, ter1, ter2, ter3) when `pinned` has at least
// 6 entries — see the `data.pinned.length >= 6` check in the template. Fewer
// than 6 real, PUBLIC repos here means that section of the card stays empty.
// A name that doesn't exist under your account is skipped silently by the
// GraphQL query (not an error), so double-check the slugs below.
pub const CONFIG: Config = Config {
    username: "msalmanrafadhlih",

    // TODO: replace each `name` with one of your own public repo slugs.
    // "topic" is just a display label shown above the card; it does not
    // come from the GitHub API.
    pinned: &[
        PinnedRepo { name: "flexinix", topic: "Nixos Configuration Flakes" },
        PinnedRepo { name: "racooonfig", topic: "Linux Dotfiles" },
        PinnedRepo { name: "nixdev", topic: "Nix Development Templates" },
        PinnedRepo { name: "termux", topic: "termux config" },
        PinnedRepo { name: "tquilla", topic: "discord bot" },
        PinnedRepo { name: "gemini-pocket", topic: "project exam" },
    ],

    // Shown in the "Core Stack (Detected)" chip row on the card.
    stack: &["NixOS", "Rust", "Helix", "Figma", "Canva"],
};
