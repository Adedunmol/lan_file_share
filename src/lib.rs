use std:: error::Error;

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
