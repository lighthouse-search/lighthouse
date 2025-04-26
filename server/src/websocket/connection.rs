use diesel::prelude::*;

use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, tungstenite::protocol::CloseFrame};
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;

use crate::global::{request_authentication, request_authentication_staticauth};
use crate::structs::*;
use crate::tables::*;

pub async fn handle_connection(stream: TcpStream) {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Upgrade the TCP stream to a WebSocket connection
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection established [server]");

    let (mut write, mut read) = ws_stream.split();

    let write = Arc::new(Mutex::new(write));
    let write2_clone = Arc::clone(&write);
    
    loop {
        // TODO: Set an actual condition here. This "break;" will cause the connection to hang, but the actual disconnect packet doesn't work for some reason.
        if (1 == 2) {
            // write.send(Message::Close(None)).await.expect("Failed to disconnect client gracefully.");
            //// write.close().await.expect("Failed to close websocket.");
            break;
        }

        if let Some(msg) = read.next().await {
            match msg {
                Ok(message) => {
                    // TODO: If there's an error, the entire socket will break.

                    // println!("Received [server]: {}", message.to_text().unwrap());
                    let mut write = write2_clone.lock().await;

                    let websocket_event: Websocket_event = serde_json::from_str(message.to_text().unwrap()).expect("From_value failed");
                    let hades_websocket = websocket_event._hades_websocket.unwrap();
                    let body = websocket_event.body.expect("No body in websocket event.");
                    let body_value: Value = serde_json::from_str(&body).expect("Failed to parse body.");

                    println!("JWT {:?}", hades_websocket.jwt.as_deref());
                    
                    let device_id = match body_value.get("device_id") {
                        Some(id) => id.as_str(),
                        None => None // No device id.
                    };

                    let project_id = match body_value.get("project_id") {
                        Some(id) => id.as_str(),
                        None => None // No device id.
                    };

                    // Authenticate signed event.
                    let request_authentication_output: Request_authentication_output = match request_authentication_staticauth(hades_websocket.jwt.as_deref(), Some(device_id.expect("device_id not provided.")), Some(project_id.expect("project_id not provided."))).await {
                        Ok(data) => data,
                        Err(e) => { println!("Authentication failed."); break; }
                    };

                    println!("{:?}", request_authentication_output);

                    if (hades_websocket.event.unwrap() == "process") {
                        println!("body_value {}", body_value);
                        // Parse incoming JSON.
                        let process_event: Websocket_event_process = serde_json::from_str(&serde_json::to_string(&body_value).unwrap()).expect("Failed to parse websocket event (process)");
                        // Process all events in bulk data.
                        for process in process_event.processes {
                            let process_insert = Rover_processes {
                                device_id: request_authentication_output.device_id.clone(),
                                PID: process.pid,
                                process: process.name,
                                user: None,
                                admin_user: Some(false),
                                is_admin_process: Some(false),
                                publisher: None,
                                hash: None,
                                threads: Some(0),
                                size: process.size,
                                pathname: process.pathname,
                                last_seen: Some(0),
                                created: Some(0)
                            };
                            // Insert processes into database.
                            diesel::insert_into(rover_processes::table)
                            .values(&process_insert)
                            .execute(&mut db)
                            .expect("fail");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }
    }

    println!("WebSocket connection closed [server]");
}