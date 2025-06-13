use std::fs::rename;
use std::path::Path;
pub fn mv(arg: String) {
    let spt: Vec<&str> = arg.split_whitespace().collect();
    if spt.len() == 0 {
        println!("mv: missing file operand");
        return
    }
    if spt.len() == 1 {
        println!("mv: missing destination file operand after '{}'", spt[0]);
        return
    }
    let dis = Path::new(spt[spt.len()-1]);
    for i in 0..spt.len()-1 {
        match rename(spt[i], dis.join(Path::new(spt[i]))) {
            Ok(_) => {},
            Err(err) => println!("{}", err.to_string())
        }
    }
}