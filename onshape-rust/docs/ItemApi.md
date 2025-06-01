# \ItemApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_item**](ItemApi.md#create_item) | **POST** /items | Create a new item.
[**delete_item**](ItemApi.md#delete_item) | **DELETE** /items/{iid} | Delete an item.
[**get_item**](ItemApi.md#get_item) | **GET** /items/{iid} | Get item by ID.
[**get_items**](ItemApi.md#get_items) | **GET** /items | Get all items owned by a company/classroom/enterprise.
[**update_item**](ItemApi.md#update_item) | **POST** /items/{iid} | Update an item.



## create_item

> models::BtItemInfo create_item(bt_item_params)
Create a new item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_item_params** | [**BtItemParams**](BtItemParams.md) |  | [required] |

### Return type

[**models::BtItemInfo**](BTItemInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_item

> serde_json::Value delete_item(iid)
Delete an item.

Items can only be deleted if `publishState = 0` (`PENDING`). [`GET /items/{iid}`](#/Items/getItem) to get the `publishState`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iid** | **String** | ID of the item to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item

> models::BtItemInfo get_item(iid, document_id, company_id)
Get item by ID.

Either `documentId` or `companyId` must be provided, in addition to the item ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iid** | **String** | Item ID.  | [required] |
**document_id** | Option<**String**> | ID of any document owned by the company/classroom/enterprise. |  |
**company_id** | Option<**String**> | Company/classroom/enterprise ID. |  |

### Return type

[**models::BtItemInfo**](BTItemInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items

> models::BtListResponseBtItemInfo get_items(document_id, company_id, q, publish_states, classification, offset, limit)
Get all items owned by a company/classroom/enterprise.

Returns a list of all items owned by the company/classroom/enterprise.    Either `documentId` or `companyId` must be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | Option<**String**> | ID of any document owned by the company/classroom/enterprise. |  |
**company_id** | Option<**String**> | Company/classroom/enterprise ID. |  |
**q** | Option<**String**> | Optional search string. |  |[default to ]
**publish_states** | Option<**String**> | Refine returned items by publish states: `0: PENDING | 1: ACTIVE | 2: INACTIVE` |  |[default to ]
**classification** | Option<**String**> | Refine returned items by classification. Classifications are managed in company/classroom/enterprise [Properties settings](https://cad.onshape.com/help/Content/Plans/items.htm#item-class). |  |[default to ]
**offset** | Option<**i32**> | Number of entries to skip in the returned list. |  |[default to 0]
**limit** | Option<**i32**> | The number of list entries to return in a single API call. |  |[default to 30]

### Return type

[**models::BtListResponseBtItemInfo**](BTListResponseBTItemInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item

> models::BtItemInfo update_item(iid, bt_item_params)
Update an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iid** | **String** | ID of the item to update. | [required] |
**bt_item_params** | [**BtItemParams**](BtItemParams.md) |  | [required] |

### Return type

[**models::BtItemInfo**](BTItemInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

