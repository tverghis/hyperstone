/// Errors that the parser can return while navigating the replay buffer.
#[derive(Debug, PartialEq)]
pub enum HyperstoneError {
    /// The replay could not be verified to be a valid Source 2 replay.
    UnverifiableBuffer,
    /// The parser encountered an unknown outer message while parsing.
    /// There are two potential causes:
    ///   1. The data in the replay buffer is corrupted; or
    ///   2. The protobufs are out-of-date and need to be updated.
    UnknownOuterMessage,
    UnknownDemoCommand,
}
