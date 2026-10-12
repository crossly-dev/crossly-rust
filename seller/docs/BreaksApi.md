# \BreaksApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_break**](BreaksApi.md#create_break) | **POST** /v1/breaks | Start a break on one of your listings: pick (buyers choose a spot) or random (drawn). The seed is committed now and revealed at the draw.
[**create_break_cancel**](BreaksApi.md#create_break_cancel) | **POST** /v1/breaks/{id}/cancel | Cancel a break before any spot is paid for; after that, refund those orders instead (409).
[**create_break_draw**](BreaksApi.md#create_break_draw) | **POST** /v1/breaks/{id}/draw | Draw a sold-out random break: assigns spots from the committed seed and reveals it.
[**create_break_opened**](BreaksApi.md#create_break_opened) | **POST** /v1/breaks/{id}/opened | Mark the product opened on stream.
[**create_break_spot_hit**](BreaksApi.md#create_break_spot_hit) | **POST** /v1/breaks/{id}/spots/{spotId}/hits | Record what a spot pulled.
[**get_break**](BreaksApi.md#get_break) | **GET** /v1/breaks/{id} | A break’s board with every spot, its buyer and its hits.
[**list_breaks**](BreaksApi.md#list_breaks) | **GET** /v1/breaks | Your card breaks, newest first.



## create_break

> crate::models::CreateBreakResponse create_break()
Start a break on one of your listings: pick (buyers choose a spot) or random (drawn). The seed is committed now and revealed at the draw.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBreakResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_break_cancel

> crate::models::CreateBreakCancelResponse create_break_cancel(id)
Cancel a break before any spot is paid for; after that, refund those orders instead (409).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_break_draw

> crate::models::CreateBreakDrawResponse create_break_draw(id)
Draw a sold-out random break: assigns spots from the committed seed and reveals it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakDrawResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_break_opened

> crate::models::CreateBreakCancelResponse create_break_opened(id)
Mark the product opened on stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_break_spot_hit

> crate::models::CreateBreakCancelResponse create_break_spot_hit(id, spot_id)
Record what a spot pulled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**spot_id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_break

> crate::models::GetBreakResponse get_break(id)
A break’s board with every spot, its buyer and its hits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBreakResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_breaks

> crate::models::ListBreaksResponse list_breaks()
Your card breaks, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBreaksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

