use anyhow::{anyhow, Result};
use stamp_aux;
pub use stamp_core::{
    dag::Transactions,
    identity::identity::IdentityID,
};

pub fn get_built_identity_by_id(id: String) -> Result<String> {
    let identities = stamp_aux::db::load_identities_by_prefix(&id)
        .map_err(|e| anyhow!("error finding identity: {}", e))?;
    if identities.len() == 0 {
        Err(anyhow!("identity {} not found", id))?;
    }
    let built = identities[0].build_identity()
        .map_err(|e| anyhow!("error building identity: {}", e))?;
    serde_json::to_string(&built).map_err(|e| anyhow!("JSON error: {}", e))
}

pub fn list_local_identities_built(search: Option<String>) -> Result<String> {
    let identities = stamp_aux::db::list_local_identities(search.as_ref().map(|x| x.as_str()))
        .map_err(|e| anyhow!("error listing identities: {}", e))?;
    let built = identities
        .iter()
        .map(|transactions| {
            transactions
                .build_identity()
                .map_err(|e| anyhow!("error building identity: {}", e))
        })
        .collect::<Result<Vec<_>>>()?;
    serde_json::to_string(&built).map_err(|e| anyhow!("JSON error: {}", e))
}

