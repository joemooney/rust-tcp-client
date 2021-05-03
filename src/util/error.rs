use thiserror::Error;

pub type TcpResult<T> = std::result::Result<T, TcpError>;

#[derive(Error, Debug)]
/// TcpError enumerates all the possible errors returned during protobuf message processing
pub enum TcpError {
    /// A error trying where client disconnected prematurely
    #[error("Client disconnected")]
    ClientDisconnectionError,
    /// A error trying to send a UDP message
    #[error("Send error")]
    SendError { source: std::io::Error },
    /// A error trying to deserialize a protobuf message into an object
    //#[error("Failed to deserialize protobuf message")]
    //DeserializeError { source: std::io::Error },
    /// A error trying to serialize an object into a protobuf message
    //#[error("Failed to serialize protobuf message")]
    //SerializeError { source: std::io::Error },
    /// A error trying to bind to a UDP port
    //#[error("Bind error")]
    //UDPBindError { source: std::io::Error },
    /// All other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
