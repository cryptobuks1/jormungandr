mod all;
mod tx_only;

use crate::mjolnir_app::MjolnirError;
pub use all::AllFragments;
use structopt::StructOpt;
pub use tx_only::TxOnly;
#[derive(StructOpt, Debug)]
pub enum Standard {
    /// Put load on endpoint using transaction fragments only.
    TxOnly(tx_only::TxOnly),
    /// Put load on endpoint using all supported fragment types
    All(all::AllFragments),
}

impl Standard {
    pub fn exec(&self) -> Result<(), MjolnirError> {
        match self {
            Standard::TxOnly(tx_only_command) => tx_only_command.exec(),
            Standard::All(all_command) => all_command.exec(),
        }
    }
}
