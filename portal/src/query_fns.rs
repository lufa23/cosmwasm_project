use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use crate::state::config_read;
use crate::msg::JumpRingCheckResponse;
use universe::species::{Traveler};

pub fn jumpring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
}