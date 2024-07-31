use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::timeout};
use tokio_native_tls::TlsAcceptor;

use crate::{
    client_message::ClientMessage,
    command::{handle_command, Commands},
    connection::{upgrade_to_tls, SMTPConnection, SMTPConnectionStatus},
    mail::Mail,
    message::Message,
    server::Controllers,
    status_code::StatusCodes,
};

/// # handle_connection_with_timeout
///
/// This function is responsible for handling the connection with the client, including the TLS handshake, and the SMTP commands, also dispatching the controllers configuring a timeout for session and operation.
pub async fn handle_connection_with_timeout<B>(
    use_tls: bool,
    tls_acceptor: Option<Arc<Mutex<TlsAcceptor>>>,
    mutex_con: Arc<Mutex<SMTPConnection<B>>>,
    controllers: Controllers<B>,
    max_size: usize,
    allowed_commands: Vec<Commands>,
    max_session_duration: Duration,
    max_op_duration: Duration,
) where
    B: 'static + Default + Send + Sync + Clone,
{
    // Dispatch on_conn controller (if exists)
    if let Some(on_conn) = &controllers.on_conn {
        let on_conn = on_conn.0.clone();
        on_conn(mutex_con.clone());
    }

    let mutex_conn_for_handle_connection = mutex_con.clone();
    // Start the main loop for handling the connection with a max session duration
    match timeout(
        max_session_duration,
        handle_connection(
            use_tls,
            tls_acceptor,
            mutex_conn_for_handle_connection,
            controllers,
            max_size,
            allowed_commands,
            max_op_duration,
        ),
    )
    .await
    {
        Ok(_) => (),
        Err(_) => {
            let conn = mutex_con.lock().await;
            let _ = conn
                .write_socket(
                    &Message::builder()
                        .status(StatusCodes::ServiceClosingTransmissionChannel)
                        .message("Service closing transmission channel".to_string())
                        .build()
                        .as_bytes(true),
                )
                .await
                .map_err(|err| log::error!("{}", err));

            let _ = conn.close().await.map_err(|err| log::error!("{}", err));
        }
    }
}

/// # handle_connection
///
/// This function is responsible for handling the connection with the client, including the TLS handshake, and the SMTP commands, also dispatching the controllers.
pub async fn handle_connection<B>(
    use_tls: bool,
    tls_acceptor: Option<Arc<Mutex<TlsAcceptor>>>,
    mutex_con: Arc<Mutex<SMTPConnection<B>>>,
    controllers: Controllers<B>,
    max_size: usize,
    allowed_commands: Vec<Commands>,
    max_op_duration: Duration,
) where
    B: 'static + Default + Send + Sync + Clone,
{
    log::trace!("[📜] Handling connection with optional TLS?: {}", use_tls);
    // Send the initial message to the client
    let conn = mutex_con.lock().await;
    // Send the initial message to the client that lets the client know that the server is ready
    match conn
        .write_socket(
            &Message::builder()
                .status(StatusCodes::SMTPServiceReady)
                .message("SMTP Service Ready".to_string())
                .build()
                .as_bytes(true),
        )
        .await
    {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    // Drop the lock to the connection
    drop(conn);

    log::trace!("[🚀] Connection initialized, and start proccessing commands");
    // Start the main loop for reading from the socket

    loop {
        match timeout(
            max_op_duration,
            handle_connection_logic(
                use_tls,
                tls_acceptor.clone(),
                mutex_con.clone(),
                controllers.clone(),
                max_size,
                allowed_commands.clone(),
            ),
        )
        .await
        {
            Ok(HandleConnectionFlow::Continue) => (),
            Ok(HandleConnectionFlow::Break) => break,
            Err(_) => {
                log::trace!("[⏳] Timeout reached, closing connection");
                break;
            }
        }
    }

    // Drop the lock to the connection
    let conn = mutex_con.lock().await;

    // Dispatch on_close controller (if exists)
    controllers.on_close.as_ref().map(|on_close| {
        let on_close = on_close.0.clone();
        drop(conn);
        let _ = on_close(mutex_con.clone());
    });

    // Re-lock the connection to send the final message to the client
    let conn = mutex_con.lock().await;

    // Send the final message to the client
    log::trace!("[👋] Sending final message to client to close");
    let _ = conn
        .write_socket(
            &Message::builder()
                .status(StatusCodes::ServiceClosingTransmissionChannel)
                .message("Service closing transmission channel".to_string())
                .build()
                .as_bytes(true),
        )
        .await
        .map_err(|err| log::error!("{}", err));

    log::trace!("[🔌] Closing connection with client");
    let _ = conn.close().await.map_err(|err| log::error!("{}", err));
}

/// # HandleConnectionFlow
/// 
/// This enum represents the possible flows that can occur while handling the connection.
pub enum HandleConnectionFlow {
    /// # Continue
    /// 
    /// Continue receiving commands/data from the client.
    Continue,
    /// # Break
    /// 
    /// Stop receiving commands/data and close the connection peacefully.
    Break,
}

/// # handle_connection_logic
/// 
/// This function is responsible for handling the connection logic, including the TLS handshake, and the SMTP commands, also dispatching the controllers.
pub async fn handle_connection_logic<B>(
    use_tls: bool,
    tls_acceptor: Option<Arc<Mutex<TlsAcceptor>>>,
    mutex_con: Arc<Mutex<SMTPConnection<B>>>,
    controllers: Controllers<B>,
    max_size: usize,
    allowed_commands: Vec<Commands>,
) -> HandleConnectionFlow
where
    B: 'static + Default + Send + Sync + Clone,
{
    let mut conn = mutex_con.lock().await;
    let mut buf = [0; 2048];

    // Read from the socket
    let n = conn.read_socket(&mut buf).await.unwrap_or_else(|err| {
        log::trace!("[🕵️‍♂️💻] Error reading from socket: {}", err);
        0
    });

    // Check if the buffer is empty, if so close the connection
    if n == 0 {
        drop(conn);
        log::trace!("[🖥️🔒] Connection closed by client");
        return HandleConnectionFlow::Break;
    }

    // Check if the buffer size is greater than 2048, if so reset the buffer
    if conn.status == SMTPConnectionStatus::WaitingCommand && conn.buffer.len() + n > 2048 {
        let _ = conn
            .write_socket(
                &Message::builder()
                    .status(StatusCodes::ExceededStorageAllocation)
                    .message("Buffer size exceeded, Resetting buffer".to_string())
                    .build()
                    .as_bytes(true),
            )
            .await
            .map_err(|err| log::error!("{}", err));

        conn.buffer.clear();

        controllers.on_reset.as_ref().map(|on_reset| {
            let on_reset = on_reset.0.clone();
            drop(conn);
            let _ = on_reset(mutex_con.clone());
        });

        return HandleConnectionFlow::Continue;
    }

    if conn.status == SMTPConnectionStatus::WaitingData && conn.mail_buffer.len() + n > max_size {
        let _ = conn
            .write_socket(
                &Message::builder()
                    .status(StatusCodes::ExceededStorageAllocation)
                    .message("Buffer size exceeded, Resetting buffer".to_string())
                    .build()
                    .as_bytes(true),
            )
            .await
            .map_err(|err| log::error!("{}", err));

        conn.mail_buffer.clear();

        controllers.on_reset.as_ref().map(|on_reset| {
            let on_reset = on_reset.0.clone();
            drop(conn);
            let _ = on_reset(mutex_con.clone());
        });

        return HandleConnectionFlow::Continue;
    }

    if conn.status == SMTPConnectionStatus::WaitingData {
        conn.mail_buffer.extend_from_slice(&buf[..n]);
    } else {
        conn.buffer.extend_from_slice(&buf[..n]);
    }

    // Check if the buffer ends with \r\n.\r\n that means that the client has sent the mail data
    if conn.status == SMTPConnectionStatus::WaitingData && conn.mail_buffer.ends_with(b"\r\n.\r\n")
    {
        // Dispatch on_email controller (if exists)
        if let Some(on_email) = &controllers.on_email {
            let on_email = on_email.0.clone();
            let mail = match Mail::<Vec<u8>>::from_bytes(conn.mail_buffer.clone()) {
                Ok(mail) => mail,
                Err(err) => {
                    log::error!("{}", err);
                    return HandleConnectionFlow::Continue;
                }
            };

            conn.mail_buffer.clear();

            // Drop conn, to allow lock on_email controller
            drop(conn);
            let response = on_email(mutex_con.clone(), Box::new(mail)).await;

            let conn = mutex_con.lock().await;
            let _ = conn
                .write_socket(&response.as_bytes(true))
                .await
                .map_err(|err| {
                    log::error!("{}", err);
                });
        } else {
            let response = Message::builder()
                .status(StatusCodes::OK)
                .message("Message received".to_string())
                .build()
                .to_string(true);

            conn.write_socket(response.as_bytes()).await.unwrap();
        }

        log::trace!("[📧] Email received, Relocking connection to ensure mail_buffer to be clean");
        let mut conn = mutex_con.lock().await;
        // Set the status to WaitingCommand
        conn.status = SMTPConnectionStatus::WaitingCommand;
        conn.buffer.clear();
        conn.mail_buffer.clear();
        log::trace!("[📧] Connection status set to WaitingCommand");
        return HandleConnectionFlow::Continue;
    }

    // Check if the buffer ends with \r\n that means that the client has sent a command
    if conn.status == SMTPConnectionStatus::WaitingCommand && conn.buffer.ends_with(b"\r\n") {
        // Parse the buffer into a ClientMessage
        let mut client_message = match ClientMessage::<String>::from_bytes(conn.buffer.clone()) {
            Ok(msg) => msg,
            Err(err) => {
                match conn
                    .write_socket(
                        &Message::builder()
                            .status(StatusCodes::SyntaxError)
                            .message(err.to_string())
                            .build()
                            .as_bytes(true),
                    )
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!("{}", err);
                        return HandleConnectionFlow::Continue;
                    }
                }

                return HandleConnectionFlow::Continue;
            }
        };

        if client_message.command == Commands::QUIT {
            log::trace!("[🚪] Connection closed by client");
            return HandleConnectionFlow::Break;
        } else if client_message.command == Commands::RSET {
            log::trace!("[🔄] Connection Reset Request, cleaning buffers and waiting commands...");
            conn.buffer.clear();
            conn.mail_buffer.clear();
            conn.status = SMTPConnectionStatus::WaitingCommand;

            log::trace!("[🔄] Connection Resetted, running on_reset controller...");
            if let Some(on_reset) = &controllers.on_reset {
                let on_reset = on_reset.0.clone();
                drop(conn);
                let _ = on_reset(mutex_con.clone());
            } else {
                let _ = conn
                    .write_socket(
                        &Message::builder()
                            .status(StatusCodes::OK)
                            .message("Connection reset".to_string())
                            .build()
                            .as_bytes(true),
                    )
                    .await
                    .map_err(|err| log::error!("{}", err));
            }

            log::trace!("[🔄] Connection Resetted, buffers cleaned, and waiting commands...");
            return HandleConnectionFlow::Continue;
        }

        log::trace!("[💬] Received Message: {:?}", client_message);

        // Drop the lock to the connection
        drop(conn);
        let (mut response, status) = match handle_command(
            mutex_con.clone(),
            controllers.clone(),
            &mut client_message,
            allowed_commands.clone(),
            max_size,
        )
        .await
        {
            Ok((res, status)) => (res, status),
            Err(err) => {
                let conn = mutex_con.lock().await;
                let _ = conn
                    .write_socket(
                        &Message::builder()
                            .status(StatusCodes::TransactionFailed)
                            .message(err.to_string())
                            .build()
                            .as_bytes(true),
                    )
                    .await
                    .map_err(|err| log::error!("{}", err));

                return HandleConnectionFlow::Continue;
            }
        };

        log::trace!(
            "[💬] Response for SMTP command {:?} is: {:?}",
            client_message.command,
            response
        );

        // Lock the connection to send the response to the client
        let mut conn = mutex_con.lock().await;

        // Set the new status
        conn.status = status;

        // Get the last index of alls messages (because last message is different)
        let last_index = response.len() - 1;
        // Get the tls_acceptor to upgrade the connection to TLS (if needed)
        let tls_acceptor = tls_acceptor.clone();

        // Check if client want to start TLS and if the server supports it
        if conn.status == SMTPConnectionStatus::Closed {
            for (i, message) in response.iter_mut().enumerate() {
                let is_last = i == last_index;
                let bytes = message.as_bytes(is_last);
                conn.write_socket(&bytes).await.unwrap();
            }
            conn.buffer.clear();
            return HandleConnectionFlow::Break;
        } else if conn.status == SMTPConnectionStatus::StartTLS && use_tls && tls_acceptor.is_some()
        {
            // let know the client that we are ready to start TLS
            match conn
                .write_socket(
                    &Message::builder()
                        .status(StatusCodes::SMTPServiceReady)
                        .message("Ready to start TLS".to_string())
                        .build()
                        .as_bytes(true),
                )
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    log::error!("{}", err);
                    return HandleConnectionFlow::Break;
                }
            }

            log::trace!("[🌐🔒] Upgrading connection to TLS");
            drop(conn);
            match upgrade_to_tls(mutex_con.clone(), tls_acceptor).await {
                Ok(_) => {
                    log::trace!("[🌐🔒🟢] Connection upgraded to TLS");

                    let mut conn = mutex_con.lock().await;
                    conn.buffer.clear();
                    conn.status = SMTPConnectionStatus::WaitingCommand;

                    return HandleConnectionFlow::Continue;
                }
                Err(err) => {
                    log::error!(
                        "[🌐🔒🚫] An error ocurred while trying to upgrade to TLS {}",
                        err
                    );

                    let mut conn = mutex_con.lock().await;
                    conn.write_socket(
                        &Message::builder()
                            .status(StatusCodes::TransactionFailed)
                            .message("TLS not available".to_string())
                            .build()
                            .as_bytes(true),
                    )
                    .await
                    .unwrap();

                    conn.buffer.clear();
                    conn.status = SMTPConnectionStatus::WaitingCommand;
                }
            };
        } else if conn.status == SMTPConnectionStatus::StartTLS && !use_tls {
            log::trace!("[🌐🔒🚫] TLS not available");

            let _ = conn
                .write_socket(
                    &Message::builder()
                        .status(StatusCodes::TransactionFailed)
                        .message("TLS not available".to_string())
                        .build()
                        .as_bytes(true),
                )
                .await
                .map_err(|err| log::error!("{}", err));

            conn.buffer.clear();
            conn.status = SMTPConnectionStatus::WaitingCommand;
        } else {
            for (i, message) in response.iter_mut().enumerate() {
                let is_last = i == last_index;
                let bytes = message.as_bytes(is_last);
                conn.write_socket(&bytes).await.unwrap();
            }
            conn.buffer.clear();
        }
    }

    HandleConnectionFlow::Continue
}
