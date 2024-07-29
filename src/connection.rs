use std::net::SocketAddr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{io::BufStream, net::TcpStream, sync::Mutex};
use tokio_native_tls::TlsStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use trust_dns_resolver::TokioAsyncResolver;

use crate::command::Commands;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SMTPConnectionStatus {
    StartTLS,
    WaitingCommand,
    WaitingData,
    Closed,
}

/// # SMTP Connection
/// 
/// This struct represents a connection to the SMTP server with the necessary information.
#[derive(Clone)]
pub struct SMTPConnection<T> {
    // Use TLS
    pub use_tls: bool,
    // TLS Buffer
    pub tls_buff_socket: Option<Arc<Mutex<BufStream<TlsStream<TcpStream>>>>>,
    // TCP Buffer
    pub tcp_buff_socket: Option<Arc<Mutex<BufStream<TcpStream>>>>,
    // Buffer
    pub buffer: Vec<u8>,
    // Mail Buffer
    pub mail_buffer: Vec<u8>,
    // Connection Status
    pub status: SMTPConnectionStatus,
    // DNS Resolver
    pub dns_resolver: Arc<Mutex<TokioAsyncResolver>>,
    // Custom state
    pub state: Arc<Mutex<T>>,
    // Keep track of the commands that are being received
    pub tracing_commands: Vec<Commands>,
}

impl<T> SMTPConnection<T> {
    /// # New
    /// 
    /// This function creates a new SMTPConnection.
    pub async fn write_socket(&self, data: &[u8]) -> std::io::Result<()> {
        if self.use_tls {
            log::trace!("[✏️] Writing to TLS socket");
            if let Some(tls_buff_socket) = &self.tls_buff_socket {
                let mut tls_buff_socket = tls_buff_socket.lock().await;
                tls_buff_socket.write_all(data).await?;
                tls_buff_socket.flush().await?;
            }
        } else {
            log::trace!("[✏️] Writing to TCP socket");
            if let Some(tcp_buff_socket) = &self.tcp_buff_socket {
                let mut tcp_buff_socket = tcp_buff_socket.lock().await;
                tcp_buff_socket.write_all(data).await?;
                tcp_buff_socket.flush().await?;
            }
        }
        Ok(())
    }

    /// # Read Socket
    /// 
    /// This function reads from the socket.
    /// Depending on the connection, it will read from the TLS socket or the TCP socket.
    pub async fn read_socket(&self, data: &mut [u8]) -> std::io::Result<usize> {
        if self.use_tls {
            if let Some(tls_buff_socket) = &self.tls_buff_socket {
                let mut tls_buff_socket = tls_buff_socket.lock().await;
                tls_buff_socket.read(data).await
            } else {
                log::trace!("[🚫] No socket to read from");
                Ok(0)
            }
        } else {
            if let Some(tcp_buff_socket) = &self.tcp_buff_socket {
                let mut tcp_buff_socket = tcp_buff_socket.lock().await;
                tcp_buff_socket.read(data).await
            } else {
                log::trace!("[🚫] No socket to read from");
                Ok(0)
            }
        }
    }

    pub async fn get_peer_addr(&self) -> std::io::Result<SocketAddr>
    {
        if self.use_tls {
            if let Some(tls_buff_socket) = &self.tls_buff_socket {
                let tls_buff_socket = tls_buff_socket.lock().await;
                Ok(tls_buff_socket.get_ref().get_ref().get_ref().get_ref().peer_addr()?)
            } else {
                log::trace!("[🚫] No socket to read from");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "No socket to read from"))
            }
        } else {
            if let Some(tcp_buff_socket) = &self.tcp_buff_socket {
                let tcp_buff_socket = tcp_buff_socket.lock().await;
                Ok(tcp_buff_socket.get_ref().peer_addr()?)
            } else {
                log::trace!("[🚫] No socket to read from");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "No socket to read from"))
            }
        }
    }

    pub async fn get_tls_buffer(&self) -> Option<Arc<Mutex<BufStream<TlsStream<TcpStream>>>>> {
        if self.use_tls {
            self.tls_buff_socket.clone()
        } else {
            None
        }
    }

    pub async fn get_tcp_buffer(&self) -> Option<Arc<Mutex<BufStream<TcpStream>>>>
    {
        if !self.use_tls {
            self.tcp_buff_socket.clone()
        } else {
            None
        }
    }

    pub async fn close(&self) -> std::io::Result<()> {
        if self.use_tls {
            if let Some(tls_buff_socket) = &self.tls_buff_socket {
                let mut tls_buff_socket = tls_buff_socket.lock().await;
                tls_buff_socket.shutdown().await?;
            }
        } else {
            if let Some(tcp_buff_socket) = &self.tcp_buff_socket {
                let mut tcp_buff_socket = tcp_buff_socket.lock().await;
                tcp_buff_socket.shutdown().await?;
            }
        }
        Ok(())
    }
}