# \SourceSyncApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_source_sync_sync**](SourceSyncApi.md#create_source_sync_sync) | **POST** /v1/source-sync/{platform}/sync | Reconcile a store now: re-read its whole catalogue and apply every change (runs in the background; 202).
[**get_source_sync_location**](SourceSyncApi.md#get_source_sync_location) | **GET** /v1/source-sync/{platform}/locations | POS only (lightspeed_r, lightspeed_x, square): the shops/outlets/locations on the account and which one Crossly syncs stock and sales with.
[**list_source_sync**](SourceSyncApi.md#list_source_sync) | **GET** /v1/source-sync | List source-capable stores with their continuous-sync settings and status.
[**list_source_sync_events**](SourceSyncApi.md#list_source_sync_events) | **GET** /v1/source-sync/events | Recent source-sync activity: each webhook delivery and reconcile change, with its outcome.
[**update_source_sync**](SourceSyncApi.md#update_source_sync) | **PATCH** /v1/source-sync/{platform} | Update a store's source-sync settings: on/off, source-owned fields, deletion handling, auto-export channels.
[**update_source_sync_location**](SourceSyncApi.md#update_source_sync_location) | **PUT** /v1/source-sync/{platform}/location | POS only: choose the shop (R-Series) / outlet (X-Series) / Square location whose stock and sales Crossly syncs. Must be one GET .../locations lists.



## create_source_sync_sync

> crate::models::CreateSourceSyncSyncResponse create_source_sync_sync(platform)
Reconcile a store now: re-read its whole catalogue and apply every change (runs in the background; 202).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateSourceSyncSyncResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_source_sync_location

> crate::models::GetSourceSyncLocationResponse get_source_sync_location(platform)
POS only (lightspeed_r, lightspeed_x, square): the shops/outlets/locations on the account and which one Crossly syncs stock and sales with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

**crate::models::GetSourceSyncLocationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_source_sync

> crate::models::ListSourceSyncResponse list_source_sync()
List source-capable stores with their continuous-sync settings and status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSourceSyncResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_source_sync_events

> crate::models::ListSourceSyncEventsResponse list_source_sync_events(platform, limit)
Recent source-sync activity: each webhook delivery and reconcile change, with its outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListSourceSyncEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_source_sync

> crate::models::ListSourceSyncItem update_source_sync(platform)
Update a store's source-sync settings: on/off, source-owned fields, deletion handling, auto-export channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

**crate::models::ListSourceSyncItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_source_sync_location

> crate::models::GetSourceSyncLocationResponse update_source_sync_location(platform, update_source_sync_location_request)
POS only: choose the shop (R-Series) / outlet (X-Series) / Square location whose stock and sales Crossly syncs. Must be one GET .../locations lists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**update_source_sync_location_request** | **UpdateSourceSyncLocationRequest** |  | [required] |

### Return type

**crate::models::GetSourceSyncLocationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

