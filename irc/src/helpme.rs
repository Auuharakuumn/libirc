use crate::message::BaseMessage;
use crate::message::IrcMessage;
use crate::error::IrcCommandError;
use irc_derive::IrcCommand;

#[derive(Debug)]
#[derive(IrcCommand)]
#[command = "TEST"]
struct TestMessage {
    #[prefix = ""]
    prefix: Option<String>,
    test: Option<String>,
    #[separator = ","]
    arg_list: Option<Vec<String>>,
    #[trailing = ""]
    trailing: Option<String>,
}
