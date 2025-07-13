pub fn echo(s: String) {
    let mut result = String::new();
    // let tmp = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() && chars[i+1] == '\\' {
            if i + 2 < chars.len() {
                match chars[i + 2] {
                    'n' => {
                        result.push('\n');
                        i += 2; 
                    }
                    't' => {
                        result.push('\t');
                        i += 2; 
                    }
                    'r' => {
                        result.push('\r');
                        i += 2;
                    }
                    '"' => {
                        result.push('"');
                        i += 2;
                    }
                    '\'' => {
                        result.push('\'');
                        i += 2;
                    }
                    _ => {
                        result.push('\\');
                        result.push(chars[i + 1]);
                        i += 1;
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(chars[i]);
        }

        i += 1;
    }
    let yu = result.trim_matches('\"').trim_matches('\'');
    println!("{}", yu.replace("\\", ""));
}
