# \ProductFeedsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_feed**](ProductFeedsApi.md#create_product_feed) | **POST** /v1/product-feeds | Create a hosted product feed for a channel. Returns its secret URL.
[**create_product_feed_rotate**](ProductFeedsApi.md#create_product_feed_rotate) | **POST** /v1/product-feeds/{id}/rotate | Rotate the feed token. The old URL stops answering immediately; paste the new one into the channel.
[**delete_product_feed**](ProductFeedsApi.md#delete_product_feed) | **DELETE** /v1/product-feeds/{id} | Delete a feed. Its URL stops answering immediately.
[**get_product_feed_report**](ProductFeedsApi.md#get_product_feed_report) | **GET** /v1/product-feeds/{id}/report | Validate a feed: per-item errors (held back) and warnings (sent, limited reach), plus sample rendered items.
[**list_product_feeds**](ProductFeedsApi.md#list_product_feeds) | **GET** /v1/product-feeds | List your hosted product feeds (Google Merchant Center, Meta catalogue, TikTok catalog) with their URLs and last-fetch stats.
[**update_product_feed**](ProductFeedsApi.md#update_product_feed) | **PATCH** /v1/product-feeds/{id} | Edit a feed: name, default format, enabled, listing filters, output settings.



## create_product_feed

> crate::models::ListProductFeedsItem create_product_feed()
Create a hosted product feed for a channel. Returns its secret URL.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListProductFeedsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_product_feed_rotate

> crate::models::ListProductFeedsItem create_product_feed_rotate(id)
Rotate the feed token. The old URL stops answering immediately; paste the new one into the channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListProductFeedsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_product_feed

> crate::models::OkResponse delete_product_feed(id)
Delete a feed. Its URL stops answering immediately.

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


## get_product_feed_report

> crate::models::GetProductFeedReportResponse get_product_feed_report(id)
Validate a feed: per-item errors (held back) and warnings (sent, limited reach), plus sample rendered items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetProductFeedReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_product_feeds

> crate::models::ListProductFeedsResponse list_product_feeds()
List your hosted product feeds (Google Merchant Center, Meta catalogue, TikTok catalog) with their URLs and last-fetch stats.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListProductFeedsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_product_feed

> crate::models::ListProductFeedsItem update_product_feed(id)
Edit a feed: name, default format, enabled, listing filters, output settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListProductFeedsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

