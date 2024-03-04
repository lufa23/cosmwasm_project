use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use crate::msg::{InstantiateMsg};
use crate::state::{config, State};
use crate::error::ContractError;

static DEFAULT_NUMBER_OF_SWIGS: u8 = 3;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let state = State {
        owner: info.sender,
        dna_length: msg.dna_length,
        dna_modulus: msg.dna_modulus,
        swigs: DEFAULT_NUMBER_OF_SWIGS,
    };
    config(deps.storage).save(&state)?;

    
    Ok(Response::default())
}