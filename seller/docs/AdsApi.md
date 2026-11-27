# \AdsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ad_offsite_campaign**](AdsApi.md#create_ad_offsite_campaign) | **POST** /v1/ads/offsite/campaigns | Launch an offsite campaign for an item.
[**create_ad_offsite_resume**](AdsApi.md#create_ad_offsite_resume) | **POST** /v1/ads/offsite/resume | Clear an auto-pause and resume offsite spend.
[**create_listing_ad_bulk_add**](AdsApi.md#create_listing_ad_bulk_add) | **POST** /v1/listing-ads/bulk-add | Add listings to a campaign by master listing id: each listing's live row on `platform` joins campaignId (a Crossly campaign uuid, or the marketplace's campaign id). A listing not live there is its own failed result.
[**create_listing_ad_campaign**](AdsApi.md#create_listing_ad_campaign) | **POST** /v1/listing-ads/{platform}/campaigns | Create a campaign on the marketplace: program (promoted_listing_rate = eBay General, a % of the sale; cpc_campaign = eBay Priority, a daily budget), name, ratePct or dailyBudgetCents, startsAt/endsAt, and the program fields. Idempotency-Key supported.
[**create_listing_ad_campaign_listing**](AdsApi.md#create_listing_ad_campaign_listing) | **POST** /v1/listing-ads/{platform}/campaigns/{campaignId}/listings | Promote listings in a marketplace campaign. platformListingIds are Crossly platform-listing ids on that marketplace; ratePct sets the ad rate (eBay General). One result per listing.
[**create_listing_ad_campaign_listing_remove**](AdsApi.md#create_listing_ad_campaign_listing_remove) | **POST** /v1/listing-ads/{platform}/campaigns/{campaignId}/listings/remove | Stop promoting listings in a marketplace campaign. One result per listing.
[**create_sponsored_ad_billing_topup_link**](AdsApi.md#create_sponsored_ad_billing_topup_link) | **POST** /v1/sponsored-ads/billing/topup-link | A Stripe Checkout link that adds amountCents ($5–$10,000) to the ad balance when the seller pays it. Nothing is charged until they do. Use an Idempotency-Key to get the same link back on a retry.
[**create_sponsored_ad_boost_remove**](AdsApi.md#create_sponsored_ad_boost_remove) | **POST** /v1/sponsored-ads/boosts/remove | Stop boosting listings in bulk.
[**create_sponsored_ad_campaign**](AdsApi.md#create_sponsored_ad_campaign) | **POST** /v1/sponsored-ads/campaigns | Create a Sponsored Listings campaign: name, dailyBudgetCents (≥100), optional totalBudgetCents, defaultBidCents (5–10000; the max bid under bidStrategy \"auto\"), placements (search, browse, product_page, home_rail, storefront, worlds_surface), goal (sales | visits), optional targetCategories / targetRegions (ISO countries) / targetDevices (mobile, desktop, app), optional startsAt/endsAt, listings [{platformListingId, bidCents?, boostBps? (100–2000, pay-on-sale Boost; null clears)}], keywords [{keyword, matchType broad|phrase|exact, negative?, bidCents?}], worldTargets [{kind showroom|convention, id, bidCents?, status?}] (your showroom or a convention you host, promoted in the Crossly World directory; at least one listing or world target), status active|draft. A catalog boost: targetType \"wholesale_catalog\" + targetCatalogId (your catalog), no listings, keywords or world targets; it ranks Sponsored in the wholesale directory. bidStrategy defaults to your account default. Clicks are charged from your ad funds (ad balance and/or CBX, ad credit first); with none it waits as out_of_budget.
[**create_sponsored_ad_campaign_duplicate**](AdsApi.md#create_sponsored_ad_campaign_duplicate) | **POST** /v1/sponsored-ads/campaigns/{id}/duplicate | Copy a campaign (settings, listings, keywords) as a new draft.
[**create_sponsored_ad_campaign_status**](AdsApi.md#create_sponsored_ad_campaign_status) | **POST** /v1/sponsored-ads/campaigns/{id}/status | Set a campaign active (launch/resume), paused, or ended (final). Resuming with no ad funds to pay for clicks leaves it out_of_budget until you top up.
[**get_ad_offsite**](AdsApi.md#get_ad_offsite) | **GET** /v1/ads/offsite | Your offsite-ads opt-in and its terms.
[**get_ad_offsite_eligibility**](AdsApi.md#get_ad_offsite_eligibility) | **GET** /v1/ads/offsite/eligibility | Whether an item can run offsite, and why not.
[**get_ad_offsite_report**](AdsApi.md#get_ad_offsite_report) | **GET** /v1/ads/offsite/report | What your offsite budget bought — including the misses.
[**get_listing_ad_listing**](AdsApi.md#get_listing_ad_listing) | **GET** /v1/listing-ads/listings/{listingId} | Every promotion of one listing: its Crossly campaigns and Boost, and each marketplace ad with status, campaign, rate and last sync.
[**get_sponsored_ad_billing**](AdsApi.md#get_sponsored_ad_billing) | **GET** /v1/sponsored-ads/billing | What pays for your ads: headroom across the billing rails (ad balance and/or CBX), unbilled click spend, settlement charges, daily spend, Boost fees and CBX ad-credit grants; funds is the full funds summary (same as /v1/sponsored-ads/funds). refresh=true re-reads the balance.
[**get_sponsored_ad_billing_statement**](AdsApi.md#get_sponsored_ad_billing_statement) | **GET** /v1/sponsored-ads/billing/statements/{month} | One month's ads statement: opening and closing ad balance, top-ups, click spend by campaign, how clicks were paid, credits, refunds and Boost fees.
[**get_sponsored_ad_billing_statement_pdf**](AdsApi.md#get_sponsored_ad_billing_statement_pdf) | **GET** /v1/sponsored-ads/billing/statements/{month}/pdf | One month's ads statement as a PDF.
[**get_sponsored_ad_billing_topup_receipt**](AdsApi.md#get_sponsored_ad_billing_topup_receipt) | **GET** /v1/sponsored-ads/billing/topups/{id}/receipt | One ad balance top-up's receipt as a PDF (each top-up in the billing history carries its id and receipt number).
[**get_sponsored_ad_campaign**](AdsApi.md#get_sponsored_ad_campaign) | **GET** /v1/sponsored-ads/campaigns/{id} | One campaign: settings, its listings (bid, suggested bid, live, per-listing stats) and keywords. A catalog boost (targetType wholesale_catalog) has `catalogs`: per boosted catalog impressions, clicks, spend, attributed trade orders and revenue, CTR and ROAS.
[**get_sponsored_ad_fund**](AdsApi.md#get_sponsored_ad_fund) | **GET** /v1/sponsored-ads/funds | What can pay for clicks now: the billing rails in effect (ad balance and/or CBX), the ad balance and ad credit, CBX headroom, unbilled clicks, available cents, auto top-up, the saved payment method and the refundable balance. refresh=true re-reads CBX.
[**get_sponsored_ad_report**](AdsApi.md#get_sponsored_ad_report) | **GET** /v1/sponsored-ads/report | Sponsored report for a range: totals (spend, impressions, clicks, CTR, average CPC, attributed orders and revenue, ROAS), a zero-filled daily series, per-campaign rows (targetType listings | wholesale_catalog) and `catalogs`, one row per boosted catalog. Listing orders are Crossly purchases of the advertised listing within 7 days of a click by the same buyer; catalog orders are trade orders with that seller within 7 days of a catalog click by the same buyer. Catalog impressions count once per viewer per day.
[**get_sponsored_ad_setting**](AdsApi.md#get_sponsored_ad_setting) | **GET** /v1/sponsored-ads/settings | Ads quick settings: adsPaused, accountDailyCapCents and spentTodayCents, defaultBidStrategy, autoTopup.
[**list_listing_ad_campaigns**](AdsApi.md#list_listing_ad_campaigns) | **GET** /v1/listing-ads/{platform}/campaigns | Your ad campaigns on one marketplace (cached up to 15 minutes; refresh=true asks the marketplace now).
[**list_listing_ad_platforms**](AdsApi.md#list_listing_ad_platforms) | **GET** /v1/listing-ads/platforms | The ads registry: each marketplace that sells promotion, its programs (kind, scope, campaign and listing fields), which verbs are wired / need a capture / are unsupported, and whether you are connected there.
[**list_sponsored_ad_billing_history**](AdsApi.md#list_sponsored_ad_billing_history) | **GET** /v1/sponsored-ads/billing/history | Billing history newest first: top-ups (status, receipt number), click charges (how each was paid: ad balance, credit, CBX, written off), credits, refunds and Boost fees. kind filters; before (ISO time) pages; limit ≤ 100.
[**list_sponsored_ad_billing_statements**](AdsApi.md#list_sponsored_ad_billing_statements) | **GET** /v1/sponsored-ads/billing/statements | Months (YYYY-MM) with any ad billing activity, newest first.
[**list_sponsored_ad_boosts**](AdsApi.md#list_sponsored_ad_boosts) | **GET** /v1/sponsored-ads/boosts | Boosted (pay-on-sale) listings: the rate, sales and fees taken so far. A boost lifts organic ranking by up to 10% and costs its rate only when the listing sells on Crossly.
[**list_sponsored_ad_campaigns**](AdsApi.md#list_sponsored_ad_campaigns) | **GET** /v1/sponsored-ads/campaigns | Your Sponsored Listings campaigns with status, pacing (spent today, daily cap reached), budgets, bids, placements and stats for the range (from/to, YYYY-MM-DD; default last 30 days).
[**list_sponsored_ad_listings**](AdsApi.md#list_sponsored_ad_listings) | **GET** /v1/sponsored-ads/listings | Per-listing ad performance across campaigns for a range, plus suggestions: live listings with views but few sales that are not sponsored yet, with a suggested bid.
[**update_ad_offsite**](AdsApi.md#update_ad_offsite) | **PUT** /v1/ads/offsite | Turn offsite ads on or off. Yours alone to set.
[**update_listing_ad_campaign**](AdsApi.md#update_listing_ad_campaign) | **PATCH** /v1/listing-ads/{platform}/campaigns/{campaignId} | Change a marketplace campaign: name, dailyBudgetCents, endsAt (null clears), status paused | running | ended.
[**update_listing_ad_listing_rate**](AdsApi.md#update_listing_ad_listing_rate) | **PUT** /v1/listing-ads/{platform}/listings/{platformListingId}/rate | Change one promoted listing's ad rate (percent of the sale) in the campaign it is in.
[**update_sponsored_ad_billing_auto_topup**](AdsApi.md#update_sponsored_ad_billing_auto_topup) | **PUT** /v1/sponsored-ads/billing/auto-topup | Auto top-up: enabled, thresholdCents (top up when the ad balance drops below it), amountCents (how much). Needs a saved payment method (the first top-up saves it).
[**update_sponsored_ad_boost**](AdsApi.md#update_sponsored_ad_boost) | **PUT** /v1/sponsored-ads/boosts | Boost listings in bulk (platformListingIds, upliftBps 100–2000 = 1–20%), or change their rate. Your own live Crossly listings only.
[**update_sponsored_ad_campaign**](AdsApi.md#update_sponsored_ad_campaign) | **PATCH** /v1/sponsored-ads/campaigns/{id} | Edit a campaign. Any of the create fields; listings and keywords, when given, replace the current set. Raising the total budget resumes a campaign stopped by it.
[**update_sponsored_ad_setting**](AdsApi.md#update_sponsored_ad_setting) | **PATCH** /v1/sponsored-ads/settings | Change ads quick settings: adsPaused (pause / resume every ad), accountDailyCapCents (null = no cap), defaultBidStrategy (auto | manual) for new campaigns, autoTopup {enabled, thresholdCents?, amountCents?}, railPreference (cash_first | cbx_first, both rails live only).



## create_ad_offsite_campaign

> crate::models::CreateAdOffsiteCampaignResponse create_ad_offsite_campaign()
Launch an offsite campaign for an item.

Creative is built from the structured fields you pass — title, condition as you recorded it, price, image. We generate no prose and invent no claims about condition or authenticity: an ad saying \"mint\" or \"authenticated\" when your listing says neither is a misrepresentation we authored and YOU would take the dispute for. Refuses with 403 when no ad network is wired, rather than returning a campaign that does not exist — a campaign row marked live with nothing behind it would read as spending your money when it is not.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAdOffsiteCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_ad_offsite_resume

> crate::models::CreateAdOffsiteResumeResponse create_ad_offsite_resume()
Clear an auto-pause and resume offsite spend.

Returns 409 when you are not actually paused. The pause reason is worth reading first — resuming without changing anything will usually just trip the floor again over the next window.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAdOffsiteResumeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_ad_bulk_add

> crate::models::CreateListingAdBulkAddResponse create_listing_ad_bulk_add()
Add listings to a campaign by master listing id: each listing's live row on `platform` joins campaignId (a Crossly campaign uuid, or the marketplace's campaign id). A listing not live there is its own failed result.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingAdBulkAddResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_ad_campaign

> crate::models::CreateListingAdCampaignResponse create_listing_ad_campaign(platform)
Create a campaign on the marketplace: program (promoted_listing_rate = eBay General, a % of the sale; cpc_campaign = eBay Priority, a daily budget), name, ratePct or dailyBudgetCents, startsAt/endsAt, and the program fields. Idempotency-Key supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateListingAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_ad_campaign_listing

> crate::models::CreateListingAdCampaignListingResponse create_listing_ad_campaign_listing(platform, campaign_id)
Promote listings in a marketplace campaign. platformListingIds are Crossly platform-listing ids on that marketplace; ratePct sets the ad rate (eBay General). One result per listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**campaign_id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingAdCampaignListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_ad_campaign_listing_remove

> crate::models::CreateListingAdCampaignListingResponse create_listing_ad_campaign_listing_remove(platform, campaign_id)
Stop promoting listings in a marketplace campaign. One result per listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**campaign_id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingAdCampaignListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sponsored_ad_billing_topup_link

> crate::models::CreateSponsoredAdBillingTopupLinkResponse create_sponsored_ad_billing_topup_link()
A Stripe Checkout link that adds amountCents ($5–$10,000) to the ad balance when the seller pays it. Nothing is charged until they do. Use an Idempotency-Key to get the same link back on a retry.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSponsoredAdBillingTopupLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sponsored_ad_boost_remove

> crate::models::CreateSponsoredAdBoostRemoveResponse create_sponsored_ad_boost_remove()
Stop boosting listings in bulk.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSponsoredAdBoostRemoveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sponsored_ad_campaign

> crate::models::SponsoredAdCampaignResponse create_sponsored_ad_campaign()
Create a Sponsored Listings campaign: name, dailyBudgetCents (≥100), optional totalBudgetCents, defaultBidCents (5–10000; the max bid under bidStrategy \"auto\"), placements (search, browse, product_page, home_rail, storefront, worlds_surface), goal (sales | visits), optional targetCategories / targetRegions (ISO countries) / targetDevices (mobile, desktop, app), optional startsAt/endsAt, listings [{platformListingId, bidCents?, boostBps? (100–2000, pay-on-sale Boost; null clears)}], keywords [{keyword, matchType broad|phrase|exact, negative?, bidCents?}], worldTargets [{kind showroom|convention, id, bidCents?, status?}] (your showroom or a convention you host, promoted in the Crossly World directory; at least one listing or world target), status active|draft. A catalog boost: targetType \"wholesale_catalog\" + targetCatalogId (your catalog), no listings, keywords or world targets; it ranks Sponsored in the wholesale directory. bidStrategy defaults to your account default. Clicks are charged from your ad funds (ad balance and/or CBX, ad credit first); with none it waits as out_of_budget.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::SponsoredAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sponsored_ad_campaign_duplicate

> crate::models::SponsoredAdCampaignResponse create_sponsored_ad_campaign_duplicate(id)
Copy a campaign (settings, listings, keywords) as a new draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SponsoredAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sponsored_ad_campaign_status

> crate::models::SponsoredAdCampaignResponse create_sponsored_ad_campaign_status(id)
Set a campaign active (launch/resume), paused, or ended (final). Resuming with no ad funds to pay for clicks leaves it out_of_budget until you top up.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SponsoredAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_ad_offsite

> crate::models::GetAdOffsiteResponse get_ad_offsite()
Your offsite-ads opt-in and its terms.

Offsite ads spend your CBX advertising budget on external networks — Google, Meta and similar — at our discretion. Your item gets promoted, and the traffic lands on Crossly. OFF unless you turn it on, and there is no path that enables it on your behalf. Media is passed through AT COST with a separate, disclosed management fee, so you can always see how much of your budget reached the auction. `minReturnBps` is a STOP, not a guarantee: below it, offsite spend auto-pauses and the rest of your budget reverts to on-platform placement. Nobody can honestly promise ad performance; what we can promise is that it stops.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAdOffsiteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_ad_offsite_eligibility

> crate::models::GetAdOffsiteEligibilityResponse get_ad_offsite_eligibility()
Whether an item can run offsite, and why not.

Reasons: `not_opted_in`, `auto_paused`, `network_not_allowed`, `category_not_allowlisted`, `no_adapter`, `no_budget`. The category check is an ALLOWLIST, so an unclassified category is not eligible. That is not bureaucracy: ad networks suspend the ACCOUNT over a prohibited item, and the account is one shared resource across every merchant using this — so one listing could take offsite ads away from all of them. A blocklist would fail open on the first category nobody thought of.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAdOffsiteEligibilityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_ad_offsite_report

> crate::models::GetAdOffsiteReportResponse get_ad_offsite_report()
What your offsite budget bought — including the misses.

Spend, impressions and clicks are reported whether or not anything converted. A report showing only conversions is a report nobody can audit, and this is a feature where you handed over discretion over your money. `mediaCostCents` versus `managementFeeCents` answers \"how much of my budget reached the auction\" — the two are recorded separately so the answer survives. `spilloverConversions` is sales your ads produced on OTHER sellers' items. Reported so you can see whether offsite traffic is reaching buyers who want your item — a high spillover rate is the signal to turn it off. `returnBps` is attributed revenue as bps of spend; compare it with your `minReturnBps` floor.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAdOffsiteReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_ad_listing

> crate::models::GetListingAdListingResponse get_listing_ad_listing(listing_id)
Every promotion of one listing: its Crossly campaigns and Boost, and each marketplace ad with status, campaign, rate and last sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing_id** | **String** |  | [required] |

### Return type

**crate::models::GetListingAdListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_billing

> crate::models::GetSponsoredAdBillingResponse get_sponsored_ad_billing()
What pays for your ads: headroom across the billing rails (ad balance and/or CBX), unbilled click spend, settlement charges, daily spend, Boost fees and CBX ad-credit grants; funds is the full funds summary (same as /v1/sponsored-ads/funds). refresh=true re-reads the balance.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSponsoredAdBillingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_billing_statement

> crate::models::GetSponsoredAdBillingStatementResponse get_sponsored_ad_billing_statement(month)
One month's ads statement: opening and closing ad balance, top-ups, click spend by campaign, how clicks were paid, credits, refunds and Boost fees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |

### Return type

**crate::models::GetSponsoredAdBillingStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_billing_statement_pdf

> std::path::PathBuf get_sponsored_ad_billing_statement_pdf(month)
One month's ads statement as a PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_billing_topup_receipt

> std::path::PathBuf get_sponsored_ad_billing_topup_receipt(id)
One ad balance top-up's receipt as a PDF (each top-up in the billing history carries its id and receipt number).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_campaign

> crate::models::SponsoredAdCampaignResponse get_sponsored_ad_campaign(id)
One campaign: settings, its listings (bid, suggested bid, live, per-listing stats) and keywords. A catalog boost (targetType wholesale_catalog) has `catalogs`: per boosted catalog impressions, clicks, spend, attributed trade orders and revenue, CTR and ROAS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SponsoredAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_fund

> crate::models::GetSponsoredAdFundResponse get_sponsored_ad_fund()
What can pay for clicks now: the billing rails in effect (ad balance and/or CBX), the ad balance and ad credit, CBX headroom, unbilled clicks, available cents, auto top-up, the saved payment method and the refundable balance. refresh=true re-reads CBX.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSponsoredAdFundResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_report

> crate::models::GetSponsoredAdReportResponse get_sponsored_ad_report()
Sponsored report for a range: totals (spend, impressions, clicks, CTR, average CPC, attributed orders and revenue, ROAS), a zero-filled daily series, per-campaign rows (targetType listings | wholesale_catalog) and `catalogs`, one row per boosted catalog. Listing orders are Crossly purchases of the advertised listing within 7 days of a click by the same buyer; catalog orders are trade orders with that seller within 7 days of a catalog click by the same buyer. Catalog impressions count once per viewer per day.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSponsoredAdReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sponsored_ad_setting

> crate::models::SponsoredAdResponse get_sponsored_ad_setting()
Ads quick settings: adsPaused, accountDailyCapCents and spentTodayCents, defaultBidStrategy, autoTopup.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::SponsoredAdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ad_campaigns

> crate::models::ListListingAdCampaignsResponse list_listing_ad_campaigns(platform)
Your ad campaigns on one marketplace (cached up to 15 minutes; refresh=true asks the marketplace now).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

**crate::models::ListListingAdCampaignsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ad_platforms

> crate::models::ListListingAdPlatformsResponse list_listing_ad_platforms()
The ads registry: each marketplace that sells promotion, its programs (kind, scope, campaign and listing fields), which verbs are wired / need a capture / are unsupported, and whether you are connected there.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListListingAdPlatformsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sponsored_ad_billing_history

> crate::models::ListSponsoredAdBillingHistoryResponse list_sponsored_ad_billing_history()
Billing history newest first: top-ups (status, receipt number), click charges (how each was paid: ad balance, credit, CBX, written off), credits, refunds and Boost fees. kind filters; before (ISO time) pages; limit ≤ 100.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSponsoredAdBillingHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sponsored_ad_billing_statements

> crate::models::ListSponsoredAdBillingStatementsResponse list_sponsored_ad_billing_statements()
Months (YYYY-MM) with any ad billing activity, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSponsoredAdBillingStatementsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sponsored_ad_boosts

> crate::models::ListSponsoredAdBoostsResponse list_sponsored_ad_boosts()
Boosted (pay-on-sale) listings: the rate, sales and fees taken so far. A boost lifts organic ranking by up to 10% and costs its rate only when the listing sells on Crossly.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSponsoredAdBoostsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sponsored_ad_campaigns

> crate::models::ListSponsoredAdCampaignsResponse list_sponsored_ad_campaigns()
Your Sponsored Listings campaigns with status, pacing (spent today, daily cap reached), budgets, bids, placements and stats for the range (from/to, YYYY-MM-DD; default last 30 days).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSponsoredAdCampaignsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sponsored_ad_listings

> crate::models::ListSponsoredAdListingsResponse list_sponsored_ad_listings()
Per-listing ad performance across campaigns for a range, plus suggestions: live listings with views but few sales that are not sponsored yet, with a suggested bid.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSponsoredAdListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_ad_offsite

> crate::models::UpdateAdOffsiteResponse update_ad_offsite()
Turn offsite ads on or off. Yours alone to set.

The toggle. Nothing else in the system sets `enabled` — no onboarding default, no bulk enable, no admin override. `maxOffsiteShareBps` caps how much of your budget may leave the platform, so opting in does not mean discovering the whole thing went to Google. `minReturnBps` sets the floor below which offsite spend auto-pauses: 20000 means we must return $2 of attributed revenue per $1 spent over the window. Toggling either way clears any existing auto-pause, so turning it back on later does not inherit a pause from months ago. When one of your ads brings a buyer who purchases somebody ELSE's item, that is reported to you as spillover so you can judge whether offsite traffic is reaching buyers who want YOUR item, and switch it off if not. It is not credited back — you bought clicks, which is how every ad market works.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateAdOffsiteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_ad_campaign

> crate::models::CreateListingAdCampaignResponse update_listing_ad_campaign(platform, campaign_id)
Change a marketplace campaign: name, dailyBudgetCents, endsAt (null clears), status paused | running | ended.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**campaign_id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_ad_listing_rate

> crate::models::UpdateListingAdListingRateResponse update_listing_ad_listing_rate(platform, platform_listing_id)
Change one promoted listing's ad rate (percent of the sale) in the campaign it is in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingAdListingRateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_sponsored_ad_billing_auto_topup

> crate::models::SponsoredAdResponse update_sponsored_ad_billing_auto_topup()
Auto top-up: enabled, thresholdCents (top up when the ad balance drops below it), amountCents (how much). Needs a saved payment method (the first top-up saves it).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::SponsoredAdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_sponsored_ad_boost

> crate::models::UpdateSponsoredAdBoostResponse update_sponsored_ad_boost()
Boost listings in bulk (platformListingIds, upliftBps 100–2000 = 1–20%), or change their rate. Your own live Crossly listings only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateSponsoredAdBoostResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_sponsored_ad_campaign

> crate::models::SponsoredAdCampaignResponse update_sponsored_ad_campaign(id)
Edit a campaign. Any of the create fields; listings and keywords, when given, replace the current set. Raising the total budget resumes a campaign stopped by it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SponsoredAdCampaignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_sponsored_ad_setting

> crate::models::SponsoredAdResponse update_sponsored_ad_setting()
Change ads quick settings: adsPaused (pause / resume every ad), accountDailyCapCents (null = no cap), defaultBidStrategy (auto | manual) for new campaigns, autoTopup {enabled, thresholdCents?, amountCents?}, railPreference (cash_first | cbx_first, both rails live only).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::SponsoredAdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

