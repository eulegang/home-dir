//! A enable expansion of tildes in paths
//!
//! built for unix, windows patch welcome
//!
//! - ~ expands using the HOME environmental variable.
//! - if HOME does not exist, lookup current user in the user database
//!
//! - ~`user` will expand to the user's home directory from the user database
//!

use std::env;
use std::path::{Component, Path, PathBuf};

use nix::unistd::{Uid, User};

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
/// Error while expanding path
pub enum Error {
    /// The user being looked up is not in the user database
    #[error("the user does not have entry")]
    MissingEntry,
}

/// The expansion trait extension
pub trait HomeDirExt {
    /// Expands a users home directory signified by a tilde.
    ///
    /// ```
    /// # use home_dir::HomeDirExt;
    /// # use std::env::var;
    /// # use std::path::PathBuf;
    /// let mut path = PathBuf::from(var("HOME").unwrap());
    /// path.push(".vimrc");
    ///
    /// assert_eq!("~/.vimrc".expand_home().unwrap(), path, "current user path expansion");
    ///
    /// # #[cfg(target_os = "macos")]
    /// # const ROOT_VIMRC: &'static str = "/var/root/.vimrc";
    /// # #[cfg(target_os = "linux")]
    /// # const ROOT_VIMRC: &'static str = "/root/.vimrc";
    /// assert_eq!("~root/.vimrc".expand_home().unwrap(), PathBuf::from(ROOT_VIMRC));
    /// ```
    fn expand_home(&self) -> Result<PathBuf, Error>;
}

impl HomeDirExt for Path {
    fn expand_home(&self) -> Result<PathBuf, Error> {
        let mut path = PathBuf::new();
        let mut comps = self.components();

        match comps.next() {
            Some(Component::Normal(os)) => {
                if let Some(s) = os.to_str() {
                    match s {
                        "~" => {
                            let p = getenv()
                                .ok_or(Error::MissingEntry)
                                .or_else(|_| getent_current())?;
                            path.push(p);
                        }
                        s if s.starts_with('~') => {
                            path.push(getent(&s[1..])?);
                        }
                        s => path.push(s),
                    }
                } else {
                    path.push(os)
                }
            }
            Some(comp) => path.push(comp),
            None => return Ok(path),
        };

        for comp in comps {
            path.push(comp);
        }

        Ok(path)
    }
}

impl<T> HomeDirExt for T
where
    T: AsRef<Path>,
{
    fn expand_home(&self) -> Result<PathBuf, Error> {
        self.as_ref().expand_home()
    }
}

pub(crate) fn getent(name: &str) -> Result<PathBuf, Error> {
    let usr = User::from_name(name).or(Err(Error::MissingEntry))?;
    let usr = usr.ok_or(Error::MissingEntry)?;

    Ok(usr.dir)
}

pub(crate) fn getenv() -> Option<PathBuf> {
    env::var("HOME").ok().map(Into::into)
}

pub(crate) fn getent_current() -> Result<PathBuf, Error> {
    let usr = User::from_uid(Uid::current()).or(Err(Error::MissingEntry))?;
    let usr = usr.ok_or(Error::MissingEntry)?;

    Ok(usr.dir)
}
