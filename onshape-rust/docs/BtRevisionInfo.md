# BtRevisionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approvers** | Option<[**Vec<models::BtRevisionApproverInfo>**](BTRevisionApproverInfo.md)> | The users who approved the release package that created this revision. | [optional]
**auto_obsoletion_release_id** | Option<**String**> |  | [optional]
**auto_obsoletion_release_name** | Option<**String**> |  | [optional]
**can_change_type** | Option<**bool**> | Whether the revision can change object type. Used in reuse part number flow | [optional][default to false]
**can_export** | Option<**bool**> |  | [optional]
**company_id** | Option<**String**> | The company or enterprise ID that owns the resource. | [optional]
**configuration** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**description** | Option<**String**> | The Revision Description metadata property if revision is of a drawing. | [optional]
**document_id** | Option<**String**> | The document that contains the item. | [optional]
**document_name** | Option<**String**> | The name of the document that contains the item. | [optional]
**document_state** | Option<**i32**> | The state of document containing this revision. Used in reuse part number flow | [optional]
**element_id** | Option<**String**> | The element that contains the item. | [optional]
**element_type** | Option<**i32**> | The type of item 0: Part Studio, 1: Assembly, 2: Drawing. 4: Blob | [optional]
**error_message** | Option<**String**> |  | [optional]
**flat_part_insertable_id** | Option<**String**> |  | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**insertable_id** | Option<**String**> |  | [optional]
**is_obsolete** | Option<**bool**> | Whether the revision has been obsoleted. | [optional]
**is_rereleasable** | Option<**bool**> | If true, the revision can be created again. | [optional]
**is_translatable** | Option<**bool**> |  | [optional]
**mime_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> | Name of the resource. | [optional]
**next_revision_id** | Option<**String**> | The next revision if applicable. null for the latest revision. | [optional]
**obsoletion_package_id** | Option<**String**> | The OBSOLETION release package that obsoleted this revision if applicable. | [optional]
**part_id** | Option<**String**> |  | [optional]
**part_identity** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> | The Part Number with which the item was revised. | [optional]
**previous_revision_id** | Option<**String**> | The previous revision if applicable. null for first revision. | [optional]
**release_created_date** | Option<**String**> |  | [optional]
**release_id** | Option<**String**> | The release package that created this revision. | [optional]
**release_name** | Option<**String**> | The name of the release package that created this item. | [optional]
**released_by** | Option<[**models::BtUserSummaryInfo**](BTUserSummaryInfo.md)> |  | [optional]
**revision** | Option<**String**> | The id of the revision. | [optional]
**revision_rule_id** | Option<**String**> |  | [optional]
**version_id** | Option<**String**> | The version of the document that contains this revision. | [optional]
**version_name** | Option<**String**> | The name of the version of the document that contains this revision. | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


