use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response,
};
use crate::msg::{ExecuteMsg};
use crate::error::ContractError;
use crate::execute_fns::{
    imbibe_potion::imbibe_potion, step_through_jumpring::step_through_jumpring,
};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ImbibePotion { name, species } => imbibe_potion(name, species, deps, info),
        ExecuteMsg::StepThroughJumpRing { portal, destination, traveler } => step_through_jumpring(portal, destination, traveler, deps, info),
    }
}