# BtAppElementBulkModifyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_id** | Option<**String**> | The latest change id for the element, after the edit was committed. Deprecated in favor of elementChangeResults. | [optional]
**document_microversion_id** | Option<**String**> | The latest change id for the document, after the edit was committed. | [optional]
**element_change_results** | Option<[**Vec<models::BtAppElementModifyInfo>**](BTAppElementModifyInfo.md)> | The results of editing each element affected by the edit. | [optional]
**element_id** | Option<**String**> | The id of the edited element, if a single element was edited. Deprecated in favor of elementChangeResults. | [optional]
**element_ids** | Option<**Vec<String>**> | The ids of the edited elements. Deprecated in favor of elementChangeResults. | [optional]
**error_code** | Option<**i32**> | `0: OK (healthy) | 1: INFO | 2: WARNING | 3: ERROR (dangling or view generation call failed) | 4: UNKNOWN` | [optional]
**error_description** | Option<**String**> | A human-readable value for the error that occurred, if one occurred. | [optional]
**error_value** | Option<[**models::BtAppElementErrorCode**](BTAppElementErrorCode.md)> |  | [optional]
**parent_change_id** | Option<**String**> | The latest change id for the element, before the edit was made. Deprecated in favor of elementChangeResults. | [optional]
**parent_document_microversion_id** | Option<**String**> | The latest change id for the document, before the edit was made. | [optional]
**property_edits_merged** | Option<**bool**> | Whether the properties of any edited application element were changed after the transaction was created. Deprecated in favor of elementChangeResults. | [optional]
**transaction_id** | Option<**String**> | The id of the transaction in which the edit was applied. Deprecated in favor of elementChangeResults. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


