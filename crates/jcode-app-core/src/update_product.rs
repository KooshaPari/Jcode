//! Product identity for the auto-updater.
//!
//! jcode upstream and this fork are **separate products**: separate release
//! channels, separate binaries, forward-only data sync and no shared binaries.
//! That makes "which GitHub repo may this build update itself from" a product
//! identity question, not a configuration one — and it must never be answered
//! by falling back to upstream. Installing an upstream release over a fork build
//! silently replaces the fork with upstream code, which is precisely the
//! product-decollision break this module exists to prevent.
//!
//! Identity is read from the build's semver **channel suffix**, reusing the
//! fork-versioning scheme this repo already documents and implements in
//! `jcode-build-meta`: `BASEVERSION-<first-letter>semver[-channel]`. Upstream
//! builds carry a bare `MAJOR.MINOR.PATCH`; fork builds carry a letter-prefixed
//! fork semver, e.g. this product's `0.85.1-k1.1.1` (`k` = the KooshaPari/KCode
//! fork line, `p` = the Phenotype org fork). Deriving identity from the same
//! signal the rest of the build metadata uses means it needs no new build-time
//! knob and cannot drift from the version already shown by `--version`.

/// GitHub repo slug for **upstream** builds.
///
/// A fork build never updates from this slug; see [`update_repo`].
pub const GITHUB_REPO: &str = "1jehuang/jcode";

/// Resolve the GitHub repo slug this build is allowed to update itself from.
///
/// Returns `None` for a fork build. `None` is the fail-closed answer and every
/// caller must treat it as "auto-update is unavailable", never as "fall back to
/// upstream".
pub fn update_repo() -> Option<&'static str> {
    update_repo_for(current_semver())
}

/// Pure form of [`update_repo`], so the product-identity decision is directly
/// testable without depending on process-global build metadata.
pub fn update_repo_for(semver: &str) -> Option<&'static str> {
    if is_fork_build_for(semver) {
        log_disabled(semver);
        None
    } else {
        Some(GITHUB_REPO)
    }
}

/// Whether this build is a fork of upstream jcode rather than upstream itself.
pub fn is_fork_build() -> bool {
    is_fork_build_for(current_semver())
}

/// Whether the given build semver identifies a fork build.
pub fn is_fork_build_for(semver: &str) -> bool {
    semver_has_channel(semver)
}

/// Whether a build semver carries a fork channel suffix (`1.5.3-k0.1.2`) as
/// opposed to a bare upstream `1.5.3`.
///
/// A bare trailing `-` is not a channel, matching the rule `jcode-build-meta`'s
/// `parse_semver_with_channel` already applies, so a malformed semver cannot
/// masquerade as a fork marker.
pub fn semver_has_channel(semver: &str) -> bool {
    semver
        .trim()
        .trim_start_matches('v')
        .split_once('-')
        .is_some_and(|(_, channel)| !channel.trim().is_empty())
}

/// Error surfaced when an update is explicitly requested on a build that must
/// not self-update, so `/update` reports a clear reason instead of silently
/// claiming success or an opaque network failure.
pub fn fork_update_disallowed_error() -> anyhow::Error {
    anyhow::anyhow!(
        "Auto-update is disabled for this build. It is a fork build ({}) and never updates \
         from upstream {}. Install updates for it from its own release channel.",
        current_semver(),
        GITHUB_REPO
    )
}

/// Resolve the repo slug for a network update call, or fail with
/// [`fork_update_disallowed_error`].
///
/// Keeping this here means the call sites in [`super::update`] do not each
/// repeat the resolve-or-bail boilerplate, which keeps that already oversized
/// module from growing.
pub fn resolve_update_repo() -> anyhow::Result<&'static str> {
    update_repo().ok_or_else(fork_update_disallowed_error)
}

/// True when `path` sits inside a git work tree.
///
/// The updater refuses to run from a developer checkout; that check happens
/// on the hot path in [`super::update`], so the walk lives here to keep the
/// oversized updater module from carrying it.
pub fn is_inside_git_repo(path: &std::path::Path) -> bool {
    let mut dir = if path.is_dir() {
        Some(path)
    } else {
        path.parent()
    };

    while let Some(d) = dir {
        if d.join(".git").exists() {
            return true;
        }
        dir = d.parent();
    }
    false
}

/// The semver this build uses for update comparisons, honoring the fast-release
/// wrapper's runtime override exactly as the updater itself does.
fn current_semver() -> &'static str {
    jcode_build_meta::update_semver()
}

fn log_disabled(semver: &str) {
    crate::logging::info(&format!(
        "Auto-update disabled: fork build ({}) does not update from upstream {}",
        semver, GITHUB_REPO
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fork build must not resolve to the upstream slug under any fork identity
    /// this repo's versioning scheme can produce. Without this the updater would
    /// fetch an upstream release and replace the fork binary with upstream code.
    #[test]
    fn fork_builds_never_resolve_to_upstream_repo() {
        for semver in [
            // This product's own build semver.
            "0.85.1-k1.1.1",
            // Other fork lines in the documented scheme.
            "1.5.3-p0.4.0",
            // Fork semver carrying a channel suffix as well.
            "0.91.0-k0.1.2-night",
            // `v` prefix, as `jcode-build-meta` preserves it.
            "v1.5.3-k0.1.2",
        ] {
            assert_eq!(
                update_repo_for(semver),
                None,
                "fork build {semver} must not resolve to an update source"
            );
            assert!(
                is_fork_build_for(semver),
                "{semver} must be recognized as a fork build"
            );
        }
    }

    /// The inverse guard: an upstream build must keep updating from upstream,
    /// unchanged. A regression here would silently disable updates for everyone
    /// on the upstream product.
    #[test]
    fn upstream_builds_still_resolve_to_upstream_repo() {
        for semver in ["0.85.1", "1.5.3", "0.0.0", "v0.91.42"] {
            assert_eq!(
                update_repo_for(semver),
                Some(GITHUB_REPO),
                "upstream build {semver} must keep updating from {GITHUB_REPO}"
            );
            assert!(
                !is_fork_build_for(semver),
                "{semver} must not be treated as a fork build"
            );
        }
    }

    /// An empty channel is not a channel, matching `jcode-build-meta`. This keeps
    /// a malformed semver from being read as a fork marker, and from disabling
    /// auto-update for an upstream build over a stray `-`.
    #[test]
    fn bare_trailing_dash_is_not_a_fork_channel() {
        assert!(!semver_has_channel("1.2.3-"));
        assert_eq!(update_repo_for("1.2.3-"), Some(GITHUB_REPO));
    }

    /// Whitespace must not change the decision: `jcode-build-meta` trims semver
    /// values before parsing them.
    #[test]
    fn channel_detection_trims_and_tolerates_v_prefix() {
        assert!(semver_has_channel("  1.5.3-k0.1.2  "));
        assert!(!semver_has_channel("  1.5.3  "));
        assert!(!semver_has_channel(""));
    }

    /// The build this binary was actually compiled with must be classified the
    /// way its own semver says, so the live resolver agrees with the pure one.
    #[test]
    fn live_resolver_agrees_with_pure_resolver() {
        assert_eq!(update_repo(), update_repo_for(current_semver()));
        assert_eq!(is_fork_build(), is_fork_build_for(current_semver()));
    }
}