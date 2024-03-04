use crate::error::ContractError;
use crate::state::{config, imbiber, Imbiber};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, MessageInfo, Response, SubMsg, WasmMsg};
use universe::section31::ExecuteMsg as Section31Execute;
use universe::species::Species;
static SECTION31_CONTRACT_ADDR: &str = "wasm_secret_address_do_not_reveal_to_anyone";
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
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: SECTION31_CONTRACT_ADDR.to_string(),
        msg: to_binary(&Section31Execute::Snitch {
            address: info.sender,
            name: name,
            species: species,
        })?,
        funds: vec![],
    });
    let submsg = SubMsg::reply_on_error(msg, 1);
    Ok(Response::new().add_submessage(submsg))
}