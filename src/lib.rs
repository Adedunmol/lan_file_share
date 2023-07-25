use std::{ error::Error, process, net::IpAddr};
use local_ip_address::local_ip;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the logic of the program here
    todo!()

    // Ok(())
}

pub struct Config {
    sub_command: String,
    file_path: String
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let sub_command = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { sub_command, file_path })
    }

}

fn get_local_ip() -> IpAddr {
    let ip_address = local_ip().unwrap_or_else(|err| {
        eprintln!("An error occurred (Probably you are not connected to a network): {err}");
        process::exit(1);
    });

    ip_address
}


mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let ip_address = get_local_ip();
        let private_ipv4_addresses = vec!["10.", "172.", "192.168"];

        assert_eq!(ip_address.is_ipv4(), true);
        // Ensures the IP address received is a private IPv4 Address
        assert!(private_ipv4_addresses.iter().any(|&val| ip_address.to_string().starts_with(val)));
    }
}