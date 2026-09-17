# \PATApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pat**](PATApi.md#create_pat) | **POST** /v1/pat | Mint a new PAT. Full token returned ONCE — store it on the client.
[**delete_pat**](PATApi.md#delete_pat) | **DELETE** /v1/pat/{id} | Revoke a PAT by id.
[**get_pat_scope**](PATApi.md#get_pat_scope) | **GET** /v1/pat/scopes | List the canonical scope catalog.
[**list_pat**](PATApi.md#list_pat) | **GET** /v1/pat | List the caller's PATs (preview only).



## create_pat

> crate::models::CreatePatResponse create_pat()
Mint a new PAT. Full token returned ONCE — store it on the client.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreatePatResponse**](CreatePatResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pat

> crate::models::DeletePatResponse delete_pat(id)
Revoke a PAT by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeletePatResponse**](DeletePatResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pat_scope

> crate::models::GetPatScopeResponse get_pat_scope()
List the canonical scope catalog.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPatScopeResponse**](GetPatScopeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pat

> crate::models::V1List list_pat()
List the caller's PATs (preview only).

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

