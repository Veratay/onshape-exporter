# BtAppArrayInfoBtAppDrawingViewInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_id** | Option<**String**> |  | [optional]
**error_code** | Option<**i32**> | `0: OK (healthy) | 1: INFO | 2: WARNING | 3: ERROR (dangling or view generation call failed) | 4: UNKNOWN` | [optional]
**error_description** | Option<**String**> | A human-readable value for the error that occurred, if one occurred. | [optional]
**error_value** | Option<[**models::BtAppElementErrorCode**](BTAppElementErrorCode.md)> |  | [optional]
**items** | Option<[**Vec<models::BtAppDrawingViewInfo>**](BTAppDrawingViewInfo.md)> |  | [optional]
**parent_change_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


