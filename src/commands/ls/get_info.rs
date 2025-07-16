use chrono::{DateTime, Duration, Local};
use std::{
    fs::{metadata, Metadata},
    os::unix::fs::MetadataExt,
    path::Path,
};
use users::{get_group_by_gid, get_user_by_uid};
pub fn get_detailed_info(path: &Path) -> Result<Vec<String>, ()> {
    let mut details = Vec::new();
    let metadata = match metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return Err(()),
    };

    let permission = get_permissions(&metadata);
    let hard_link = metadata.nlink().to_string();
    let owner = get_owner(&metadata);
    let groupe = get_group(&metadata);
    let file_size = metadata.len().to_string();
    let last_modif = match get_date_time(&metadata) {
        Ok(date) => date,
        Err(_) => return Err(()),
    };
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from(""),
    };

    details.push(format!(
        " {}   {}    {}   {}    {}     {}   {}",
        permission, hard_link, owner, groupe, file_size, last_modif, file_name
    ));
    Ok(details)
}

fn get_permissions(metadata: &Metadata) -> String {
    let mode_str = format!("{:?}", metadata.permissions());
    let splited_mode: Vec<&str> = mode_str.split(" ").collect();
    let permission = &splited_mode[4].to_string()[1..splited_mode[4].len() - 1];
    permission.to_string()
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

fn get_date_time(metadata: &Metadata) -> Result<String, ()> {
    let syst_time = match metadata.modified() {
        Ok(time) => time,
        Err(_) => return Err(()),
    };
    let mut date: DateTime<Local> = syst_time.into();
    date += Duration::seconds(3600);

    let now = Local::now();
    let six_month = now - Duration::seconds(6 * 30 * 24 * 60 * 60);
    if date > six_month {
        Ok(date.format("%b  %d  %H:%M").to_string())
    } else {
        Ok(date.format("%b  %d  %Y").to_string())
    }
}
