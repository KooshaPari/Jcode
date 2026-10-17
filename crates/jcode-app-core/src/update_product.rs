//! KCode must not install through the inherited jcode updater.
//!
//! Changing the release repository alone is insufficient: the installer still
//! resolves jcode's build directories and launcher. Keep both automatic and
//! explicit updates disabled until the complete install path is product scoped.

pub(super) const UPDATE_REPO: &str = "KooshaPari/KCode";

pub(super) fn ensure_update_allowed() -> Result<(), &'static str> {
    Err(
        "KCode self-update is disabled: the inherited installer targets jcode's build directories and launcher. Install KCode separately; the installed jcode will not be modified.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kcode_never_uses_the_jcode_release_repository() {
        assert_eq!(UPDATE_REPO, "KooshaPari/KCode");
    }

    #[test]
    fn inherited_installer_fails_closed_even_without_a_fork_version_suffix() {
        // Product identity cannot depend on a renamed executable or semver
        // suffix: source builds of this fork can still carry a plain version.
        let error = ensure_update_allowed().unwrap_err();
        assert!(error.contains("KCode self-update is disabled"));
        assert!(error.contains("jcode's build directories and launcher"));
    }
}
