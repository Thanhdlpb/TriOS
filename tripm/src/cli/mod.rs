#[derive(Debug)]
pub enum Command {

    List,

    Info(String),

    Search(String),

    Install(String),

    Doctor,

    Help,

}



pub fn parse() -> Command {

    let args:Vec<String>=
        std::env::args().collect();

    if args.len()<2{
        return Command::Help;
    }


    match args[1].as_str(){

        "list"=>{
            Command::List
        }

        "doctor"=>{
            Command::Doctor
        }

        "info"=>{

            if args.len()<3{

                return Command::Help;
            }

            Command::Info(args[2].clone())
        }

        "search"=>{

            if args.len()<3{

                return Command::Help;
            }

            Command::Search(args[2].clone())
        }

        "install"=>{

            if args.len()<3{

                return Command::Help;
            }

            Command::Install(args[2].clone())
        }

        _=>Command::Help
    }
}
