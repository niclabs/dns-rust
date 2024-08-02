use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
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

}use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
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

}use tokio::time::Instant;

use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
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

}use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
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

}use tokio::time::Instant;

use tokio::time::Instant;

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    request_global_counter: u32,
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
    pub fn new(request_global_limit: u32) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            request_global_counter: request_global_limit,
        }
    }

    /// Decrements the `request_global_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_request_global_counter();
    /// ```
    pub fn decrement_request_global_counter(&mut self) {
        self.request_global_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `request_global_counter` of the request.
    pub fn get_request_global_counter(&self) -> u32 {
        return self.request_global_counter;
    }
}