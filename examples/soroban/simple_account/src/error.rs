use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SimpleAccError {
    IncorrectSignatureCount = 1,
    OwnerAlreadySet = 2,
}
