/*
 * Onshape REST API
 *
 * ## Welcome to the Onshape REST API Explorer  To use this API explorer, sign in to your [Onshape](https://cad.onshape.com) account in another tab, then click the **Try it out** button below (it toggles to a **Cancel** button when selected).  See the **[API Explorer Guide](https://onshape-public.github.io/docs/api-intro/explorer/)** for help navigating this API Explorer, including **[authentication](https://onshape-public.github.io/docs/api-intro/explorer/#authentication)**.  **Tip:** To ensure the current session isn't used when trying other authentication techniques, make sure to [remove the Onshape cookie](https://support.google.com/chrome/answer/95647#zippy=%2Cdelete-cookies-from-a-site) as per the instructions for your browser. Alternatively, you can use a private or incognito window.  ## See Also  * [Onshape API Guide](https://onshape-public.github.io/docs/): Our full suite of developer guides, to be used as an accompaniment to this API Explorer. * [Onshape Developer Portal](https://cad.onshape.com/appstore/dev-portal): The Onshape portal for managing your API keys, OAuth2 credentials, your Onshape applications, and your Onshape App Store entries. * [Authentication Guide](https://onshape-public.github.io/docs/auth/): Our guide to using API keys, request signatures, and OAuth2 in  your Onshape applications.
 *
 * The version of the OpenAPI document: 1.198.56658-8969b956ea73
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "btType")]
pub enum BtmParameterReferenceWithConfiguration3028 {
    #[serde(rename="BTMParameterReferenceAssembly-938")]
    BtmParameterReferenceAssembly938 {
        #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
        configuration: Option<Vec<models::BtmParameter1>>,
        #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
        document_id: Option<String>,
        #[serde(rename = "documentVersionId", skip_serializing_if = "Option::is_none")]
        document_version_id: Option<String>,
        #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
        element_id: Option<String>,
        #[serde(rename = "elementLibraryData", skip_serializing_if = "Option::is_none")]
        element_library_data: Option<Box<models::BtElementLibraryReferenceData3133>>,
        #[serde(rename = "featureScriptType", skip_serializing_if = "Option::is_none")]
        feature_script_type: Option<String>,
        #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
        ids: Option<Vec<String>>,
        /// Element microversion that is being imported.
        #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
        import_microversion: Option<String>,
        #[serde(rename = "microversioId", skip_serializing_if = "Option::is_none")]
        microversio_id: Option<String>,
        #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
        namespace: Option<String>,
        /// ID of the parameter's node.
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        /// Unique ID of the parameter.
        #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
        parameter_id: Option<String>,
    },
    #[serde(rename="BTMParameterReferencePartStudio-3302")]
    BtmParameterReferencePartStudio3302 {
        #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
        configuration: Option<Vec<models::BtmParameter1>>,
        #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
        document_id: Option<String>,
        #[serde(rename = "documentVersionId", skip_serializing_if = "Option::is_none")]
        document_version_id: Option<String>,
        #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
        element_id: Option<String>,
        #[serde(rename = "elementLibraryData", skip_serializing_if = "Option::is_none")]
        element_library_data: Option<Box<models::BtElementLibraryReferenceData3133>>,
        #[serde(rename = "featureScriptType", skip_serializing_if = "Option::is_none")]
        feature_script_type: Option<String>,
        #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
        ids: Option<Vec<String>>,
        /// Element microversion that is being imported.
        #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
        import_microversion: Option<String>,
        #[serde(rename = "microversioId", skip_serializing_if = "Option::is_none")]
        microversio_id: Option<String>,
        #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
        namespace: Option<String>,
        /// ID of the parameter's node.
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        /// Unique ID of the parameter.
        #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
        parameter_id: Option<String>,
    },
    #[serde(rename="BTMParameterReferenceVariableStudio-3550")]
    BtmParameterReferenceVariableStudio3550 {
        #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
        configuration: Option<Vec<models::BtmParameter1>>,
        #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
        document_id: Option<String>,
        #[serde(rename = "documentVersionId", skip_serializing_if = "Option::is_none")]
        document_version_id: Option<String>,
        #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
        element_id: Option<String>,
        #[serde(rename = "elementLibraryData", skip_serializing_if = "Option::is_none")]
        element_library_data: Option<Box<models::BtElementLibraryReferenceData3133>>,
        #[serde(rename = "featureScriptType", skip_serializing_if = "Option::is_none")]
        feature_script_type: Option<String>,
        #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
        ids: Option<Vec<String>>,
        /// Element microversion that is being imported.
        #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
        import_microversion: Option<String>,
        #[serde(rename = "microversioId", skip_serializing_if = "Option::is_none")]
        microversio_id: Option<String>,
        #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
        namespace: Option<String>,
        /// ID of the parameter's node.
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        /// Unique ID of the parameter.
        #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
        parameter_id: Option<String>,
    },
}

impl Default for BtmParameterReferenceWithConfiguration3028 {
    fn default() -> Self {
        Self::BtmParameterReferenceAssembly938 {
            configuration: Default::default(),
            document_id: Default::default(),
            document_version_id: Default::default(),
            element_id: Default::default(),
            element_library_data: Default::default(),
            feature_script_type: Default::default(),
            ids: Default::default(),
            import_microversion: Default::default(),
            microversio_id: Default::default(),
            namespace: Default::default(),
            node_id: Default::default(),
            parameter_id: Default::default(),
        }
        
    }
}


