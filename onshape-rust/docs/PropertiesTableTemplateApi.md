# \PropertiesTableTemplateApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_table_template**](PropertiesTableTemplateApi.md#create_table_template) | **POST** /tabletemplates | Create a new properties table template.
[**delete_table_template**](PropertiesTableTemplateApi.md#delete_table_template) | **DELETE** /tabletemplates/{tid} | Delete a properties table template.
[**get_by_company_id**](PropertiesTableTemplateApi.md#get_by_company_id) | **GET** /tabletemplates/companies/{cid} | Get all properties table templates available for a company.
[**get_by_document_id**](PropertiesTableTemplateApi.md#get_by_document_id) | **GET** /tabletemplates/d/{did} | Get all table templates that are available to use on the provided document.
[**get_table_template**](PropertiesTableTemplateApi.md#get_table_template) | **GET** /tabletemplates/{tid} | Get a properties table template by template ID.



## create_table_template

> models::BtPropertiesTableTemplateInfo create_table_template(bt_properties_table_template_params, template_group_id)
Create a new properties table template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_properties_table_template_params** | [**BtPropertiesTableTemplateParams**](BtPropertiesTableTemplateParams.md) |  | [required] |
**template_group_id** | Option<**String**> |  |  |

### Return type

[**models::BtPropertiesTableTemplateInfo**](BTPropertiesTableTemplateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_table_template

> serde_json::Value delete_table_template(tid)
Delete a properties table template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** | The id of the template in which to perform the operation. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_company_id

> Vec<models::BtPropertiesTableTemplateInfo> get_by_company_id(cid, template_type, only_active, include_defaults)
Get all properties table templates available for a company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | The id of the company in which to perform the operation. | [required] |
**template_type** | Option<[**BtPropertiesTableTemplateType**](.md)> | Indicates filter for table templates: 0 (BOM) or 1 (Revision Table). |  |
**only_active** | Option<**bool**> |  |  |[default to false]
**include_defaults** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::BtPropertiesTableTemplateInfo>**](BTPropertiesTableTemplateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_document_id

> Vec<models::BtPropertiesTableTemplateInfo> get_by_document_id(did, template_type, only_active, include_defaults)
Get all table templates that are available to use on the provided document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | The id of the document in which to perform the operation. | [required] |
**template_type** | Option<[**BtPropertiesTableTemplateType**](.md)> | Indicates filter for table templates: 0 (BOM) or 1 (Revision Table). |  |
**only_active** | Option<**bool**> |  |  |[default to true]
**include_defaults** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::BtPropertiesTableTemplateInfo>**](BTPropertiesTableTemplateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table_template

> models::BtPropertiesTableTemplateInfo get_table_template(tid)
Get a properties table template by template ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** | The id of the template in which to perform the operation. | [required] |

### Return type

[**models::BtPropertiesTableTemplateInfo**](BTPropertiesTableTemplateInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

