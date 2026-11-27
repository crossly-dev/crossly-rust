# \BulkBatchesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bulk_batch**](BulkBatchesApi.md#create_bulk_batch) | **POST** /v1/bulk-batches | Open an empty capture batch for a preset.
[**create_bulk_batch_identify**](BulkBatchesApi.md#create_bulk_batch_identify) | **POST** /v1/bulk-batches/{id}/identify | Identify items: match to Crossly Market, Magic List, or auto (by preset). Runs in the background.
[**create_bulk_batch_item**](BulkBatchesApi.md#create_bulk_batch_item) | **POST** /v1/bulk-batches/{id}/items | Append one item (every photo of it, primary first).
[**create_bulk_batch_item_bulk**](BulkBatchesApi.md#create_bulk_batch_item_bulk) | **POST** /v1/bulk-batches/{id}/items/bulk | Set price/quantity/condition/status on many items.
[**create_bulk_batch_publish**](BulkBatchesApi.md#create_bulk_batch_publish) | **POST** /v1/bulk-batches/{id}/publish | Publish reviewed items: drafts only (default) or live on destinations; adds quantity to an existing matching listing.
[**delete_bulk_batch**](BulkBatchesApi.md#delete_bulk_batch) | **DELETE** /v1/bulk-batches/{id} | Discard a batch (listings it produced stay).
[**delete_bulk_batch_item**](BulkBatchesApi.md#delete_bulk_batch_item) | **DELETE** /v1/bulk-batches/items/{id} | Remove one unpublished item.
[**get_bulk_batch**](BulkBatchesApi.md#get_bulk_batch) | **GET** /v1/bulk-batches/{id} | Get one bulk batch with its items (capture order) and matched catalog rows.
[**list_bulk_batches**](BulkBatchesApi.md#list_bulk_batches) | **GET** /v1/bulk-batches | List recent bulk list batches, newest first.
[**update_bulk_batch**](BulkBatchesApi.md#update_bulk_batch) | **PATCH** /v1/bulk-batches/{id} | Change batch settings, or set stage \"identify\" when capture is done.
[**update_bulk_batch_item**](BulkBatchesApi.md#update_bulk_batch_item) | **PATCH** /v1/bulk-batches/items/{id} | Correct one item (match, title, price, photos…).



## create_bulk_batch

> crate::models::ListBulkBatchesItem create_bulk_batch()
Open an empty capture batch for a preset.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBulkBatchesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_bulk_batch_identify

> crate::models::CreateBulkBatchIdentifyResponse create_bulk_batch_identify(id)
Identify items: match to Crossly Market, Magic List, or auto (by preset). Runs in the background.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBulkBatchIdentifyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_bulk_batch_item

> crate::models::CreateBulkBatchItemResponse create_bulk_batch_item(id)
Append one item (every photo of it, primary first).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBulkBatchItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_bulk_batch_item_bulk

> crate::models::UpdatedResponse create_bulk_batch_item_bulk(id)
Set price/quantity/condition/status on many items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdatedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_bulk_batch_publish

> crate::models::CreateBulkBatchPublishResponse create_bulk_batch_publish(id)
Publish reviewed items: drafts only (default) or live on destinations; adds quantity to an existing matching listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBulkBatchPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_bulk_batch

> crate::models::OkResponse delete_bulk_batch(id)
Discard a batch (listings it produced stay).

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


## delete_bulk_batch_item

> crate::models::OkResponse delete_bulk_batch_item(id)
Remove one unpublished item.

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


## get_bulk_batch

> crate::models::GetBulkBatchResponse get_bulk_batch(id)
Get one bulk batch with its items (capture order) and matched catalog rows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBulkBatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_bulk_batches

> crate::models::ListBulkBatchesResponse list_bulk_batches(limit)
List recent bulk list batches, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListBulkBatchesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_bulk_batch

> crate::models::ListBulkBatchesItem update_bulk_batch(id)
Change batch settings, or set stage \"identify\" when capture is done.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBulkBatchesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_bulk_batch_item

> crate::models::CreateBulkBatchItemResponse update_bulk_batch_item(id)
Correct one item (match, title, price, photos…).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBulkBatchItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

