#![cfg(test)]

use crate::message::BaseMessage;
use crate::message::Command;
use std::str::FromStr;

use crate::constants::generate_reply_codes;

#[test]
fn privmsg_test() {
    let message = "PRIVMSG #main testing\r\n";
    let bm = BaseMessage::from_str(message).unwrap();

    //println!("Base Message: {:?}", bm);

    assert!(bm.prefix.is_none());

    match bm.command {
        Command::IrcCommand(cmd) => {
            assert_eq!(cmd, "PRIVMSG");
        }
        Command::IrcResponse(_) => {
            assert!(false);
        }
    }

    assert!(bm.parameters.is_some());

    let param = bm.parameters.unwrap();

    assert!(param.trailing.is_none());
    assert_eq!(param.middle.len(), 2);

    assert_eq!(param.middle[0], "#main");
    assert_eq!(param.middle[1], "testing");
}

#[test]
fn csv_read_test() {
    let target = "irc_response_codes.csv";
    let line_count = 160;
    let reply_codes = generate_reply_codes(target).expect("Test failed.");

    assert_eq!(reply_codes.len(), line_count);
}

#[derive(Debug)]
#[derive(IrcCommand)]
#[command = "TEST"]
struct TestMessage {
    #[prefix]
    prefix: Option<String>,
    test: Option<String>,
    #[separator = ","]
    arg_list: Option<Vec<String>>,
    #[trailing]
    trailing: Option<String>,
}

use crate::message::IrcMessage;
use crate::error::IrcCommandError;
use irc_derive::IrcCommand;

#[test]
fn irc_command_derive_test() {
    let good_commands = vec![
        ":testprefix TEST testarg testlist1,testlist2 :testtrailing",
        "TEST testarg testlist1,testlist2 :testtrailing",
        ":testprefix TEST testlist1,testlist2 :testtrailing",
        ":testprefix TEST testarg testlist1,testlist2",
        "TEST testarg testlist1,testlist2",
        ":testprefix TEST :testtrailing",
        "TEST",
    ];
    let bad_command = String::from("NOTTEST");

    let good_commands: Vec<String> = good_commands.iter().map(|s|
        format!("{}\r\n", s)
    ).collect();

    for command in &good_commands {
        println!("Testing: {}", command);

        let base_message = BaseMessage::from_str(command);
        assert!(base_message.is_ok());

        let test_message = TestMessage::parse_message(base_message.unwrap());
        if let Err(ref msg) = test_message {
            println!("{}", msg);
        }
        assert!(test_message.is_ok());

        let test_str = test_message.unwrap().create_message();
        assert_eq!(&test_str, command);
    }

    let base_message = BaseMessage::from_str(&bad_command);
    assert!(base_message.is_ok());

    let test_message = TestMessage::parse_message(base_message.unwrap());
    assert!(test_message.is_err());
}
