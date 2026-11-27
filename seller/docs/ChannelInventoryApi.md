# \ChannelInventoryApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_channel_inventory**](ChannelInventoryApi.md#delete_channel_inventory) | **DELETE** /v1/channel-inventory/{platform}/{slot} | Reset a channel's inventory controls to the default (no cap, no buffer, sync on, end on sell-out).
[**get_channel_inventory**](ChannelInventoryApi.md#get_channel_inventory) | **GET** /v1/channel-inventory/{platform}/{slot} | Get one channel's inventory controls.
[**list_channel_inventory**](ChannelInventoryApi.md#list_channel_inventory) | **GET** /v1/channel-inventory | List every sales channel with its inventory controls (cap, buffer, sync, out-of-stock action, exclusions).
[**update_channel_inventory**](ChannelInventoryApi.md#update_channel_inventory) | **PUT** /v1/channel-inventory/{platform}/{slot} | Set a channel's quantity cap, buffer, sync toggle, out-of-stock action and exclusion lists (merge write).



## delete_channel_inventory

> crate::models::ListChannelInventoryItem delete_channel_inventory(platform, slot)
Reset a channel's inventory controls to the default (no cap, no buffer, sync on, end on sell-out).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**slot** | **i32** |  | [required] |

### Return type

**crate::models::ListChannelInventoryItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_channel_inventory

> crate::models::ListChannelInventoryItem get_channel_inventory(platform, slot)
Get one channel's inventory controls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**slot** | **i32** |  | [required] |

### Return type

**crate::models::ListChannelInventoryItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_channel_inventory

> crate::models::ListChannelInventoryResponse list_channel_inventory()
List every sales channel with its inventory controls (cap, buffer, sync, out-of-stock action, exclusions).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListChannelInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_channel_inventory

> crate::models::ListChannelInventoryItem update_channel_inventory(platform, slot)
Set a channel's quantity cap, buffer, sync toggle, out-of-stock action and exclusion lists (merge write).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**slot** | **i32** |  | [required] |

### Return type

**crate::models::ListChannelInventoryItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

