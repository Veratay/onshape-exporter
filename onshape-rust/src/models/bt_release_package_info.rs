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
pub struct BtReleasePackageInfo {
    #[serde(rename = "addAllDrawingsActive", skip_serializing_if = "Option::is_none")]
    pub add_all_drawings_active: Option<bool>,
    #[serde(rename = "changeOrderId", skip_serializing_if = "Option::is_none")]
    pub change_order_id: Option<String>,
    #[serde(rename = "columnNames", skip_serializing_if = "Option::is_none")]
    pub column_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<models::BtCommentInfo>>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<models::BtUserBasicSummaryInfo>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// URI to fetch complete information of the resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Id of the resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isObsoletion", skip_serializing_if = "Option::is_none")]
    pub is_obsoletion: Option<bool>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::BtReleasePackageItemInfo>>,
    #[serde(rename = "linkedVersionIds", skip_serializing_if = "Option::is_none")]
    pub linked_version_ids: Option<Vec<String>>,
    #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<models::BtUserBasicSummaryInfo>>,
    /// Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "originalWorkspaceId", skip_serializing_if = "Option::is_none")]
    pub original_workspace_id: Option<String>,
    #[serde(rename = "packageThumbnail", skip_serializing_if = "Option::is_none")]
    pub package_thumbnail: Option<String>,
    #[serde(rename = "parentComments", skip_serializing_if = "Option::is_none")]
    pub parent_comments: Option<Vec<models::BtReleaseCommentListInfo>>,
    #[serde(rename = "parentPackages", skip_serializing_if = "Option::is_none")]
    pub parent_packages: Option<Vec<String>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::BtWorkflowPropertyInfo>>,
    /// Indicates whether the release is still in setup state and saved as a draft.
    #[serde(rename = "retainedAsDraft", skip_serializing_if = "Option::is_none")]
    pub retained_as_draft: Option<bool>,
    #[serde(rename = "revisionRuleId", skip_serializing_if = "Option::is_none")]
    pub revision_rule_id: Option<String>,
    #[serde(rename = "rootItemsToRebuild", skip_serializing_if = "Option::is_none")]
    pub root_items_to_rebuild: Option<Vec<String>>,
    #[serde(rename = "updatedItemIds", skip_serializing_if = "Option::is_none")]
    pub updated_item_ids: Option<Vec<String>>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// URI to visualize the resource in a webclient if applicable.
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Box<models::BtWorkflowSnapshotInfo>>,
    #[serde(rename = "workflowError", skip_serializing_if = "Option::is_none")]
    pub workflow_error: Option<String>,
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<Box<models::BtPublishedWorkflowId>>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl BtReleasePackageInfo {
    pub fn new() -> BtReleasePackageInfo {
        BtReleasePackageInfo {
            add_all_drawings_active: None,
            change_order_id: None,
            column_names: None,
            comments: None,
            company_id: None,
            created_at: None,
            created_by: None,
            description: None,
            detailed: None,
            document_id: None,
            href: None,
            id: None,
            is_obsoletion: None,
            items: None,
            linked_version_ids: None,
            modified_at: None,
            modified_by: None,
            name: None,
            original_workspace_id: None,
            package_thumbnail: None,
            parent_comments: None,
            parent_packages: None,
            properties: None,
            retained_as_draft: None,
            revision_rule_id: None,
            root_items_to_rebuild: None,
            updated_item_ids: None,
            version_id: None,
            view_ref: None,
            workflow: None,
            workflow_error: None,
            workflow_id: None,
            workspace_id: None,
        }
    }
}

