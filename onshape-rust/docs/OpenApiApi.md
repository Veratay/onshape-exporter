# \OpenApiApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_open_api**](OpenApiApi.md#get_open_api) | **GET** /openapi | Get the OpenAPI specification for the Onshape REST API.
[**get_tags**](OpenApiApi.md#get_tags) | **GET** /openapi/tags | Get the list of tags in the Onshape OpenAPI specification.



## get_open_api

> models::OpenApi get_open_api(force_reload, version, version_alias, no_filter, included_tags, excluded_tags, include_deprecated, only_deprecated, documentation_statuses, rest_user_role, operation_ids, excluded_operation_ids)
Get the OpenAPI specification for the Onshape REST API.

The Onshape API OpenAPI specification is returned in the JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_reload** | Option<**bool**> | Force reload the OpenApi definition. Only works when asking for the latest version. |  |
**version** | Option<**String**> | Specify a version of Onshape from which the OpenAPI is generated. If '*' is specified in any of the version fields, that indicates any version if acceptable. |  |
**version_alias** | Option<[**VersionAlias**](.md)> | Version aliases based on the currently released version. |  |
**no_filter** | Option<**bool**> | Do not filter the specification at all. |  |[default to false]
**included_tags** | Option<[**Vec<String>**](String.md)> | Return only operations with tags included in includedTags. |  |
**excluded_tags** | Option<[**Vec<String>**](String.md)> | If an operation contains an excluded tag, it is not returned from this endpoint. |  |
**include_deprecated** | Option<**bool**> | Include deprecated endpoints. |  |[default to false]
**only_deprecated** | Option<**bool**> | Only include deprecated endpoints. |  |[default to false]
**documentation_statuses** | Option<[**Vec<models::Status>**](models::Status.md)> | Only return endpoints that have the specified documentation status. Default is to return all the endpoints the user should have access to. |  |
**rest_user_role** | Option<[**BtRestUserRole**](.md)> | The REST user role for which this spec is requested. |  |
**operation_ids** | Option<[**Vec<String>**](String.md)> | Only return operations with specified ids. |  |
**excluded_operation_ids** | Option<[**Vec<String>**](String.md)> | Do not return operations with specified ids. |  |

### Return type

[**models::OpenApi**](OpenAPI.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09, application/yaml;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> Vec<models::Tag> get_tags()
Get the list of tags in the Onshape OpenAPI specification.

Tags are used to group operations. For example, `Document` groups operations on documents.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Tag>**](Tag.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09, application/yaml;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

