use chrono::{DateTime, Duration, Local};
use std::{
    ffi::CString,
    fs::{self, symlink_metadata, DirEntry, Metadata},
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::Path,
};
use users::{get_group_by_gid, get_user_by_uid};

pub fn get_detailed_info(path: &Path) -> Result<Vec<String>, String> {
    let metadata = match symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => return Err(e.to_string()),
    };

    let permission = get_permissions(&metadata, path);
    let hard_link = metadata.nlink().to_string();
    let owner = get_owner(&metadata);
    let groupe = get_group(&metadata);
    let file_size = get_file_size(&metadata);
    let last_modified = match get_date_time(&metadata) {
        Ok(date) => date,
        Err(e) => return Err(e),
    };
    let file_name = get_name_with_link(&metadata, path);

    Ok(vec![
        permission,
        hard_link,
        owner,
        groupe,
        file_size,
        last_modified,
        file_name,
    ])
}

fn get_permissions(metadata: &Metadata, path: &Path) -> String {
    let mode_str = format!("{:?}", metadata.permissions());
    let splited_mode: Vec<&str> = mode_str.split(" ").collect();
    let mut permission = splited_mode[4].to_string()[1..splited_mode[4].len() - 1].to_string();
    if has_acl(path) {
        permission.push('+')
    }
    permission
}

fn has_acl(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_string();
    let path_c = match CString::new(path_str) {
        Ok(c_str) => c_str,
        Err(_) => return false,
    };

    unsafe {
        let result = libc::getxattr(
            path_c.as_ptr(),
            b"system.posix_acl_access\0".as_ptr() as *const i8,
            std::ptr::null_mut(),
            0,
        );
        result > 0
    }
}

fn get_owner(metadata: &Metadata) -> String {
    let uid = metadata.uid();

    match get_user_by_uid(uid) {
        Some(user) => user.name().to_string_lossy().to_string(),
        None => String::from("{uid}"),
    }
}

fn get_group(metadata: &Metadata) -> String {
    let gid = metadata.gid();

    match get_group_by_gid(gid) {
        Some(groupe) => groupe.name().to_string_lossy().to_string(),
        None => String::from("{gid}"),
    }
}

fn get_date_time(metadata: &Metadata) -> Result<String, String> {
    let syst_time = match metadata.modified() {
        Ok(time) => time,
        Err(e) => return Err(e.to_string()),
    };
    let mut date: DateTime<Local> = syst_time.into();
    date += Duration::seconds(3600);

    let now = Local::now();
    let six_month = now - Duration::seconds(6 * 30 * 24 * 60 * 60);
    if date > six_month {
        Ok(date.format("%b %d %H:%M").to_string())
    } else {
        Ok(date.format("%b %d %Y").to_string())
    }
}

fn get_file_size(metadata: &Metadata) -> String {
    if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
        get_major_minor(metadata)
    } else {
        metadata.len().to_string()
    }
}

fn get_major_minor(metadata: &Metadata) -> String {
    let rdev = metadata.rdev();

    let major = libc::major(rdev);
    let minor = libc::minor(rdev);

    format!("{}, {}", major, minor)
}

fn get_name_with_link(metadata: &Metadata, path: &Path) -> String {
    let mut full_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from(""),
    };

    if metadata.file_type().is_symlink() {
        let link_name = match fs::read_link(path) {
            Ok(target) => target.to_string_lossy().to_string(),
            Err(_) => String::from(""),
        };
        full_name.push_str(" -> ");
        full_name.push_str(&link_name);
    }

    full_name
}

pub fn get_total_blocks(entry: &DirEntry) -> u64 {
    if let Ok(metadata) = entry.metadata() {
        metadata.blocks() / 2
    } else {
        0
    }
}
