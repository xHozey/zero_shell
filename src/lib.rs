mod commands;


pub fn parse_command(s: &str) -> Vec<(String, String)> {
    let mut res: Vec<(String, String)> = Vec::new();
    for spt in s.split("&&") {
        match spt.trim().split_once(' '){
            Some((cmd, str)) => {
            res.push((cmd.to_string(), str.to_string()))

            },
            None => {
                res.push((spt.to_string(), "".to_string()))
            }
        }
    }
    res
}