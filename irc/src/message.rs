use pest_derive::Parser;
use pest::error::Error;
use pest::Parser;

use std::str::FromStr;
use std::fmt;

pub trait IrcMessage {
    fn parse_message(message: BaseMessage) -> Result<Box<Self>, Box<dyn std::error::Error>>;
    fn create_message(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum Command {
    IrcCommand(String),
    IrcResponse(String)
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = match self {
            Command::IrcCommand(cmd) => cmd,
            Command::IrcResponse(cmd) => cmd,
        }.to_string();

        write!(f, "{}", command)
    }
}

#[derive(Debug)]
pub struct UserPrefix {
    pub nickname: String,
    pub user: Option<String>,
    pub host: Option<String>
}

impl fmt::Display for UserPrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut prefix = self.nickname.clone();
        
        match self.host {
            Some(ref host) => {
                match self.user {
                    Some(ref user) => {
                        prefix.push_str(&format!("!{}", user));
                    },
                    None => {}
                }
                prefix.push_str(&format!("@{}", host));
            },
            None => {}
        }

        write!(f, "{}", prefix)
    }
}

#[derive(Debug)]
pub enum Prefix {
    ServerName(String),
    UserName(UserPrefix)
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self {
            Prefix::ServerName(p) => p.to_string(),
            Prefix::UserName(up) => up.to_string()
        };

        write!(f, "{}", prefix)
    }
}

#[derive(Debug)]
pub struct Parameters {
    pub middle: Vec<String>,
    pub trailing: Option<String>
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let middle = self.middle.join(" ");
        
        if let Some(ref trailing) = self.trailing {
            write!(f, "{} :{}", middle, trailing)
        } else {
            write!(f, "{}", middle)
        }
    }
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

impl fmt::Display for BaseMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message = String::from("");

        if let Some(ref prefix) = self.prefix {
            message.push_str(&format!("{} ", &prefix.to_string()));
        }

        message.push_str(&self.command.to_string());

        if let Some(ref parameters) = self.parameters {
            message.push_str(&format!(" {}", &parameters.to_string()));
        }

        write!(f, "{}", message)
    }
}

impl FromStr for BaseMessage {
    type Err = Error<Rule>;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let parse_result = IrcParser::parse(Rule::message, message);
        let message_pairs = parse_result?.next().unwrap().into_inner();

        let mut prefix: Option<Prefix> = None;
        let mut command: Command = Command::IrcCommand(String::from(""));
        let mut parameters: Option<Parameters> = None;

        for pair in message_pairs {
            match pair.as_rule() {
                Rule::prefix => {
                    prefix = parse_prefix(pair);
                }
                Rule::command => {
                    command = parse_command(pair);
                }
                Rule::params => {
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
                return Some(Prefix::ServerName(String::from(p.as_str())));
            }
            Rule::username => {
                let username_pair = p.into_inner();
                let mut nickname: String = String::from("");
                let mut user: Option<String> = None;
                let mut host: Option<String> = None;
                
                for up in username_pair {
                    match up.as_rule() {
                        Rule::nickname => {
                            nickname = String::from(up.as_str());
                        }
                        Rule::user => {
                            user = Some(String::from(up.as_str()));
                        }
                        Rule::host => {
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
                return Command::IrcCommand(String::from(p.as_str()));
            }
            Rule::numresponse => {
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
                middle.push(String::from(p.as_str()));
            }
            Rule::trailing => {
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
