use std::path::Path;

pub(crate) trait RCompatiblePath
where
    Self: AsRef<Path>,
{
    fn adjust_for_r(&self) -> String {
        let path = self.as_ref().to_string_lossy();
        if cfg!(target_os = "windows") && path.starts_with(r"\\?\") {
            path[4..].replace('\\', "/")
        } else {
            path.to_string()
        }
    }
}

impl<T> RCompatiblePath for T where T: AsRef<Path> {}
