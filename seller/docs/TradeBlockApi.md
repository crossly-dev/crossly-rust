# \TradeBlockApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_trade_block_evolution_claim**](TradeBlockApi.md#create_trade_block_evolution_claim) | **POST** /v1/trade-block/evolutions/{id}/claim | Burn COSM and mint the evolved (or set-master) NFT for one of your unlocks.
[**create_trade_block_evolution_recipe**](TradeBlockApi.md#create_trade_block_evolution_recipe) | **POST** /v1/trade-block/evolution-recipes | Author a Trade Evolution recipe (a graded item or pair that, when swapped, unlocks a cosmetic you designed). Born unreviewed until ops approves.
[**create_trade_block_offer_respond**](TradeBlockApi.md#create_trade_block_offer_respond) | **POST** /v1/trade-block/offers/{id}/respond | Accept, decline, counter, or withdraw a Trade Block offer.
[**create_trade_block_post_close**](TradeBlockApi.md#create_trade_block_post_close) | **POST** /v1/trade-block/posts/{id}/close | Close one of your posts so it stops taking offers.
[**create_trade_block_post_offer**](TradeBlockApi.md#create_trade_block_post_offer) | **POST** /v1/trade-block/posts/{id}/offers | Respond to a post with a concrete swap: items on each side (0..N), optional cash.
[**create_trade_block_set_recipe**](TradeBlockApi.md#create_trade_block_set_recipe) | **POST** /v1/trade-block/set-recipes | Author a set-completion recipe (a set of items that, once assembled through trading, unlocks a set-master cosmetic you designed). Born unreviewed until ops approves.
[**get_trade_block_evolution**](TradeBlockApi.md#get_trade_block_evolution) | **GET** /v1/trade-block/evolutions | List your Trade Evolution unlocks — completed swaps that unlocked an evolved or set-master NFT you can mint by burning COSM.
[**get_trade_block_post**](TradeBlockApi.md#get_trade_block_post) | **GET** /v1/trade-block/posts | Browse open Trade Block posts (wants + offers) on the Crossly Market.
[**get_trade_block_post_by_id**](TradeBlockApi.md#get_trade_block_post_by_id) | **GET** /v1/trade-block/posts/{id} | Get one Trade Block post with its items and open-offer count.
[**get_trade_block_trade**](TradeBlockApi.md#get_trade_block_trade) | **GET** /v1/trade-block/trades | List the trades you are a party to, newest first.



## create_trade_block_evolution_claim

> crate::models::CreateTradeBlockEvolutionClaimResponse create_trade_block_evolution_claim(id)
Burn COSM and mint the evolved (or set-master) NFT for one of your unlocks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateTradeBlockEvolutionClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_trade_block_evolution_recipe

> crate::models::CreateTradeBlockEvolutionRecipeResponse create_trade_block_evolution_recipe()
Author a Trade Evolution recipe (a graded item or pair that, when swapped, unlocks a cosmetic you designed). Born unreviewed until ops approves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateTradeBlockEvolutionRecipeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_trade_block_offer_respond

> crate::models::CreateTradeBlockOfferRespondResponse create_trade_block_offer_respond(id)
Accept, decline, counter, or withdraw a Trade Block offer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateTradeBlockOfferRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_trade_block_post_close

> crate::models::OkResponse create_trade_block_post_close(id)
Close one of your posts so it stops taking offers.

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


## create_trade_block_post_offer

> crate::models::IdResponse create_trade_block_post_offer(id)
Respond to a post with a concrete swap: items on each side (0..N), optional cash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_trade_block_set_recipe

> crate::models::CreateTradeBlockSetRecipeResponse create_trade_block_set_recipe()
Author a set-completion recipe (a set of items that, once assembled through trading, unlocks a set-master cosmetic you designed). Born unreviewed until ops approves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateTradeBlockSetRecipeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_trade_block_evolution

> crate::models::GetTradeBlockEvolutionResponse get_trade_block_evolution()
List your Trade Evolution unlocks — completed swaps that unlocked an evolved or set-master NFT you can mint by burning COSM.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetTradeBlockEvolutionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_trade_block_post

> crate::models::GetTradeBlockPostResponse get_trade_block_post(kind, category_slug, brand, model, q, limit, offset)
Browse open Trade Block posts (wants + offers) on the Crossly Market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | Option<**String**> | Filter to wants or offers. |  |
**category_slug** | Option<**String**> | Filter to a market category. |  |
**brand** | Option<**String**> | Filter to posts with a product of this brand (case-insensitive). |  |
**model** | Option<**String**> | Filter to posts with a product of this catalogue model (case-insensitive). |  |
**q** | Option<**String**> | Search title + want text. |  |
**limit** | Option<**i32**> |  |  |[default to 40]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

**crate::models::GetTradeBlockPostResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_trade_block_post_by_id

> crate::models::GetTradeBlockPostByIdResponse get_trade_block_post_by_id(id)
Get one Trade Block post with its items and open-offer count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTradeBlockPostByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_trade_block_trade

> crate::models::GetTradeBlockTradeResponse get_trade_block_trade()
List the trades you are a party to, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetTradeBlockTradeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

