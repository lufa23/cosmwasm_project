use crate::error::ContractError;
use crate::execute_fns::check_sapience_level::check_sapience_level;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use universe::species::Traveler;

pub fn step_through_jumpring(
    portal: Addr,
    destination: Addr,
    traveler: Traveler,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // step_through_jumpring code needs to be added here
}