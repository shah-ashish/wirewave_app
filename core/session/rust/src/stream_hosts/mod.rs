//! Adapters for the existing audio host interfaces into the common feature contract.

mod microphone;
mod speaker;

pub use microphone::MicStreamHost;
pub use speaker::SpeakerStreamHost;

pub(crate) use microphone::MicHostAdapter;
pub(crate) use speaker::SpeakerHostAdapter;
