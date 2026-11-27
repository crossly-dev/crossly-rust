# \RestockPromptsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_restock_prompt_dismiss**](RestockPromptsApi.md#create_restock_prompt_dismiss) | **POST** /v1/restock-prompts/{id}/dismiss | Dismiss a pending restock prompt.
[**create_restock_prompt_republish**](RestockPromptsApi.md#create_restock_prompt_republish) | **POST** /v1/restock-prompts/{id}/republish | Republish a restock prompt to platforms.
[**get_restock_prompt_setting**](RestockPromptsApi.md#get_restock_prompt_setting) | **GET** /v1/restock-prompts/settings | Whether restocked items are republished automatically.
[**list_restock_prompts**](RestockPromptsApi.md#list_restock_prompts) | **GET** /v1/restock-prompts | List pending restock prompts.
[**update_restock_prompt_setting**](RestockPromptsApi.md#update_restock_prompt_setting) | **PUT** /v1/restock-prompts/settings | Turn automatic republish of restocked items on or off.



## create_restock_prompt_dismiss

> crate::models::OkResponse create_restock_prompt_dismiss(id)
Dismiss a pending restock prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_restock_prompt_republish

> crate::models::CreateRestockPromptRepublishResponse create_restock_prompt_republish(id)
Republish a restock prompt to platforms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateRestockPromptRepublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_restock_prompt_setting

> crate::models::GetRestockPromptSettingResponse get_restock_prompt_setting()
Whether restocked items are republished automatically.

autoRepublish (default true): when an item that sold out comes back in stock, Crossly republishes it to the marketplaces it came off without asking. False: a restock prompt is raised instead. A listing the seller ended by hand is never republished automatically.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetRestockPromptSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_restock_prompts

> crate::models::ListRestockPromptsResponse list_restock_prompts()
List pending restock prompts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListRestockPromptsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_restock_prompt_setting

> crate::models::GetRestockPromptSettingResponse update_restock_prompt_setting(get_restock_prompt_setting_response)
Turn automatic republish of restocked items on or off.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_restock_prompt_setting_response** | **GetRestockPromptSettingResponse** |  | [required] |

### Return type

**crate::models::GetRestockPromptSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

