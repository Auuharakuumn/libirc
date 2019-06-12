extern crate pest;
extern crate pest_derive;
extern crate csv;
extern crate irc_derive;

pub mod message;
pub mod constants; 
pub mod error;
pub mod default_messages;

#[cfg(test)]
mod tests {
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
}

