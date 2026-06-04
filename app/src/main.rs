
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::sleep;

struct TowerConfig {
    name: &'static str,
    address: &'static str,
}

#[tokio::main]
async fn main() {
    // Defined matching the targets in your docker-compose.yml
    let targets = vec![
        TowerConfig { name: "Tower A", address: "127.0.0.1:10000" },
        TowerConfig { name: "Tower B", address: "127.0.0.1:10001" },
        TowerConfig { name: "Tower C", address: "127.0.0.1:10002" },
    ];

    println!("Starting Tower Monitoring Pipeline System Status...\n");

    loop {
        for tower in &targets {
            let status = check_tower_status(tower.address).await;
            
            // Format console output cleanly
            match status {
                true => println!("[ \x1b[32mUP\x1b[0m ] {}", tower.name),
                false => println!("[ \x1b[31mDOWN\x1b[0m ] {}", tower.name),
            }
        }
        
        println!("------------------------------------");
        sleep(Duration::from_secs(5)).await; // Poll every 5 seconds
    }
}

/// Dispatches an SNMP v2c GetRequest heartbeat packet over a UDP Socket 
/// to evaluate if a target simulator node is responsive[cite: 37, 38].
async fn check_tower_status(target: &str) -> bool {
    let remote_addr: SocketAddr = target.parse().unwrap();
    
    // Bind to any available local ephemeral port
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Standard raw SNMP v2c GetRequest byte payload for SysUpTime (1.3.6.1.2.1.1.3.0)
    // Community String used here matches your file name: "test"
    let snmp_get_request = vec![
        0x30, 0x29,       // Sequence container
        0x02, 0x01, 0x01, // Version: v2c (value 1)
        0x04, 0x04, 0x74, 0x65, 0x73, 0x74, // Community string: "test"
        0xa0, 0x1e,       // PDU Type: GetRequest
        0x02, 0x04, 0x00, 0x00, 0x00, 0x01, // Request ID
        0x02, 0x01, 0x00, // Error Status
        0x02, 0x01, 0x00, // Error Index
        0x30, 0x10,       // Varbind List Sequence
        0x30, 0x0e,       // Varbind Container
        0x06, 0x0a,       // Object Identifier Type & Length
        0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x01, 0x03, 0x00, // OID: 1.3.6.1.2.1.1.3.0
        0x05, 0x00,       // Null Value placeholder
    ];

    if socket.send_to(&snmp_get_request, remote_addr).await.is_err() {
        return false;
    }

    // Allocate a network read buffer
    let mut buffer = [0u8; 1024];
    
    // Dead Man's Switch / TTL constraint: 1.5 seconds timeout [cite: 84, 116]
    let timeout_duration = Duration::from_millis(1500);

    match tokio::time::timeout(timeout_duration, socket.recv_from(&mut buffer)).await {
        Ok(Ok((bytes_read, _src))) => {
            // Verify we got a semi-valid payload back from snmpsim-command-responder
            bytes_read > 0 && buffer[0] == 0x30 
        }
        _ => {
            // Either a flat socket error or a response timeout occurred (Tower is Down) [cite: 116]
            false
        }
    }
}
