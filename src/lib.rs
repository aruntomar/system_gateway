use std::process::{Stdio,Command};


pub fn gateway() ->  String {
    
        // for mac
        let netstat_cmd = Command::new("netstat")
                            .args(&["-f", "inet", "-nr" ])
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Failed to start netstat process");
        let netstat_output = netstat_cmd.stdout.expect("failed to read netstat stdout");
    
        let grep_cmd = Command::new("grep")
                          .arg("default")
                          .stdin(Stdio::from(netstat_output))
                          .output()
                          .unwrap();
    
        let output_string = String::from_utf8(grep_cmd.stdout).unwrap();
        let split_output: Vec<&str> = output_string.split_whitespace().collect();
        split_output[1].to_string()
    
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn check_output() {
        assert!(!gateway().is_empty());
    }
}
