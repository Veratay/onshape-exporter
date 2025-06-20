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
#[serde(tag = "jsonType")]
pub enum BtTeamSummaryInfo {
    #[serde(rename="team")]
    BtTeamInfo {
        #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
        active: Option<bool>,
        #[serde(rename = "canMove", skip_serializing_if = "Option::is_none")]
        can_move: Option<bool>,
        #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
        created_at: Option<String>,
        #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
        created_by: Option<Box<models::BtUserBasicSummaryInfo>>,
        #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// URI to fetch complete information of the resource.
        #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        /// Id of the resource.
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "isContainer", skip_serializing_if = "Option::is_none")]
        is_container: Option<bool>,
        #[serde(rename = "isEnterpriseOwned", skip_serializing_if = "Option::is_none")]
        is_enterprise_owned: Option<bool>,
        #[serde(rename = "isMutable", skip_serializing_if = "Option::is_none")]
        is_mutable: Option<bool>,
        #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
        modified_at: Option<String>,
        #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
        modified_by: Option<Box<models::BtUserBasicSummaryInfo>>,
        /// Name of the resource.
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
        owner: Option<Box<models::BtOwnerInfo>>,
        #[serde(rename = "predefinedTeam", skip_serializing_if = "Option::is_none")]
        predefined_team: Option<i32>,
        #[serde(rename = "predefinedTeamMutable", skip_serializing_if = "Option::is_none")]
        predefined_team_mutable: Option<bool>,
        #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
        project_id: Option<String>,
        #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
        resource_type: Option<String>,
        #[serde(rename = "treeHref", skip_serializing_if = "Option::is_none")]
        tree_href: Option<String>,
        #[serde(rename = "unparentHref", skip_serializing_if = "Option::is_none")]
        unparent_href: Option<String>,
        /// URI to visualize the resource in a webclient if applicable.
        #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
        view_ref: Option<String>,
    },
}

impl Default for BtTeamSummaryInfo {
    fn default() -> Self {
        Self::BtTeamInfo {
            active: Default::default(),
            can_move: Default::default(),
            created_at: Default::default(),
            created_by: Default::default(),
            description: Default::default(),
            href: Default::default(),
            id: Default::default(),
            is_container: Default::default(),
            is_enterprise_owned: Default::default(),
            is_mutable: Default::default(),
            modified_at: Default::default(),
            modified_by: Default::default(),
            name: Default::default(),
            owner: Default::default(),
            predefined_team: Default::default(),
            predefined_team_mutable: Default::default(),
            project_id: Default::default(),
            resource_type: Default::default(),
            tree_href: Default::default(),
            unparent_href: Default::default(),
            view_ref: Default::default(),
        }
        
    }
}


