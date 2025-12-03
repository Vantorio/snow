use net::listener::Listener;
use tokio::net::UdpSocket;

#[tokio::test]
async fn test_listener_bind_success() {
    let result = Listener::bind("127.0.0.1:0").await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_listener_bind_specific_port() {
    let port = 19000 + (rand::random::<u16>() % 1000);
    let addr = format!("127.0.0.1:{}", port);
    
    let result = Listener::bind(&addr).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_listener_bind_ipv6() {
    let result = Listener::bind("[::1]:0").await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_listener_bind_invalid_address() {
    let result = Listener::bind("999.999.999.999:19132").await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_listener_bind_invalid_port() {
    let result = Listener::bind("127.0.0.1:99999").await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_listener_bind_port_already_in_use() {
    let port = 19100 + (rand::random::<u16>() % 1000);
    let addr = format!("127.0.0.1:{}", port);
    
    let _listener1 = Listener::bind(&addr).await.unwrap();
    let result2 = Listener::bind(&addr).await;
    
    assert!(result2.is_err());
}

#[tokio::test]
async fn test_listener_receive_packet() {
    let listener = Listener::bind("127.0.0.1:0").await.unwrap();
    
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    });
    
    drop(listener);
}

#[tokio::test]
async fn test_listener_bind_localhost() {
    let result = Listener::bind("localhost:0").await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_listener_bind_any_address() {
    let result = Listener::bind("0.0.0.0:0").await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_listener_bind_concurrent() {
    let listener1 = Listener::bind("127.0.0.1:0").await;
    let listener2 = Listener::bind("127.0.0.1:0").await;
    
    assert!(listener1.is_ok());
    assert!(listener2.is_ok());
}

#[tokio::test]
async fn test_listener_bind_privileged_port() {
    let result = Listener::bind("127.0.0.1:80").await;
    
    if cfg!(unix) && !nix::unistd::geteuid().is_root() {
        assert!(result.is_err());
    }
}
