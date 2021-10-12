use super::{getent, Error};
use crate::HomeDirExt;
use std::path::PathBuf;

#[test]
fn test_root() {
    #[cfg(target_os = "macos")]
    const ROOT_DIR: &'static str = "/var/root";

    #[cfg(target_os = "linux")]
    const ROOT_DIR: &'static str = "/root";

    assert_eq!(getent("root").unwrap(), PathBuf::from(ROOT_DIR));
}

#[test]
fn test_expand_root() {
    #[cfg(target_os = "macos")]
    const ROOT_DIR: &'static str = "/var/root/foobar";

    #[cfg(target_os = "linux")]
    const ROOT_DIR: &'static str = "/root/foobar";

    assert_eq!(
        "~root/foobar".expand_home().unwrap(),
        PathBuf::from(ROOT_DIR)
    );
}

#[test]
fn test_expand_nonexpansion() {
    assert_eq!(
        "/etc/some.conf".expand_home().unwrap(),
        PathBuf::from("/etc/some.conf")
    );
}

#[test]
fn test_missing() {
    assert!(matches!(
        getent("_foobar_").unwrap_err(),
        Error::MissingEntry(_)
    ));
}
