# BtmVariableStudioReference2764

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_type** | Option<**String**> | Type of JSON object. | [optional]
**feature_id** | Option<**String**> | Unique ID of the feature instance within this Part Studio. | [optional]
**feature_type** | Option<**String**> | The name of the feature spec that this feature instantiates. | [optional]
**import_microversion** | Option<**String**> | Element microversion that is being imported. | [optional]
**mate_connector_feature** | Option<**bool**> |  | [optional]
**name** | Option<**String**> | User-visible name of the feature. | [optional]
**namespace** | Option<**String**> | Indicates where the feature definition lives. Features in the FeatureScript standard library have a namespace value of `\"\"`. Custom features identify the Feature Studio that contains the definition. | [optional]
**node_id** | Option<**String**> | ID for the feature node. | [optional]
**parameters** | Option<[**Vec<models::BtmParameter1>**](BTMParameter-1.md)> | A list of parameter values for instantiation of the feature spec. Parameters are present for all defined parameters, even if not used in a specific instantiation. | [optional]
**return_after_subfeatures** | Option<**bool**> | For internal use only. Should always be `false`. | [optional]
**sub_features** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> | List of subfeatures belonging to the feature. | [optional]
**suppressed** | Option<**bool**> | If `true`, the feature is suppressed. It will skip regeneration, denoted by a line through the name in the Feature list. | [optional]
**suppression_configured** | Option<**bool**> | `true` if the suppression is configured in the Part Studio. | [optional]
**variable_studio_reference** | Option<**bool**> | If `true`, the feature references a Variable Studio. | [optional]
**api_configuration** | Option<[**models::BtApiConfiguration**](BTApiConfiguration.md)> |  | [optional]
**configuration** | Option<[**Vec<models::BtmParameter1>**](BTMParameter-1.md)> |  | [optional]
**document_id** | Option<**String**> |  | [optional]
**element_id** | Option<**String**> |  | [optional]
**entire_variable_studio** | Option<**bool**> |  | [optional]
**is_automatic** | Option<**bool**> |  | [optional]
**microversion_id** | Option<[**models::BtMicroversionId366**](BTMicroversionId-366.md)> |  | [optional]
**partial_reference** | Option<**bool**> |  | [optional]
**reference_id** | Option<**String**> |  | [optional]
**reference_namespace** | Option<**String**> |  | [optional]
**reference_parameter** | Option<[**models::BtmParameterReferenceWithConfiguration3028**](BTMParameterReferenceWithConfiguration-3028.md)> |  | [optional]
**unset_automatic_edit** | Option<[**models::BtTreeEdit13**](BTTreeEdit-13.md)> |  | [optional]
**valid_revision_reference** | Option<**bool**> |  | [optional]
**variable_names** | Option<**Vec<String>**> |  | [optional]
**version_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


