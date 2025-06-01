# BtObjectWorkflowInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_be_discarded** | Option<**bool**> | Whether workflowable object can be discarded. | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**is_discarded** | Option<**bool**> | Whether workflowable object has been discarded. | [optional]
**is_frozen** | Option<**bool**> | Whether workflowable object has reached terminal state and is frozen. | [optional]
**metadata_state** | Option<[**models::BtMetadataStateType**](BTMetadataStateType.md)> |  | [optional]
**name** | Option<**String**> | Name of the resource. | [optional]
**object_type** | Option<[**models::BtapiWorkflowableType**](BTAPIWorkflowableType.md)> |  | [optional]
**state_id** | Option<**String**> | The current state of object like SETUP, REJECTED etc. Custom workflows can have any declared state. | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]
**workflow_id** | Option<**String**> | The workflow definition id that governs this object's states and transitions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


