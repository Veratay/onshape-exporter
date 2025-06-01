# \FeatureStudioApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feature_studio**](FeatureStudioApi.md#create_feature_studio) | **POST** /featurestudios/d/{did}/w/{wid} | Create a new Feature Studio tab in a document.
[**get_feature_studio_contents**](FeatureStudioApi.md#get_feature_studio_contents) | **GET** /featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid} | Get the text for a Feature Studio element.
[**get_feature_studio_specs**](FeatureStudioApi.md#get_feature_studio_specs) | **GET** /featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | Get the feature specs for a Feature Studio element.
[**update_feature_studio_contents**](FeatureStudioApi.md#update_feature_studio_contents) | **POST** /featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid} | Update the text for a Feature Studio element.



## create_feature_studio

> models::BtDocumentElementInfo create_feature_studio(did, wid, bt_model_element_params)
Create a new Feature Studio tab in a document.

Specify the name for the new tab in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

[**models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_studio_contents

> models::BtFeatureStudioContents2239 get_feature_studio_contents(did, wvm, wvmid, eid)
Get the text for a Feature Studio element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**models::BtFeatureStudioContents2239**](BTFeatureStudioContents-2239.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_studio_specs

> models::BtFeatureSpecsResponse664 get_feature_studio_specs(did, wvm, wvmid, eid)
Get the feature specs for a Feature Studio element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**models::BtFeatureSpecsResponse664**](BTFeatureSpecsResponse-664.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feature_studio_contents

> models::BtFeatureStudioContents2239 update_feature_studio_contents(did, wvm, wvmid, eid, bt_feature_studio_contents2239)
Update the text for a Feature Studio element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_feature_studio_contents2239** | Option<[**BtFeatureStudioContents2239**](BtFeatureStudioContents2239.md)> |  |  |

### Return type

[**models::BtFeatureStudioContents2239**](BTFeatureStudioContents-2239.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

