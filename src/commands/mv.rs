use std::fs::rename;
use std::path::Path;
pub fn mv(s: String) {
    let spt: Vec<&str> = s.split_whitespace().collect();
    if spt.len() == 0 {
        eprintln!("mv: missing file operand");
        return
    }
    if spt.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", spt[0]);
        return
    }
    let dis = Path::new(spt[spt.len()-1]);
    for i in 0..spt.len()-1 {
        if let Err(err) = rename(spt[i], dis.join(Path::new(spt[i]))) {
            eprintln!("mv: {}", err.to_string().to_ascii_lowercase())
        }
    }
}