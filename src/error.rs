use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Doublin Assets")]
    DoublingAssets {},

    #[error("Missing Data")]
    MissingData {},

    #[error("Not Supported")]
    NonSupported {},

    #[error("Strategy is not yet bonded")]
    Notbonded {},

    #[error("Already unbonding")]
    Unbonded {},

    #[error("Unbond lock time not reached")]
    BondTime {},

    #[error("Insuficient shares")]
    InsufficientBalance {},

    #[error("Wrong data")]
    DataError {},

    #[error("Wrong amount")]
    BadAmount {},
}
