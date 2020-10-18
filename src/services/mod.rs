pub mod legacy;
mod serve;
mod thread;
mod message;
mod bbs;

pub use serve::run;
pub use thread::get_threads_handler;
pub use message::get_messages_handler;
pub use bbs::get_bbs_list_handler;
