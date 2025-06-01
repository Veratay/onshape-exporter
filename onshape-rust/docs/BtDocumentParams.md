# BtDocumentParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Document description. | [optional]
**elements** | Option<[**Vec<models::BtDocumentElementCreationDescriptor>**](BTDocumentElementCreationDescriptor.md)> | List of element IDs to include in the document. | [optional]
**force_export_rules** | Option<**bool**> | `true` if the current user can toggle the Force Export Rule flag on a document. | [optional]
**generate_unknown_messages** | Option<**bool**> | Set to `true` for debugging. | [optional]
**is_empty_content** | Option<**bool**> | Set to `true` to generate an empty document. | [optional]
**is_public** | Option<**bool**> | Set to `true` to make the document public. | [optional]
**name** | Option<**String**> | Document name. | [optional]
**not_revision_managed** | Option<**bool**> | Set to `true` to indicate that revisions are not managed for this document. | [optional]
**owner_email** | Option<**String**> | The document owner's email address. | [optional]
**owner_id** | Option<**String**> | If `ownerType=USER`, this is the user ID. If `ownerType=COMPANY`, this is the company ID. | [optional]
**owner_type** | Option<**i32**> | The document's owner type. `USER=0` | `COMPANY=1` | `ONSHAPE=2` | [optional][default to 0]
**parent_id** | Option<**String**> | Document ID of this document's parent. | [optional]
**project_id** | Option<**String**> | ID of the project this document belongs to. | [optional]
**tags** | Option<**Vec<String>**> | Array of strings to set as tags for the document. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


