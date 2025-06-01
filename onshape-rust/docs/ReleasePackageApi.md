# \ReleasePackageApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_obsoletion_package**](ReleasePackageApi.md#create_obsoletion_package) | **POST** /releasepackages/obsoletion/{wfid} | Create an obsoletion package to make an existing revision obsolete.
[**create_release_package**](ReleasePackageApi.md#create_release_package) | **POST** /releasepackages/release/{wfid} | Create a new release package for one or more items.
[**get_release_package**](ReleasePackageApi.md#get_release_package) | **GET** /releasepackages/{rpid} | Get details about the specified release package.
[**update_release_package**](ReleasePackageApi.md#update_release_package) | **POST** /releasepackages/{rpid} | Update the release/obsoletion package/item properties.



## create_obsoletion_package

> serde_json::Value create_obsoletion_package(wfid, revision_id, debug_mode)
Create an obsoletion package to make an existing revision obsolete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wfid** | **String** |  | [required] |
**revision_id** | **String** |  | [required] |
**debug_mode** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_release_package

> serde_json::Value create_release_package(wfid, bt_release_package_params, debug_mode)
Create a new release package for one or more items.

All revisionable items must be from the same document. Once a release package is successfully created, use `updateReleasePackage` to update all desired item/package properties, and transition it to the desired state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wfid** | **String** |  | [required] |
**bt_release_package_params** | [**BtReleasePackageParams**](BtReleasePackageParams.md) |  | [required] |
**debug_mode** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_release_package

> models::BtReleasePackageInfo get_release_package(rpid, detailed)
Get details about the specified release package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rpid** | **String** |  | [required] |
**detailed** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtReleasePackageInfo**](BTReleasePackageInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_release_package

> models::BtReleasePackageInfo update_release_package(rpid, bt_update_release_package_params, action, wfaction)
Update the release/obsoletion package/item properties.

Use the `wfaction` query param to also perform a workflow transition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rpid** | **String** |  | [required] |
**bt_update_release_package_params** | [**BtUpdateReleasePackageParams**](BtUpdateReleasePackageParams.md) |  | [required] |
**action** | Option<**String**> |  |  |[default to UPDATE]
**wfaction** | Option<**String**> |  |  |

### Return type

[**models::BtReleasePackageInfo**](BTReleasePackageInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

