# \ActivityApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_action_log**](ActivityApi.md#get_action_log) | **GET** /v1/action-log/{id} | Get one action-log event by id (ownership-checked).
[**get_action_log_facet**](ActivityApi.md#get_action_log_facet) | **GET** /v1/action-log/facets | Distinct platforms / actions / categories present in the caller's action log (last 90 days) — powers filter dropdowns before you query.
[**list_action_log**](ActivityApi.md#list_action_log) | **GET** /v1/action-log | List action-log events — the semantic \"what happened\" record of every user + platform action. Filter by platform / action / category / status / source / target, and a since/until created_at window.
[**list_action_log_calls**](ActivityApi.md#list_action_log_calls) | **GET** /v1/action-log/{id}/calls | The outbound platform HTTP calls under an event (oldest first) — url, method, status, latency, redacted request/response bodies, proxy + recipe/hash. Answers \"what was sent / what went wrong\".



## get_action_log

> crate::models::GetActionLogResponse get_action_log(id)
Get one action-log event by id (ownership-checked).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetActionLogResponse**](GetActionLogResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action_log_facet

> crate::models::GetActionLogFacetResponse get_action_log_facet()
Distinct platforms / actions / categories present in the caller's action log (last 90 days) — powers filter dropdowns before you query.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetActionLogFacetResponse**](GetActionLogFacetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_action_log

> crate::models::V1List list_action_log(platform, action, category, status, source, target_type, target_id, since, until, limit, offset)
List action-log events — the semantic \"what happened\" record of every user + platform action. Filter by platform / action / category / status / source / target, and a since/until created_at window.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> |  |  |
**action** | Option<**String**> |  |  |
**category** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**target_type** | Option<**String**> |  |  |
**target_id** | Option<**String**> |  |  |
**since** | Option<**String**> | ISO lower bound (inclusive) on created_at. |  |
**until** | Option<**String**> | ISO upper bound (exclusive) on created_at. |  |
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_action_log_calls

> crate::models::V1List list_action_log_calls(id)
The outbound platform HTTP calls under an event (oldest first) — url, method, status, latency, redacted request/response bodies, proxy + recipe/hash. Answers \"what was sent / what went wrong\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

