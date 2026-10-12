# \ReturnsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy_return_received**](ReturnsApi.md#create_policy_return_received) | **POST** /v1/policy-returns/{id}/received | Mark a return received: refunds the buyer, less the restocking or try fee you keep.
[**create_return**](ReturnsApi.md#create_return) | **POST** /v1/returns | Open a return record on an order (rejects if another open return exists).
[**get_return**](ReturnsApi.md#get_return) | **GET** /v1/returns/{id} | Get one return record.
[**list_policy_returns**](ReturnsApi.md#list_policy_returns) | **GET** /v1/policy-returns | Returns Crossly buyers opened under your return policy or try at home; filter by orderId.
[**list_returns**](ReturnsApi.md#list_returns) | **GET** /v1/returns | List returns (physical-return workflow). status=open|closed|<exact>.
[**update_return**](ReturnsApi.md#update_return) | **PATCH** /v1/returns/{id} | Transition return status (received/inspected/restocked) and bump inventory on restock.



## create_policy_return_received

> crate::models::CreatePolicyReturnReceivedResponse create_policy_return_received(id)
Mark a return received: refunds the buyer, less the restocking or try fee you keep.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatePolicyReturnReceivedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_return

> crate::models::ListReturnsItem create_return()
Open a return record on an order (rejects if another open return exists).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListReturnsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_return

> crate::models::ListReturnsItem get_return(id)
Get one return record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListReturnsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_policy_returns

> crate::models::ListPolicyReturnsResponse list_policy_returns()
Returns Crossly buyers opened under your return policy or try at home; filter by orderId.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPolicyReturnsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_returns

> crate::models::ListReturnsResponse list_returns()
List returns (physical-return workflow). status=open|closed|<exact>.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListReturnsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_return

> crate::models::ListReturnsItem update_return(id)
Transition return status (received/inspected/restocked) and bump inventory on restock.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListReturnsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

