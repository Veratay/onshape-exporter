# BtAppElementModifyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_id** | **String** | The latest change id for the element, after the edit was committed. | 
**element_id** | Option<**String**> | The id of the edited element. | [optional]
**element_ids** | Option<**Vec<String>**> | The ids of the edited elements, if multiple elements were edited. | [optional]
**error_code** | Option<**i32**> | `0: OK (healthy) | 1: INFO | 2: WARNING | 3: ERROR (dangling or view generation call failed) | 4: UNKNOWN` | [optional]
**error_description** | Option<**String**> | A human-readable value for the error that occurred, if one occurred. | [optional]
**error_value** | Option<[**models::BtAppElementErrorCode**](BTAppElementErrorCode.md)> |  | [optional]
**json_difference** | Option<[**models::BtDiffJsonResponse2725**](BTDiffJsonResponse-2725.md)> |  | [optional]
**parent_change_id** | Option<**String**> | The latest change id for the element, before the edit was made. | [optional]
**property_edits_merged** | Option<**bool**> | When committing a transaction, this field indicates if the properties of the application element were changed after the transaction was created. | [optional]
**transaction_id** | Option<**String**> | The id of the transaction in which the edit was applied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


