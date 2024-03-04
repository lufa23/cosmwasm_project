use cosmwasm_std::{DepsMut, MessageInfo, Response};
use crate::error::ContractError;

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
}