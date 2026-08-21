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
        PinnedRepo { name: "nixos-development-templates", topic: "TODO: topic 1" },
        PinnedRepo { name: "TODO-repo-2", topic: "TODO: topic 2" },
        PinnedRepo { name: "TODO-repo-3", topic: "TODO: topic 3" },
        PinnedRepo { name: "TODO-repo-4", topic: "TODO: topic 4" },
        PinnedRepo { name: "TODO-repo-5", topic: "TODO: topic 5" },
        PinnedRepo { name: "TODO-repo-6", topic: "TODO: topic 6" },
    ],

    // Shown in the "Core Stack (Detected)" chip row on the card.
    stack: &["NixOS", "Rust", "Nix"],
};
