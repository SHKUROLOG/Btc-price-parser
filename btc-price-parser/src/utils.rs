#[cfg(unix)]
pub mod imp {
    use std::error::Error;
    use std::fs::File;

    use std::os::unix::fs::MetadataExt;

    pub fn get_file_size(file: &File) -> Result<u64, Box<dyn Error>> {
        Ok(file.metadata()?.size())
    }
}

#[cfg(windows)]
pub mod imp {
    use std::error::Error;
    use std::fs::File;

    use std::os::windows::fs::MetadataExt;

    pub fn get_file_size(file: &File) -> Result<u64, Box<dyn Error>> {
        Ok(file.metadata()?.file_size())
    }
}