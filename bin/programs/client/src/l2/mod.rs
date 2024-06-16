//! Contains the L2-specifc contstructs of the client program, such as the
//! [StatelessL2BlockExecutor]

mod executor;
pub use executor::{StatelessL2BlockExecutor, TrieDBHintWriter};

mod chain_provider;
pub use chain_provider::OracleL2ChainProvider;