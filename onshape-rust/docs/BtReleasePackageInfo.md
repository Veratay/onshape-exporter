# BtReleasePackageInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_all_drawings_active** | Option<**bool**> |  | [optional]
**change_order_id** | Option<**String**> |  | [optional]
**column_names** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**comments** | Option<[**Vec<models::BtCommentInfo>**](BTCommentInfo.md)> |  | [optional]
**company_id** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**created_by** | Option<[**models::BtUserBasicSummaryInfo**](BTUserBasicSummaryInfo.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**detailed** | Option<**bool**> |  | [optional]
**document_id** | Option<**String**> |  | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**is_obsoletion** | Option<**bool**> |  | [optional]
**items** | Option<[**Vec<models::BtReleasePackageItemInfo>**](BTReleasePackageItemInfo.md)> |  | [optional]
**linked_version_ids** | Option<**Vec<String>**> |  | [optional]
**modified_at** | Option<**String**> |  | [optional]
**modified_by** | Option<[**models::BtUserBasicSummaryInfo**](BTUserBasicSummaryInfo.md)> |  | [optional]
**name** | Option<**String**> | Name of the resource. | [optional]
**original_workspace_id** | Option<**String**> |  | [optional]
**package_thumbnail** | Option<**String**> |  | [optional]
**parent_comments** | Option<[**Vec<models::BtReleaseCommentListInfo>**](BTReleaseCommentListInfo.md)> |  | [optional]
**parent_packages** | Option<**Vec<String>**> |  | [optional]
**properties** | Option<[**Vec<models::BtWorkflowPropertyInfo>**](BTWorkflowPropertyInfo.md)> |  | [optional]
**retained_as_draft** | Option<**bool**> | Indicates whether the release is still in setup state and saved as a draft. | [optional]
**revision_rule_id** | Option<**String**> |  | [optional]
**root_items_to_rebuild** | Option<**Vec<String>**> |  | [optional]
**updated_item_ids** | Option<**Vec<String>**> |  | [optional]
**version_id** | Option<**String**> |  | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]
**workflow** | Option<[**models::BtWorkflowSnapshotInfo**](BTWorkflowSnapshotInfo.md)> |  | [optional]
**workflow_error** | Option<**String**> |  | [optional]
**workflow_id** | Option<[**models::BtPublishedWorkflowId**](BTPublishedWorkflowId.md)> |  | [optional]
**workspace_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


