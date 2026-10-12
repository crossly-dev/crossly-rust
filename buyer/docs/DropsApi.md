# \DropsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_drop_join**](DropsApi.md#create_buyer_drop_join) | **POST** /v1/buyer/drops/{id}/join | Join a drop's line while it is open: one entry per account, the same odds as joining in the world. Repeating it returns the same entry.
[**delete_buyer_drop_join**](DropsApi.md#delete_buyer_drop_join) | **DELETE** /v1/buyer/drops/{id}/join | Leave a drop's line before the draw.
[**get_buyer_drop_me**](DropsApi.md#get_buyer_drop_me) | **GET** /v1/buyer/drops/{id}/me | Your entry in a drop: in line or not, your draw rank and outcome, and the checkout path while your 15-minute hold is live.



## create_buyer_drop_join

> crate::models::CreateBuyerDropJoinResponse create_buyer_drop_join(id)
Join a drop's line while it is open: one entry per account, the same odds as joining in the world. Repeating it returns the same entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerDropJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_drop_join

> crate::models::DeleteBuyerDropJoinResponse delete_buyer_drop_join(id)
Leave a drop's line before the draw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteBuyerDropJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_drop_me

> crate::models::GetBuyerDropMeResponse get_buyer_drop_me(id)
Your entry in a drop: in line or not, your draw rank and outcome, and the checkout path while your 15-minute hold is live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerDropMeResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

