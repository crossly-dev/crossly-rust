# \ToolboxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_toolbox_asset**](ToolboxApi.md#create_toolbox_asset) | **POST** /v1/toolbox/assets | Publish an asset (place nodes). Scanned (lint, text, credits, uploads) then reviewed. Free by default; paid needs Pro and a verified identity.
[**create_toolbox_asset_purchase**](ToolboxApi.md#create_toolbox_asset_purchase) | **POST** /v1/toolbox/assets/{assetId}/purchase | Open a Crossly checkout for a paid asset. Nothing is charged here: a person pays at checkoutUrl. The author earns 80% of net; the licence is use in your own experiences.
[**create_toolbox_asset_retire**](ToolboxApi.md#create_toolbox_asset_retire) | **POST** /v1/toolbox/assets/{assetId}/retire | Take an asset off the store for good. Licences already sold keep working.
[**create_toolbox_asset_use**](ToolboxApi.md#create_toolbox_asset_use) | **POST** /v1/toolbox/assets/{assetId}/use | Get the asset's nodes stamped with its id for a place you can edit; insert:true also inserts them into the place's live session. Paid assets need the experience owner's licence.
[**create_toolbox_asset_version**](ToolboxApi.md#create_toolbox_asset_version) | **POST** /v1/toolbox/assets/{assetId}/versions | A new version (reviewed); the live one keeps serving until it is approved.
[**create_toolbox_purchase_refund**](ToolboxApi.md#create_toolbox_purchase_refund) | **POST** /v1/toolbox/purchases/{purchaseId}/refund | Refund a licence within 14 days while no published version of yours uses the asset. Ends the licence; the author's share is reversed.
[**delete_toolbox_asset_rating_mine**](ToolboxApi.md#delete_toolbox_asset_rating_mine) | **DELETE** /v1/toolbox/assets/{assetId}/ratings/mine | Remove your rating of an asset.
[**get_toolbox_asset**](ToolboxApi.md#get_toolbox_asset) | **GET** /v1/toolbox/assets/{assetId} | One asset: versions, credits (assets it includes), licence, contents, your purchase.
[**get_toolbox_asset_rating**](ToolboxApi.md#get_toolbox_asset_rating) | **GET** /v1/toolbox/assets/{assetId}/ratings | Ratings of an asset: average stars, breakdown, recent reviews, yours, and whether you may rate it.
[**get_toolbox_purchase**](ToolboxApi.md#get_toolbox_purchase) | **GET** /v1/toolbox/purchases/{purchaseId} | A purchase: status, price, tax, refundable until.
[**list_toolbox_assets**](ToolboxApi.md#list_toolbox_assets) | **GET** /v1/toolbox/assets | Search the Toolbox: models, scripts, sounds, images, packages. Free and paid; `owned` says whether you may insert it now.
[**list_toolbox_library**](ToolboxApi.md#list_toolbox_library) | **GET** /v1/toolbox/library | Assets you may insert: licences you bought, your own, free ones you used.
[**list_toolbox_mine**](ToolboxApi.md#list_toolbox_mine) | **GET** /v1/toolbox/mine | Your published assets: review state, reviewer note, what each earned you.
[**list_toolbox_sales**](ToolboxApi.md#list_toolbox_sales) | **GET** /v1/toolbox/sales | Your Toolbox sales: gross, net, your 80% share after refunds. Never who bought.
[**update_toolbox_asset**](ToolboxApi.md#update_toolbox_asset) | **PATCH** /v1/toolbox/assets/{assetId} | Edit name, description, tags, thumbnail or price.
[**update_toolbox_asset_rating_mine**](ToolboxApi.md#update_toolbox_asset_rating_mine) | **PUT** /v1/toolbox/assets/{assetId}/ratings/mine | Rate an asset you bought or installed: 1–5 stars and an optional review (1000 characters). Not your own.



## create_toolbox_asset

> crate::models::ToolboxAssetResponse create_toolbox_asset()
Publish an asset (place nodes). Scanned (lint, text, credits, uploads) then reviewed. Free by default; paid needs Pro and a verified identity.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ToolboxAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_toolbox_asset_purchase

> crate::models::GetToolboxPurchaseResponse create_toolbox_asset_purchase(asset_id)
Open a Crossly checkout for a paid asset. Nothing is charged here: a person pays at checkoutUrl. The author earns 80% of net; the licence is use in your own experiences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::GetToolboxPurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_toolbox_asset_retire

> crate::models::ToolboxAssetResponse create_toolbox_asset_retire(asset_id)
Take an asset off the store for good. Licences already sold keep working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::ToolboxAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_toolbox_asset_use

> crate::models::CreateToolboxAssetUseResponse create_toolbox_asset_use(asset_id)
Get the asset's nodes stamped with its id for a place you can edit; insert:true also inserts them into the place's live session. Paid assets need the experience owner's licence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::CreateToolboxAssetUseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_toolbox_asset_version

> crate::models::ToolboxAssetResponse create_toolbox_asset_version(asset_id)
A new version (reviewed); the live one keeps serving until it is approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::ToolboxAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_toolbox_purchase_refund

> crate::models::CreateWorldMeReceiptRefundResponse create_toolbox_purchase_refund(purchase_id)
Refund a licence within 14 days while no published version of yours uses the asset. Ends the licence; the author's share is reversed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldMeReceiptRefundResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_toolbox_asset_rating_mine

> crate::models::DeleteSpatialSceneGrantResponse delete_toolbox_asset_rating_mine(asset_id)
Remove your rating of an asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_toolbox_asset

> crate::models::GetToolboxAssetResponse get_toolbox_asset(asset_id)
One asset: versions, credits (assets it includes), licence, contents, your purchase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::GetToolboxAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_toolbox_asset_rating

> crate::models::GetToolboxAssetRatingResponse get_toolbox_asset_rating(asset_id)
Ratings of an asset: average stars, breakdown, recent reviews, yours, and whether you may rate it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::GetToolboxAssetRatingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_toolbox_purchase

> crate::models::GetToolboxPurchaseResponse get_toolbox_purchase(purchase_id)
A purchase: status, price, tax, refundable until.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_id** | **String** |  | [required] |

### Return type

**crate::models::GetToolboxPurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_toolbox_assets

> crate::models::ListToolboxAssetsResponse list_toolbox_assets()
Search the Toolbox: models, scripts, sounds, images, packages. Free and paid; `owned` says whether you may insert it now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListToolboxAssetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_toolbox_library

> crate::models::ListToolboxLibraryResponse list_toolbox_library()
Assets you may insert: licences you bought, your own, free ones you used.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListToolboxLibraryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_toolbox_mine

> crate::models::ListToolboxMineResponse list_toolbox_mine()
Your published assets: review state, reviewer note, what each earned you.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListToolboxMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_toolbox_sales

> crate::models::ListToolboxSalesResponse list_toolbox_sales()
Your Toolbox sales: gross, net, your 80% share after refunds. Never who bought.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListToolboxSalesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_toolbox_asset

> crate::models::ToolboxAssetResponse update_toolbox_asset(asset_id)
Edit name, description, tags, thumbnail or price.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::ToolboxAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_toolbox_asset_rating_mine

> crate::models::UpdateToolboxAssetRatingMineResponse update_toolbox_asset_rating_mine(asset_id)
Rate an asset you bought or installed: 1–5 stars and an optional review (1000 characters). Not your own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateToolboxAssetRatingMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

