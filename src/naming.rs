use std::path::Path;
use chrono::Local;

const MAX_NAME_LEN: usize = 1024;

pub fn blob_name_from_path(path: &Path, folder_name: &str) -> Option<String> {

    let file_name = path.as_os_str().to_string_lossy().into_owned();

    let sanitized_name = path_name_sanitizer(&file_name, folder_name)?;

    let date = Local::now().format("%Y-%m-%d/%H-%M-%S").to_string();

    let blob_name = format!("{}--{}", date, sanitized_name);

    (blob_name.len() <= MAX_NAME_LEN).then_some(blob_name)
}

fn path_name_sanitizer(path_name: &str, folder_name: &str) -> Option<String>{
    let short_path = path_name.trim().trim_end_matches('.').trim_start_matches(&format!("{folder_name}\\"));
    let sanitized_name = short_path.replace('\\', "/").replace(" ", "-");

    if sanitized_name.is_empty() {
        return None;
    }
    Some(sanitized_name.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn converts_normal_path() {
        let path_name = "C:\\Users\\fizi3\\OneDrive\\Pictures\\Screenshots";
        let p: &Path = &PathBuf::from(path_name);
        let folder_name = "C:\\Users\\fizi3\\OneDrive\\Pictures";
        let blob_name = path_name_sanitizer(&p.as_os_str().to_string_lossy(), folder_name).unwrap();
        assert_eq!(blob_name, "Screenshots");

        let path_name = "C:\\Users\\fizi3\\OneDrive\\Pictures\\Screenshots";
        let p: &Path = &PathBuf::from(path_name);
        let folder_name = "C:\\Users\\fizi3\\OneDrive";
        let blob_name = path_name_sanitizer(&p.as_os_str().to_string_lossy(), folder_name).unwrap();
        assert_eq!(blob_name, "Pictures/Screenshots");
    }

}