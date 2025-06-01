# BtPartMetadataInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**appearance** | Option<[**models::BtPartAppearanceInfo**](BTPartAppearanceInfo.md)> |  | [optional]
**body_type** | Option<**String**> |  | [optional]
**configuration_id** | Option<**String**> |  | [optional]
**custom_properties** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**default_color_hash** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**element_id** | Option<**String**> |  | [optional]
**href** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_flattened_body** | Option<**bool**> |  | [optional]
**is_hidden** | Option<**bool**> |  | [optional]
**is_mesh** | Option<**bool**> |  | [optional]
**material** | Option<[**models::BtPartMaterialInfo**](BTPartMaterialInfo.md)> |  | [optional]
**mesh_state** | Option<[**models::GbtMeshState**](GBTMeshState.md)> |  | [optional]
**metadata_microversion** | Option<**String**> |  | [optional]
**microversion_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**ordinal** | Option<**i32**> |  | [optional]
**part_id** | Option<**String**> |  | [optional]
**part_identity** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> |  | [optional]
**part_query** | Option<**String**> |  | [optional]
**product_line** | Option<**String**> |  | [optional]
**project** | Option<**String**> |  | [optional]
**property_source_types** | Option<**std::collections::HashMap<String, i32>**> | `0: AUTOMATIC` Set automatically, like a part name |  `1: MERGED` Merged from another Part Studio | `2: FEATURE` Custom feature | `3: UNCONFIGURED` | `4: CONFIGURED` |  `5: STANDARD_CONTENT` | `6: DEFAULT` Applied from metadata property configuration | `7: COMPUTED` Non-overriden, non-configured, computed property |  `8: COMPUTED_CONFIGURED` Property is computed in this configuration; may also be configured in other configurations  `9: IMPORT` Imported properties are handled separately | [optional]
**referencing_configured_part_node_ids** | Option<**Vec<String>**> |  | [optional]
**revision** | Option<**String**> |  | [optional]
**state** | Option<[**models::BtMetadataStateType**](BTMetadataStateType.md)> |  | [optional]
**thumbnail_configuration_id** | Option<**String**> |  | [optional]
**thumbnail_info** | Option<[**models::BtThumbnailInfo**](BTThumbnailInfo.md)> |  | [optional]
**title1** | Option<**String**> |  | [optional]
**title2** | Option<**String**> |  | [optional]
**title3** | Option<**String**> |  | [optional]
**unflattened_part_id** | Option<**String**> |  | [optional]
**vendor** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


