use error::{Error, NetError, ServerError};
use std::io;

#[test]
fn test_net_error_bind_error_display() {
    let err = NetError::BindError("Failed to bind to 0.0.0.0:19132".to_string());
    let display = format!("{}", err);

    assert!(display.contains("Bind Error"));
    assert!(display.contains("Failed to bind to 0.0.0.0:19132"));
}

#[test]
fn test_net_error_debug() {
    let err = NetError::BindError("Test error".to_string());
    let debug = format!("{:?}", err);

    assert!(debug.contains("BindError"));
    assert!(debug.contains("Test error"));
}

#[test]
fn test_error_from_net_error() {
    let net_err = NetError::BindError("Network issue".to_string());
    let err: Error = net_err.into();

    match err {
        Error::Net(_) => {}
        _ => panic!("Expected Net error variant"),
    }
}

#[test]
fn test_error_from_server_error() {
    let server_err = ServerError {};
    let err: Error = server_err.into();

    match err {
        Error::Server(_) => {}
        _ => panic!("Expected Server error variant"),
    }
}

#[test]
fn test_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let err: Error = io_err.into();

    match err {
        Error::Io(_) => {}
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_error_display_net() {
    let net_err = NetError::BindError("Socket error".to_string());
    let err: Error = net_err.into();
    let display = format!("{}", err);

    assert!(display.contains("Net Layer Error"));
}

#[test]
fn test_error_display_io() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
    let err: Error = io_err.into();
    let display = format!("{}", err);

    assert!(display.contains("I/O Error"));
}

#[test]
fn test_result_type_ok() {
    let result: error::Result<i32> = Ok(42);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_type_err() {
    let net_err = NetError::BindError("Test".to_string());
    let result: error::Result<i32> = Err(net_err.into());

    assert!(result.is_err());
}

#[test]
fn test_error_debug() {
    let err = Error::Net(NetError::BindError("Debug test".to_string()));
    let debug = format!("{:?}", err);

    assert!(debug.contains("Net"));
}

#[test]
fn test_server_error_display() {
    let err = ServerError {};
    let display = format!("{}", err);

    assert!(!display.is_empty());
}

#[test]
fn test_server_error_debug() {
    let err = ServerError {};
    let debug = format!("{:?}", err);

    assert!(debug.contains("ServerError"));
}
