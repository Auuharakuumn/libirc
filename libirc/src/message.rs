use pest_derive::Parser;
use pest::error::Error;
use pest::Parser;

use std::str::FromStr;

pub trait IrcMessage {
    fn parse_message(message: BaseMessage) -> Result<Box<Self>, Box<dyn std::error::Error>>;
    fn create_message(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum Command {
    IrcCommand(String),
    IrcResponse(String)
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::IrcCommand(cmd) => cmd,
            Command::IrcResponse(cmd) => cmd,
        }.to_string()
    }
}

#[derive(Debug)]
pub struct UserPrefix {
    pub nickname: String,
    pub user: Option<String>,
    pub host: Option<String>
}

#[derive(Debug)]
pub enum Prefix {
    ServerName(String),
    UserName(UserPrefix)
}

#[derive(Debug)]
pub struct Parameters {
    pub middle: Vec<String>,
    pub trailing: Option<String>
}

#[derive(Parser)]
#[grammar = "irc.pest"]
struct IrcParser;

#[derive(Debug)]
pub struct BaseMessage {
    pub prefix: Option<Prefix>,
    pub command: Command,
    pub parameters: Option<Parameters>
}

impl FromStr for BaseMessage {
    type Err = Error<Rule>;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let parse_result = IrcParser::parse(Rule::message, message);
        ////println!("{:?}", parse_result);

        let message_pairs = parse_result?.next().unwrap().into_inner();

        let mut prefix: Option<Prefix> = None;
        let mut command: Command = Command::IrcCommand(String::from(""));
        let mut parameters: Option<Parameters> = None;

        for pair in message_pairs {
            match pair.as_rule() {
                Rule::prefix => {
                    //println!("Prefix Pair: {:?}\n\n", pair);
                    prefix = parse_prefix(pair);
                }
                Rule::command => {
                    //println!("Command Pair: {:?}\n\n", pair);
                    command = parse_command(pair);
                }
                Rule::params => {
                    //println!("Param Pair: {:?}\n\n", pair);
                    parameters = parse_parameters(pair);
                }
                _ => {}
            }
        }

        Ok(BaseMessage {
            prefix: prefix,
            command: command,
            parameters: parameters
        })
    }
}

fn parse_prefix(pair: pest::iterators::Pair<Rule>) -> Option<Prefix> {
    let pairs = pair.into_inner();

    for p in pairs {
        match p.as_rule() {
            Rule::servername => {
                //println!("ServerName Pair: {:?}\n\n", p);
                return Some(Prefix::ServerName(String::from(p.as_str())));
            }
            Rule::username => {
                //println!("UserName Pair: {:?}\n\n", p);
                let username_pair = p.into_inner();
                let mut nickname: String = String::from("");
                let mut user: Option<String> = None;
                let mut host: Option<String> = None;
                
                for up in username_pair {
                    match up.as_rule() {
                        Rule::nickname => {
                            //println!("NickName Pair: {:?}\n\n", up);
                            nickname = String::from(up.as_str());
                        }
                        Rule::user => {
                            //println!("User Pair: {:?}\n\n", up);
                            user = Some(String::from(up.as_str()));
                        }
                        Rule::host => {
                            //println!("Host Pair: {:?}\n\n", up);
                            host = Some(String::from(up.as_str()));
                        }
                        _ => {}
                    }
                }

                let user_prefix = UserPrefix {
                    nickname: nickname,
                    user: user,
                    host: host
                };

                return Some(Prefix::UserName(user_prefix));
            }
            _ => {}
        }
    }

    return None;
}

fn parse_command(pair: pest::iterators::Pair<Rule>) -> Command {
    let pairs = pair.into_inner();

    for p in pairs {
        match p.as_rule() {
            Rule::textcommand => {
                //println!("Command Pair: {:?}\n\n", p);
                //println!("as_str test: {:?}\n", Command::IrcCommand(String::from(p.as_str())));
                return Command::IrcCommand(String::from(p.as_str()));
            }
            Rule::numresponse => {
                //println!("Resp Pair: {:?}\n\n", p);
                return Command::IrcResponse(String::from(p.as_str()));
            }
            _ => {}
        }
    }

    return Command::IrcCommand(String::from(""));
}

fn parse_parameters(pair: pest::iterators::Pair<Rule>) -> Option<Parameters> {
    let pairs = pair.into_inner();
    let mut middle: Vec<String> = Vec::new();
    let mut trailing: Option<String> = None;

    for p in pairs {
        match p.as_rule() {
            Rule::middle => {
                //println!("Middle Pair: {:?}\n\n", p);
                middle.push(String::from(p.as_str()));
            }
            Rule::trailing => {
                //println!("Trailing Pair: {:?}\n\n", p);
                trailing = Some(String::from(p.as_str()));
            }
            _ => {}
        }
    }

    let params = Parameters {
        middle: middle,
        trailing: trailing
    };

    return Some(params);
}
