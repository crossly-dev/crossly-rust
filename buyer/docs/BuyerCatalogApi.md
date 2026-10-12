# \BuyerCatalogApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_identify**](BuyerCatalogApi.md#create_buyer_identify) | **POST** /v1/buyer/identify | Identify a held object and return a HUD-ready answer.
[**create_buyer_lockon**](BuyerCatalogApi.md#create_buyer_lockon) | **POST** /v1/buyer/lockons | Lock on to an object the buyer is holding.
[**create_buyer_lockon_confirm**](BuyerCatalogApi.md#create_buyer_lockon_confirm) | **POST** /v1/buyer/lockons/{id}/confirm | The buyer picked one of the candidates.
[**create_buyer_lockon_observe**](BuyerCatalogApi.md#create_buyer_lockon_observe) | **POST** /v1/buyer/lockons/{id}/observe | Add what this frame revealed, and get the current best answer.
[**create_buyer_scan**](BuyerCatalogApi.md#create_buyer_scan) | **POST** /v1/buyer/scan | Identify a physical item and find the cheapest place to buy it.
[**create_buyer_scan_session**](BuyerCatalogApi.md#create_buyer_scan_session) | **POST** /v1/buyer/scan/sessions | Open a Live Shop session.
[**create_buyer_scan_session_end**](BuyerCatalogApi.md#create_buyer_scan_session_end) | **POST** /v1/buyer/scan/sessions/{id}/end | Close a Live Shop session.
[**get_buyer_anywhere**](BuyerCatalogApi.md#get_buyer_anywhere) | **GET** /v1/buyer/anywhere | Cheapest source for an item — Crossly first, then other retailers.
[**get_buyer_catalog_facet**](BuyerCatalogApi.md#get_buyer_catalog_facet) | **GET** /v1/buyer/catalog/facets | Brands, categories and conditions that currently have stock.
[**get_buyer_catalog_fx**](BuyerCatalogApi.md#get_buyer_catalog_fx) | **GET** /v1/buyer/catalog/fx | Display-only exchange rates (units per 1 USD).
[**get_buyer_catalog_listing**](BuyerCatalogApi.md#get_buyer_catalog_listing) | **GET** /v1/buyer/catalog/listings/{slug} | One listing, in full.
[**get_buyer_catalog_listing_availability**](BuyerCatalogApi.md#get_buyer_catalog_listing_availability) | **GET** /v1/buyer/catalog/listings/{slug}/availability | Is it still buyable, and at what price.
[**get_buyer_catalog_listing_shipping**](BuyerCatalogApi.md#get_buyer_catalog_listing_shipping) | **GET** /v1/buyer/catalog/listings/{slug}/shipping | Shipping services and prices to a destination.
[**get_buyer_listing_landed_cost**](BuyerCatalogApi.md#get_buyer_listing_landed_cost) | **GET** /v1/buyer/listings/{slug}/landed-cost | Import duties, taxes and fees to a country (landed cost).
[**get_buyer_scan_session**](BuyerCatalogApi.md#get_buyer_scan_session) | **GET** /v1/buyer/scan/sessions/{id} | One trip and everything it found.
[**list_buyer_catalog_browse**](BuyerCatalogApi.md#list_buyer_catalog_browse) | **GET** /v1/buyer/catalog/browse | Browse the way the shop does — buying format, returns, size, color, sold items, Best Match.
[**list_buyer_catalog_listing_sold_history**](BuyerCatalogApi.md#list_buyer_catalog_listing_sold_history) | **GET** /v1/buyer/catalog/listings/{slug}/sold-history | What THIS listing has sold for, newest first (last 50), with the median.
[**list_buyer_catalog_search**](BuyerCatalogApi.md#list_buyer_catalog_search) | **GET** /v1/buyer/catalog/search | Search the Crossly catalogue.
[**list_buyer_catalog_sellers**](BuyerCatalogApi.md#list_buyer_catalog_sellers) | **GET** /v1/buyer/catalog/sellers | Find a seller by shop handle or shop name.
[**list_buyer_scan_sessions**](BuyerCatalogApi.md#list_buyer_scan_sessions) | **GET** /v1/buyer/scan/sessions | Your scanning trips, newest first.



## create_buyer_identify

> crate::models::CreateBuyerIdentifyResponse create_buyer_identify()
Identify a held object and return a HUD-ready answer.

The gesture endpoint for Live Shop. Runs a cost ladder: a decoded BARCODE resolves in ~50ms for nothing; failing that, self-hosted CLIP matches the catalogue; failing that, a vision model names it (the only rung that costs anything, capped per buyer per day). `hud` is pre-formatted for a 600×600 lens — one headline, one subline, up to three fact chips and exactly ONE action, because a pinch cannot choose between buttons. A vision label is WORDS, never an identity: it names the thing so the buyer can search, and never drives a price comparison.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerIdentifyResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_lockon

> crate::models::CreateBuyerLockonResponse create_buyer_lockon()
Lock on to an object the buyer is holding.

Open this when on-device tracking acquires an object, then post observations to it as the buyer turns the thing over. The answer improves as evidence arrives — the style code inside a shoe settles what the front of it could not.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerLockonResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_lockon_confirm

> crate::models::CreateBuyerLockonConfirmResponse create_buyer_lockon_confirm(id)
The buyer picked one of the candidates.

Promotes a text match to a CONFIRMED identity — the strongest evidence in the system, because a person holding the object said yes. Validated against the candidates we actually offered, so it cannot be claimed about an arbitrary product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerLockonConfirmResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_lockon_observe

> crate::models::CreateBuyerLockonConfirmResponse create_buyer_lockon_observe(id)
Add what this frame revealed, and get the current best answer.

Send only what you LEARNED: a decoded barcode, newly-read OCR text, or a frame when neither settled it. Do not post every frame — tracking and decoding happen on-device for free, and this endpoint is for evidence, not video. Evidence is RANKED (confirmed > barcode > ocr > visual), so a late weak reading can never overwrite a strong early one. When text evidence finds several products, `candidates` comes back for the buyer to pick from — a vision label is words, and only a human confirmation turns it into an identity we will price against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerLockonConfirmResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_scan

> crate::models::CreateBuyerScanResponse create_buyer_scan()
Identify a physical item and find the cheapest place to buy it.

Send a barcode identifier OR a photo. A BARCODE establishes identity, so the response carries a full price verdict across Crossly and other retailers. A PHOTO establishes resemblance only: you get visual matches from the Crossly catalogue, and a price verdict ONLY if the matched listing carries a real identifier. When it does not, `comparable` is false and there is no verdict — a price comparison built on a visual guess is a claim about a different product. Most second-hand items have no identifier by nature, so this is expected rather than a failure.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerScanResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_scan_session

> crate::models::CreateBuyerScanSessionResponse create_buyer_scan_session()
Open a Live Shop session.

Call this when the glasses connect, then pass the returned id as `sessionId` on each scan. Opening a session CLOSES any other live one — a person is in one shop at a time, and two live sessions split a trip across both.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerScanSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_scan_session_end

> crate::models::CreateBuyerScanSessionEndResponse create_buyer_scan_session_end(id)
Close a Live Shop session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerScanSessionEndResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_anywhere

> crate::models::GetBuyerAnywhereResponse get_buyer_anywhere()
Cheapest source for an item — Crossly first, then other retailers.

Answers with a VERDICT, not a list: crossly_best, offsite_cheaper, offsite_only or no_match. Offsite offers come from licensed affiliate product feeds, are ranked CHEAPEST-FIRST — commission only ever breaks a sub-$1 tie — and only appear when they beat the price you passed in. `shippingUnknown: true` means a compared price omitted postage, so present the result as \"before postage\" rather than as a delivered total. Crossly wins ties within $1; beyond that the honest answer wins.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerAnywhereResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_facet

> crate::models::GetBuyerCatalogFacetResponse get_buyer_catalog_facet()
Brands, categories and conditions that currently have stock.

The vocabulary the search filters accept. Counts are live, so a filter built from this will never return an empty page for a value that has since sold out.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerCatalogFacetResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_fx

> crate::models::GetBuyerCatalogFxResponse get_buyer_catalog_fx()
Display-only exchange rates (units per 1 USD).

ECB daily reference rates rebased to US dollars: `rates.EUR = 0.92` means $1 ≈ €0.92. `asOf` is the ECB publication date (null until the first refresh). For showing an approximate price only — every Crossly price and charge is in USD, and the card issuer sets the final exchange rate.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerCatalogFxResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing

> crate::models::GetBuyerCatalogListingResponse get_buyer_catalog_listing(slug)
One listing, in full.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCatalogListingResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing_availability

> crate::models::GetBuyerCatalogListingAvailabilityResponse get_buyer_catalog_listing_availability(slug)
Is it still buyable, and at what price.

The cheapest endpoint here, and the one to poll if you are going to poll — a single indexed row, no joins beyond stock, and an ETag so an unchanged answer is a 304. If you want to be TOLD instead of asking, create a monitor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCatalogListingAvailabilityResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing_shipping

> crate::models::GetBuyerCatalogListingShippingResponse get_buyer_catalog_listing_shipping(slug, zip, country, quantity)
Shipping services and prices to a destination.

Lists every shipping service the seller offers to `country` (default: the seller’s own country) with its price for `quantity` units and its delivery window in business days (handling included). `zip` is required for a domestic quote. `shipsToCountry: false` means the seller does not ship there. Pass a service `id` as `shippingServiceId` to POST /api/v1/buyer/checkout, or in `shippingServiceBySeller` to POST /api/v1/buyer/cart/quote.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**zip** | Option<**String**> |  |  |
**country** | Option<**String**> |  |  |
**quantity** | Option<**i32**> |  |  |

### Return type

**crate::models::GetBuyerCatalogListingShippingResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_landed_cost

> crate::models::GetBuyerListingLandedCostResponse get_buyer_listing_landed_cost(country, slug, postal_code, quantity)
Import duties, taxes and fees to a country (landed cost).

Quotes the import duties, import taxes and fees for this listing shipped to `country` (ISO 3166-1 alpha-2), using the cheapest shipping service the seller offers there. `status: included` means checkout charges them now and the parcel ships DDP — no bill at the door; `unavailable` means it ships DDU and `notice` says the carrier or customs may charge on delivery; `none` means the parcel does not cross a border. Amounts are US-dollar cents. Answers 503 with `missingEnv` when landed-cost quoting is not configured on this deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | **String** |  | [required] |
**slug** | **String** |  | [required] |
**postal_code** | Option<**String**> |  |  |
**quantity** | Option<**i32**> |  |  |

### Return type

**crate::models::GetBuyerListingLandedCostResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_scan_session

> crate::models::GetBuyerScanSessionResponse get_buyer_scan_session(id)
One trip and everything it found.

Verdicts are returned EXACTLY as they were given at the time, not re-priced. A history screen that silently refreshes old prices shows a saving that was never actually on offer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerScanSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_catalog_browse

> crate::models::ListBuyerCatalogBrowseResponse list_buyer_catalog_browse(q, format, returns, sold, size, color, ships_to, sort, min_price_cents, max_price_cents, page, limit)
Browse the way the shop does — buying format, returns, size, color, sold items, Best Match.

Page-numbered and capped at 50 pages of up to 60, like the storefront. For walking the whole catalogue use /v1/buyer/catalog/search, which pages by cursor. Prices are integer CENTS (`minPriceCents=2500` is $25.00). `sold=true` returns recent sales of matching listings (last 90 days) with `soldPriceCents` and `soldAt`, instead of live stock.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Free text; every word must appear in the title or brand. |  |
**format** | Option<**String**> | auction, buy_it_now, or offers (accepts offers). |  |
**returns** | Option<**String**> | true = only listings that accept returns. |  |
**sold** | Option<**String**> | true = recent sales instead of live listings. |  |
**size** | Option<**String**> | Exact size, case-insensitive (M does not match XL). |  |
**color** | Option<**String**> | Exact colour, case-insensitive. |  |
**ships_to** | Option<**String**> | ISO 3166-1 alpha-2 destination (e.g. GB): only listings whose seller ships there, decided as checkout decides it (the listing's shipping policy and its own restricted countries). An embargoed destination matches nothing. Omitted, or `any`, means no destination filter — the storefront grid defaults to the buyer's country, this endpoint does not. |  |
**sort** | Option<**String**> | Default best_match with q, newest without. |  |
**min_price_cents** | Option<**i32**> | Lowest price, in cents. |  |
**max_price_cents** | Option<**i32**> | Highest price, in cents. |  |
**page** | Option<**i32**> | 1-indexed page. |  |[default to 1]
**limit** | Option<**i32**> | Rows per page. |  |[default to 24]

### Return type

**crate::models::ListBuyerCatalogBrowseResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_catalog_listing_sold_history

> crate::models::ListBuyerCatalogListingSoldHistoryResponse list_buyer_catalog_listing_sold_history(slug)
What THIS listing has sold for, newest first (last 50), with the median.

This listing only — never \"similar items\", which would mean matching other listings by resemblance. Prices are per unit, in cents, excluding shipping and tax.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The listing slug. | [required] |

### Return type

**crate::models::ListBuyerCatalogListingSoldHistoryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_catalog_search

> crate::models::ListBuyerCatalogSearchResponse list_buyer_catalog_search()
Search the Crossly catalogue.

Keyset-paginated. Pass the `nextCursor` you were given back as `cursor`; page 500 costs the same as page 1. Cursors are opaque — do not parse them. Responses carry an ETag: send it back as If-None-Match and an unchanged page answers 304, which is free. `sort=popular` is deliberately unavailable, because a cursor into a continuously-reordering list silently skips rows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerCatalogSearchResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_catalog_sellers

> crate::models::ListBuyerCatalogSellersResponse list_buyer_catalog_sellers(q, limit)
Find a seller by shop handle or shop name.

Only shops that have listed on Crossly. Exact matches first, then prefix, then by live listing count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | At least two characters of a handle or shop name. | [required] |
**limit** | Option<**i32**> | How many shops to return. |  |[default to 12]

### Return type

**crate::models::ListBuyerCatalogSellersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_scan_sessions

> crate::models::ListBuyerScanSessionsResponse list_buyer_scan_sessions()
Your scanning trips, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerScanSessionsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

