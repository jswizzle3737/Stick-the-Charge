use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
pub fn storage_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let base = std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Library/Application Support"));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    base.map(|dir| dir.join("StickTheCharge").join("saved.txt"))
}

#[cfg(target_arch = "wasm32")]
pub fn storage_path() -> Option<PathBuf> {
    Some(PathBuf::from("stick-the-charge-saved"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_saved(path: &Path) -> std::io::Result<Vec<String>> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents.lines().map(str::to_owned).collect()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(err),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn write_saved(path: &Path, ids: &[String]) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind, Write};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);
    let parent = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;

    // A temporary file in the same directory can replace the old file atomically.
    let mut created = None;
    for _ in 0..8 {
        let suffix = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let temp = parent.join(format!(".stick-charge-{}-{suffix}.tmp", std::process::id()));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&temp) {
            Ok(file) => { created = Some((temp, file)); break; }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    let (temp, mut file) = created.ok_or_else(|| Error::new(ErrorKind::AlreadyExists, "Could not create a temporary save file"))?;
    let result = file.write_all(ids.join("\n").as_bytes()).and_then(|()| file.sync_all());
    drop(file);
    let result = result.and_then(|()| std::fs::rename(&temp, path));
    if result.is_err() { let _ = std::fs::remove_file(&temp); }
    result
}

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> std::io::Result<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        .ok_or_else(|| std::io::Error::other("Browser storage is unavailable"))
}

#[cfg(target_arch = "wasm32")]
pub fn read_saved(path: &Path) -> std::io::Result<Vec<String>> {
    let key = path.to_string_lossy();
    let contents = browser_storage()?.get_item(&key)
        .map_err(|_| std::io::Error::other("Could not read browser storage"))?
        .unwrap_or_default();
    Ok(contents.lines().map(str::to_owned).collect())
}

#[cfg(target_arch = "wasm32")]
pub fn write_saved(path: &Path, ids: &[String]) -> std::io::Result<()> {
    browser_storage()?.set_item(&path.to_string_lossy(), &ids.join("\n"))
        .map_err(|_| std::io::Error::other("Could not write browser storage"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_charges_survive_reopening_and_can_be_removed() {
        let path = std::env::temp_dir().join(format!("stick-charge-test-{}", std::process::id())).join("saved.txt");
        let expected = vec!["VI:266".to_string(), "PR:322".to_string()];
        write_saved(&path, &expected).unwrap();
        assert_eq!(read_saved(&path).unwrap(), expected);
        write_saved(&path, &expected[1..]).unwrap();
        assert_eq!(read_saved(&path).unwrap(), expected[1..]);
        std::fs::remove_file(path).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacing_saved_charges_does_not_modify_the_previous_file() {
        let directory = std::env::temp_dir().join(format!("stick-charge-atomic-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("saved.txt");
        let original = directory.join("original.txt");
        std::fs::write(&path, "VI:266").unwrap();
        std::fs::hard_link(&path, &original).unwrap();
        write_saved(&path, &["PR:322".to_string()]).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "PR:322");
        assert_eq!(std::fs::read_to_string(&original).unwrap(), "VI:266");
        std::fs::remove_file(original).unwrap();
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }
}
