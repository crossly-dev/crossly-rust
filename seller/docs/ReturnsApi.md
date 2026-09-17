# \ReturnsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_return**](ReturnsApi.md#create_return) | **POST** /v1/returns | Open a return record on an order (rejects if another open return exists).
[**get_return**](ReturnsApi.md#get_return) | **GET** /v1/returns/{id} | Get one return record.
[**list_returns**](ReturnsApi.md#list_returns) | **GET** /v1/returns | List returns (physical-return workflow). status=open|closed|<exact>.
[**update_return**](ReturnsApi.md#update_return) | **PATCH** /v1/returns/{id} | Transition return status (received/inspected/restocked) and bump inventory on restock.



## create_return

> crate::models::CreateReturnResponse create_return()
Open a return record on an order (rejects if another open return exists).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateReturnResponse**](CreateReturnResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_return

> crate::models::GetReturnResponse get_return(id)
Get one return record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetReturnResponse**](GetReturnResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_returns

> crate::models::V1List list_returns()
List returns (physical-return workflow). status=open|closed|<exact>.

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


## update_return

> crate::models::UpdateReturnResponse update_return(id)
Transition return status (received/inspected/restocked) and bump inventory on restock.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateReturnResponse**](UpdateReturnResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

