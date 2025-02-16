
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateV2Args {
    pub data_state: DataState,
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
    pub external_plugin_adapters: Option<Vec<ExternalPluginAdapterInitInfo>>,
}
