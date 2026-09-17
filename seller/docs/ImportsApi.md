# \ImportsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_import**](ImportsApi.md#create_import) | **POST** /v1/imports | Start a bulk-import job for an existing platform connection.
[**get_import**](ImportsApi.md#get_import) | **GET** /v1/imports/{id} | Get one import job.
[**list_imports**](ImportsApi.md#list_imports) | **GET** /v1/imports | List bulk-import jobs.



## create_import

> crate::models::CreateImportResponse create_import()
Start a bulk-import job for an existing platform connection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateImportResponse**](CreateImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import

> crate::models::GetImportResponse get_import(id)
Get one import job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetImportResponse**](GetImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_imports

> crate::models::V1List list_imports()
List bulk-import jobs.

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

