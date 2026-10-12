# \NFTMarketApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nft_market_external_resale**](NFTMarketApi.md#create_nft_market_external_resale) | **POST** /v1/nft-market/external-resale | Ops: record a Crossly NFT resold on ANY marketplace — maps mint → design and funds the pot. Admin PAT only.
[**create_nft_market_listing**](NFTMarketApi.md#create_nft_market_listing) | **POST** /v1/nft-market/listings | List an NFT you own for resale on the Crossly marketplace.
[**create_nft_market_listing_cancel**](NFTMarketApi.md#create_nft_market_listing_cancel) | **POST** /v1/nft-market/listings/{id}/cancel | Cancel one of your open NFT listings.
[**create_nft_market_listing_confirm_sale**](NFTMarketApi.md#create_nft_market_listing_confirm_sale) | **POST** /v1/nft-market/listings/{id}/confirm-sale | Ops: record a finalized on-Crossly NFT sale (verifies the royalty landed, funds the pot). Admin PAT only.
[**get_nft_market_listing**](NFTMarketApi.md#get_nft_market_listing) | **GET** /v1/nft-market/listings | Browse open on-platform NFT listings (Crossly cosmetics for resale in CBX/SOL).



## create_nft_market_external_resale

> crate::models::CreateNftMarketExternalResaleResponse create_nft_market_external_resale()
Ops: record a Crossly NFT resold on ANY marketplace — maps mint → design and funds the pot. Admin PAT only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateNftMarketExternalResaleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_nft_market_listing

> crate::models::CreateNftMarketListingResponse create_nft_market_listing()
List an NFT you own for resale on the Crossly marketplace.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateNftMarketListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_nft_market_listing_cancel

> crate::models::OkResponse create_nft_market_listing_cancel(id)
Cancel one of your open NFT listings.

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


## create_nft_market_listing_confirm_sale

> crate::models::OkResponse create_nft_market_listing_confirm_sale(id)
Ops: record a finalized on-Crossly NFT sale (verifies the royalty landed, funds the pot). Admin PAT only.

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


## get_nft_market_listing

> crate::models::GetNftMarketListingResponse get_nft_market_listing()
Browse open on-platform NFT listings (Crossly cosmetics for resale in CBX/SOL).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetNftMarketListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

