# \WorkflowsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow_chain**](WorkflowsApi.md#create_workflow_chain) | **POST** /v1/workflow-chains | Create a multi-step workflow chain.
[**create_workflow_chain_run_now**](WorkflowsApi.md#create_workflow_chain_run_now) | **POST** /v1/workflow-chains/{id}/run-now | Enqueue an ad-hoc run of a workflow chain.
[**create_workflow_chain_toggle**](WorkflowsApi.md#create_workflow_chain_toggle) | **POST** /v1/workflow-chains/{id}/toggle | Flip a workflow chain between active and inactive.
[**delete_workflow_chain**](WorkflowsApi.md#delete_workflow_chain) | **DELETE** /v1/workflow-chains/{id} | Delete a workflow chain (cascades steps + runs).
[**get_workflow_chain**](WorkflowsApi.md#get_workflow_chain) | **GET** /v1/workflow-chains/{id} | Get one workflow chain with its steps.
[**list_workflow_chains**](WorkflowsApi.md#list_workflow_chains) | **GET** /v1/workflow-chains | List workflow chains with their step graph.
[**update_workflow_chain**](WorkflowsApi.md#update_workflow_chain) | **PUT** /v1/workflow-chains/{id} | Replace a workflow chain wholesale.



## create_workflow_chain

> crate::models::CreateWorkflowChainResponse create_workflow_chain()
Create a multi-step workflow chain.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateWorkflowChainResponse**](CreateWorkflowChainResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workflow_chain_run_now

> crate::models::CreateWorkflowChainRunNowResponse create_workflow_chain_run_now(id)
Enqueue an ad-hoc run of a workflow chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateWorkflowChainRunNowResponse**](CreateWorkflowChainRunNowResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workflow_chain_toggle

> crate::models::CreateWorkflowChainToggleResponse create_workflow_chain_toggle(id)
Flip a workflow chain between active and inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateWorkflowChainToggleResponse**](CreateWorkflowChainToggleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_chain

> crate::models::DeleteWorkflowChainResponse delete_workflow_chain(id)
Delete a workflow chain (cascades steps + runs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteWorkflowChainResponse**](DeleteWorkflowChainResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_chain

> crate::models::GetWorkflowChainResponse get_workflow_chain(id)
Get one workflow chain with its steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetWorkflowChainResponse**](GetWorkflowChainResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_chains

> crate::models::V1List list_workflow_chains()
List workflow chains with their step graph.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_chain

> crate::models::UpdateWorkflowChainResponse update_workflow_chain(id)
Replace a workflow chain wholesale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateWorkflowChainResponse**](UpdateWorkflowChainResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

