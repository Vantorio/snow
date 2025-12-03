use server::Server;

#[tokio::test]
async fn test_server_creation_success() {
    let result = Server::new("127.0.0.1".to_string(), 19132).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_with_different_host() {
    let result = Server::new("0.0.0.0".to_string(), 19132).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_with_different_port() {
    let result = Server::new("127.0.0.1".to_string(), 25565).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_localhost() {
    let result = Server::new("localhost".to_string(), 19132).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_ipv6() {
    let result = Server::new("::1".to_string(), 19132).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_custom_port() {
    let port = 30000 + (rand::random::<u16>() % 1000);
    let result = Server::new("127.0.0.1".to_string(), port).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_high_port() {
    let result = Server::new("127.0.0.1".to_string(), 65535).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_low_port() {
    let result = Server::new("127.0.0.1".to_string(), 1024).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_start_and_shutdown() {
    let port = 19200 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    handle.abort();
    
    let result = handle.await;
    assert!(result.is_err() || result.unwrap().is_ok());
}

#[tokio::test]
async fn test_server_multiple_instances() {
    let port1 = 19300 + (rand::random::<u16>() % 100);
    let port2 = port1 + 100;
    
    let server1 = Server::new("127.0.0.1".to_string(), port1).await;
    let server2 = Server::new("127.0.0.1".to_string(), port2).await;
    
    assert!(server1.is_ok());
    assert!(server2.is_ok());
}

#[tokio::test]
async fn test_server_creation_empty_host() {
    let result = Server::new("".to_string(), 19132).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_server_creation_with_hostname() {
    let result = Server::new("example.local".to_string(), 19132).await;
    
    assert!(result.is_ok());
}
