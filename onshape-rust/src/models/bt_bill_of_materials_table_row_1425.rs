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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtBillOfMaterialsTableRow1425 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "columnIdToCell", skip_serializing_if = "Option::is_none")]
    pub column_id_to_cell: Option<std::collections::HashMap<String, models::BtTableCell1114>>,
    #[serde(rename = "expandableState", skip_serializing_if = "Option::is_none")]
    pub expandable_state: Option<models::ExpandableState>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<models::BtTreeNode20>>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "rowMetadata", skip_serializing_if = "Option::is_none")]
    pub row_metadata: Option<Box<models::BtTableBaseRowMetadata3181>>,
    #[serde(rename = "excludeIsEditable", skip_serializing_if = "Option::is_none")]
    pub exclude_is_editable: Option<bool>,
    #[serde(rename = "exclusionStatus", skip_serializing_if = "Option::is_none")]
    pub exclusion_status: Option<models::GbtBillOfMaterialsExclusionStatus>,
    #[serde(rename = "expansionStatus", skip_serializing_if = "Option::is_none")]
    pub expansion_status: Option<models::GbtBillOfMaterialsExpansionStatus>,
    #[serde(rename = "indentLevel", skip_serializing_if = "Option::is_none")]
    pub indent_level: Option<i32>,
    #[serde(rename = "isComponentsOnly", skip_serializing_if = "Option::is_none")]
    pub is_components_only: Option<bool>,
    #[serde(rename = "isSuppressed", skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
    #[serde(rename = "metadataObjectType", skip_serializing_if = "Option::is_none")]
    pub metadata_object_type: Option<i32>,
    #[serde(rename = "metadataUpdateHref", skip_serializing_if = "Option::is_none")]
    pub metadata_update_href: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "relatedOccurrencePaths", skip_serializing_if = "Option::is_none")]
    pub related_occurrence_paths: Option<Vec<String>>,
    #[serde(rename = "suppressionStatus", skip_serializing_if = "Option::is_none")]
    pub suppression_status: Option<models::GbtBillOfMaterialsSuppressionStatus>,
    #[serde(rename = "uniqueItemId", skip_serializing_if = "Option::is_none")]
    pub unique_item_id: Option<Box<models::BtBillOfMaterialsUniqueItemId2029>>,
}

impl BtBillOfMaterialsTableRow1425 {
    pub fn new() -> BtBillOfMaterialsTableRow1425 {
        BtBillOfMaterialsTableRow1425 {
            bt_type: None,
            column_id_to_cell: None,
            expandable_state: None,
            id: None,
            meta_data: None,
            node_id: None,
            row_metadata: None,
            exclude_is_editable: None,
            exclusion_status: None,
            expansion_status: None,
            indent_level: None,
            is_components_only: None,
            is_suppressed: None,
            metadata_object_type: None,
            metadata_update_href: None,
            name: None,
            related_occurrence_paths: None,
            suppression_status: None,
            unique_item_id: None,
        }
    }
}

