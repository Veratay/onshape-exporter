# BtFeatureListResponse2457

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_type** | Option<**String**> | Type of JSON object. | [optional]
**default_features** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> | List of Onshape-defined features instantiated within the Part Studio. | [optional]
**feature_states** | Option<[**std::collections::HashMap<String, models::BtFeatureState1688>**](BTFeatureState-1688.md)> | State of each feature, indicating if the feature is valid. Incorrectly defined features will still appear in the Feature list. | [optional]
**features** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> | List of user-defined features instantiated within the Part Studio. | [optional]
**imports** | Option<[**Vec<models::BtmImport136>**](BTMImport-136.md)> | Internal only. Do not modify. | [optional]
**is_complete** | Option<**bool**> | `true` if the features represent the entire part studio or `false` for a filtered subset. | [optional]
**library_version** | Option<**i32**> | FeatureScript version used in the Part Studio. Do not modify. | [optional]
**microversion_skew** | Option<**bool**> | On output, `true` indicates a microversion mismatch was encountered. | [optional]
**reject_microversion_skew** | Option<**bool**> | If `true`, the call will refuse to make the addition if the current microversion for the document does not match the source microversion. If `false`, a best-effort attempt is made to re-interpret the feature addition in the context of a newer document microversion. | [optional]
**rollback_index** | Option<**i32**> | Index of the rollback bar location. `-1` indicates the bar is at the end of the Feature List. | [optional]
**serialization_version** | Option<**String**> | Version of the structure serialization rules used to encode the output. This enables incompatibility detection during software updates. | [optional]
**source_microversion** | Option<**String**> | The document microversion from which the result was extracted. Part, face, edge, and vertex IDs are only valid for the same microversion. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


