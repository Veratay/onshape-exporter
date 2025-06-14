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

/// BtRevisionInfo : A revision of PART/ASSEMBLY etc created by release management.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtRevisionInfo {
    /// The users who approved the release package that created this revision.
    #[serde(rename = "approvers", skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<models::BtRevisionApproverInfo>>,
    #[serde(rename = "autoObsoletionReleaseId", skip_serializing_if = "Option::is_none")]
    pub auto_obsoletion_release_id: Option<String>,
    #[serde(rename = "autoObsoletionReleaseName", skip_serializing_if = "Option::is_none")]
    pub auto_obsoletion_release_name: Option<String>,
    /// Whether the revision can change object type. Used in reuse part number flow
    #[serde(rename = "canChangeType", skip_serializing_if = "Option::is_none")]
    pub can_change_type: Option<bool>,
    #[serde(rename = "canExport", skip_serializing_if = "Option::is_none")]
    pub can_export: Option<bool>,
    /// The company or enterprise ID that owns the resource.
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The Revision Description metadata property if revision is of a drawing.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The document that contains the item.
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// The name of the document that contains the item.
    #[serde(rename = "documentName", skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// The state of document containing this revision. Used in reuse part number flow
    #[serde(rename = "documentState", skip_serializing_if = "Option::is_none")]
    pub document_state: Option<i32>,
    /// The element that contains the item.
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    /// The type of item 0: Part Studio, 1: Assembly, 2: Drawing. 4: Blob
    #[serde(rename = "elementType", skip_serializing_if = "Option::is_none")]
    pub element_type: Option<i32>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "flatPartInsertableId", skip_serializing_if = "Option::is_none")]
    pub flat_part_insertable_id: Option<String>,
    /// URI to fetch complete information of the resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Id of the resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "insertableId", skip_serializing_if = "Option::is_none")]
    pub insertable_id: Option<String>,
    /// Whether the revision has been obsoleted.
    #[serde(rename = "isObsolete", skip_serializing_if = "Option::is_none")]
    pub is_obsolete: Option<bool>,
    /// If true, the revision can be created again.
    #[serde(rename = "isRereleasable", skip_serializing_if = "Option::is_none")]
    pub is_rereleasable: Option<bool>,
    #[serde(rename = "isTranslatable", skip_serializing_if = "Option::is_none")]
    pub is_translatable: Option<bool>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The next revision if applicable. null for the latest revision.
    #[serde(rename = "nextRevisionId", skip_serializing_if = "Option::is_none")]
    pub next_revision_id: Option<String>,
    /// The OBSOLETION release package that obsoleted this revision if applicable.
    #[serde(rename = "obsoletionPackageId", skip_serializing_if = "Option::is_none")]
    pub obsoletion_package_id: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "partIdentity", skip_serializing_if = "Option::is_none")]
    pub part_identity: Option<String>,
    /// The Part Number with which the item was revised.
    #[serde(rename = "partNumber", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    /// The previous revision if applicable. null for first revision.
    #[serde(rename = "previousRevisionId", skip_serializing_if = "Option::is_none")]
    pub previous_revision_id: Option<String>,
    #[serde(rename = "releaseCreatedDate", skip_serializing_if = "Option::is_none")]
    pub release_created_date: Option<String>,
    /// The release package that created this revision.
    #[serde(rename = "releaseId", skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// The name of the release package that created this item.
    #[serde(rename = "releaseName", skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[serde(rename = "releasedBy", skip_serializing_if = "Option::is_none")]
    pub released_by: Option<Box<models::BtUserSummaryInfo>>,
    /// The id of the revision.
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "revisionRuleId", skip_serializing_if = "Option::is_none")]
    pub revision_rule_id: Option<String>,
    /// The version of the document that contains this revision.
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// The name of the version of the document that contains this revision.
    #[serde(rename = "versionName", skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    /// URI to visualize the resource in a webclient if applicable.
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
}

impl BtRevisionInfo {
    /// A revision of PART/ASSEMBLY etc created by release management.
    pub fn new() -> BtRevisionInfo {
        BtRevisionInfo {
            approvers: None,
            auto_obsoletion_release_id: None,
            auto_obsoletion_release_name: None,
            can_change_type: None,
            can_export: None,
            company_id: None,
            configuration: None,
            created_at: None,
            description: None,
            document_id: None,
            document_name: None,
            document_state: None,
            element_id: None,
            element_type: None,
            error_message: None,
            flat_part_insertable_id: None,
            href: None,
            id: None,
            insertable_id: None,
            is_obsolete: None,
            is_rereleasable: None,
            is_translatable: None,
            mime_type: None,
            name: None,
            next_revision_id: None,
            obsoletion_package_id: None,
            part_id: None,
            part_identity: None,
            part_number: None,
            previous_revision_id: None,
            release_created_date: None,
            release_id: None,
            release_name: None,
            released_by: None,
            revision: None,
            revision_rule_id: None,
            version_id: None,
            version_name: None,
            view_ref: None,
        }
    }
}

