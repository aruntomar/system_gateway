use std::process::{Command, Stdio, exit};
use std::error::Error;

pub fn gateway() ->  Result<String, Box<dyn Error>> {
    if cfg!(target_os="macos") {
        // get_mac_gateway()
        let mac_cmd_str = "netstat -f inet -nr";
        let output = get_output(mac_cmd_str)?;
        let split_output: Vec<&str> = output.split_whitespace().collect();
        Ok(split_output[1].to_string())
    } else if cfg!(target_os = "linux"){
        // get_linux_gateway()
        let linux_cmd_str = "ip route show";
        let output = get_output(linux_cmd_str)?;
        let output = output.split_whitespace().collect::<Vec<&str>>();
        Ok(output[2].to_string())        
    } else {
        eprint!("Unsupported OS");
        exit(1);
    }
}

fn get_cmd(cmd_str: String) -> Command {
    let (program, args) = get_cmd_args(&cmd_str);
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd
}

fn get_cmd_args(cmd_str: &str) -> (&str, Vec<&str>) {
    let mut cmd_iter = cmd_str.split_whitespace();
    let prog = cmd_iter.next().unwrap();
    let args = cmd_iter.collect();
    (prog, args)
}

fn get_output(cmd_str: &str) -> Result<String, Box<dyn Error>> {
    let cmd = get_cmd(cmd_str.to_string())
                            .stdout(Stdio::piped())
                            .spawn()?;
    let netstat_output = cmd.stdout.expect("failed to read netstat stdout");
    let grep_str = "grep default";
    let grep_cmd = get_cmd(grep_str.to_string())
                        .stdin(Stdio::from(netstat_output))
                        .output()?;
    Ok(String::from_utf8(grep_cmd.stdout)?)
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

}
