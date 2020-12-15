pub mod stream;
use stream::*;
#[cfg(test)]
mod tests {
    #[test]
    fn cehckStream() {
        let clientBidirectional = crate::stream::Stream { id: 0 };
        let clientUnidirectional = crate::stream::Stream { id: 2 };
        let serverUnidirectional = crate::stream::Stream { id: 3 };
        let serverBidirectional = crate::stream::Stream { id: 1 };
        let assert_eq!(test1.is_server_initiated(), false);
    }
}
