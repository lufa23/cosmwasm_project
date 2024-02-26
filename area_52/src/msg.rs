use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient};

#[derive(Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,

}