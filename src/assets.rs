use alloc::{boxed::Box, string::String, vec};
use uefi::{
    prelude::{cstr16, BootServices},
    proto::media::{
        file::{Directory, File, FileAttribute, FileInfo, FileMode, FileSystemInfo},
        fs::SimpleFileSystem,
    },
};
use uefi_services::println;

pub struct Assets {
    directory: Directory,
}

impl Assets {
    pub unsafe fn open(bt: &BootServices) -> Self {
        let handles = bt
            .find_handles::<SimpleFileSystem>()
            .expect("Failed to get handles for `SimpleFileSystem` protocol");

        let handle = handles[0];

        let mut sfs = bt
            .open_protocol_exclusive::<SimpleFileSystem>(handle)
            .expect("Failed to get simple file system");
        let mut root_directory = sfs.open_volume().unwrap();

        let mut fs_info_buf = vec![0; 128];

        let fs_info = root_directory
            .get_info::<FileSystemInfo>(&mut fs_info_buf)
            .unwrap();

        println!("Asset FileSystem Label: {}", fs_info.volume_label());

        let directory = root_directory
            .open(cstr16!("assets"), FileMode::Read, FileAttribute::empty())
            .expect("failed to open directory");
        let mut directory = directory
            .into_directory()
            .expect("assets shouldn`t be a file!");

        let mut buffer = vec![0; 128];

        directory.read_entry(&mut buffer).unwrap();
        directory.read_entry(&mut buffer).unwrap();

        Self { directory }
    }
}

impl Iterator for Assets {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0; 128];
        let entry = self.directory.read_entry(&mut buffer).ok()??;
        let fentry = self
            .directory
            .open(entry.file_name(), FileMode::Read, FileAttribute::empty())
            .ok()?;
        let mut fentry = fentry.into_regular_file()?;
        let info: Box<FileInfo> = fentry.get_boxed_info().unwrap();
        let size = info.file_size() as usize;
        let mut data = vec![0; size];
        fentry.read(&mut data).unwrap();

        String::from_utf8(data).ok()
    }
}
