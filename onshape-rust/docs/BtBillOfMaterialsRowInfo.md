# BtBillOfMaterialsRowInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**header_id_to_value** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**indent_level** | Option<**i32**> |  | [optional]
**item_source** | Option<[**models::BtBillOfMaterialsItemSourceInfo**](BTBillOfMaterialsItemSourceInfo.md)> |  | [optional]
**name** | Option<**String**> | Name of the resource. | [optional]
**related_occurrences** | Option<**Vec<String>**> | Occurrence IDs in the assembly that refer to the part described by this BOM row. | [optional]
**row_id** | Option<**String**> |  | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


