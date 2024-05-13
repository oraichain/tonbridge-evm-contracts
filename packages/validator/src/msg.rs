use cosmwasm_schema::{cw_serde, QueryResponses};
use tonbridge_parser::types::VdataHex;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ParseCandidatesRootBlock {
        boc: String,
    },
    InitValidators {},
    SetValidatorSet {},
    VerifyValidators {
        root_hash: String,
        file_hash: String,
        vdata: [VdataHex; 5],
    },
    AddCurrentBlockToVerifiedSet {
        root_hash: String,
    },
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(Vec<UserFriendlyValidator>)]
    GetCandidatesForValidators {},
    #[returns(Vec<UserFriendlyValidator>)]
    GetValidators {},
    #[returns(bool)]
    IsVerifiedBlock { root_hash: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<String>,
}

#[cw_serde]
pub struct UserFriendlyValidator {
    pub ctype: u8,
    pub weight: u64,
    pub adnl_addr: String,
    pub pub_key: String,
    pub node_id: String,
}
