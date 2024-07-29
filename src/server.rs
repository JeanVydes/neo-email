use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tokio::io::BufStream;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_native_tls::TlsAcceptor;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

use crate::client_message::ClientMessage;
use crate::controllers::on_auth::OnAuthController;
use crate::controllers::on_mail::OnMailCommandController;
use crate::controllers::on_rcpt::OnRCPTCommandController;
use crate::controllers::on_unknown_command::OnUnknownCommandController;
use crate::mail::Mail;

use super::command::Commands;
use super::connection::SMTPConnection;
use super::connection::SMTPConnectionStatus;
use super::controllers::on_close::OnCloseController;
use super::controllers::on_email::OnEmailController;
use super::controllers::on_reset::OnResetController;
use super::errors::SMTPError;
use super::message::Message;
use super::status_code::StatusCodes;

/// # SMTPServer
///
/// This struct is responsible for holding the SMTPServer configuration and state.
pub struct SMTPServer<B> {
    /// # use_tls
    ///
    /// This field is responsible for holding the information if the server can use TLS.
    use_tls: bool,
    /// # listener
    ///
    /// This field is responsible for holding the listener that will be used by the server.
    listener: Option<Arc<tokio::net::TcpListener>>,
    /// # workers
    ///
    /// This field is responsible for holding the number of workers that will be used in the ThreadPool.
    workers: usize,
    /// # threads_pool
    ///
    /// This field is responsible for holding the ThreadPool that will be used by the server.
    threads_pool: Option<Arc<rayon::ThreadPool>>,
    /// # tls_acceptor
    ///
    /// This field is responsible for holding the TLS Acceptor that will be used by the server.
    tls_acceptor: Option<Arc<Mutex<tokio_native_tls::TlsAcceptor>>>,
    /// # controllers
    ///
    /// This field is responsible for holding the controllers that will be used by the server.
    controllers: Controllers<B>,
    /// # max_size
    ///
    /// This field is responsible for holding the max size of the email that can be received.
    max_size: usize,

    allowed_commands: Vec<Commands>,

    max_session_duration: Duration,
    max_op_duration: Duration,
    dns_resolver: Arc<Mutex<TokioAsyncResolver>>,
}

/// # Controllers
///
/// This struct is responsible for holding the controllers that will be used by the SMTPServer.
#[derive(Debug)]
pub struct Controllers<B> {
    on_auth: Option<OnAuthController<B>>,
    on_email: Option<OnEmailController<B>>,
    on_reset: Option<OnResetController<B>>,
    on_close: Option<OnCloseController<B>>,
    on_mail_cmd: Option<OnMailCommandController<B>>,
    on_rcpt_cmd: Option<OnRCPTCommandController<B>>,
    on_unknown_cmd: Option<OnUnknownCommandController<B>>,
}

/// # Clone for Controllers
///
/// This implementation is responsible for cloning the Controllers struct.
impl<B> Clone for Controllers<B>
where
    B: Default + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Controllers {
            on_auth: self.on_auth.clone(),
            on_email: self.on_email.clone(),
            on_reset: self.on_reset.clone(),
            on_close: self.on_close.clone(),
            on_mail_cmd: self.on_mail_cmd.clone(),
            on_rcpt_cmd: self.on_rcpt_cmd.clone(),
            on_unknown_cmd: self.on_unknown_cmd.clone(),
        }
    }
}

impl<B> SMTPServer<B> {
    /// # new
    ///
    /// Create a new SMTPServer with default values.
    pub fn new() -> Self {
        let dns_resolver =
            TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        let dns_resolver = Arc::new(Mutex::new(dns_resolver));

        SMTPServer {
            use_tls: false,
            listener: None,
            workers: 1,
            threads_pool: None,
            tls_acceptor: None,
            controllers: Controllers {
                on_auth: None,
                on_email: None,
                on_reset: None,
                on_close: None,
                on_mail_cmd: None,
                on_rcpt_cmd: None,
                on_unknown_cmd: None,
            },
            max_size: 1024 * 1024 * 10, // 10MB
            allowed_commands: vec![
                Commands::HELO,
                Commands::EHLO,
                Commands::MAIL,
                Commands::RCPT,
                Commands::DATA,
                Commands::RSET,
                Commands::VRFY,
                Commands::EXPN,
                Commands::HELP,
                Commands::NOOP,
                Commands::QUIT,
                Commands::AUTH,
                Commands::STARTTLS,
            ],
            max_session_duration: Duration::from_secs(300),
            max_op_duration: Duration::from_secs(30),
            dns_resolver,
        }
    }

    /// # workers
    ///
    /// Set the number of workers to be used in the ThreadPool, 1 by default.
    pub fn workers(&mut self, workers: usize) -> &mut Self {
        log::info!("[🚧] Setting workers to {}", workers);
        self.workers = workers;
        self
    }

    /// # set_tls_acceptor
    ///
    /// Set the TLS Acceptor to be used when upgrading the connection to TLS.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    pub fn set_tls_acceptor(&mut self, acceptor: tokio_native_tls::TlsAcceptor) -> &mut Self {
        log::debug!("[📃] TLS Acceptor set");
        self.use_tls = true;
        self.tls_acceptor = Some(Arc::new(Mutex::new(acceptor)));
        self
    }

    /// # set_dns_resolver
    ///
    /// Set the DNS Resolver to be used when resolving the domain of the email.
    /// This ovverides the default DNS Resolver.
    pub fn set_dns_resolver(&mut self, resolver: TokioAsyncResolver) -> &mut Self {
        log::debug!("[📃] DNS Resolver set");
        self.dns_resolver = Arc::new(Mutex::new(resolver));
        self
    }

    /// # set_max_size
    ///
    /// Set the max size of the email that can be received.
    /// size in bytes
    pub fn set_max_size(&mut self, max_size: usize) -> &mut Self {
        log::debug!("[📃] Setting max size to {}", max_size);
        self.max_size = max_size;
        self
    }

    /// # set_allowed_commands
    ///
    /// Set the allowed commands that the server will accept.
    pub fn set_allowed_commands(&mut self, commands: Vec<Commands>) -> &mut Self {
        log::debug!("[📃] Setting allowed commands");
        self.allowed_commands = commands;
        self
    }

    /// # on_auth
    ///
    /// Set the OnAuthController to be used when an auth command is received.
    pub fn on_auth(&mut self, on_auth: OnAuthController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnAuthController");
        self.controllers.on_auth = Some(on_auth);
        self
    }

    /// # on_email
    ///
    /// Set the OnEmailController to be used when a email is received.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    pub fn on_email(&mut self, on_email: OnEmailController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnEmailController");
        self.controllers.on_email = Some(on_email);
        self
    }

    /// # on_reset
    ///
    /// Set the OnResetController to be used when a connection is reset.
    pub fn on_reset(&mut self, on_reset: OnResetController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnResetController");
        self.controllers.on_reset = Some(on_reset);
        self
    }

    /// # on_close
    ///
    /// Set the OnCloseController to be used when a connection will be closed.
    pub fn on_close(&mut self, on_close: OnCloseController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnCloseController");
        self.controllers.on_close = Some(on_close);
        self
    }

    /// # on_mail_cmd
    ///
    /// Set the OnMailCommandController to be used when a mail command is received.
    pub fn on_mail_cmd(&mut self, on_mail_cmd: OnMailCommandController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnMailCommandController");
        self.controllers.on_mail_cmd = Some(on_mail_cmd);
        self
    }

    /// # on_rcpt_cmd
    ///
    /// Set the OnRCPTCommandController to be used when a rcpt command is received.
    pub fn on_rcpt_cmd(&mut self, on_rcpt_cmd: OnRCPTCommandController<B>) -> &mut Self {
        log::debug!("[📃] Setting OnRCPTCommandController");
        self.controllers.on_rcpt_cmd = Some(on_rcpt_cmd);
        self
    }

    /// # set_max_session_duration
    ///
    /// Set the max session duration.
    pub fn set_max_session_duration(&mut self, duration: Duration) -> &mut Self {
        log::debug!("[📃] Setting max session duration to {:?}", duration);
        self.max_session_duration = duration;
        self
    }

    /// # set_max_op_duration
    ///
    /// Set the max operation duration.
    pub fn set_max_op_duration(&mut self, duration: Duration) -> &mut Self {
        log::debug!("[📃] Setting max operation duration to {:?}", duration);
        self.max_op_duration = duration;
        self
    }

    /// # bind
    ///
    /// This function is responsible for binding the SMTPServer to a specific address.
    pub async fn bind(&mut self, address: SocketAddr) -> Result<&mut Self, tokio::io::Error> {
        log::info!("[🔗 ] Binding to {}", address);
        let listener = tokio::net::TcpListener::bind(address).await?;
        self.listener = Some(Arc::new(listener));
        Ok(self)
    }

    /// # run
    ///
    /// This function is responsible for running the SMTPServer, accepting connections and handling them, binding is required before running.
    pub async fn run(&mut self)
    where
        B: 'static + Default + Send + Sync + Clone,
    {
        // Clone the listener to be used in the main loop
        let listener = match self.listener.clone() {
            Some(lstnr) => lstnr,
            None => panic!("There isn't listener"),
        };

        // Build the ThreadPool with the number of workers, 1 by default
        log::info!("[🚧] Building ThreadPool with {} workers", self.workers);
        self.threads_pool = match rayon::ThreadPoolBuilder::new()
            .num_threads(self.workers)
            .build()
        {
            Ok(pool) => Some(Arc::new(pool)),
            Err(err) => panic!("{}", err),
        };

        // Start the main loop for accepting connections
        log::info!("[🔧] Starting main loop for accepting connections");
        loop {
            // Accept a new connection
            let (socket, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(err) => {
                    log::error!(
                        "An error ocurred while trying to accept and TcpStream connection {}",
                        err
                    );
                    continue;
                }
            };

            log::trace!(
                "[🔍] Connection received from {}",
                socket.peer_addr().unwrap()
            );

            // Clone the thread pool, use_tls, tls_acceptor and controllers to be used in the tokio::spawn
            let pool = self.threads_pool.clone();
            let use_tls = self.use_tls;
            let tls_acceptor = self.tls_acceptor.clone();
            let controllers = self.controllers.clone();
            let max_size = self.max_size;
            let allowed_commands = self.allowed_commands.clone();
            let max_session_duration = self.max_session_duration;
            let max_op_duration = self.max_op_duration;
            let dns_resolver = self.dns_resolver.clone();

            // Spawn a new task to handle the connection
            tokio::spawn(async move {
                log::trace!("[🟢] Initializing TCP connection");

                // Create a new SMTPConnection and wrap it in an Arc<Mutex> to be shared safely between threads
                let conn = Arc::new(Mutex::new(SMTPConnection {
                    use_tls: false,
                    tls_buff_socket: None,
                    tcp_buff_socket: Some(Arc::new(Mutex::new(BufStream::new(socket)))),
                    buffer: Vec::new(),
                    mail_buffer: Vec::new(),
                    status: SMTPConnectionStatus::WaitingCommand,
                    dns_resolver,
                    state: Arc::new(Mutex::new(B::default())),
                    tracing_commands: Vec::new(),
                }));

                if let Some(pool) = pool {
                    pool.install(|| {
                        tokio::runtime::Runtime::new().unwrap().block_on(
                            handle_connection_with_timeout(
                                use_tls,
                                tls_acceptor,
                                conn,
                                controllers,
                                max_size,
                                allowed_commands,
                                max_session_duration,
                                max_op_duration,
                            ),
                        );
                    });
                }
            });
        }
    }
}

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
    let mutex_conn_for_handle_connection = mutex_con.clone();
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

pub enum HandleConnectionFlow {
    Continue,
    Break,
}

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

pub async fn upgrade_to_tls<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    tls_acceptor: Option<Arc<Mutex<tokio_native_tls::TlsAcceptor>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::trace!("[🌐🔒] Upgrading connection to TLS");

    let tls_acceptor = match tls_acceptor {
        Some(tls_acceptor) => tls_acceptor,
        None => return Err("TLS Acceptor not set".into()),
    };

    log::trace!("[🌐🔒] Locking connection to upgrade to TLS");
    let mut conn_locked = conn.lock().await;
    log::trace!("[🌐🔒] Connection locked");

    // Take out the TcpStream from the connection and set tcp_buff_socket to None
    let tcp_buff_socket = conn_locked
        .tcp_buff_socket
        .take()
        .ok_or("No TcpStream found")?;
    let tcp_buff_socket = Arc::try_unwrap(tcp_buff_socket).map_err(|_| "Failed to unwrap Arc")?;
    let tcp_buff_socket = tcp_buff_socket.into_inner();
    let tcp_stream = tcp_buff_socket.into_inner();

    // Acquire the TlsAcceptor and accept the TcpStream to create a TlsStream
    log::trace!("[🌐🔒] Locking TLS Acceptor");
    let tls_acceptor = tls_acceptor.lock().await.clone();
    log::trace!("[🌐🔒] TLS Acceptor locked");

    log::trace!("[🌐🔒🟢] Accepting TLS connection");

    let tls_stream = match timeout(Duration::from_secs(10), tls_acceptor.accept(tcp_stream)).await {
        Ok(Ok(tls_stream)) => {
            log::trace!("[🌐🔒🟢] TLS connection Accepted");
            tls_stream
        }
        Ok(Err(err)) => {
            log::error!("[🌐🔒🚫] Error during TLS handshake: {}", err);
            return Err(err.into());
        }
        Err(_) => {
            log::error!("[🌐🔒🚫] TLS handshake timed out");
            return Err("TLS handshake timed out".into());
        }
    };

    // Set the tls_buff_socket to the new TlsStream wrapped in BufStream
    conn_locked.tls_buff_socket = Some(Arc::new(Mutex::new(BufStream::new(tls_stream))));
    conn_locked.use_tls = true;
    conn_locked.status = SMTPConnectionStatus::WaitingCommand;

    Ok(())
}

pub async fn handle_command<B>(
    conn: Arc<Mutex<SMTPConnection<B>>>,
    controllers: Controllers<B>,
    client_message: &mut ClientMessage<String>,
    allowed_commands: Vec<Commands>,
    max_size: usize,
) -> Result<(Vec<Message>, SMTPConnectionStatus), SMTPError>
where
    B: 'static + Default + Send + Sync + Clone,
{
    log::trace!("[⚙️] Handling SMTP command: {:?}", client_message.command);

    // Check if the command is allowed
    if allowed_commands
        .iter()
        .find(|&cmd| cmd == &client_message.command)
        .is_none()
    {
        return Err(SMTPError::UnknownCommand(client_message.command.clone()));
    }

    let result = match client_message.command {
        Commands::HELO => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message(format!("Hello {}", "unknown"))
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::EHLO => {
            let mut ehlo_messages = vec![
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("Hello".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message(format!("SIZE {}", max_size))
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("8BITMIME".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("PIPELINING".to_string())
                    .build(),
                Message::builder()
                    .status(StatusCodes::OK)
                    .message("HELP".to_string())
                    .build(),
            ];

            let conn = conn.lock().await;
            if !conn.use_tls {
                ehlo_messages.push(
                    Message::builder()
                        .status(StatusCodes::OK)
                        .message("STARTTLS".to_string())
                        .build(),
                )
            }

            if controllers.on_auth.is_some() {
                ehlo_messages.push(
                    Message::builder()
                        .status(StatusCodes::OK)
                        .message(
                            "AUTH PLAIN LOGIN CRAM-MD5 DIGEST-MD5 GSSAPI NTLM XOAUTH2".to_string(),
                        )
                        .build(),
                );
            }

            drop(conn);

            (ehlo_messages, SMTPConnectionStatus::WaitingCommand)
        }
        Commands::MAIL => {
            if let Some(on_mail_cmd) = &controllers.on_mail_cmd {
                let on_mail_cmd = on_mail_cmd.0.clone();
                match on_mail_cmd(conn.clone(), client_message.data.clone()).await {
                    Ok(response) => {
                        return Ok((vec![response], SMTPConnectionStatus::WaitingCommand))
                    }
                    Err(response) => return Ok((vec![response], SMTPConnectionStatus::Closed)),
                }
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::OK)
                        .message("Ok".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            }
        }
        Commands::RCPT => {
            let last_command = conn.lock().await;
            let last_command = last_command
                .tracing_commands
                .last()
                .unwrap_or(&Commands::HELO);

            if last_command != &Commands::MAIL && last_command != &Commands::RCPT {
                (
                    vec![Message::builder()
                        .status(StatusCodes::BadSequenceOfCommands)
                        .message("Bad sequence of commands".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            } else {
                if let Some(on_rcpt_cmd) = &controllers.on_rcpt_cmd {
                    let on_rcpt_cmd = on_rcpt_cmd.0.clone();
                    match on_rcpt_cmd(conn.clone(), client_message.data.clone()).await {
                        Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                        Err(response) => (vec![response], SMTPConnectionStatus::Closed),
                    }
                } else {
                    (
                        vec![Message::builder()
                            .status(StatusCodes::OK)
                            .message("Ok".to_string())
                            .build()],
                        SMTPConnectionStatus::WaitingCommand,
                    )
                }
            }
        }
        Commands::DATA => (
            vec![Message::builder()
                .status(StatusCodes::StartMailInput)
                .message("Start mail input; end with <CRLF>.<CRLF>".to_string())
                .build()],
            SMTPConnectionStatus::WaitingData,
        ),
        Commands::RSET => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message("Hello".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::VRFY => (
            vec![Message::builder()
                .status(StatusCodes::CannotVerifyUserButWillAcceptMessageAndAttemptDelivery)
                .message(
                    "Cannot VRFY user, but will accept message and attempt delivery".to_string(),
                )
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::EXPN => (
            vec![Message::builder()
                .status(StatusCodes::CommandNotImplemented)
                .message(
                    "Cannot EXPN user, but will accept message and attempt delivery".to_string(),
                )
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::HELP => (
            vec![Message::builder()
                .status(StatusCodes::HelpMessage)
                .message("Help message".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::NOOP => (
            vec![Message::builder()
                .status(StatusCodes::OK)
                .message("NOOP Command successful".to_string())
                .build()],
            SMTPConnectionStatus::WaitingCommand,
        ),
        Commands::QUIT => (
            vec![Message::builder()
                .status(StatusCodes::ServiceClosingTransmissionChannel)
                .message("Service closing transmission channel".to_string())
                .build()],
            SMTPConnectionStatus::Closed,
        ),
        Commands::AUTH => {
            if let Some(on_auth) = &controllers.on_auth {
                let on_auth = on_auth.0.clone();
                match on_auth(conn.clone(), client_message.data.clone()).await {
                    Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                    Err(response) => return Ok((vec![response], SMTPConnectionStatus::Closed)),
                }
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::CommandNotImplemented)
                        .message("Command not recognized".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            }
        }
        Commands::STARTTLS => {
            let conn = conn.lock().await;

            if conn.use_tls {
                (
                    vec![Message::builder()
                        .status(StatusCodes::TransactionFailed)
                        .message("Already using TLS".to_string())
                        .build()],
                    SMTPConnectionStatus::WaitingCommand,
                )
            } else {
                (
                    vec![Message::builder()
                        .status(StatusCodes::SMTPServiceReady)
                        .message("Ready to start TLS".to_string())
                        .build()],
                    SMTPConnectionStatus::StartTLS,
                )
            }
        }
        _ => if let Some(on_unknown_cmd) = &controllers.on_unknown_cmd {
            let on_unknown_cmd = on_unknown_cmd.0.clone();
            match on_unknown_cmd(conn.clone(), client_message.command.clone()).await {
                Ok(response) => (vec![response], SMTPConnectionStatus::WaitingCommand),
                Err(response) => (vec![response], SMTPConnectionStatus::Closed),
            }
        } else {
            (
                vec![Message::builder()
                    .status(StatusCodes::CommandNotImplemented)
                    .message("Command not recognized".to_string())
                    .build()],
                SMTPConnectionStatus::WaitingCommand,
            )
        },
    };

    let mut guarded_conn = conn.lock().await;
    guarded_conn
        .tracing_commands
        .push(client_message.command.clone());
    drop(guarded_conn);

    Ok(result)
}
