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
pub enum BtParametricOutputInstance2288 {
    #[serde(rename="BTClonedInstance-2505")]
    BtClonedInstance2505 {
        /// Microversion that resulted from the import.
        #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
        import_microversion: Option<String>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "assemblyInstance", skip_serializing_if = "Option::is_none")]
        assembly_instance: Option<bool>,
        #[serde(rename = "assemblyMirror", skip_serializing_if = "Option::is_none")]
        assembly_mirror: Option<bool>,
        #[serde(rename = "assemblyPattern", skip_serializing_if = "Option::is_none")]
        assembly_pattern: Option<bool>,
        #[serde(rename = "assemblyReplicate", skip_serializing_if = "Option::is_none")]
        assembly_replicate: Option<bool>,
        #[serde(rename = "clonedInstance", skip_serializing_if = "Option::is_none")]
        cloned_instance: Option<bool>,
        #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
        custom_data: Option<std::collections::HashMap<String, models::BtReferenceCustomData1551>>,
        #[serde(rename = "derivedAssemblyMirror", skip_serializing_if = "Option::is_none")]
        derived_assembly_mirror: Option<bool>,
        #[serde(rename = "instanceFolder", skip_serializing_if = "Option::is_none")]
        instance_folder: Option<bool>,
        #[serde(rename = "instanceName", skip_serializing_if = "Option::is_none")]
        instance_name: Option<String>,
        #[serde(rename = "isFlattenedPart", skip_serializing_if = "Option::is_none")]
        is_flattened_part: Option<bool>,
        #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
        locked: Option<bool>,
        #[serde(rename = "parametricInstance", skip_serializing_if = "Option::is_none")]
        parametric_instance: Option<bool>,
        #[serde(rename = "parametricOutputInstance", skip_serializing_if = "Option::is_none")]
        parametric_output_instance: Option<bool>,
        #[serde(rename = "parametricPartStudioChildInstance", skip_serializing_if = "Option::is_none")]
        parametric_part_studio_child_instance: Option<bool>,
        #[serde(rename = "parametricPartStudioInstance", skip_serializing_if = "Option::is_none")]
        parametric_part_studio_instance: Option<bool>,
        #[serde(rename = "partInstance", skip_serializing_if = "Option::is_none")]
        part_instance: Option<bool>,
        #[serde(rename = "releasable", skip_serializing_if = "Option::is_none")]
        releasable: Option<bool>,
        #[serde(rename = "revisionCustomData", skip_serializing_if = "Option::is_none")]
        revision_custom_data: Option<Box<models::BtRevisionCustomData2090>>,
        #[serde(rename = "standardContent", skip_serializing_if = "Option::is_none")]
        standard_content: Option<bool>,
        #[serde(rename = "standardContentParametersId", skip_serializing_if = "Option::is_none")]
        standard_content_parameters_id: Option<String>,
        #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
        suppressed: Option<bool>,
        #[serde(rename = "suppressedFieldIndex", skip_serializing_if = "Option::is_none")]
        suppressed_field_index: Option<i32>,
        /// `true` if the suppression is configured in the Part Studio.
        #[serde(rename = "suppressionConfigured", skip_serializing_if = "Option::is_none")]
        suppression_configured: Option<bool>,
        #[serde(rename = "suppressionState", skip_serializing_if = "Option::is_none")]
        suppression_state: Option<Box<models::BtmSuppressionState1924>>,
        #[serde(rename = "validRevisionReference", skip_serializing_if = "Option::is_none")]
        valid_revision_reference: Option<bool>,
        #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
        version: Option<i32>,
    },
    #[serde(rename="BTParametricPartStudioChildInstance-3696")]
    BtParametricPartStudioChildInstance3696 {
        /// Microversion that resulted from the import.
        #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
        import_microversion: Option<String>,
        #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
        node_id: Option<String>,
        #[serde(rename = "assemblyInstance", skip_serializing_if = "Option::is_none")]
        assembly_instance: Option<bool>,
        #[serde(rename = "assemblyMirror", skip_serializing_if = "Option::is_none")]
        assembly_mirror: Option<bool>,
        #[serde(rename = "assemblyPattern", skip_serializing_if = "Option::is_none")]
        assembly_pattern: Option<bool>,
        #[serde(rename = "assemblyReplicate", skip_serializing_if = "Option::is_none")]
        assembly_replicate: Option<bool>,
        #[serde(rename = "clonedInstance", skip_serializing_if = "Option::is_none")]
        cloned_instance: Option<bool>,
        #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
        custom_data: Option<std::collections::HashMap<String, models::BtReferenceCustomData1551>>,
        #[serde(rename = "derivedAssemblyMirror", skip_serializing_if = "Option::is_none")]
        derived_assembly_mirror: Option<bool>,
        #[serde(rename = "instanceFolder", skip_serializing_if = "Option::is_none")]
        instance_folder: Option<bool>,
        #[serde(rename = "instanceName", skip_serializing_if = "Option::is_none")]
        instance_name: Option<String>,
        #[serde(rename = "isFlattenedPart", skip_serializing_if = "Option::is_none")]
        is_flattened_part: Option<bool>,
        #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
        locked: Option<bool>,
        #[serde(rename = "parametricInstance", skip_serializing_if = "Option::is_none")]
        parametric_instance: Option<bool>,
        #[serde(rename = "parametricOutputInstance", skip_serializing_if = "Option::is_none")]
        parametric_output_instance: Option<bool>,
        #[serde(rename = "parametricPartStudioChildInstance", skip_serializing_if = "Option::is_none")]
        parametric_part_studio_child_instance: Option<bool>,
        #[serde(rename = "parametricPartStudioInstance", skip_serializing_if = "Option::is_none")]
        parametric_part_studio_instance: Option<bool>,
        #[serde(rename = "partInstance", skip_serializing_if = "Option::is_none")]
        part_instance: Option<bool>,
        #[serde(rename = "releasable", skip_serializing_if = "Option::is_none")]
        releasable: Option<bool>,
        #[serde(rename = "revisionCustomData", skip_serializing_if = "Option::is_none")]
        revision_custom_data: Option<Box<models::BtRevisionCustomData2090>>,
        #[serde(rename = "standardContent", skip_serializing_if = "Option::is_none")]
        standard_content: Option<bool>,
        #[serde(rename = "standardContentParametersId", skip_serializing_if = "Option::is_none")]
        standard_content_parameters_id: Option<String>,
        #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
        suppressed: Option<bool>,
        #[serde(rename = "suppressedFieldIndex", skip_serializing_if = "Option::is_none")]
        suppressed_field_index: Option<i32>,
        /// `true` if the suppression is configured in the Part Studio.
        #[serde(rename = "suppressionConfigured", skip_serializing_if = "Option::is_none")]
        suppression_configured: Option<bool>,
        #[serde(rename = "suppressionState", skip_serializing_if = "Option::is_none")]
        suppression_state: Option<Box<models::BtmSuppressionState1924>>,
        #[serde(rename = "validRevisionReference", skip_serializing_if = "Option::is_none")]
        valid_revision_reference: Option<bool>,
        #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
        version: Option<i32>,
    },
}

impl Default for BtParametricOutputInstance2288 {
    fn default() -> Self {
        Self::BtClonedInstance2505 {
            import_microversion: Default::default(),
            node_id: Default::default(),
            assembly_instance: Default::default(),
            assembly_mirror: Default::default(),
            assembly_pattern: Default::default(),
            assembly_replicate: Default::default(),
            cloned_instance: Default::default(),
            custom_data: Default::default(),
            derived_assembly_mirror: Default::default(),
            instance_folder: Default::default(),
            instance_name: Default::default(),
            is_flattened_part: Default::default(),
            locked: Default::default(),
            parametric_instance: Default::default(),
            parametric_output_instance: Default::default(),
            parametric_part_studio_child_instance: Default::default(),
            parametric_part_studio_instance: Default::default(),
            part_instance: Default::default(),
            releasable: Default::default(),
            revision_custom_data: Default::default(),
            standard_content: Default::default(),
            standard_content_parameters_id: Default::default(),
            suppressed: Default::default(),
            suppressed_field_index: Default::default(),
            suppression_configured: Default::default(),
            suppression_state: Default::default(),
            valid_revision_reference: Default::default(),
            version: Default::default(),
        }
        
    }
}


