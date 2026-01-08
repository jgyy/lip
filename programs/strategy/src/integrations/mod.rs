pub mod mock_amm;
pub mod mock_lending;

pub use mock_amm::*;
pub use mock_lending::*;

/// Integration trait for protocol interactions
pub trait ProtocolIntegration {
    /// Deposit into the protocol
    fn deposit(&mut self, amount: u64) -> Result<(), Box<dyn std::error::Error>>;

    /// Withdraw from the protocol
    fn withdraw(&mut self, amount: u64) -> Result<(), Box<dyn std::error::Error>>;

    /// Calculate current yield
    fn calculate_yield(&self) -> u64;
}
