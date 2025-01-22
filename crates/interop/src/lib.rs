#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/maili/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod safety;
pub use safety::SafetyLevel;

mod message;
pub use message::{
    extract_executing_messages, EnrichedExecutingMessage, ExecutingMessage, MessageIdentifier,
    MessagePayload,
};

mod constants;
pub use constants::{CROSS_L2_INBOX_ADDRESS, MESSAGE_EXPIRY_WINDOW, SUPER_ROOT_VERSION};
