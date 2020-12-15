/// # Stream Id Types
/// streams either bidirectional or unidirectional. Moreover, streams could be created by the server or client, therefore there will be four possibilities.
/// page 12 draft IETF
enum StreamIdTypes {
    ServerBid,
    ServerUni,
    ClientBi,
    ClientUni,
}

pub struct Stream {
    pub id: u64,
}

/// #Stream
impl Stream {
    /// check the least significant bit
    /// if it's 1 it's server initiated
    /// if it's 0 it's client initiated
    pub fn is_server_initiated(&self) -> bool {
        if (self.id & 1 == 1) {
            return true;
        } else {
            return false;
        }
    }
    /// check the second least significant bit
    /// if it's 1 it's unidirectional
    /// if it's 0 it's bidirectional
    pub fn is_bidirectional() -> bool {
        if self.id & 2 == 1 {
            return false;
        } else {
            return true;
        }
    }
}
