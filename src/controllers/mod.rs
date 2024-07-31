/// # Controllers
/// 
/// This module contains all the controllers for the SMTP server.
/// The controllers are responsible for handling the commands and data from the client in a custom way.

/// # on_auth
/// 
/// This module contains the controller for the AUTH command, usually used to authenticate the client.
pub mod on_auth;
/// # on_close
/// 
/// This module contains the controller for the QUIT command, usually used to close the connection.
pub mod on_close;
/// # on_data
/// 
/// This module contains the controller for the DATA command, usually used to send the email data.
pub mod on_email;
/// # on_mail_cmd
/// 
/// This module contains the controller for the MAIL command, usually used to set the sender of the email.
pub mod on_mail_cmd;
/// # on_rcpt
/// 
/// This module contains the controller for the RCPT command, usually used to set the recipient of the email.
pub mod on_rcpt;
/// # on_reset
/// 
/// This module contains the controller for the RSET command, usually used to reset the connection.
pub mod on_reset;
/// # on_unknown_command
/// 
/// This module contains the controller for the unknown command, usually used to handle unknown commands.
pub mod on_unknown_command;
