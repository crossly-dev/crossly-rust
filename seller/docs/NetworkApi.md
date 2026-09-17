# \NetworkApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_network_pool**](NetworkApi.md#create_network_pool) | **POST** /v1/network/pool | Join the Crossly Network reciprocal engagement pool.
[**delete_network_pool**](NetworkApi.md#delete_network_pool) | **DELETE** /v1/network/pool | Leave the Crossly Network pool.
[**get_network_pool**](NetworkApi.md#get_network_pool) | **GET** /v1/network/pool | The seller's Crossly Network pool membership row.
[**get_network_pool_size**](NetworkApi.md#get_network_pool_size) | **GET** /v1/network/pool/size | Total members in the Crossly Network pool.
[**list_network_pool_log**](NetworkApi.md#list_network_pool_log) | **GET** /v1/network/pool/log | Recent engagement history — both sent and received.
[**update_network_pool**](NetworkApi.md#update_network_pool) | **PATCH** /v1/network/pool | Update per-action toggles + platforms on pool membership.



## create_network_pool

> crate::models::CreateNetworkPoolResponse create_network_pool()
Join the Crossly Network reciprocal engagement pool.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateNetworkPoolResponse**](CreateNetworkPoolResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_network_pool

> crate::models::DeleteNetworkPoolResponse delete_network_pool()
Leave the Crossly Network pool.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeleteNetworkPoolResponse**](DeleteNetworkPoolResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_pool

> crate::models::GetNetworkPoolResponse get_network_pool()
The seller's Crossly Network pool membership row.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetNetworkPoolResponse**](GetNetworkPoolResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_pool_size

> crate::models::GetNetworkPoolSizeResponse get_network_pool_size()
Total members in the Crossly Network pool.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetNetworkPoolSizeResponse**](GetNetworkPoolSizeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_network_pool_log

> crate::models::V1List list_network_pool_log(limit)
Recent engagement history — both sent and received.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_network_pool

> crate::models::UpdateNetworkPoolResponse update_network_pool()
Update per-action toggles + platforms on pool membership.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UpdateNetworkPoolResponse**](UpdateNetworkPoolResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

