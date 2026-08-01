/// Service name for the desktop OS keyring. Debug builds default to a distinct
/// service, while standalone worktree launches may request a scoped dev service.
///
/// Release builds may set `BUZZ_STORAGE_NAMESPACE` (lowercase `[a-z0-9-]+`) to
/// isolate keychain entries for an externally branded app variant without
/// forking the desktop crate. Example: `fam` → `buzz-desktop-fam`.
fn dev_keyring_service(configured: Option<String>) -> String {
    configured
        .filter(|service| service.starts_with("buzz-desktop-dev."))
        .unwrap_or_else(|| "buzz-desktop-dev".to_string())
}

fn release_keyring_service() -> String {
    match storage_namespace() {
        Some(ns) => format!("buzz-desktop-{ns}"),
        None => "buzz-desktop".to_string(),
    }
}

/// Validated storage namespace from `BUZZ_STORAGE_NAMESPACE`, if set.
pub(crate) fn storage_namespace() -> Option<String> {
    std::env::var("BUZZ_STORAGE_NAMESPACE").ok().and_then(|raw| {
        let ns = raw.trim().to_ascii_lowercase();
        if ns.is_empty()
            || ns == "desktop"
            || ns == "dev"
            || !ns
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            None
        } else {
            Some(ns)
        }
    })
}

pub(crate) fn keyring_service() -> &'static str {
    if cfg!(debug_assertions) {
        static DEV_SERVICE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        DEV_SERVICE
            .get_or_init(|| dev_keyring_service(std::env::var("BUZZ_DEV_KEYRING_SERVICE").ok()))
            .as_str()
    } else {
        static RELEASE_SERVICE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        RELEASE_SERVICE.get_or_init(release_keyring_service).as_str()
    }
}

pub(super) fn migration_marker_name(service: &str, default_name: &str) -> String {
    if service == "buzz-desktop" || service == "buzz-desktop-dev" {
        default_name.to_string()
    } else {
        format!("identity.{service}.migrated")
    }
}

#[cfg(test)]
mod tests {
    use super::{dev_keyring_service, migration_marker_name};

    #[test]
    fn standalone_scope_must_remain_under_dev_service() {
        assert_eq!(
            dev_keyring_service(Some("buzz-desktop-dev.example".to_string())),
            "buzz-desktop-dev.example"
        );
        assert_eq!(
            dev_keyring_service(Some("buzz-desktop".to_string())),
            "buzz-desktop-dev"
        );
    }

    #[test]
    fn standalone_scope_uses_its_own_migration_marker() {
        assert_eq!(
            migration_marker_name("buzz-desktop", "identity.migrated"),
            "identity.migrated"
        );
        assert_eq!(
            migration_marker_name("buzz-desktop-dev", "identity.migrated"),
            "identity.migrated"
        );
        assert_eq!(
            migration_marker_name("buzz-desktop-dev.example", "identity.migrated"),
            "identity.buzz-desktop-dev.example.migrated"
        );
    }

    #[test]
    fn release_namespace_builds_isolated_service_name() {
        assert_eq!(
            super::release_keyring_service_for("fam"),
            "buzz-desktop-fam"
        );
    }
}

#[cfg(test)]
fn release_keyring_service_for(ns: &str) -> String {
    format!("buzz-desktop-{ns}")
}
