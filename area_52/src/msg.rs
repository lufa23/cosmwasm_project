use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient, Traveler};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}