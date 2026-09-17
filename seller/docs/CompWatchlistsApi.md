# \CompWatchlistsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_comp_watchlist**](CompWatchlistsApi.md#create_comp_watchlist) | **POST** /v1/comp-watchlists | Create a sold-comp watchlist.
[**create_comp_watchlist_scrape**](CompWatchlistsApi.md#create_comp_watchlist_scrape) | **POST** /v1/comp-watchlists/{id}/scrape | Manually trigger a watchlist scrape.
[**delete_comp_watchlist**](CompWatchlistsApi.md#delete_comp_watchlist) | **DELETE** /v1/comp-watchlists/{id} | Delete a sold-comp watchlist.
[**list_comp_watchlist_recent**](CompWatchlistsApi.md#list_comp_watchlist_recent) | **GET** /v1/comp-watchlists/{id}/recent | Recent external sold comps matching this watchlist.
[**list_comp_watchlists**](CompWatchlistsApi.md#list_comp_watchlists) | **GET** /v1/comp-watchlists | List the seller's sold-comp watchlists.



## create_comp_watchlist

> crate::models::CreateCompWatchlistResponse create_comp_watchlist()
Create a sold-comp watchlist.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCompWatchlistResponse**](CreateCompWatchlistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_comp_watchlist_scrape

> crate::models::CreateCompWatchlistScrapeResponse create_comp_watchlist_scrape(id)
Manually trigger a watchlist scrape.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCompWatchlistScrapeResponse**](CreateCompWatchlistScrapeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comp_watchlist

> crate::models::DeleteCompWatchlistResponse delete_comp_watchlist(id)
Delete a sold-comp watchlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteCompWatchlistResponse**](DeleteCompWatchlistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_comp_watchlist_recent

> crate::models::V1List list_comp_watchlist_recent(id)
Recent external sold comps matching this watchlist.

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


## list_comp_watchlists

> crate::models::V1List list_comp_watchlists()
List the seller's sold-comp watchlists.

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

