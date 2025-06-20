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
pub enum BtDocumentInfo {
    #[serde(rename="document")]
    BtDocumentInfo {
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
        #[serde(rename = "anonymousAccessAllowed", skip_serializing_if = "Option::is_none")]
        anonymous_access_allowed: Option<bool>,
        #[serde(rename = "anonymousAllowsExport", skip_serializing_if = "Option::is_none")]
        anonymous_allows_export: Option<bool>,
        #[serde(rename = "canUnshare", skip_serializing_if = "Option::is_none")]
        can_unshare: Option<bool>,
        #[serde(rename = "createdWithEducationPlan", skip_serializing_if = "Option::is_none")]
        created_with_education_plan: Option<bool>,
        #[serde(rename = "defaultElementId", skip_serializing_if = "Option::is_none")]
        default_element_id: Option<String>,
        #[serde(rename = "defaultVersionGraphMode", skip_serializing_if = "Option::is_none")]
        default_version_graph_mode: Option<models::BtVersionGraphMode>,
        #[serde(rename = "defaultVersionGraphShowAutoVersions", skip_serializing_if = "Option::is_none")]
        default_version_graph_show_auto_versions: Option<bool>,
        #[serde(rename = "defaultVersionGraphShowMerges", skip_serializing_if = "Option::is_none")]
        default_version_graph_show_merges: Option<bool>,
        #[serde(rename = "defaultWorkspace", skip_serializing_if = "Option::is_none")]
        default_workspace: Option<Box<models::BtWorkspaceInfo>>,
        #[serde(rename = "documentLabels", skip_serializing_if = "Option::is_none")]
        document_labels: Option<Vec<models::BtDocumentLabelInfo>>,
        #[serde(rename = "documentType", skip_serializing_if = "Option::is_none")]
        document_type: Option<i32>,
        #[serde(rename = "forceExportRules", skip_serializing_if = "Option::is_none")]
        force_export_rules: Option<bool>,
        #[serde(rename = "hasReleaseRevisionableObjects", skip_serializing_if = "Option::is_none")]
        has_release_revisionable_objects: Option<bool>,
        #[serde(rename = "hasRelevantInsertables", skip_serializing_if = "Option::is_none")]
        has_relevant_insertables: Option<bool>,
        #[serde(rename = "isOrphaned", skip_serializing_if = "Option::is_none")]
        is_orphaned: Option<bool>,
        #[serde(rename = "isUsingManagedWorkflow", skip_serializing_if = "Option::is_none")]
        is_using_managed_workflow: Option<bool>,
        #[serde(rename = "likedByCurrentUser", skip_serializing_if = "Option::is_none")]
        liked_by_current_user: Option<bool>,
        #[serde(rename = "likes", skip_serializing_if = "Option::is_none")]
        likes: Option<i64>,
        #[serde(rename = "notRevisionManaged", skip_serializing_if = "Option::is_none")]
        not_revision_managed: Option<bool>,
        #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(rename = "numberOfTimesCopied", skip_serializing_if = "Option::is_none")]
        number_of_times_copied: Option<i64>,
        #[serde(rename = "numberOfTimesReferenced", skip_serializing_if = "Option::is_none")]
        number_of_times_referenced: Option<i64>,
        #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
        permission: Option<models::BtOldPermission>,
        #[serde(rename = "permissionSet", skip_serializing_if = "Option::is_none")]
        permission_set: Option<Vec<String>>,
        #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
        public: Option<bool>,
        #[serde(rename = "publishedVersionId", skip_serializing_if = "Option::is_none")]
        published_version_id: Option<String>,
        #[serde(rename = "recentVersion", skip_serializing_if = "Option::is_none")]
        recent_version: Option<Box<models::BtBaseInfo>>,
        #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
        sequence: Option<String>,
        #[serde(rename = "supportTeamUserAndShared", skip_serializing_if = "Option::is_none")]
        support_team_user_and_shared: Option<bool>,
        #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
        thumbnail: Option<Box<models::BtThumbnailInfo>>,
        #[serde(rename = "totalWorkspacesScheduledForUpdate", skip_serializing_if = "Option::is_none")]
        total_workspaces_scheduled_for_update: Option<i32>,
        #[serde(rename = "totalWorkspacesUpdating", skip_serializing_if = "Option::is_none")]
        total_workspaces_updating: Option<i32>,
        #[serde(rename = "trash", skip_serializing_if = "Option::is_none")]
        trash: Option<bool>,
        #[serde(rename = "trashedAt", skip_serializing_if = "Option::is_none")]
        trashed_at: Option<String>,
        #[serde(rename = "userAccountLimitsBreached", skip_serializing_if = "Option::is_none")]
        user_account_limits_breached: Option<bool>,
        #[serde(rename = "documentThumbnailElementId", skip_serializing_if = "Option::is_none")]
        document_thumbnail_element_id: Option<String>,
        #[serde(rename = "duplicateNameViolationError", skip_serializing_if = "Option::is_none")]
        duplicate_name_violation_error: Option<String>,
        #[serde(rename = "isUpgradedToLatestVersion", skip_serializing_if = "Option::is_none")]
        is_upgraded_to_latest_version: Option<bool>,
        #[serde(rename = "tracingEnabled", skip_serializing_if = "Option::is_none")]
        tracing_enabled: Option<bool>,
    },
    #[serde(rename="document-processing")]
    BtDocumentProcessingInfo {
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
        #[serde(rename = "anonymousAccessAllowed", skip_serializing_if = "Option::is_none")]
        anonymous_access_allowed: Option<bool>,
        #[serde(rename = "anonymousAllowsExport", skip_serializing_if = "Option::is_none")]
        anonymous_allows_export: Option<bool>,
        #[serde(rename = "canUnshare", skip_serializing_if = "Option::is_none")]
        can_unshare: Option<bool>,
        #[serde(rename = "createdWithEducationPlan", skip_serializing_if = "Option::is_none")]
        created_with_education_plan: Option<bool>,
        #[serde(rename = "defaultElementId", skip_serializing_if = "Option::is_none")]
        default_element_id: Option<String>,
        #[serde(rename = "defaultVersionGraphMode", skip_serializing_if = "Option::is_none")]
        default_version_graph_mode: Option<models::BtVersionGraphMode>,
        #[serde(rename = "defaultVersionGraphShowAutoVersions", skip_serializing_if = "Option::is_none")]
        default_version_graph_show_auto_versions: Option<bool>,
        #[serde(rename = "defaultVersionGraphShowMerges", skip_serializing_if = "Option::is_none")]
        default_version_graph_show_merges: Option<bool>,
        #[serde(rename = "defaultWorkspace", skip_serializing_if = "Option::is_none")]
        default_workspace: Option<Box<models::BtWorkspaceInfo>>,
        #[serde(rename = "documentLabels", skip_serializing_if = "Option::is_none")]
        document_labels: Option<Vec<models::BtDocumentLabelInfo>>,
        #[serde(rename = "documentType", skip_serializing_if = "Option::is_none")]
        document_type: Option<i32>,
        #[serde(rename = "forceExportRules", skip_serializing_if = "Option::is_none")]
        force_export_rules: Option<bool>,
        #[serde(rename = "hasReleaseRevisionableObjects", skip_serializing_if = "Option::is_none")]
        has_release_revisionable_objects: Option<bool>,
        #[serde(rename = "hasRelevantInsertables", skip_serializing_if = "Option::is_none")]
        has_relevant_insertables: Option<bool>,
        #[serde(rename = "isOrphaned", skip_serializing_if = "Option::is_none")]
        is_orphaned: Option<bool>,
        #[serde(rename = "isUsingManagedWorkflow", skip_serializing_if = "Option::is_none")]
        is_using_managed_workflow: Option<bool>,
        #[serde(rename = "likedByCurrentUser", skip_serializing_if = "Option::is_none")]
        liked_by_current_user: Option<bool>,
        #[serde(rename = "likes", skip_serializing_if = "Option::is_none")]
        likes: Option<i64>,
        #[serde(rename = "notRevisionManaged", skip_serializing_if = "Option::is_none")]
        not_revision_managed: Option<bool>,
        #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(rename = "numberOfTimesCopied", skip_serializing_if = "Option::is_none")]
        number_of_times_copied: Option<i64>,
        #[serde(rename = "numberOfTimesReferenced", skip_serializing_if = "Option::is_none")]
        number_of_times_referenced: Option<i64>,
        #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
        permission: Option<models::BtOldPermission>,
        #[serde(rename = "permissionSet", skip_serializing_if = "Option::is_none")]
        permission_set: Option<Vec<String>>,
        #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
        public: Option<bool>,
        #[serde(rename = "publishedVersionId", skip_serializing_if = "Option::is_none")]
        published_version_id: Option<String>,
        #[serde(rename = "recentVersion", skip_serializing_if = "Option::is_none")]
        recent_version: Option<Box<models::BtBaseInfo>>,
        #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
        sequence: Option<String>,
        #[serde(rename = "supportTeamUserAndShared", skip_serializing_if = "Option::is_none")]
        support_team_user_and_shared: Option<bool>,
        #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<String>>,
        #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
        thumbnail: Option<Box<models::BtThumbnailInfo>>,
        #[serde(rename = "totalWorkspacesScheduledForUpdate", skip_serializing_if = "Option::is_none")]
        total_workspaces_scheduled_for_update: Option<i32>,
        #[serde(rename = "totalWorkspacesUpdating", skip_serializing_if = "Option::is_none")]
        total_workspaces_updating: Option<i32>,
        #[serde(rename = "trash", skip_serializing_if = "Option::is_none")]
        trash: Option<bool>,
        #[serde(rename = "trashedAt", skip_serializing_if = "Option::is_none")]
        trashed_at: Option<String>,
        #[serde(rename = "userAccountLimitsBreached", skip_serializing_if = "Option::is_none")]
        user_account_limits_breached: Option<bool>,
        #[serde(rename = "documentThumbnailElementId", skip_serializing_if = "Option::is_none")]
        document_thumbnail_element_id: Option<String>,
        #[serde(rename = "duplicateNameViolationError", skip_serializing_if = "Option::is_none")]
        duplicate_name_violation_error: Option<String>,
        #[serde(rename = "isUpgradedToLatestVersion", skip_serializing_if = "Option::is_none")]
        is_upgraded_to_latest_version: Option<bool>,
        #[serde(rename = "tracingEnabled", skip_serializing_if = "Option::is_none")]
        tracing_enabled: Option<bool>,
    },
}

impl Default for BtDocumentInfo {
    fn default() -> Self {
        Self::BtDocumentInfo {
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
            project_id: Default::default(),
            resource_type: Default::default(),
            tree_href: Default::default(),
            unparent_href: Default::default(),
            view_ref: Default::default(),
            anonymous_access_allowed: Default::default(),
            anonymous_allows_export: Default::default(),
            can_unshare: Default::default(),
            created_with_education_plan: Default::default(),
            default_element_id: Default::default(),
            default_version_graph_mode: Default::default(),
            default_version_graph_show_auto_versions: Default::default(),
            default_version_graph_show_merges: Default::default(),
            default_workspace: Default::default(),
            document_labels: Default::default(),
            document_type: Default::default(),
            force_export_rules: Default::default(),
            has_release_revisionable_objects: Default::default(),
            has_relevant_insertables: Default::default(),
            is_orphaned: Default::default(),
            is_using_managed_workflow: Default::default(),
            liked_by_current_user: Default::default(),
            likes: Default::default(),
            not_revision_managed: Default::default(),
            notes: Default::default(),
            number_of_times_copied: Default::default(),
            number_of_times_referenced: Default::default(),
            parent_id: Default::default(),
            permission: Default::default(),
            permission_set: Default::default(),
            public: Default::default(),
            published_version_id: Default::default(),
            recent_version: Default::default(),
            sequence: Default::default(),
            support_team_user_and_shared: Default::default(),
            tags: Default::default(),
            thumbnail: Default::default(),
            total_workspaces_scheduled_for_update: Default::default(),
            total_workspaces_updating: Default::default(),
            trash: Default::default(),
            trashed_at: Default::default(),
            user_account_limits_breached: Default::default(),
            document_thumbnail_element_id: Default::default(),
            duplicate_name_violation_error: Default::default(),
            is_upgraded_to_latest_version: Default::default(),
            tracing_enabled: Default::default(),
        }
        
    }
}


