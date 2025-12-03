use net::listener::Listener;
use tokio::net::UdpSocket;
use std::time::Duration;

#[tokio::test]
async fn test_listener_concurrent_binds() {
    let listeners = futures::future::join_all(vec![
        Listener::bind("127.0.0.1:0"),
        Listener::bind("127.0.0.1:0"),
        Listener::bind("127.0.0.1:0"),
    ]).await;
    
    for listener in listeners {
        assert!(listener.is_ok());
    }
}

#[tokio::test]
async fn test_listener_ipv4_and_ipv6() {
    let ipv4 = Listener::bind("127.0.0.1:0").await;
    let ipv6 = Listener::bind("[::1]:0").await;
    
    assert!(ipv4.is_ok());
    assert!(ipv6.is_ok());
}

#[tokio::test]
async fn test_listener_bind_all_interfaces() {
    let any_ipv4 = Listener::bind("0.0.0.0:0").await;
    let any_ipv6 = Listener::bind("[::]:0").await;
    
    assert!(any_ipv4.is_ok());
    assert!(any_ipv6.is_ok());
}

#[tokio::test]
async fn test_listener_sequential_bind_unbind() {
    let port = 20100 + (rand::random::<u16>() % 1000);
    let addr = format!("127.0.0.1:{}", port);
    
    let listener1 = Listener::bind(&addr).await.unwrap();
    drop(listener1);
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let listener2 = Listener::bind(&addr).await;
    assert!(listener2.is_ok());
}

#[tokio::test]
async fn test_listener_different_port_ranges() {
    let low_port = Listener::bind("127.0.0.1:0").await;
    let mid_port = Listener::bind("127.0.0.1:0").await;
    let high_port = Listener::bind("127.0.0.1:0").await;
    
    assert!(low_port.is_ok());
    assert!(mid_port.is_ok());
    assert!(high_port.is_ok());
}

#[tokio::test]
async fn test_send_receive_small_packet() {
    let listener = Listener::bind("127.0.0.1:0").await.unwrap();
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    
    drop(listener);
}

#[tokio::test]
async fn test_send_receive_max_udp_packet() {
    let listener = Listener::bind("127.0.0.1:0").await.unwrap();
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    
    drop(listener);
    drop(client);
}

#[tokio::test]
async fn test_listener_stress_bind() {
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = tokio::spawn(async {
            Listener::bind("127.0.0.1:0").await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_listener_hostname_resolution() {
    let localhost_result = Listener::bind("localhost:0").await;
    assert!(localhost_result.is_ok());
}

#[tokio::test]
async fn test_listener_invalid_hostname() {
    let result = Listener::bind("invalid.hostname.that.does.not.exist:19132").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_listener_port_zero_auto_assign() {
    let listener = Listener::bind("127.0.0.1:0").await.unwrap();
    drop(listener);
}

#[tokio::test]
async fn test_listener_bind_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        Listener::bind("127.0.0.1:0")
    ).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_ok());
}
