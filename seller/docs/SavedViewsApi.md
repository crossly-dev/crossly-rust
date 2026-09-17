# \SavedViewsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saved_view**](SavedViewsApi.md#create_saved_view) | **POST** /v1/saved-views | Create a saved view preset.
[**delete_saved_view**](SavedViewsApi.md#delete_saved_view) | **DELETE** /v1/saved-views/{id} | Delete a saved view preset.
[**list_saved_views**](SavedViewsApi.md#list_saved_views) | **GET** /v1/saved-views | List the seller's saved view presets.
[**update_saved_view**](SavedViewsApi.md#update_saved_view) | **PATCH** /v1/saved-views/{id} | Update a saved view preset.



## create_saved_view

> crate::models::CreateSavedViewResponse create_saved_view()
Create a saved view preset.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateSavedViewResponse**](CreateSavedViewResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saved_view

> crate::models::DeleteSavedViewResponse delete_saved_view(id)
Delete a saved view preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteSavedViewResponse**](DeleteSavedViewResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saved_views

> crate::models::V1List list_saved_views(resource)
List the seller's saved view presets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saved_view

> crate::models::UpdateSavedViewResponse update_saved_view(id)
Update a saved view preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateSavedViewResponse**](UpdateSavedViewResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

