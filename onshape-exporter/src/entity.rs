use onshape_rust::{apis::{assembly_api::{get_assembly_definition, GetAssemblyDefinitionError}, configuration::Configuration}, models::{BtAssemblyDefinitionInfo, BtAssemblyInstanceInfo, BtRootAssemblyInfo}};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EntityID {
    pub did: String,
    pub wvm: String,
    pub wvmid: String,
    pub eid: String,
}

impl EntityID {
    pub fn new(did: String, wvm: String, wvmid: String, eid: String) -> Self {
        Self {
            did,
            wvm,
            wvmid,
            eid,
        }
    }

    pub fn from_root_assembly(root: &BtRootAssemblyInfo) -> Option<Self> {
        Some(EntityID::new(
            root.document_id.as_deref()?.to_owned(),
            if root.document_microversion.is_some() {
                "m".to_owned()
            } else {
                "v".to_owned()
            },
            root.document_microversion
                .as_deref()
                .or(root.document_version.as_deref())?
                .to_owned(),
            root.element_id.as_deref()?.to_owned(),
        ))
    }

    pub fn from_instance(instance: &BtAssemblyInstanceInfo) -> Option<Self> {
        Some(EntityID::new(
            instance.document_id.as_deref()?.to_owned(),
            if instance.document_microversion.is_some() {
                "m".to_owned()
            } else {
                "v".to_owned()
            },
            instance
                .document_microversion
                .as_deref()
                .or(instance.document_version.as_deref())?
                .to_owned(),
            instance.element_id.as_deref()?.to_owned(),
        ))
    }

    pub async fn get_assembly(
        &self,
        configuration: &Configuration,
    ) -> Result<BtAssemblyDefinitionInfo, onshape_rust::apis::Error<GetAssemblyDefinitionError>>
    {
        get_assembly_definition(
            configuration,
            &self.did,
            &self.wvm,
            &self.wvmid,
            &self.eid,
            None,
            None,
            None,
            Some(true),
            Some(false),
            Some(false),
            Some(false),
        )
        .await
    }
}
