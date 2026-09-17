# \BuyerMonitorsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_monitor**](BuyerMonitorsApi.md#create_buyer_monitor) | **POST** /v1/buyer/monitors | Watch a search, and be told when it matches.
[**delete_buyer_monitor**](BuyerMonitorsApi.md#delete_buyer_monitor) | **DELETE** /v1/buyer/monitors/{id} | Delete a monitor.
[**list_buyer_monitor_matches**](BuyerMonitorsApi.md#list_buyer_monitor_matches) | **GET** /v1/buyer/monitors/{id}/matches | What this monitor has matched.
[**list_buyer_monitors**](BuyerMonitorsApi.md#list_buyer_monitors) | **GET** /v1/buyer/monitors | Your monitors.
[**update_buyer_monitor**](BuyerMonitorsApi.md#update_buyer_monitor) | **PATCH** /v1/buyer/monitors/{id} | Pause, resume or rename a monitor.



## create_buyer_monitor

> crate::models::CreateBuyerMonitorResponse create_buyer_monitor()
Watch a search, and be told when it matches.

Works immediately — there is no review step. The signing secret is returned ONCE, here; it is never readable again. The first sweep SEEDS without firing: a restock alert created while the item is already in stock has not observed a restock, and a new-listing monitor would otherwise deliver the entire back catalogue.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateBuyerMonitorResponse**](CreateBuyerMonitorResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_buyer_monitor

> delete_buyer_monitor(id)
Delete a monitor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_monitor_matches

> crate::models::V1List list_buyer_monitor_matches(id)
What this monitor has matched.

The read side of a `poll` monitor, and an audit trail for a `webhook` one — so a missed delivery does not mean lost data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buyer_monitors

> crate::models::V1List list_buyer_monitors()
Your monitors.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_buyer_monitor

> crate::models::UpdateBuyerMonitorResponse update_buyer_monitor(id)
Pause, resume or rename a monitor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateBuyerMonitorResponse**](UpdateBuyerMonitorResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

