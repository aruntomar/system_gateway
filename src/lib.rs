use std::process::{Command, Stdio, exit};


pub fn gateway() ->  String {
    if cfg!(target_os="macos") {
        get_mac_gateway()
    } else if cfg!(target_os = "linux"){
        get_linux_gateway()
    } else {
        eprint!("Unsupported OS");
        exit(1);
    }
}

fn get_mac_gateway() -> String {
    let cmd = "netstat -f inet -nr";
    let (program, args) = get_cmd_args(cmd);
     // for mac
    let netstat_cmd = Command::new(program)
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to run netstat process");
    let netstat_output = netstat_cmd.stdout.expect("failed to read netstat stdout");

    let grep_cmd = Command::new("grep")
                        .arg("default")
                        .stdin(Stdio::from(netstat_output))
                        .output()
                        .expect("Error while running grep default");

    let output_string = String::from_utf8(grep_cmd.stdout).unwrap();
    let split_output: Vec<&str> = output_string.split_whitespace().collect();
    split_output[1].to_string()
}

fn get_linux_gateway() -> String {
    let cmd = "ip route show";
    let (program, args) = get_cmd_args(cmd);
    let ip_cmd = Command::new(program)
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to run ip route command");
    let ip_out = ip_cmd.stdout.expect("Failed to read ip route output");

    let grep_cmd = Command::new("grep")
                        .arg("default")
                        .stdin(Stdio::from(ip_out))
                        .output()
                        .expect("Error while running grep default");
    let output_string = String::from_utf8(grep_cmd.stdout).unwrap();
    let output = output_string.split_whitespace().collect::<Vec<&str>>();
    output[2].to_string()
}

fn get_cmd_args(cmd_str: &str) -> (&str, Vec<&str>) {
    let mut cmd_iter = cmd_str.split_whitespace();
    let prog = cmd_iter.next().unwrap();
    let args = cmd_iter.collect();
    (prog, args)
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_cmd_arg() {
        let cmd = "netstat -f inet -nr";
        let (p, args) = get_cmd_args(cmd);
        assert_eq!(get_cmd_args(cmd), ("netstat", vec!["-f", "inet", "-nr"]));
        assert_eq!(p, "netstat");
        assert_eq!(args, vec!["-f", "inet", "-nr"]);
    }

    #[test]
    fn test_gateway() {
        let length = gateway().split('.').count();
        // assert_eq!(gateway(), "192.168.0.1");
        assert!(!gateway().is_empty());
        assert!(length == 4);
    }
}
