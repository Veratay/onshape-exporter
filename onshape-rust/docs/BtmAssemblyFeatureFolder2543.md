# BtmAssemblyFeatureFolder2543

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
**auxiliary_tree_feature** | Option<**bool**> |  | [optional]
**feature_folder** | Option<**bool**> |  | [optional]
**feature_list_field_index** | Option<**i32**> |  | [optional]
**field_index_for_owned_mate_connectors** | Option<**i32**> |  | [optional]
**mate_connectors** | Option<[**Vec<models::BtmMateConnector66>**](BTMMateConnector-66.md)> |  | [optional]
**occurrence_queries_from_all_configurations** | Option<[**Vec<models::BtmIndividualQueryWithOccurrenceBase904>**](BTMIndividualQueryWithOccurrenceBase-904.md)> |  | [optional]
**parametric_instance_feature** | Option<**bool**> |  | [optional]
**sub_features_not_used_in_query** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> |  | [optional]
**version** | Option<**i32**> |  | [optional]
**folder_end** | Option<**bool**> |  | [optional]
**folder_start** | Option<**bool**> |  | [optional]
**start_node_id** | Option<**String**> |  | [optional]
**start_node_id_raw** | Option<[**models::BtObjectId**](BTObjectId.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


