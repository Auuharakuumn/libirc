#![allow(dead_code)]

struct Admin {
    command: String,
    target: String
}

struct Away {
    command: String,
    message: String
}

struct CNotice {
    command: String,
    nickname: String,
    channel: String,
    message: String
}

struct CPrivMessage {
    command: String,
    nickname: String,
    channel: String,
    message: String
}

struct Connect {
    command: String,
    target_server: String,
    port: Option<String>,
    remote_server: Option<String>
}

struct Die {
    command: String
}

struct Encap {
    source: String,
    command: String,
    destination: String,
    subcommand: String,
    parameters: String
}

struct Error {
    command: String,
    error_message: String
}

struct Help {
    command: String
}

struct Info {
    command: String,
    target: String
}

struct Invite {
    command: String,
    nickname: String,
    channel: String
}

struct IsOn {
    command: String,
    nicknames: Vec<String>,
}

struct Join {
    command: String,
    channels: Vec<String>,
    keys: Option<Vec<String>>,
}

struct Kick {
    command: String,
    channel: String,
    client: String,
    message: String,
}

struct Kill {
    command: String,
    client: String,
    comment: String,
}

struct Knock {
    command: String,
    channel: String,
    message: Option<String>,
}

struct Links {
    command: String,
    remote_server: Option<String>,
    server_mask: Option<String>,
}

struct List {
    command: String,
    channels: Option<Vec<String>>,
    server: Option<String>,
}

struct LUsers {
    command: String,
    mask: Option<String>,
    server: Option<String>,
}

struct UserMode {
    command: String,
    nickname: String,
    flags: String,
    user: String,
}

struct ChannelMode {
    command: String,
    channel: String,
    flags: String,
    args: Option<Vec<String>>,
}

struct Motd {
    command: String,
    server: Option<String>,
}

struct Names {
    command: String,
    channels: Option<Vec<String>>,
    server: Option<String>,
}

struct Namesx {
    command: String
}

struct Nick {
    command: String,
    nickname: String,
    hopcount: Option<String>,
}

struct Notice {
    command: String,
    target: String,
    message: String,
}

struct Oper {
    command: String,
    username: String,
    password: String,
}

struct Part {
    command: String,
    channels: Vec<String>,
    message: Option<String>,
}

struct Pass {
    command: String,
    password: String,
}

struct Ping {
    command: String,
    server1: String,
    server2: Option<String>,
}

struct Pong {
    command: String,
    server1: String,
    server2: Option<String>,
}

struct PrivMsg {
    command: String,
    target: String,
    message: String,
}

struct Quit {
    command: String,
    message: Option<String>,
}

struct Rehash {
    command: String
}

struct Restart {
    command: String
}

struct Rules {
    command: String
}

struct Server {
    command: String,
    server: String,
    hopcount: String,
    info: String,
}

struct Service {
    command: String,
    nickname: String,
    reserved1: String,
    distribution: String,
    service_type: String,
    reserved2: String,
    info: String,
}

struct ServList {
    command: String,
    mask: Option<String>,
    service_type: Option<String>,
}

struct SQuery {
    command: String,
    service: String,
    text: String,
}

struct SQuit {
    command: String,
    server: String,
    comment: String,
}

struct SetName {
    command: String,
    name: String,
}

struct Silence {
    command: String,
    mask: Option<Vec<(String, bool)>>,
}

struct Stats {
    command: String,
    query: String,
    server: Option<String>,
}

struct Summon {
    command: String,
    user: String,
    server: Option<String>,
    channel: Option<String>,
}

struct Time {
    command: String,
    server: Option<String>,
}

struct Topic {
    command: String,
    channel: String,
    topic: Option<String>,
}

struct Trace {
    command: String,
    target: Option<String>,
}

struct UHNames {
    command: String
}

struct User {
    command: String,
    user: String,
    mode: String,
    unused: String,
    realname: String,
}

struct UserHost {
    command: String,
    nickname: String,
    extra_nicknames: Option<Vec<String>>,
}

struct UserIP {
    command: String,
    nickname: String,
}

struct Users {
    command: String,
    server: Option<String>,
}

struct Version {
    command: String,
    server: Option<String>,
}

struct WAllOps {
    command: String,
    message: String,
}

struct Watch {
    command: String,
    nicknames: Option<Vec<(String, bool)>>,
}

struct Who {
    command: String,
    name: Option<String>,
    o: bool,
}

struct WhoIs {
    command: String,
    server: Option<String>,
    nicknames: Vec<String>
}

struct WhoWas {
    command: String,
    nickname: String,
    count: Option<String>,
    server: Option<String>,
}

