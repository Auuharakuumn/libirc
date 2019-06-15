#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::IrcCommandError;
use crate::message::BaseMessage;
use crate::message::IrcMessage;
use irc_derive::IrcCommand;

#[derive(IrcCommand)]
struct Admin {
    target: Option<String>,
}

#[derive(IrcCommand)]
struct Away {
    message: Option<String>,
}

#[derive(IrcCommand)]
struct CNotice {
    nickname: String,
    channel: String,
    #[trailing]
    message: String,
}

#[derive(IrcCommand)]
struct CPrivMessage {
    nickname: String,
    channel: String,
    #[trailing]
    message: String,
}

#[derive(IrcCommand)]
struct Connect {
    target_server: String,
    port: String,
    remote_server: Option<String>,
}

#[derive(IrcCommand)]
struct Die {}

#[derive(IrcCommand)]
struct Encap {
    #[prefix]
    source: String,
    destination: String,
    parameters: String,
}

#[derive(IrcCommand)]
#[command = "ERROR"]
struct ErrorCommand {
    error_message: String,
}

#[derive(IrcCommand)]
struct Help {}

#[derive(IrcCommand)]
struct Info {
    target: Option<String>,
}

#[derive(IrcCommand)]
struct Invite {
    nickname: String,
    channel: String,
}

#[derive(IrcCommand)]
struct IsOn {
    #[separator = " "]
    nicknames: Vec<String>,
}

#[derive(IrcCommand)]
struct Join {
    #[separator = ","]
    channels: Vec<String>,
    #[separator = ","]
    keys: Option<Vec<String>>,
}

#[derive(IrcCommand)]
struct Kick {
    channel: String,
    client: String,
    #[trailing]
    message: Option<String>,
}

#[derive(IrcCommand)]
struct Kill {
    client: String,
    comment: String,
}

#[derive(IrcCommand)]
struct Knock {
    channel: String,
    message: Option<String>,
}

#[derive(IrcCommand)]
struct Links {
    remote_server: Option<String>,
    server_mask: Option<String>,
}

#[derive(IrcCommand)]
struct List {
    #[separator = ","]
    channels: Option<Vec<String>>,
    server: Option<String>,
}

#[derive(IrcCommand)]
struct LUsers {
    mask: Option<String>,
    server: Option<String>,
}

#[derive(IrcCommand)]
struct UserMode {
    nickname: String,
    flags: String,
    user: String,
}

#[derive(IrcCommand)]
struct ChannelMode {
    channel: String,
    flags: String,
    args: Option<String>,
}

#[derive(IrcCommand)]
struct Motd {
    server: Option<String>,
}

#[derive(IrcCommand)]
struct Names {
    #[separator = ","]
    channels: Option<Vec<String>>,
    server: Option<String>,
}

#[derive(IrcCommand)]
#[command = "PROTOCTL NAMESX"]
struct Namesx {}

#[derive(IrcCommand)]
struct Nick {
    nickname: String,
}

#[derive(IrcCommand)]
struct Notice {
    target: String,
    message: String,
}

#[derive(IrcCommand)]
struct Oper {
    username: String,
    password: String,
}

#[derive(IrcCommand)]
struct Part {
    #[separator = ","]
    channels: Vec<String>,
    message: Option<String>,
}

#[derive(IrcCommand)]
struct Pass {
    password: String,
}

#[derive(IrcCommand)]
struct Ping {
    server1: String,
    server2: Option<String>,
}

#[derive(IrcCommand)]
struct Pong {
    server1: String,
    server2: Option<String>,
}

#[derive(IrcCommand)]
struct PrivMsg {
    target: String,
    message: String,
}

#[derive(IrcCommand)]
struct Quit {
    message: Option<String>,
}

#[derive(IrcCommand)]
struct Rehash {}

#[derive(IrcCommand)]
struct Restart {}

#[derive(IrcCommand)]
struct Rules {}

#[derive(IrcCommand)]
struct Server {
    server: String,
    hopcount: String,
    info: String,
}

#[derive(IrcCommand)]
struct Service {
    nickname: String,
    reserved1: String,
    distribution: String,
    service_type: String,
    reserved2: String,
    info: String,
}

#[derive(IrcCommand)]
struct ServList {
    mask: Option<String>,
    service_type: Option<String>,
}

#[derive(IrcCommand)]
struct SQuery {
    service: String,
    text: String,
}

#[derive(IrcCommand)]
struct SQuit {
    server: String,
    comment: String,
}

#[derive(IrcCommand)]
struct SetName {
    name: String,
}

// TODO: Watch and Silence will probably need manual implementations
struct Silence {
    mask: Option<Vec<(String, bool)>>,
}

#[derive(IrcCommand)]
struct Stats {
    query: String,
    server: Option<String>,
}

#[derive(IrcCommand)]
struct Summon {
    user: String,
    server: Option<String>,
    channel: Option<String>,
}

#[derive(IrcCommand)]
struct Time {
    server: Option<String>,
}

#[derive(IrcCommand)]
struct Topic {
    channel: String,
    topic: Option<String>,
}

#[derive(IrcCommand)]
struct Trace {
    target: Option<String>,
}

#[derive(IrcCommand)]
#[command = "PROTOCTL UHNAMES"]
struct UHNames {}

#[derive(IrcCommand)]
struct User {
    user: String,
    mode: String,
    unused: String,
    realname: String,
}

#[derive(IrcCommand)]
struct UserHost {
    nickname: String,
    #[separator = " "]
    extra_nicknames: Option<Vec<String>>,
}

#[derive(IrcCommand)]
struct UserIP {
    nickname: String,
}

#[derive(IrcCommand)]
struct Users {
    server: Option<String>,
}

#[derive(IrcCommand)]
struct Version {
    server: Option<String>,
}

#[derive(IrcCommand)]
struct WAllOps {
    message: String,
}

// TODO: Watch and Silence will probably need manual implementations
struct Watch {
    nicknames: Option<Vec<(String, bool)>>,
}

// TODO: potentially needs manual implementations
struct Who {
    name: Option<String>,
    o: bool,
}

#[derive(IrcCommand)]
struct WhoIs {
    server: Option<String>,
    #[separator = ","]
    nicknames: Vec<String>,
}

#[derive(IrcCommand)]
struct WhoWas {
    nickname: String,
    count: Option<String>,
    server: Option<String>,
}
