# \ConnectionsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_connection_health**](ConnectionsApi.md#get_connection_health) | **GET** /v1/connection-health | Health of each connected marketplace account, with a plain-English diagnosis.
[**list_devices**](ConnectionsApi.md#list_devices) | **GET** /v1/devices | Machines paired to this account, and what each can do.



## get_connection_health

> crate::models::GetConnectionHealthResponse get_connection_health(include_unconnected)
Health of each connected marketplace account, with a plain-English diagnosis.

One entry per ACCOUNT, not per platform — a healthy slot 1 must not speak for a dead slot 2. States that were never measured are reported as \"unknown\" rather than as failures.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_unconnected** | Option<**bool**> |  |  |

### Return type

[**crate::models::GetConnectionHealthResponse**](GetConnectionHealthResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_devices

> crate::models::V1List list_devices()
Machines paired to this account, and what each can do.

Capabilities are what the machine DECLARED it can do at pairing — print, scan_watch, browser, cookie_jar, proxy_bind, scale. A machine only declares \"scale\" when one actually answered, never on the assumption that one might.

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

