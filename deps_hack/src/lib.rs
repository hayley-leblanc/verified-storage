pub use core;
pub use crc64fast;
pub use rand;
pub use winapi;
#[cfg(windows)]
pub use winapi::shared::winerror::SUCCEEDED;
#[cfg(windows)]
pub use winapi::um::fileapi::{CreateFileA, OPEN_ALWAYS};
#[cfg(windows)]
pub use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(windows)]
pub use winapi::um::memoryapi::{MapViewOfFile, FILE_MAP_ALL_ACCESS};
#[cfg(windows)]
pub use winapi::um::winbase::CreateFileMappingA;
#[cfg(windows)]
pub use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ,
    GENERIC_WRITE, PAGE_READWRITE, ULARGE_INTEGER,
};
