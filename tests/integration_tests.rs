use server::Server;
use std::time::Duration;
use tokio::net::UdpSocket;

#[tokio::test]
async fn test_full_server_lifecycle() {
    let port = 19400 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_server_receives_udp_packets() {
    let port = 19500 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let server_addr = format!("127.0.0.1:{}", port);
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let send_result = client.send_to(b"test packet", &server_addr).await;
    
    assert!(send_result.is_ok());
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_server_handles_multiple_packets() {
    let port = 19600 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let server_addr = format!("127.0.0.1:{}", port);
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    
    for i in 0..10 {
        let msg = format!("packet {}", i);
        let result = client.send_to(msg.as_bytes(), &server_addr).await;
        assert!(result.is_ok());
    }
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_server_handles_large_packet() {
    let port = 19700 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let server_addr = format!("127.0.0.1:{}", port);
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let large_packet = vec![0u8; 1400];
    let result = client.send_to(&large_packet, &server_addr).await;
    
    assert!(result.is_ok());
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_server_handles_empty_packet() {
    let port = 19800 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let server_addr = format!("127.0.0.1:{}", port);
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let result = client.send_to(b"", &server_addr).await;
    
    assert!(result.is_ok());
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_multiple_clients_connect() {
    let port = 19900 + (rand::random::<u16>() % 1000);
    let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
    
    let server_addr = format!("127.0.0.1:{}", port);
    
    let handle = tokio::spawn(async move {
        server.start().await
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut clients = Vec::new();
    for _ in 0..5 {
        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        clients.push(client);
    }
    
    for (i, client) in clients.iter().enumerate() {
        let msg = format!("client {}", i);
        let result = client.send_to(msg.as_bytes(), &server_addr).await;
        assert!(result.is_ok());
    }
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_rapid_start_stop() {
    for i in 0..3 {
        let port = 20000 + i * 10 + (rand::random::<u16>() % 10);
        let server = Server::new("127.0.0.1".to_string(), port).await.unwrap();
        
        let handle = tokio::spawn(async move {
            server.start().await
        });
        
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        handle.abort();
        let _ = handle.await;
        
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
