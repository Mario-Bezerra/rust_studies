use clap::{Command, Arg};

extern crate cro8s;

fn main() {
    let matches = Command::new("Cr8s")
                                        .about("Cr8s commands")
                                        .arg_required_else_help(true)
                                        .subcommand(
                                            Command::new("users")
                                                            .about("User management")
                                                            .arg_required_else_help(true)
                                                            .subcommand(
                                                                Command::new("create")
                                                                                .about("Create a new user")
                                                                                .arg_required_else_help(true)
                                                                                .arg(Arg::new("username").required(true))
                                                                                .arg(Arg::new("password").required(true))
                                                                                .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))   
                                                            )
                                                            .subcommand(
                                                                Command::new("list")
                                                                                .about("List existing users")
                                                            )
                                                            .subcommand(
                                                                Command::new("delete")
                                                                                .about("Delete user by ID")
                                                                                .arg_required_else_help(true)
                                                                                .arg(Arg::new("id").required(true))
                                )   
                                
            )
            .get_matches();
        
        match matches.subcommand(){
            Some(("users", sub_matches)) => match sub_matches.subcommand() {
                Some(("create", _)) => {},
                Some(("list", _)) => {},
                Some(("delete", _)) => {},
                _ => {}
            }
            _ => {}
        } 
            
}