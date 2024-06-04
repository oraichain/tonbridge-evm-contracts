use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Api, HexBinary, Order, StdError, StdResult, Storage};
use tonbridge_parser::{
    block_parser::{compute_node_id, BlockParser, ValidatorSet},
    tree_of_cells_parser::EMPTY_HASH,
    types::{Bytes32, ValidatorDescription, Vdata},
};
use tonbridge_validator::shard_validator::MESSAGE_PREFIX;
use tonlib::cell::{BagOfCells, Cell, TonCellError};

use crate::{
    error::ContractError,
    state::{
        get_signature_candidate_validators, get_signature_validator_set,
        reset_signature_candidate_validators, reset_signature_validator_set, validator_set,
        SIGNATURE_CANDIDATE_VALIDATOR, SIGNED_BLOCKS,
    },
};

pub trait ISignatureValidator {
    fn add_current_block_to_verified_set(
        &self,
        storage: &dyn Storage,
        root_h: Bytes32,
        validator_set: ValidatorSet,
    ) -> StdResult<Bytes32>;

    fn verify_validators(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        root_h: Bytes32,
        file_hash: Bytes32,
        vdata: &[Vdata],
    ) -> StdResult<()>;

    fn parse_candidates_root_block(
        &mut self,
        storage: &mut dyn Storage,
        boc: &[u8],
    ) -> Result<(), ContractError>;

    fn is_signed_by_validator(
        &self,
        storage: &dyn Storage,
        node_id: Bytes32,
        root_h: Bytes32,
    ) -> bool;

    fn set_validator_set(&mut self, storage: &mut dyn Storage, api: &dyn Api)
        -> StdResult<Bytes32>;

    fn init_validators(&mut self, storage: &mut dyn Storage) -> StdResult<Bytes32>;

    fn get_validators_set_from_boc(&mut self, boc: &[u8]) -> Result<ValidatorSet, ContractError>;
}

// need to deserialize from storage and better access directly from storage
#[cw_serde]
#[derive(Default)]
pub struct SignatureValidator {
    // pub validator_set: ValidatorSet,
    total_weight: u64,
    // pub candidates_for_validator_set: ValidatorSet,
    candidates_total_weight: u64,
    pub root_hash: Bytes32,
    block_parser: BlockParser,
}

impl SignatureValidator {
    pub fn new() -> Self {
        Self::default()
    }
    fn parse_validators(
        &mut self,
        storage: &mut dyn Storage,
        validators: &mut ValidatorSet,
    ) -> StdResult<()> {
        let mut candidates_for_validator_set = get_signature_candidate_validators(storage);
        // let mut j = self.candidates_for_validator_set.len();
        let mut j = candidates_for_validator_set.len();

        for i in 0..validators.len() {
            // if the candidate is already in the list, we compare weight with the input
            if let Some(candidate) = candidates_for_validator_set.iter_mut().find(|val| {
                HexBinary::from(val.pubkey)
                    .to_hex()
                    .eq(&HexBinary::from(&validators[i].pubkey).to_string())
            }) {
                // old validator has less weight then new
                if candidate.weight < validators[i].weight {
                    self.candidates_total_weight += validators[i].weight;
                    self.candidates_total_weight -= candidate.weight;

                    std::mem::swap(candidate, &mut validators[i]);

                    candidate.node_id = compute_node_id(candidate.pubkey);
                }
            }
            // not found, we push a new default validator and update its info
            candidates_for_validator_set.push(ValidatorDescription::default());

            self.candidates_total_weight += validators[i].weight;
            candidates_for_validator_set[j] = validators[i];
            candidates_for_validator_set[j].node_id =
                compute_node_id(candidates_for_validator_set[j].pubkey);

            // increment size of validator set
            j += 1;
        }

        // store candidate validator
        for (i, candidate) in candidates_for_validator_set.iter().enumerate() {
            SIGNATURE_CANDIDATE_VALIDATOR.save(storage, i as u64, candidate)?;
        }

        Ok(())
    }
}

impl ISignatureValidator for SignatureValidator {
    fn is_signed_by_validator(
        &self,
        storage: &dyn Storage,
        node_id: Bytes32,
        root_h: Bytes32,
    ) -> bool {
        SIGNED_BLOCKS
            .load(storage, &[node_id, root_h].concat())
            .unwrap_or_default()
    }

    fn add_current_block_to_verified_set(
        &self,
        storage: &dyn Storage,
        root_h: Bytes32,
        validator_set: ValidatorSet,
    ) -> StdResult<Bytes32> {
        let mut current_weight = 0;
        for j in 0..validator_set.len() {
            if self.is_signed_by_validator(storage, validator_set[j].node_id, root_h) {
                current_weight += validator_set[j].weight;
            }
        }

        if current_weight * 3 <= self.total_weight * 2 {
            return Err(StdError::generic_err("not enough votes"));
        }
        Ok(root_h)
    }

    fn verify_validators(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        root_h: Bytes32,
        file_hash: Bytes32,
        vdata: &[Vdata],
    ) -> StdResult<()> {
        let test_root_hash = if self.root_hash == EMPTY_HASH {
            root_h
        } else {
            self.root_hash
        };

        if test_root_hash == EMPTY_HASH || file_hash == EMPTY_HASH {
            return Err(StdError::generic_err("wrong root_hash or file_hash"));
        }

        for vdata_item in vdata {
            // 1. found validator
            api.debug(&format!(
                "node id: {:?}",
                HexBinary::from(vdata_item.node_id).to_hex()
            ));
            let validators: Vec<(String, ValidatorDescription)> = validator_set()
                .idx
                .node_id
                .prefix(HexBinary::from(vdata_item.node_id).to_hex())
                .range(storage, None, None, Order::Ascending)
                .collect::<StdResult<_>>()?;

            api.debug(&format!(
                "total validator set node idx indexes: {:?}",
                validators.len()
            ));

            if validators.len() == 0 {
                api.debug(&format!("Node id not found"));
                continue;
            }

            if validators.len() > 1 {
                return Err(StdError::generic_err(
                    "Validator list has more than one identical node id",
                ));
            }
            let validator = validators[0].1;

            if self.is_signed_by_validator(storage, validator.node_id, test_root_hash) {
                continue;
            }

            // require(validator_idx != validator_set.length, "wrong node_id");
            let mut message = MESSAGE_PREFIX.to_vec();
            message.extend_from_slice(&test_root_hash);
            message.extend_from_slice(&file_hash);

            // signature = r + s
            if api.ed25519_verify(
                &message,
                &[vdata_item.r, vdata_item.s].concat(),
                &validator.pubkey,
            )? {
                // update as verified
                SIGNED_BLOCKS.save(
                    storage,
                    &[validator.node_id, test_root_hash].concat(),
                    &true,
                )?;
            } else {
                api.debug(&format!(
                    "validator with node id: {:?} failed to verify signature",
                    HexBinary::from(vdata_item.node_id).to_hex()
                ));
            }
        }

        Ok(())
    }

    fn init_validators(&mut self, storage: &mut dyn Storage) -> StdResult<Bytes32> {
        let candidates_for_validator_set = get_signature_candidate_validators(storage);
        // self.validator_set = self.candidates_for_validator_set.to_owned();
        for (i, candidate) in candidates_for_validator_set.iter().enumerate() {
            validator_set().save(storage, i as u64, candidate)?;
        }

        // reset candidate for validator set
        reset_signature_candidate_validators(storage);

        self.total_weight = self.candidates_total_weight;
        self.candidates_total_weight = 0;
        let rh = self.root_hash;
        self.root_hash = Bytes32::default();

        Ok(rh)
    }

    fn set_validator_set(
        &mut self,
        storage: &mut dyn Storage,
        api: &dyn Api,
    ) -> StdResult<Bytes32> {
        let val_set = get_signature_validator_set(storage);
        // remove old validators from the list to prevent unexpected errors
        reset_signature_validator_set(storage);
        let candidates_for_validator_set = get_signature_candidate_validators(storage);
        // if current validator_set is empty, check caller
        // else check votes
        if val_set[0].weight == 0 {
            return Err(StdError::generic_err("current validator_set is empty"));
        }

        let mut current_weight = 0;
        for validator in &val_set {
            if self.is_signed_by_validator(storage, validator.node_id, self.root_hash) {
                current_weight += validator.weight;
            }
        }

        api.debug(&format!(
            "current weight: {:?}, total weight: {:?}",
            current_weight * 3,
            self.total_weight * 2
        ));

        if current_weight * 3 <= self.total_weight * 2 {
            return Err(StdError::generic_err(&format!(
                "not enough votes. Wanted {:?}; has {:?}",
                current_weight * 3,
                self.total_weight * 2
            )));
        }

        for (i, candidate) in candidates_for_validator_set.iter().enumerate() {
            validator_set().save(storage, i as u64, candidate)?;
        }
        reset_signature_candidate_validators(storage);

        self.total_weight = self.candidates_total_weight;
        self.candidates_total_weight = 0;
        let rh = self.root_hash;
        self.root_hash = EMPTY_HASH;

        Ok(rh)
    }

    fn parse_candidates_root_block(
        &mut self,
        storage: &mut dyn Storage,
        boc: &[u8],
    ) -> Result<(), ContractError> {
        // self.candidates_for_validator_set = ValidatorSet::default();
        self.candidates_total_weight = 0;
        println!(
            "root hash in parse candidates: {:?}",
            HexBinary::from(self.root_hash).to_hex()
        );

        let validators = self.get_validators_set_from_boc(boc)?;
        self.parse_validators(storage, &mut validators.to_vec())?;

        Ok(())
    }

    fn get_validators_set_from_boc(&mut self, boc: &[u8]) -> Result<ValidatorSet, ContractError> {
        // ref index = 3 because we skip load_block_info, load_value_flow, and load_merkle_update refs (dont care)
        let ref_index = &mut 3;
        let cells = BagOfCells::parse(boc)?;
        let first_root = cells.single_root()?;
        // set root hash as the hash of the first root
        self.root_hash = first_root.hashes[0].as_slice().try_into()?;
        let mut parser = first_root.parser();

        // magic number
        parser.load_u32(32)?;
        // global id
        parser.load_i32(32)?;
        let block_extra = first_root
            .load_ref_if_exist(ref_index, Some(Cell::load_block_extra))?
            .0;
        if block_extra.is_none() {
            return Err(ContractError::TonCellError(
                TonCellError::cell_parser_error("Wrong boc for keyblock parsing"),
            ));
        }
        let block_extra = block_extra.unwrap();
        let validator_infos = block_extra.custom.config.config.get("22");
        if validator_infos.is_none() {
            return Err(ContractError::TonCellError(
                TonCellError::cell_parser_error("Validation infos not found"),
            ));
        }
        let validator_infos = validator_infos.unwrap();
        if validator_infos.is_none() {
            return Err(ContractError::TonCellError(
                TonCellError::cell_parser_error("Validation infos not found"),
            ));
        }
        let validator_infos = validator_infos.clone().unwrap().cur_validators;
        let mut validators = vec![ValidatorDescription::default(); validator_infos.list.len()];
        for (key, validator) in validator_infos.list.iter() {
            let index = usize::from_str_radix(key, 16)?;
            validators[index] = ValidatorDescription {
                c_type: 0x73,
                weight: validator.weight,
                adnl_addr: validator.adnl_addr.as_slice().try_into()?,
                pubkey: validator.public_key.as_slice().try_into()?,
                node_id: Bytes32::default(),
            };
        }
        let mut validators: Vec<ValidatorDescription> =
            vec![ValidatorDescription::default(); validator_infos.list.len()];

        for (key, validator) in validator_infos.list.iter() {
            let index = usize::from_str_radix(key, 16)?;
            validators[index] = ValidatorDescription {
                c_type: 0x73,
                weight: validator.weight,
                adnl_addr: validator.adnl_addr.as_slice().try_into()?,
                pubkey: validator.public_key.as_slice().try_into()?,
                node_id: Bytes32::default(),
            };
        }

        Ok(validators)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Api, HexBinary};

    const ED25519_MESSAGE_HEX: &str = "af82";
    const ED25519_SIGNATURE_HEX: &str = "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a";
    const ED25519_PUBLIC_KEY_HEX: &str =
        "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";

    #[test]
    fn test_signature_verify() {
        let deps = mock_dependencies();
        let message = HexBinary::from_hex(ED25519_MESSAGE_HEX).unwrap();
        let signature = HexBinary::from_hex(ED25519_SIGNATURE_HEX).unwrap();
        let public_key = HexBinary::from_hex(ED25519_PUBLIC_KEY_HEX).unwrap();
        let verfied = deps
            .api
            .ed25519_verify(&message, &signature, &public_key)
            .unwrap();

        println!("verified {}", verfied);
    }
}
