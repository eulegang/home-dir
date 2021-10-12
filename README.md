# Example tilde notation in paths

This crate provides a function to expand tilde notation, for referring
to home directories, in paths. For example

```
~/foo
```

would be expanded to

```
/home/tomjon/foo
```

if the current user's home directory is `/home/tomjon`.

Example:

```
use home_dir::HomeDirExt;

let public_html = "~/public_html".expand_home().unwrap();
```
