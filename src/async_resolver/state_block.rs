use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the next name server address to
/// query
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,
}

impl StateBlock {
    /// Creates a new StateBlock for a request.
    /// 
    /// # Arguments
    /// * `timestamp` - A Instant that represents the time the request began.
    /// 
    /// # Example
    /// ```
    /// let state_block = StateBlock::new(Instant::now());
    /// ```
    pub fn new() -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
        }
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

}