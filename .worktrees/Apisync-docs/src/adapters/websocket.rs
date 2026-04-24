//! WebSocket adapter for real-time bidirectional communication
//!
//! Provides WebSocket client and server implementations using tokio-tungstenite

use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, client_async, tungstenite::Message, WebSocketStream};
use std::net::SocketAddr;
use tracing::{debug, error, info, warn};

use crate::error::{Error, Result};
use crate::ports::{Connection, ConnectionStatus, MessageStream, Transport};

/// WebSocket transport implementation
pub struct WebSocketTransport;

#[async_trait]
impl Transport for WebSocketTransport {
    type Connection = WebSocketConnection;
    type Config = WebSocketConfig;

    fn name() -> &'static str {
        "websocket"
    }

    async fn connect(config: &Self::Config) -> Result<Self::Connection> {
        let url = format!("ws://{}:{}", config.host, config.port);
        let (stream, _) = client_async(&url, TcpStream::connect((config.host.clone(), config.port)).await?).await?;
        
        info!("WebSocket client connected to {}", url);
        
        Ok(WebSocketConnection {
            stream,
            status: ConnectionStatus::Connected,
            remote_addr: format!("{}:{}", config.host, config.port),
        })
    }

    async fn bind(config: &Self::Config) -> Result<WebSocketListener> {
        let addr = SocketAddr::new(config.host.parse()?, config.port);
        let listener = TcpListener::bind(&addr).await?;
        
        info!("WebSocket server bound to {}", addr);
        
        Ok(WebSocketListener { listener })
    }
}

/// WebSocket configuration
#[derive(Clone, Debug)]
pub struct WebSocketConfig {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub max_message_size: usize,
    pub heartbeat_interval: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            path: "/ws".to_string(),
            max_message_size: 10 * 1024 * 1024, // 10MB
            heartbeat_interval: 30,
        }
    }
}

/// WebSocket listener for incoming connections
pub struct WebSocketListener {
    listener: TcpListener,
}

impl WebSocketListener {
    /// Accept a new WebSocket connection
    pub async fn accept(&self) -> Result<WebSocketConnection> {
        let (stream, addr) = self.listener.accept().await?;
        let (ws_stream, _) = accept_async(stream).await?;
        
        info!("WebSocket connection accepted from {}", addr);
        
        Ok(WebSocketConnection {
            stream: ws_stream,
            status: ConnectionStatus::Connected,
            remote_addr: addr.to_string(),
        })
    }
}

/// WebSocket connection wrapper
pub struct WebSocketConnection {
    stream: WebSocketStream<TcpStream>,
    status: ConnectionStatus,
    remote_addr: String,
}

#[async_trait]
impl Connection for WebSocketConnection {
    async fn send(&mut self, message: crate::ports::Message) -> Result<()> {
        let ws_message = match message {
            crate::ports::Message::Text(text) => Message::Text(text),
            crate::ports::Message::Binary(data) => Message::Binary(data.to_vec()),
            crate::ports::Message::Ping => Message::Ping(vec![]),
            crate::ports::Message::Pong => Message::Pong(vec![]),
        };
        
        self.stream.send(ws_message).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<crate::ports::Message> {
        match self.stream.next().await {
            Some(Ok(Message::Text(text))) => Ok(crate::ports::Message::Text(text)),
            Some(Ok(Message::Binary(data))) => Ok(crate::ports::Message::Binary(data.into())),
            Some(Ok(Message::Ping(_))) => Ok(crate::ports::Message::Ping),
            Some(Ok(Message::Pong(_))) => Ok(crate::ports::Message::Pong),
            Some(Ok(Message::Close(_))) => {
                self.status = ConnectionStatus::Disconnected;
                Err(Error::disconnected("Connection closed by peer"))
            }
            Some(Err(e)) => {
                error!("WebSocket error: {}", e);
                Err(Error::transport(e.to_string()))
            }
            None => {
                self.status = ConnectionStatus::Disconnected;
                Err(Error::disconnected("Connection closed"))
            }
        }
    }

    async fn close(mut self) -> Result<()> {
        self.stream.close(None).await?;
        self.status = ConnectionStatus::Disconnected;
        info!("WebSocket connection closed");
        Ok(())
    }

    fn status(&self) -> ConnectionStatus {
        self.status.clone()
    }

    fn remote_addr(&self) -> Option<&str> {
        Some(&self.remote_addr)
    }
}

/// WebSocket message stream for async iteration
pub struct WebSocketMessageStream {
    connection: WebSocketConnection,
}

impl MessageStream for WebSocketMessageStream {
    async fn next(&mut self) -> Option<Result<crate::ports::Message>> {
        match self.connection.receive().await {
            Ok(msg) => Some(Ok(msg)),
            Err(e) if e.is_disconnected() => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_websocket_transport() {
        let config = WebSocketConfig {
            host: "127.0.0.1".to_string(),
            port: 0, // Let OS assign port
            ..Default::default()
        };

        // Start server
        let listener = WebSocketTransport::bind(&config).await.unwrap();
        let local_addr = listener.listener.local_addr().unwrap();
        
        let server_task = tokio::spawn(async move {
            let mut conn = listener.accept().await.unwrap();
            let msg = conn.receive().await.unwrap();
            assert!(matches!(msg, crate::ports::Message::Text(_)));
            
            conn.send(crate::ports::Message::Text("Hello back!".to_string())).await.unwrap();
        });

        // Connect client
        let client_config = WebSocketConfig {
            host: "127.0.0.1".to_string(),
            port: local_addr.port(),
            ..Default::default()
        };
        
        let mut client = WebSocketTransport::connect(&client_config).await.unwrap();
        client.send(crate::ports::Message::Text("Hello!".to_string())).await.unwrap();
        
        let response = timeout(Duration::from_secs(1), client.receive()).await.unwrap().unwrap();
        assert!(matches!(response, crate::ports::Message::Text(_)));
        
        server_task.await.unwrap();
    }
}
