# \TaskApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TaskApi.md#create_task) | **POST** /tasks | Create a new task in a draft state.
[**get_action_items**](TaskApi.md#get_action_items) | **GET** /tasks | Lists tasks assigned to the specified user
[**get_task**](TaskApi.md#get_task) | **GET** /tasks/{tid} | Get a task by id.
[**transition_task**](TaskApi.md#transition_task) | **POST** /tasks/{tid}/{transition} | Execute a workflow transition.
[**update_task**](TaskApi.md#update_task) | **POST** /tasks/{tid} | Update the task and its properties.



## create_task

> models::BtTaskInfo create_task(bt_create_task_params)
Create a new task in a draft state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_create_task_params** | [**BtCreateTaskParams**](BtCreateTaskParams.md) |  | [required] |

### Return type

[**models::BtTaskInfo**](BTTaskInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action_items

> models::BtTaskListResponse get_action_items(user_id, offset, limit, status, role, order, r#type, document_id)
Lists tasks assigned to the specified user

Returns a list of tasks assigneed to the userId specified in the request. Only company admins can view tasks that were not created by them and are not assigned to them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 50]
**status** | Option<**i32**> |  |  |[default to 2]
**role** | Option<**i32**> |  |  |[default to 1]
**order** | Option<**i32**> |  |  |[default to 1]
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**document_id** | Option<**String**> |  |  |

### Return type

[**models::BtTaskListResponse**](BTTaskListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::BtTaskInfo get_task(tid)
Get a task by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

[**models::BtTaskInfo**](BTTaskInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transition_task

> models::BtTaskInfo transition_task(tid, transition)
Execute a workflow transition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |
**transition** | **String** |  | [required] |

### Return type

[**models::BtTaskInfo**](BTTaskInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> models::BtTaskInfo update_task(tid, bt_update_task_params)
Update the task and its properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |
**bt_update_task_params** | [**BtUpdateTaskParams**](BtUpdateTaskParams.md) |  | [required] |

### Return type

[**models::BtTaskInfo**](BTTaskInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

