# \OffersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_offer_respond**](OffersApi.md#create_offer_respond) | **POST** /v1/offers/{id}/respond | Accept, decline, or counter a buyer offer on a Crossly marketplace listing.
[**get_offer**](OffersApi.md#get_offer) | **GET** /v1/offers | List buyer offers on your Crossly marketplace listings, including bundles.



## create_offer_respond

> crate::models::CreateOfferRespondResponse create_offer_respond(id, inline_object2)
Accept, decline, or counter a buyer offer on a Crossly marketplace listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Offer UUID. | [required] |
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

### Return type

[**crate::models::CreateOfferRespondResponse**](CreateOfferRespondResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_offer

> crate::models::GetOfferResponse get_offer(status, limit)
List buyer offers on your Crossly marketplace listings, including bundles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter to one status. Omit for all. |  |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**crate::models::GetOfferResponse**](GetOfferResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

