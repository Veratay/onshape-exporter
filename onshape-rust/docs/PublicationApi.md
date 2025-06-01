# \PublicationApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_item_to_publication**](PublicationApi.md#add_item_to_publication) | **POST** /publications/{pid}/item | Add an item in a publication.
[**add_items_to_publication**](PublicationApi.md#add_items_to_publication) | **POST** /publications/{pid}/items | Add publication items in bulk.
[**create_publication**](PublicationApi.md#create_publication) | **POST** /publications | Create a new publication.
[**delete_publication**](PublicationApi.md#delete_publication) | **DELETE** /publications/{pid} | Delete a publication.
[**delete_publication_item**](PublicationApi.md#delete_publication_item) | **DELETE** /publications/{pid}/item/{iid} | Remove an item from a publication.
[**get_publication_items**](PublicationApi.md#get_publication_items) | **GET** /publications/{pid}/items | Get all items in a publication.
[**update_publication_attributes**](PublicationApi.md#update_publication_attributes) | **POST** /publications/{pid} | Update publication's attributes name, description, and notes.



## add_item_to_publication

> models::BtPublicationInfo add_item_to_publication(pid, bt_publication_item_params)
Add an item in a publication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |
**bt_publication_item_params** | [**BtPublicationItemParams**](BtPublicationItemParams.md) |  | [required] |

### Return type

[**models::BtPublicationInfo**](BTPublicationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_items_to_publication

> models::BtPublicationInfo add_items_to_publication(pid, bt_publication_bulk_item_params)
Add publication items in bulk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |
**bt_publication_bulk_item_params** | [**BtPublicationBulkItemParams**](BtPublicationBulkItemParams.md) |  | [required] |

### Return type

[**models::BtPublicationInfo**](BTPublicationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_publication

> models::BtPublicationInfo create_publication(bt_publication_params)
Create a new publication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_publication_params** | [**BtPublicationParams**](BtPublicationParams.md) |  | [required] |

### Return type

[**models::BtPublicationInfo**](BTPublicationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_publication

> serde_json::Value delete_publication(pid, forever)
Delete a publication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |
**forever** | Option<**bool**> | If true, publication is deleted forever. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_publication_item

> serde_json::Value delete_publication_item(pid, iid)
Remove an item from a publication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |
**iid** | **String** | Publication item ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_publication_items

> models::BtPublicationInfo get_publication_items(pid)
Get all items in a publication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |

### Return type

[**models::BtPublicationInfo**](BTPublicationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_publication_attributes

> serde_json::Value update_publication_attributes(pid, bt_publication_params)
Update publication's attributes name, description, and notes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | Publication ID. | [required] |
**bt_publication_params** | [**BtPublicationParams**](BtPublicationParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

