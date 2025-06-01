# BtSetFeatureRollbackResponse1042

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_type** | Option<**String**> | Type of JSON object. | [optional]
**library_version** | Option<**i32**> | FeatureScript version used in the Part Studio. Do not modify. | [optional]
**microversion_id** | Option<[**models::BtMicroversionId366**](BTMicroversionId-366.md)> |  | [optional]
**microversion_skew** | Option<**bool**> | On output, `true` indicates a microversion mismatch was encountered. | [optional]
**reject_microversion_skew** | Option<**bool**> | If `true`, the call will refuse to make the addition if the current microversion for the document does not match the source microversion. If `false`, a best-effort attempt is made to re-interpret the feature addition in the context of a newer document microversion. | [optional]
**rollback_index** | Option<**i32**> |  | [optional]
**serialization_version** | Option<**String**> | Version of the structure serialization rules used to encode the output. This enables incompatibility detection during software updates. | [optional]
**source_microversion** | Option<**String**> | The state from which the result was extracted. Geometry ID interpretation is dependent on this document microversion. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


