# \DefaultApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cosm_designer_chain_auto_unlock**](DefaultApi.md#create_cosm_designer_chain_auto_unlock) | **POST** /v1/cosm/designer/chain-auto-unlock | 
[**create_cosm_designer_chain_unlock**](DefaultApi.md#create_cosm_designer_chain_unlock) | **POST** /v1/cosm/designer/chain-unlock | 
[**create_cosm_designer_materialise**](DefaultApi.md#create_cosm_designer_materialise) | **POST** /v1/cosm/designer/materialise | 
[**create_cosm_designer_materialise_confirm**](DefaultApi.md#create_cosm_designer_materialise_confirm) | **POST** /v1/cosm/designer/materialise/confirm | 
[**create_cosm_designer_materialise_release**](DefaultApi.md#create_cosm_designer_materialise_release) | **POST** /v1/cosm/designer/materialise/release | 
[**create_cosm_designer_submit_for_review**](DefaultApi.md#create_cosm_designer_submit_for_review) | **POST** /v1/cosm/designer/submit-for-review | 
[**create_cosm_designer_term**](DefaultApi.md#create_cosm_designer_term) | **POST** /v1/cosm/designer/terms | 
[**get_cosm_designer_copy_state**](DefaultApi.md#get_cosm_designer_copy_state) | **GET** /v1/cosm/designer/copy-state | 
[**get_cosm_designer_earning**](DefaultApi.md#get_cosm_designer_earning) | **GET** /v1/cosm/designer/earnings | 
[**get_cosm_designer_term**](DefaultApi.md#get_cosm_designer_term) | **GET** /v1/cosm/designer/terms | 
[**get_listing_variation_group**](DefaultApi.md#get_listing_variation_group) | **GET** /v1/listings/{id}/variation-group | 
[**get_market_metal_spot_history**](DefaultApi.md#get_market_metal_spot_history) | **GET** /v1/market/metals/spot/history | 
[**get_market_product**](DefaultApi.md#get_market_product) | **GET** /v1/market/products/{id} | 
[**get_market_product_related**](DefaultApi.md#get_market_product_related) | **GET** /v1/market/products/{id}/related | 
[**get_market_scrap_quote**](DefaultApi.md#get_market_scrap_quote) | **GET** /v1/market/scrap/quote | 
[**get_market_variant_book**](DefaultApi.md#get_market_variant_book) | **GET** /v1/market/variants/{id}/book | 
[**get_variation_group**](DefaultApi.md#get_variation_group) | **GET** /v1/variation-groups/{id} | 
[**get_variation_group_publish_plan**](DefaultApi.md#get_variation_group_publish_plan) | **GET** /v1/variation-groups/{id}/publish-plan | 
[**list_market_bullion**](DefaultApi.md#list_market_bullion) | **GET** /v1/market/bullion | 
[**list_market_graders**](DefaultApi.md#list_market_graders) | **GET** /v1/market/graders | 
[**list_market_metal_spot**](DefaultApi.md#list_market_metal_spot) | **GET** /v1/market/metals/spot | 
[**list_market_products**](DefaultApi.md#list_market_products) | **GET** /v1/market/products | 
[**list_market_variant_tiers**](DefaultApi.md#list_market_variant_tiers) | **GET** /v1/market/variants/{id}/tiers | 
[**list_variation_groups**](DefaultApi.md#list_variation_groups) | **GET** /v1/variation-groups | 



## create_cosm_designer_chain_auto_unlock

> crate::models::CreateCosmDesignerChainAutoUnlockResponse create_cosm_designer_chain_auto_unlock(request_body)


Opt a design in or out of the free chain unlock a popular limited design earns (on by default). Designer-only. Answers the popularity of the design toward the bar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerChainAutoUnlockResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_chain_unlock

> crate::models::CreateCosmDesignerChainUnlockResponse create_cosm_designer_chain_unlock(request_body)


Burn COSM to make a design mintable on chain. Designer-only and idempotent: paying twice is impossible, and a second call returns the existing unlock without charging. The fee is DESTROYED rather than collected. Limited editions only. Once unlocked, any holder may take their own copy on chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerChainUnlockResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_materialise

> crate::models::CreateCosmDesignerMaterialiseResponse create_cosm_designer_materialise(request_body)


Lock a copy and return an UNSIGNED Metaplex Core mint for the holder to sign. One call: it resolves the designer royalty, locks the copy, publishes immutable content-addressed metadata and builds the transaction — so a lock is never taken without a transaction to go with it, and a failure after the lock releases it. Does NOT void the copy; call confirm once the chain reaches FINALIZED commitment. Every royalty recipient needs a connected wallet first, and the designer must have unlocked the design.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerMaterialiseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_materialise_confirm

> crate::models::CreateCosmDesignerMaterialiseConfirmResponse create_cosm_designer_materialise_confirm(request_body)


Record a confirmed mint and void the in-world copy, in one transaction. The caller MUST have verified the transaction at finalized commitment first — an NFT recorded against a transaction that never landed cannot be recalled, and the copy it voided is gone. Idempotent on an already-confirmed request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerMaterialiseConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_materialise_release

> crate::models::CreateWorldJobShiftReleaseResponse create_cosm_designer_materialise_release(request_body)


Release a mint lock that was never signed. The ordinary outcome of somebody closing a wallet prompt, and it leaves the copy exactly as it was. Refuses a request that has already minted, because releasing one would unlock a copy that is on chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateWorldJobShiftReleaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_submit_for_review

> crate::models::CreateCosmDesignerSubmitForReviewResponse create_cosm_designer_submit_for_review(request_body)


Submit a user-authored design for moderation. Designer-only, and refuses a design with no preview image HERE rather than at approval so the designer learns what is missing while they can still fix it. Submitting twice returns the existing pending review instead of queueing a second one — a moderator facing two reviews of one piece does not know which they are deciding. A design already on sale is refused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerSubmitForReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_cosm_designer_term

> crate::models::CreateCosmDesignerTermResponse create_cosm_designer_term(request_body)


Publish terms for a design. Never edits: every change is a new version, and a copy sold under an earlier one keeps those terms forever. Only the designer may call it. Collaborator shares are of the ROYALTY rather than the sale price and may total at most 100%, with the remainder staying with the designer; a total above 100% is refused rather than scaled down.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | **::std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerTermResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_cosm_designer_copy_state

> crate::models::GetCosmDesignerCopyStateResponse get_cosm_designer_copy_state(ownership_id)


Where a copy currently is: held (in-world, wearable, listable for COSM), minting (locked while a mint is in flight) or materialised (an NFT in the holder wallet). A copy is in exactly one of these, never two.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ownership_id** | **String** |  | [required] |

### Return type

**crate::models::GetCosmDesignerCopyStateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_cosm_designer_earning

> crate::models::GetCosmDesignerEarningResponse get_cosm_designer_earning(subject_id)


What a designer has earned, in both denominations. COSM royalties come from COSM-priced sales; cash earnings come from cash-priced sales and are funded by the buyer payment itself. The two are never converted into one another, and COSM is never cashed out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

**crate::models::GetCosmDesignerEarningResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_cosm_designer_term

> crate::models::GetCosmDesignerTermResponse get_cosm_designer_term(cosmetic_id)


The published terms for a design: resale, price window, secondary royalty and collaborator splits. Also reports the platform rates a designer does not control, and whether the terms are enforceable on chain — inside Crossly every rule is enforced, but a third-party marketplace honours a royalty voluntarily at best.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosmetic_id** | **String** |  | [required] |

### Return type

**crate::models::GetCosmDesignerTermResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_variation_group

> crate::models::GetListingVariationGroupResponse get_listing_variation_group(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetListingVariationGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_market_metal_spot_history

> crate::models::GetMarketMetalSpotHistoryResponse get_market_metal_spot_history()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMarketMetalSpotHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_market_product

> crate::models::GetMarketProductResponse get_market_product(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetMarketProductResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_market_product_related

> crate::models::GetMarketProductRelatedResponse get_market_product_related(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetMarketProductRelatedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_market_scrap_quote

> crate::models::GetMarketScrapQuoteResponse get_market_scrap_quote()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMarketScrapQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_market_variant_book

> crate::models::GetMarketVariantBookResponse get_market_variant_book(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetMarketVariantBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_variation_group

> crate::models::ListVariationGroupsItem get_variation_group(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVariationGroupsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_variation_group_publish_plan

> crate::models::GetVariationGroupPublishPlanResponse get_variation_group_publish_plan(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVariationGroupPublishPlanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_market_bullion

> crate::models::ListMarketBullionResponse list_market_bullion()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMarketBullionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_market_graders

> crate::models::ListMarketGradersResponse list_market_graders()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMarketGradersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_market_metal_spot

> crate::models::ListMarketMetalSpotResponse list_market_metal_spot()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMarketMetalSpotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_market_products

> crate::models::ListMarketProductsResponse list_market_products()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMarketProductsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_market_variant_tiers

> crate::models::ListMarketVariantTiersResponse list_market_variant_tiers(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListMarketVariantTiersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_variation_groups

> crate::models::ListVariationGroupsResponse list_variation_groups()


### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVariationGroupsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

