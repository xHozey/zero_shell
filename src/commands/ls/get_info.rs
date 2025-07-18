use chrono::{DateTime, Duration, Local};
use std::{
    fs::{symlink_metadata, Metadata},
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
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from(""),
    };

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

fn has_acl(_path: &Path) -> bool {
    return false;
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
