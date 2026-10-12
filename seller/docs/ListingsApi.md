# \ListingsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ai_detection_dispute**](ListingsApi.md#create_ai_detection_dispute) | **POST** /v1/ai-detections/{id}/dispute | Dispute an AI-image detection on your photo (sends it to Crossly review).
[**create_listing**](ListingsApi.md#create_listing) | **POST** /v1/listings | Create a listing and fan out crosspost jobs across platforms.
[**create_listing_bulk_check_status**](ListingsApi.md#create_listing_bulk_check_status) | **POST** /v1/listings/bulk-check-status | Check listing status on platforms
[**create_listing_bulk_crosspost**](ListingsApi.md#create_listing_bulk_crosspost) | **POST** /v1/listings/bulk-crosspost | Bulk crosspost (no delist phase)
[**create_listing_bulk_delete**](ListingsApi.md#create_listing_bulk_delete) | **POST** /v1/listings/bulk-delete | Bulk archive + delist
[**create_listing_bulk_delist**](ListingsApi.md#create_listing_bulk_delist) | **POST** /v1/listings/bulk-delist | Bulk delist from platforms
[**create_listing_bulk_delist_preview**](ListingsApi.md#create_listing_bulk_delist_preview) | **POST** /v1/listings/bulk-delist-preview | Preview which marketplaces a delist would touch
[**create_listing_bulk_hard_delete**](ListingsApi.md#create_listing_bulk_hard_delete) | **POST** /v1/listings/bulk-hard-delete | Permanently delete listings (live ones need allowLive)
[**create_listing_bulk_relist**](ListingsApi.md#create_listing_bulk_relist) | **POST** /v1/listings/bulk-relist | Bulk relist across platforms
[**create_listing_bulk_update**](ListingsApi.md#create_listing_bulk_update) | **POST** /v1/listings/bulk-update | Bulk update listing fields
[**create_listing_by_id**](ListingsApi.md#create_listing_by_id) | **POST** /v1/listings/by-ids | Fetch hydrated listings by ID
[**create_listing_check_duplicate**](ListingsApi.md#create_listing_check_duplicate) | **POST** /v1/listings/check-duplicates | Check whether the seller already owns something matching this title/photo, and what to do about it.
[**create_listing_combine**](ListingsApi.md#create_listing_combine) | **POST** /v1/listings/combine | Combine duplicate listings into one: sums their stock, delists and archives the rest.
[**create_listing_discrepancy_resolve**](ListingsApi.md#create_listing_discrepancy_resolve) | **POST** /v1/listings/{id}/discrepancies/{discrepancyId}/resolve | Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.
[**create_listing_import_by_url**](ListingsApi.md#create_listing_import_by_url) | **POST** /v1/listings/{id}/import-by-url | Attach a real platform listing to this listing by pasting its live URL.
[**create_listing_magic_fill**](ListingsApi.md#create_listing_magic_fill) | **POST** /v1/listings/{id}/magic-fill | Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.
[**create_listing_merge**](ListingsApi.md#create_listing_merge) | **POST** /v1/listings/merge | Merge the same item listed on different marketplaces into one listing; all stay live.
[**create_live_stream_pin_launch**](ListingsApi.md#create_live_stream_pin_launch) | **POST** /v1/live-streams/{id}/pin-launch | Pin one of your live NFT launches on your live show; viewers mint it from the stream. Returns `{ ok }`.
[**create_passport**](ListingsApi.md#create_passport) | **POST** /v1/passports | Register a product passport for a Crossly listing you sell, optionally bound to an NFC tag UID (hex). Returns the URL to write to the NFC tag or print as a QR; ownership then follows the item’s paid sales.
[**create_passport_revoke**](ListingsApi.md#create_passport_revoke) | **POST** /v1/passports/{id}/revoke | Revoke a passport (a lost or misprinted tag). Refused once the item has sold — the passport is then part of its history.
[**create_passport_tag**](ListingsApi.md#create_passport_tag) | **POST** /v1/passports/{id}/tag | Bind (or re-bind) a passport to the NFC tag you wrote its nfcUrl to: the tag UID (hex), and whether you locked the tag read-only. Refused once the item has sold, for a locked tag, or a tag already on another item.
[**create_variation_group**](ListingsApi.md#create_variation_group) | **POST** /v1/variation-groups | Group listings you already have: a variation (one product, an axis such as Size) or a purchase-rule group (\"buy any 3\").
[**create_variation_group_generate**](ListingsApi.md#create_variation_group_generate) | **POST** /v1/variation-groups/generate | Turn one listing into a variation group: each new value becomes its own listing (a copy, with its own price, quantity and SKU) and all of them one group.
[**create_variation_group_publish**](ListingsApi.md#create_variation_group_publish) | **POST** /v1/variation-groups/{id}/publish | Publish a variation group per platform: one listing with a variation picker where the platform supports it (eBay, Shopify, Etsy), one per option elsewhere. Each platform reports its own outcome.
[**delete_listing**](ListingsApi.md#delete_listing) | **DELETE** /v1/listings/{id} | Delist a listing (optionally narrowed to specific platforms via ?platforms=).
[**delete_live_stream_pin_launch**](ListingsApi.md#delete_live_stream_pin_launch) | **DELETE** /v1/live-streams/{id}/pin-launch | Unpin the launch from your live show. Returns `{ ok }`.
[**delete_rental_offer**](ListingsApi.md#delete_rental_offer) | **DELETE** /v1/rentals/offers/{platformListingId} | Stop offering a listing for rent (existing bookings stand).
[**delete_variation_group**](ListingsApi.md#delete_variation_group) | **DELETE** /v1/variation-groups/{id} | Dissolve a group. Its options stay as ordinary listings.
[**get_listing**](ListingsApi.md#get_listing) | **GET** /v1/listings/{id} | Get one listing with its platform rows.
[**get_listing_facet**](ListingsApi.md#get_listing_facet) | **GET** /v1/listings/facets | Distinct brands + categories across listings + inventory.
[**get_listing_sku_exist**](ListingsApi.md#get_listing_sku_exist) | **GET** /v1/listings/sku-exists | Check whether a SKU is already used by one of this user's items.
[**get_pricing_setting**](ListingsApi.md#get_pricing_setting) | **GET** /v1/pricing/settings | Selling options: own-price match (Crossly sells your items at your own lower price for the same item on another marketplace; dollar-priced copies only) and layaway (buyers may put items on layaway). Both on by default.
[**get_pricing_sold_comp**](ListingsApi.md#get_pricing_sold_comp) | **GET** /v1/pricing/sold-comps | What one catalog product (market_sku_id, in one condition) actually sold for across Crossly in the last 180 days: median, p25/p75, p10/p90 (null under 10 sales), sale count, last sold, and days from listing to sale (median, p25, p75). Pass marketSkuId (+ optional condition) or one of your listingIds. comps is null until there are 3+ sales from 2+ sellers; holdCode says why there is nothing (no_market_sku, mixed_inventory, below_privacy_floor) — the same reasons a sold_median reprice rule holds.
[**list_listing_ai_detections**](ListingsApi.md#list_listing_ai_detections) | **GET** /v1/listings/{id}/ai-detections | Crossly's AI-image detections for a listing's photos.
[**list_listing_discrepancies**](ListingsApi.md#list_listing_discrepancies) | **GET** /v1/listings/{id}/discrepancies | List detected marketplace-drift discrepancies for a listing.
[**list_listing_ids**](ListingsApi.md#list_listing_ids) | **GET** /v1/listings/ids | Filter listings → return matching id list (no pagination).
[**list_listings**](ListingsApi.md#list_listings) | **GET** /v1/listings | List active platform listings.
[**list_passports**](ListingsApi.md#list_passports) | **GET** /v1/passports | Product passports you registered (optionally for one Crossly listing): the code and URL each tag carries.
[**list_rental_offers**](ListingsApi.md#list_rental_offers) | **GET** /v1/rentals/offers | What you offer for rent.
[**update_hotspot**](ListingsApi.md#update_hotspot) | **PUT** /v1/hotspots | Tag your listings at points on a story or lookbook photo (shop the look). Replaces that photo’s tags; at most 12.
[**update_listing**](ListingsApi.md#update_listing) | **PATCH** /v1/listings/{id} | Edit a listing and fan out update jobs to existing platform listings.
[**update_listing_custom**](ListingsApi.md#update_listing_custom) | **PUT** /v1/listings/{id}/customs | Set a listing’s customs: HS code (6–10 digits), country of origin, and countries this item must not ship to on Crossly. Omitted fields are left alone; null (or [] for restrictedCountries) clears.
[**update_listing_platform_listing**](ListingsApi.md#update_listing_platform_listing) | **PATCH** /v1/listings/{id}/platform-listings/{plId} | Set one marketplace copy's price (e.g. the ebay.com copy of a listing also live on ebay.co.uk), in that copy's own currency.
[**update_live_stream_lineup**](ListingsApi.md#update_live_stream_lineup) | **PUT** /v1/live-streams/{id}/lineup | Set a live show’s lineup: your Crossly listings, in the order you will sell them. Buyers see it before they join; a sale of a lineup item while the show is live is recorded at its replay moment.
[**update_pricing_setting**](ListingsApi.md#update_pricing_setting) | **PUT** /v1/pricing/settings | Turn own-price match and/or layaway on or off.
[**update_rental_offer**](ListingsApi.md#update_rental_offer) | **PUT** /v1/rentals/offers/{platformListingId} | Offer a Crossly listing for rent: day rate, optional week rate, deposit, min/max days, turnaround buffer, and rent-only (off sale while offered). Needs a ship-from address.
[**update_variation_group**](ListingsApi.md#update_variation_group) | **PATCH** /v1/variation-groups/{id} | Retitle a group, rename its axis, or change a group's purchase rule (written into every member's description).



## create_ai_detection_dispute

> crate::models::CreateAiDetectionDisputeResponse create_ai_detection_dispute(id)
Dispute an AI-image detection on your photo (sends it to Crossly review).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateAiDetectionDisputeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing

> crate::models::CreateListingResponse create_listing()
Create a listing and fan out crosspost jobs across platforms.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_check_status

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_check_status()
Check listing status on platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_crosspost

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_crosspost()
Bulk crosspost (no delist phase)

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delete

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_delete()
Bulk archive + delist

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delist

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_delist()
Bulk delist from platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delist_preview

> crate::models::CreateListingBulkDelistPreviewResponse create_listing_bulk_delist_preview()
Preview which marketplaces a delist would touch

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDelistPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_hard_delete

> crate::models::CreateListingBulkHardDeleteResponse create_listing_bulk_hard_delete()
Permanently delete listings (live ones need allowLive)

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkHardDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_relist

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_relist()
Bulk relist across platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_update

> crate::models::CreateListingBulkUpdateResponse create_listing_bulk_update()
Bulk update listing fields

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkUpdateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_by_id

> crate::models::CreateListingByIdResponse create_listing_by_id()
Fetch hydrated listings by ID

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_check_duplicate

> crate::models::CreateListingCheckDuplicateResponse create_listing_check_duplicate()
Check whether the seller already owns something matching this title/photo, and what to do about it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingCheckDuplicateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_combine

> crate::models::CreateListingCombineResponse create_listing_combine()
Combine duplicate listings into one: sums their stock, delists and archives the rest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingCombineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_discrepancy_resolve

> crate::models::CreateListingDiscrepancyResolveResponse create_listing_discrepancy_resolve(id, discrepancy_id)
Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**discrepancy_id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingDiscrepancyResolveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_import_by_url

> crate::models::CreateListingImportByUrlResponse create_listing_import_by_url(id)
Attach a real platform listing to this listing by pasting its live URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingImportByUrlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_magic_fill

> crate::models::CreateListingMagicFillResponse create_listing_magic_fill(id)
Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingMagicFillResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_merge

> crate::models::CreateListingMergeResponse create_listing_merge()
Merge the same item listed on different marketplaces into one listing; all stay live.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingMergeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream_pin_launch

> crate::models::OkResponse create_live_stream_pin_launch(id)
Pin one of your live NFT launches on your live show; viewers mint it from the stream. Returns `{ ok }`.

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


## create_passport

> crate::models::PassportResponse create_passport()
Register a product passport for a Crossly listing you sell, optionally bound to an NFC tag UID (hex). Returns the URL to write to the NFC tag or print as a QR; ownership then follows the item’s paid sales.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_revoke

> crate::models::PassportResponse create_passport_revoke(id)
Revoke a passport (a lost or misprinted tag). Refused once the item has sold — the passport is then part of its history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_tag

> crate::models::PassportResponse create_passport_tag(id)
Bind (or re-bind) a passport to the NFC tag you wrote its nfcUrl to: the tag UID (hex), and whether you locked the tag read-only. Refused once the item has sold, for a locked tag, or a tag already on another item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group

> crate::models::CreateVariationGroupResponse create_variation_group()
Group listings you already have: a variation (one product, an axis such as Size) or a purchase-rule group (\"buy any 3\").

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVariationGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group_generate

> crate::models::CreateVariationGroupGenerateResponse create_variation_group_generate()
Turn one listing into a variation group: each new value becomes its own listing (a copy, with its own price, quantity and SKU) and all of them one group.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVariationGroupGenerateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group_publish

> crate::models::CreateVariationGroupPublishResponse create_variation_group_publish(id)
Publish a variation group per platform: one listing with a variation picker where the platform supports it (eBay, Shopify, Etsy), one per option elsewhere. Each platform reports its own outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVariationGroupPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_listing

> crate::models::DeleteListingResponse delete_listing(id, platforms, platform_listing_ids)
Delist a listing (optionally narrowed to specific platforms via ?platforms=).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platforms** | Option<**String**> | Comma-separated platform slugs to limit the delist fan-out. |  |
**platform_listing_ids** | Option<**String**> | Comma-separated platform listing ids — end only those copies (e.g. one eBay site's copy). |  |

### Return type

**crate::models::DeleteListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_live_stream_pin_launch

> crate::models::OkResponse delete_live_stream_pin_launch(id)
Unpin the launch from your live show. Returns `{ ok }`.

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


## delete_rental_offer

> crate::models::CreateBreakCancelResponse delete_rental_offer(platform_listing_id)
Stop offering a listing for rent (existing bookings stand).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_variation_group

> crate::models::CreateBreakCancelResponse delete_variation_group(id)
Dissolve a group. Its options stay as ordinary listings.

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


## get_listing

> crate::models::GetListingResponse get_listing(id)
Get one listing with its platform rows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_facet

> crate::models::GetListingFacetResponse get_listing_facet()
Distinct brands + categories across listings + inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetListingFacetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_sku_exist

> crate::models::GetListingSkuExistResponse get_listing_sku_exist(sku)
Check whether a SKU is already used by one of this user's items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sku** | **String** |  | [required] |

### Return type

**crate::models::GetListingSkuExistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pricing_setting

> crate::models::GetPricingSettingResponse get_pricing_setting()
Selling options: own-price match (Crossly sells your items at your own lower price for the same item on another marketplace; dollar-priced copies only) and layaway (buyers may put items on layaway). Both on by default.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pricing_sold_comp

> crate::models::GetPricingSoldCompResponse get_pricing_sold_comp()
What one catalog product (market_sku_id, in one condition) actually sold for across Crossly in the last 180 days: median, p25/p75, p10/p90 (null under 10 sales), sale count, last sold, and days from listing to sale (median, p25, p75). Pass marketSkuId (+ optional condition) or one of your listingIds. comps is null until there are 3+ sales from 2+ sellers; holdCode says why there is nothing (no_market_sku, mixed_inventory, below_privacy_floor) — the same reasons a sold_median reprice rule holds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSoldCompResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ai_detections

> crate::models::ListListingAiDetectionsResponse list_listing_ai_detections(id)
Crossly's AI-image detections for a listing's photos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingAiDetectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_discrepancies

> crate::models::ListListingDiscrepanciesResponse list_listing_discrepancies(id)
List detected marketplace-drift discrepancies for a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingDiscrepanciesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ids

> crate::models::ListListingIdsResponse list_listing_ids()
Filter listings → return matching id list (no pagination).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListListingIdsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listings

> crate::models::ListListingsResponse list_listings(page, limit, platform, status, kind)
List active platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**platform** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**kind** | Option<**String**> | Physical goods or Web3 assets (NFT listings). Omit for both. |  |

### Return type

**crate::models::ListListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_passports

> crate::models::ListPassportsResponse list_passports()
Product passports you registered (optionally for one Crossly listing): the code and URL each tag carries.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPassportsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_rental_offers

> crate::models::ListRentalOffersResponse list_rental_offers()
What you offer for rent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListRentalOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_hotspot

> crate::models::UpdateHotspotResponse update_hotspot()
Tag your listings at points on a story or lookbook photo (shop the look). Replaces that photo’s tags; at most 12.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateHotspotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing

> crate::models::UpdateListingResponse update_listing(id)
Edit a listing and fan out update jobs to existing platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_custom

> crate::models::UpdateListingCustomResponse update_listing_custom(id)
Set a listing’s customs: HS code (6–10 digits), country of origin, and countries this item must not ship to on Crossly. Omitted fields are left alone; null (or [] for restrictedCountries) clears.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingCustomResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_platform_listing

> crate::models::UpdateListingPlatformListingResponse update_listing_platform_listing(id, pl_id)
Set one marketplace copy's price (e.g. the ebay.com copy of a listing also live on ebay.co.uk), in that copy's own currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**pl_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingPlatformListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_live_stream_lineup

> crate::models::UpdateLiveStreamLineupResponse update_live_stream_lineup(id)
Set a live show’s lineup: your Crossly listings, in the order you will sell them. Buyers see it before they join; a sale of a lineup item while the show is live is recorded at its replay moment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateLiveStreamLineupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_pricing_setting

> crate::models::GetPricingSettingResponse update_pricing_setting()
Turn own-price match and/or layaway on or off.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_rental_offer

> crate::models::UpdateRentalOfferResponse update_rental_offer(platform_listing_id)
Offer a Crossly listing for rent: day rate, optional week rate, deposit, min/max days, turnaround buffer, and rent-only (off sale while offered). Needs a ship-from address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateRentalOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_variation_group

> crate::models::CreateVariationGroupResponse update_variation_group(id)
Retitle a group, rename its axis, or change a group's purchase rule (written into every member's description).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVariationGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

