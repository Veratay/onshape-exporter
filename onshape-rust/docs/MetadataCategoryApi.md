# \MetadataCategoryApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_category_properties**](MetadataCategoryApi.md#get_category_properties) | **GET** /metadatacategory/categoryproperties | Get properties associated with the specified metadata categories.



## get_category_properties

> models::BtListResponseBtCategoryPropertyInfo get_category_properties(owner_id, owner_type, document_id, category_ids, object_type, strict, include_object_type_defaults, include_computed_properties, include_part_properties_table_only_properties, only_active, only_object_type_defaults)
Get properties associated with the specified metadata categories.

An object's category specifies its type: Part, Assembly, Drawing, etc. Available properties depend on the object's category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | Option<**String**> |  |  |
**owner_type** | Option<**i32**> |  |  |[default to 1]
**document_id** | Option<**String**> |  |  |
**category_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**object_type** | Option<**i32**> |  |  |
**strict** | Option<**bool**> |  |  |[default to true]
**include_object_type_defaults** | Option<**bool**> |  |  |[default to false]
**include_computed_properties** | Option<**bool**> |  |  |[default to true]
**include_part_properties_table_only_properties** | Option<**bool**> |  |  |[default to true]
**only_active** | Option<**bool**> |  |  |[default to false]
**only_object_type_defaults** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtListResponseBtCategoryPropertyInfo**](BTListResponseBTCategoryPropertyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

