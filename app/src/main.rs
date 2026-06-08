use std::time::Duration;
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

    let poll_interval = 5; 
                           
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
        sleep(Duration::from_secs(poll_interval)).await; 
    }
}

/// Evaluates if a target tower is up using the official community snmp client 
async fn check_tower_status(target: &str) -> bool {
    let target_addr = target.to_string();

    // Leverage spawn_blocking to process the synchronous socket read/write operations 
    // safely away from async event loop queueing friction 
    tokio::task::spawn_blocking(move || {
        // Match the loose 1.5-second fallback window used previously 
        let timeout = Duration::from_millis(1500);
        
        // Initialize an SNMP SyncSession using community "public" 
        let mut client = match snmp::SyncSession::new(&target_addr, b"public", Some(timeout), 0) {
            Ok(session) => session,
            Err(_) => return false,
        };

        // Define the standard SysUpTime OID array: 1.3.6.1.2.1.1.3.0
        let sys_uptime_oid = [1, 3, 6, 1, 2, 1, 1, 3, 0];
        
        // Issue a formal GetRequest through the client driver library 
        match client.get(&sys_uptime_oid) {
            Ok(mut response) => {
                // If a valid SNMP response sequence containing our target data arrives, the node is UP 
                if let Some((_oid, _value)) = response.varbinds.next() {
                    return true;
                }
                false
            }
            Err(_) => {
                // Any packet corruption, timeout, or socket drop defaults to DOWN 
                false
            }
        }
    })
    .await
    .unwrap_or(false)
}
