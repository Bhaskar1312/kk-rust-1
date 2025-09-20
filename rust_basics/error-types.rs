// Define two enums:

// NetworkError with the following variants:
// Disconnected
// Timeout
// CustomError with the following variants:
// NotFound
// Network which contains a NetworkError
// Implement the From trait for CustomError to automatically convert a NetworkError into a CustomError::Network.

// Create a function simulate_network() that returns a Result<(), NetworkError>:

// This function should always return an error with the variant NetworkError::Disconnected.
// Implement the main function:

// Use the simulate_network() function, and use the ? operator to propagate the error.
// Return Ok(()) on successful execution or handle the error appropriately.

#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
}


#[derive(Debug)]
enum CustomError {
    NotFound,
    Network(NetworkError),
}

impl From<NetworkError> for CustomError {
    fn from(err: NetworkError) -> Self {
        CustomError::Network(err)
    }
}

fn simulate_network() -> Result<(), NetworkError> {
    Err(NetworkError::Disconnected)
}

fn main()  -> Result<(), CustomError> {
    simulate_network()?;
    Ok(())
}