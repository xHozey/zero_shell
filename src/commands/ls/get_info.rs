use std::{
    fs::{Metadata, metadata},
    os::unix::fs::MetadataExt,
    path::Path,
};
use users::{get_group_by_gid, get_user_by_uid};
pub fn get_detailed_info(path : &Path) -> String {
    let metadata = metadata(path).unwrap();

    let permission = get_permissions(&metadata);
    let owner = get_owner(&metadata);
    let groupe = get_groupe(&metadata);
    let hard_link = metadata.nlink().to_string();

    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from(""),
    };
    format!(
        " {}   {}    {}   {}    {}",
         permission, hard_link, owner, groupe, file_name
    )
}

fn get_permissions(metadata: &Metadata) -> String {
    let mode_str = format!("{:?}", metadata.permissions());
    let splited_mode: Vec<&str> = mode_str.split(" ").collect();
    let permission = &splited_mode[4].to_string()[1..splited_mode[4].len() - 1];
    permission.to_string()
}

pub fn totale_files() {
   
}

fn get_owner(metadata: &Metadata) -> String {
    let uid = metadata.uid();

    match get_user_by_uid(uid) {
        Some(user) => user.name().to_string_lossy().to_string(),
        None => String::from(""),
    }
}

fn get_groupe(metadata: &Metadata) -> String {
    let gid = metadata.gid();

    match get_group_by_gid(gid) {
        Some(groupe) => groupe.name().to_string_lossy().to_string(),
        None => String::from(""),
    }
}
