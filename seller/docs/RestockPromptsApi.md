# \RestockPromptsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_restock_prompt_dismiss**](RestockPromptsApi.md#create_restock_prompt_dismiss) | **POST** /v1/restock-prompts/{id}/dismiss | Dismiss a pending restock prompt.
[**create_restock_prompt_republish**](RestockPromptsApi.md#create_restock_prompt_republish) | **POST** /v1/restock-prompts/{id}/republish | Republish a restock prompt to platforms.
[**list_restock_prompts**](RestockPromptsApi.md#list_restock_prompts) | **GET** /v1/restock-prompts | List pending restock prompts.



## create_restock_prompt_dismiss

> crate::models::CreateRestockPromptDismissResponse create_restock_prompt_dismiss(id)
Dismiss a pending restock prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateRestockPromptDismissResponse**](CreateRestockPromptDismissResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_restock_prompt_republish

> crate::models::CreateRestockPromptRepublishResponse create_restock_prompt_republish(id)
Republish a restock prompt to platforms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateRestockPromptRepublishResponse**](CreateRestockPromptRepublishResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_restock_prompts

> crate::models::V1List list_restock_prompts()
List pending restock prompts.

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

