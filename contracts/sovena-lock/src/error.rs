use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("ScheduleNotFound")]
    ScheduleNotFound {},

    #[error("NothingToClaim")]
    NothingToClaim {},

    #[error("InvalidConfig: {reason}")]
    InvalidConfig { reason: String },
}
