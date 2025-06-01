# BtmModel141

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bt_type** | Option<**String**> | Type of JSON object. | [optional]
**import_microversion** | Option<**String**> | Microversion that resulted from the import. | [optional]
**node_id** | Option<**String**> |  | [optional]
**all_features** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> |  | [optional]
**all_features_and_other_references** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> |  | [optional]
**all_features_and_sub_features** | Option<[**Vec<models::BtmFeature134>**](BTMFeature-134.md)> |  | [optional]
**child_node_id_to_index** | Option<**std::collections::HashMap<String, i32>**> |  | [optional]
**configurable_tree_nodes** | Option<[**Vec<models::BtConfigurableTreeNode>**](BTConfigurableTreeNode.md)> |  | [optional]
**configuration_data** | Option<[**models::BtmConfigurationData1560**](BTMConfigurationData-1560.md)> |  | [optional]
**configured** | Option<**bool**> |  | [optional]
**deep_imports** | Option<[**std::collections::HashMap<String, Vec<models::BtImport>>**](Vec.md)> |  | [optional]
**default_features** | Option<[**models::BtDefaultFeatures119**](BTDefaultFeatures-119.md)> |  | [optional]
**default_units** | Option<[**models::BtmUnitsDefault160**](BTMUnitsDefault-160.md)> |  | [optional]
**feature_imports** | Option<[**std::collections::HashMap<String, Vec<models::BtImport>>**](Vec.md)> |  | [optional]
**first_rollback_index** | Option<**i32**> |  | [optional]
**import_set** | Option<[**Vec<models::BtpModuleId235>**](BTPModuleId-235.md)> |  | [optional]
**imports** | Option<[**Vec<models::BtmImport136>**](BTMImport-136.md)> |  | [optional]
**is_variable_studio** | Option<**bool**> |  | [optional]
**last_feature_before_roll_back** | Option<[**models::BtmFeature134**](BTMFeature-134.md)> |  | [optional]
**model_annotations** | Option<[**models::BtModelAnnotations3945**](BTModelAnnotations-3945.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**part_properties** | Option<[**models::BtPartProperties293**](BTPartProperties-293.md)> |  | [optional]
**path_to_cache** | Option<[**models::BtCacheDataPath191**](BTCacheDataPath-191.md)> |  | [optional]
**properties** | Option<[**models::BtModelProperties1258**](BTModelProperties-1258.md)> |  | [optional]
**rollback_bar** | Option<[**models::BtmRollback150**](BTMRollback-150.md)> |  | [optional]
**rolled_back_to_end** | Option<**bool**> |  | [optional]
**variable_studios** | Option<[**Vec<models::BtmVariableStudioReference2764>**](BTMVariableStudioReference-2764.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


