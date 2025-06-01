# BtTaskItemParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body_type** | Option<**String**> | Body type to reference from a task. | [optional]
**configuration** | Option<**String**> | Configuration of reference. Used to restrict a reference to a specific configuration of an element. | [optional]
**description** | Option<**String**> | Description of the reference. | [optional]
**document_id** | Option<**String**> | Id of a document. Required to reference a document or anything contained within it. | [optional]
**element_id** | Option<**String**> | Id of an element reference. Used when referencing an element. | [optional]
**element_type** | Option<**i32**> | Type of element reference. Options are 0 (Part Studio), 1 (Assembly), 2 (Drawing), 3 (Feature Studio), 4 (Blob), 5 (Application), 6 (Table), 7 (Bill of Materials),  8 (Variable Studio), or 9 (Publication Item). | [optional]
**mime_type** | Option<**String**> | Mimetype of reference. Used when referencing blob elements. | [optional]
**name** | Option<**String**> | Name of the reference. | [optional]
**part_id** | Option<**String**> | Determinstic Id of a part. Used when referencing parts. | [optional]
**revision_id** | Option<**String**> | Id of a revision to reference from a task. | [optional]
**version_id** | Option<**String**> | Id of a document version. Used when referencing the version itself or an element or part in it. | [optional]
**workspace_id** | Option<**String**> | Id of a document workspace. Used when referencing the workspace itself or an element or part in it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


