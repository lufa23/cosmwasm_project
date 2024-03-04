use crate::error::ContractError;
use crate::state::{config};
use cosmwasm_std::{
    CosmosMsg, DepsMut, MessageInfo, Response, SubMsg,
};
use universe::species::Species;
pub fn imbibe_potion(
    name: String,
    species: Species,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    let swigs = state.swigs;
    if swigs == 0 {
        return Err(ContractError::OutOfSwigs {});
    }
    state.swigs = swigs - 1;
    config(deps.storage).save(&state)?;
    let cyborg_dna = b"123ABC".to_vec();
    let new_cyborg = Imbiber {
        address: info.sender.clone(),
        species: species.clone(),
        name: name.clone(),
        cyborg_dna: cyborg_dna,
    };    
    let key = info.sender.to_string();
    imbiber(deps.storage).save(key.as_bytes(), &cyborg)?;
    let msg = CosmosMsg::Wasm();
    let submsg = SubMsg::reply_on_error(msg, 1);
    Ok(Response::new().add_submessage(submsg))
}