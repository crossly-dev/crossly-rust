# \StorefrontApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storefront_highlight**](StorefrontApi.md#create_storefront_highlight) | **POST** /v1/storefront/highlights | Create a highlight: title (up to 24 characters), optional coverUrl, and optional storyIds (your live or archived stories) to save into it. Up to 50 per store.
[**create_storefront_highlight_item**](StorefrontApi.md#create_storefront_highlight_item) | **POST** /v1/storefront/highlights/{id}/items | Save stories into a highlight (appended in the order given). Each is copied, so it stays after the story expires. A story already in the highlight is skipped.
[**create_storefront_profile_suggest_shelve**](StorefrontApi.md#create_storefront_profile_suggest_shelve) | **POST** /v1/storefront/profile/suggest-shelves | Profile shelves proposed from your top tags and categories (not saved).
[**create_storefront_section**](StorefrontApi.md#create_storefront_section) | **POST** /v1/storefront/sections | Add a section at the bottom: kind hero | listings | collections | about | policies | social | announcement | faq (default listings), with its config.
[**create_storefront_site_preview**](StorefrontApi.md#create_storefront_site_preview) | **POST** /v1/storefront/site/preview | Resolve an unsaved design page like the live site would: every section with its listings and extras (preview: true).
[**create_storefront_site_publish**](StorefrontApi.md#create_storefront_site_publish) | **POST** /v1/storefront/site/publish | Publish the draft: it becomes the live site and a publish version is kept (the last 30 versions are kept).
[**create_storefront_site_resolve**](StorefrontApi.md#create_storefront_site_resolve) | **POST** /v1/storefront/site/resolve | The live listings a ListingSource (manual, tags, category, smart rule or all) selects from your store, and how many match.
[**create_storefront_site_starter**](StorefrontApi.md#create_storefront_site_starter) | **POST** /v1/storefront/site/starter | Replace the draft with a starter site (vintage, sneakers, streetwear, collectibles, boutique, home, minimal, drop, general). The current draft is kept as a version first.
[**create_storefront_site_suggest**](StorefrontApi.md#create_storefront_site_suggest) | **POST** /v1/storefront/site/suggest | Auto-build: website sections proposed from your top tags and categories (not saved).
[**create_storefront_site_version_restore**](StorefrontApi.md#create_storefront_site_version_restore) | **POST** /v1/storefront/site/versions/{id}/restore | Copy a saved version into the draft (the current draft is kept as a version first). Publish to make it live.
[**create_storefront_trade_site_publish**](StorefrontApi.md#create_storefront_trade_site_publish) | **POST** /v1/storefront/trade-site/publish | Publish the trade site draft (a version is kept).
[**create_storefront_trade_site_resolve**](StorefrontApi.md#create_storefront_trade_site_resolve) | **POST** /v1/storefront/trade-site/resolve | The listings a ListingSource selects on your trade site: only items in your active wholesale catalogs (narrowed further by source.catalogIds), and how many match.
[**create_storefront_trade_site_starter**](StorefrontApi.md#create_storefront_trade_site_starter) | **POST** /v1/storefront/trade-site/starter | Replace the trade site draft with a trade starter (trade_classic, trade_line_sheet, trade_brand_house, trade_apply_first). The current draft is kept as a version first.
[**create_storefront_trade_site_suggest**](StorefrontApi.md#create_storefront_trade_site_suggest) | **POST** /v1/storefront/trade-site/suggest | Trade Auto-build: trade site sections proposed from your wholesale catalogs (a spotlight, a rail per catalog, terms, apply banner). Not saved.
[**delete_storefront_highlight**](StorefrontApi.md#delete_storefront_highlight) | **DELETE** /v1/storefront/highlights/{id} | Delete a highlight and its saved stories. The original stories are unaffected.
[**delete_storefront_highlight_item**](StorefrontApi.md#delete_storefront_highlight_item) | **DELETE** /v1/storefront/highlights/{id}/items/{itemId} | Remove one saved story from a highlight.
[**delete_storefront_section**](StorefrontApi.md#delete_storefront_section) | **DELETE** /v1/storefront/sections/{id} | Remove a section. Listings on a removed shelf stay live.
[**get_storefront**](StorefrontApi.md#get_storefront) | **GET** /v1/storefront | Your storefront design: banner, logo, tagline, bio, theme, SEO title and description, social links, and whether it is published.
[**get_storefront_merch**](StorefrontApi.md#get_storefront_merch) | **GET** /v1/storefront/merch | Your store's live listings by tag, category, condition and brand, with the price range: what product sources can select.
[**get_storefront_profile**](StorefrontApi.md#get_storefront_profile) | **GET** /v1/storefront/profile | Your shop profile (the Crossly Shop page): banner, logo, tagline, bio, accent, announcement, shelves, blocks and All items options.
[**get_storefront_shipping**](StorefrontApi.md#get_storefront_shipping) | **GET** /v1/storefront/shipping | Your Crossly shipping policy (domestic and international services, which countries you ship to) and your default ship-from address.
[**get_storefront_site**](StorefrontApi.md#get_storefront_site) | **GET** /v1/storefront/site | Your store website: the draft design, what is live, when each changed, whether the site is switched on, and its custom domain.
[**get_storefront_trade_site**](StorefrontApi.md#get_storefront_trade_site) | **GET** /v1/storefront/trade-site | Your store's trade (wholesale) site at /site/:slug/trade: the draft, what is live, whether it is on and its gating (public | prices_private | approved_only).
[**list_storefront_highlights**](StorefrontApi.md#list_storefront_highlights) | **GET** /v1/storefront/highlights | Your store’s story highlights in display order, each with its saved stories (media, overlays, product tie-in, when posted). Empty highlights included.
[**list_storefront_sections**](StorefrontApi.md#list_storefront_sections) | **GET** /v1/storefront/sections | Your storefront sections in display order: kind, title, visible, config, and listing ids for listings shelves.
[**list_storefront_site_versions**](StorefrontApi.md#list_storefront_site_versions) | **GET** /v1/storefront/site/versions | Saved versions of the website, newest first (publish, restore and template points; up to 30).
[**list_storefront_story_archive**](StorefrontApi.md#list_storefront_story_archive) | **GET** /v1/storefront/stories/archive | Your stories, live and expired, newest first: what can be saved to a highlight. Paginated: pass before = meta.nextCursor of the previous page.
[**update_storefront**](StorefrontApi.md#update_storefront) | **PUT** /v1/storefront | Update your storefront design. Omitted fields are unchanged, null clears. published=false keeps the design as a draft and buyers see the standard storefront.
[**update_storefront_highlight**](StorefrontApi.md#update_storefront_highlight) | **PATCH** /v1/storefront/highlights/{id} | Rename a highlight or change its cover (coverUrl null = use the first story).
[**update_storefront_highlight_item_order**](StorefrontApi.md#update_storefront_highlight_item_order) | **PUT** /v1/storefront/highlights/{id}/items/order | Set the order of a highlight’s saved stories: every item id, first to last.
[**update_storefront_highlight_order**](StorefrontApi.md#update_storefront_highlight_order) | **PUT** /v1/storefront/highlights/order | Set the highlight order in one call: every highlight id, first to last.
[**update_storefront_profile**](StorefrontApi.md#update_storefront_profile) | **PUT** /v1/storefront/profile | Replace your shop profile (a complete ShopProfile). Saved straight away.
[**update_storefront_section**](StorefrontApi.md#update_storefront_section) | **PATCH** /v1/storefront/sections/{id} | Rename, show/hide, refill (listings shelves) or reconfigure a section.
[**update_storefront_section_reorder**](StorefrontApi.md#update_storefront_section_reorder) | **PUT** /v1/storefront/sections/reorder | Set the section order in one atomic call: every section id, top to bottom.
[**update_storefront_shipping**](StorefrontApi.md#update_storefront_shipping) | **PUT** /v1/storefront/shipping | Set your shipping policy (international.enabled, shipsTo worldwide or ISO codes, excludes) and/or your default ship-from address (any country; 422 for a sanctioned origin).
[**update_storefront_site_draft**](StorefrontApi.md#update_storefront_site_draft) | **PUT** /v1/storefront/site/draft | Save the website draft (a complete SiteDesign). Validated strictly (a bad field is a 400 naming its path) and sanitized; buyers see nothing until you publish.
[**update_storefront_site_enabled**](StorefrontApi.md#update_storefront_site_enabled) | **PUT** /v1/storefront/site/enabled | Switch the website on or off. On requires a publish first; off makes /site/:slug 404 and the custom domain show the shop profile.
[**update_storefront_trade_site_draft**](StorefrontApi.md#update_storefront_trade_site_draft) | **PUT** /v1/storefront/trade-site/draft | Save the trade site draft (a complete SiteDesign; trade sections included). Validated strictly and sanitized.
[**update_storefront_trade_site_enabled**](StorefrontApi.md#update_storefront_trade_site_enabled) | **PUT** /v1/storefront/trade-site/enabled | Switch the trade site on (needs a publish first) or off.
[**update_storefront_trade_site_gating**](StorefrontApi.md#update_storefront_trade_site_gating) | **PUT** /v1/storefront/trade-site/gating | Who sees the trade site: public (everyone, with prices), prices_private (everyone, prices for approved buyers) or approved_only (approved buyers; others get the apply page).



## create_storefront_highlight

> crate::models::CreateStorefrontHighlightResponse create_storefront_highlight()
Create a highlight: title (up to 24 characters), optional coverUrl, and optional storyIds (your live or archived stories) to save into it. Up to 50 per store.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontHighlightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_highlight_item

> crate::models::StorefrontHighlightResponse create_storefront_highlight_item(id)
Save stories into a highlight (appended in the order given). Each is copied, so it stays after the story expires. A story already in the highlight is skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontHighlightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_profile_suggest_shelve

> crate::models::CreateStorefrontProfileSuggestShelveResponse create_storefront_profile_suggest_shelve()
Profile shelves proposed from your top tags and categories (not saved).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontProfileSuggestShelveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_section

> crate::models::StorefrontSectionResponse create_storefront_section()
Add a section at the bottom: kind hero | listings | collections | about | policies | social | announcement | faq (default listings), with its config.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontSectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_preview

> crate::models::CreateStorefrontSitePreviewResponse create_storefront_site_preview()
Resolve an unsaved design page like the live site would: every section with its listings and extras (preview: true).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontSitePreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_publish

> crate::models::StorefrontResponse2 create_storefront_site_publish()
Publish the draft: it becomes the live site and a publish version is kept (the last 30 versions are kept).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_resolve

> crate::models::CreateStorefrontSiteResolveResponse create_storefront_site_resolve()
The live listings a ListingSource (manual, tags, category, smart rule or all) selects from your store, and how many match.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontSiteResolveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_starter

> crate::models::StorefrontResponse2 create_storefront_site_starter()
Replace the draft with a starter site (vintage, sneakers, streetwear, collectibles, boutique, home, minimal, drop, general). The current draft is kept as a version first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_suggest

> crate::models::CreateStorefrontSiteSuggestResponse create_storefront_site_suggest()
Auto-build: website sections proposed from your top tags and categories (not saved).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontSiteSuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_site_version_restore

> crate::models::StorefrontResponse3 create_storefront_site_version_restore(id)
Copy a saved version into the draft (the current draft is kept as a version first). Publish to make it live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_trade_site_publish

> crate::models::StorefrontResponse2 create_storefront_trade_site_publish()
Publish the trade site draft (a version is kept).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_trade_site_resolve

> crate::models::CreateStorefrontSiteResolveResponse create_storefront_trade_site_resolve()
The listings a ListingSource selects on your trade site: only items in your active wholesale catalogs (narrowed further by source.catalogIds), and how many match.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontSiteResolveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_trade_site_starter

> crate::models::StorefrontResponse2 create_storefront_trade_site_starter()
Replace the trade site draft with a trade starter (trade_classic, trade_line_sheet, trade_brand_house, trade_apply_first). The current draft is kept as a version first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_storefront_trade_site_suggest

> crate::models::CreateStorefrontSiteSuggestResponse create_storefront_trade_site_suggest()
Trade Auto-build: trade site sections proposed from your wholesale catalogs (a spotlight, a rail per catalog, terms, apply banner). Not saved.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStorefrontSiteSuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront_highlight

> crate::models::DeleteStoryResponse delete_storefront_highlight(id)
Delete a highlight and its saved stories. The original stories are unaffected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront_highlight_item

> crate::models::StorefrontHighlightResponse delete_storefront_highlight_item(id, item_id)
Remove one saved story from a highlight.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**item_id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontHighlightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_storefront_section

> crate::models::UpdateGroupBuyResponse delete_storefront_section(id)
Remove a section. Listings on a removed shelf stay live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront

> crate::models::GetStorefrontResponse get_storefront()
Your storefront design: banner, logo, tagline, bio, theme, SEO title and description, social links, and whether it is published.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_merch

> crate::models::GetStorefrontMerchResponse get_storefront_merch()
Your store's live listings by tag, category, condition and brand, with the price range: what product sources can select.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontMerchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_profile

> crate::models::GetStorefrontProfileResponse get_storefront_profile()
Your shop profile (the Crossly Shop page): banner, logo, tagline, bio, accent, announcement, shelves, blocks and All items options.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_shipping

> crate::models::GetStorefrontShippingResponse get_storefront_shipping()
Your Crossly shipping policy (domestic and international services, which countries you ship to) and your default ship-from address.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontShippingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_site

> crate::models::StorefrontResponse3 get_storefront_site()
Your store website: the draft design, what is live, when each changed, whether the site is switched on, and its custom domain.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_storefront_trade_site

> crate::models::StorefrontResponse3 get_storefront_trade_site()
Your store's trade (wholesale) site at /site/:slug/trade: the draft, what is live, whether it is on and its gating (public | prices_private | approved_only).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_highlights

> crate::models::ListStorefrontHighlightsResponse list_storefront_highlights()
Your store’s story highlights in display order, each with its saved stories (media, overlays, product tie-in, when posted). Empty highlights included.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontHighlightsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_sections

> crate::models::StorefrontResponse4 list_storefront_sections()
Your storefront sections in display order: kind, title, visible, config, and listing ids for listings shelves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse4**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_site_versions

> crate::models::ListStorefrontSiteVersionsResponse list_storefront_site_versions()
Saved versions of the website, newest first (publish, restore and template points; up to 30).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontSiteVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_storefront_story_archive

> crate::models::ListStorefrontStoryArchiveResponse list_storefront_story_archive()
Your stories, live and expired, newest first: what can be saved to a highlight. Paginated: pass before = meta.nextCursor of the previous page.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontStoryArchiveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront

> crate::models::GetStorefrontResponse update_storefront()
Update your storefront design. Omitted fields are unchanged, null clears. published=false keeps the design as a draft and buyers see the standard storefront.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_highlight

> crate::models::StorefrontHighlightResponse update_storefront_highlight(id)
Rename a highlight or change its cover (coverUrl null = use the first story).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontHighlightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_highlight_item_order

> crate::models::StorefrontHighlightResponse update_storefront_highlight_item_order(id)
Set the order of a highlight’s saved stories: every item id, first to last.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontHighlightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_highlight_order

> crate::models::ListStorefrontHighlightsResponse update_storefront_highlight_order()
Set the highlight order in one call: every highlight id, first to last.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStorefrontHighlightsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_profile

> crate::models::UpdateStorefrontProfileResponse update_storefront_profile()
Replace your shop profile (a complete ShopProfile). Saved straight away.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateStorefrontProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_section

> crate::models::StorefrontSectionResponse update_storefront_section(id)
Rename, show/hide, refill (listings shelves) or reconfigure a section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StorefrontSectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_section_reorder

> crate::models::StorefrontResponse4 update_storefront_section_reorder()
Set the section order in one atomic call: every section id, top to bottom.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse4**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_shipping

> crate::models::GetStorefrontShippingResponse update_storefront_shipping()
Set your shipping policy (international.enabled, shipsTo worldwide or ISO codes, excludes) and/or your default ship-from address (any country; 422 for a sanctioned origin).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontShippingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_site_draft

> crate::models::StorefrontResponse2 update_storefront_site_draft()
Save the website draft (a complete SiteDesign). Validated strictly (a bad field is a 400 naming its path) and sanitized; buyers see nothing until you publish.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_site_enabled

> crate::models::StorefrontResponse3 update_storefront_site_enabled()
Switch the website on or off. On requires a publish first; off makes /site/:slug 404 and the custom domain show the shop profile.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_trade_site_draft

> crate::models::StorefrontResponse2 update_storefront_trade_site_draft()
Save the trade site draft (a complete SiteDesign; trade sections included). Validated strictly and sanitized.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_trade_site_enabled

> crate::models::StorefrontResponse3 update_storefront_trade_site_enabled()
Switch the trade site on (needs a publish first) or off.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_trade_site_gating

> crate::models::StorefrontResponse3 update_storefront_trade_site_gating()
Who sees the trade site: public (everyone, with prices), prices_private (everyone, prices for approved buyers) or approved_only (approved buyers; others get the apply page).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StorefrontResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

