use std::{ error::Error, process, net::IpAddr};
use local_ip_address::local_ip;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // run the logic of the program here
    todo!()

    // Ok(())
}

pub struct Config {
    sub_command: String,
    file_path: Option<String>, // we do not get a file_path when establishing a server
}

impl Config {

    pub fn build(
        mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let sub_command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a subcommand"),
        };
        let file_path = args.next();
    
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