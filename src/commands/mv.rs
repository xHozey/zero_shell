use std::fs::rename;
use std::path::Path;
pub fn mv(s: Vec<String>) {
    if s.len() == 0 {
        eprintln!("mv: missing file operand");
        return;
    }
    if s.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", s[0]);
        return;
    }
    let dis = Path::new(&s[s.len() - 1]);
    for i in 0..s.len() - 1 {
        if let Err(err) = rename(s[i].clone(), dis.join(Path::new(&s[i]))) {
            eprintln!("mv: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
