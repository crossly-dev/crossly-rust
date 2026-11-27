# \AnalyticsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analytic_experiment**](AnalyticsApi.md#create_analytic_experiment) | **POST** /v1/analytics/experiments | Start a listing experiment. split: 2-4 live platform listings observed over the same window; sequential: one platform listing, before vs after changeAt (optionally applying a new price).
[**create_analytic_experiment_declare**](AnalyticsApi.md#create_analytic_experiment_declare) | **POST** /v1/analytics/experiments/{id}/declare | Record the outcome: a winning arm, no difference, or inconclusive. Concludes the experiment.
[**create_analytic_experiment_stop**](AnalyticsApi.md#create_analytic_experiment_stop) | **POST** /v1/analytics/experiments/{id}/stop | Stop an experiment: no more data is counted after now.
[**delete_analytic_experiment**](AnalyticsApi.md#delete_analytic_experiment) | **DELETE** /v1/analytics/experiments/{id} | Delete an experiment. The listings are untouched.
[**get_analytic_bookkeeping**](AnalyticsApi.md#get_analytic_bookkeeping) | **GET** /v1/analytics/bookkeeping | Monthly P&L + per-platform breakdown for a calendar year.
[**get_analytic_by_platform**](AnalyticsApi.md#get_analytic_by_platform) | **GET** /v1/analytics/by-platform | Sales, net sales, fees and net profit by platform for the last N days.
[**get_analytic_compare**](AnalyticsApi.md#get_analytic_compare) | **GET** /v1/analytics/compare | Compare 2-4 of your listings: columns of price, quality, channels, views, favorites, offers and outcome, engagement curves, the fields that differ, and observational tests of favorite rate, view-to-offer rate and views per day.
[**get_analytic_dashboard**](AnalyticsApi.md#get_analytic_dashboard) | **GET** /v1/analytics/dashboard | Composite dashboard: KPIs + breakdowns + recent activity.
[**get_analytic_experiment**](AnalyticsApi.md#get_analytic_experiment) | **GET** /v1/analytics/experiments | Your listing A/B experiments, newest first, with status, days running and the current verdict.
[**get_analytic_experiment_by_id**](AnalyticsApi.md#get_analytic_experiment_by_id) | **GET** /v1/analytics/experiments/{id} | One experiment: arms with their windows, exposure and rates, the primary-metric test per arm against A, guardrail tests, the daily series and what to do next.
[**get_analytic_inventory**](AnalyticsApi.md#get_analytic_inventory) | **GET** /v1/analytics/inventory | Unsold stock: aging buckets with cost, listed value and market value, the 20 listings most in need of attention, shelf value over time, cohort sell-through at the chosen maturity, listed vs sold velocity and data health.
[**get_analytic_item**](AnalyticsApi.md#get_analytic_item) | **GET** /v1/analytics/items | Per-item P&L for sold inventory.
[**get_analytic_item_by_listing_id**](AnalyticsApi.md#get_analytic_item_by_listing_id) | **GET** /v1/analytics/items/{listingId} | One listing in depth: channels with views, favorites and offers, the merged timeline, engagement series, price history, comps, money waterfall, days on market, offers, messages, experiments and suggestions.
[**get_analytic_marketplace**](AnalyticsApi.md#get_analytic_marketplace) | **GET** /v1/analytics/marketplaces | Every marketplace side by side: net sales and profit, units, ASP, take rate, margin, shipping subsidy, days to sell, sell-through, cancel and return rates, offers and engagement; with vs=2-3 platforms, a head-to-head over listings live on all of them.
[**get_analytic_marketplace_by_platform**](AnalyticsApi.md#get_analytic_marketplace_by_platform) | **GET** /v1/analytics/marketplaces/{platform} | One marketplace in depth: its row, the money series, fee breakdown, days-to-sell and price-realization histograms, categories, top items, at-risk listings, a day-by-hour sales heatmap and offer stats.
[**get_analytic_overview**](AnalyticsApi.md#get_analytic_overview) | **GET** /v1/analytics/overview | KPIs with deltas (net sales, net profit, units, ASP, margin, sell-through, median days to sell, active listings, take rate), the trend series, by-marketplace split, top categories, brands and items, sell-through cohort, data health and insights.
[**get_analytic_performance**](AnalyticsApi.md#get_analytic_performance) | **GET** /v1/analytics/performance | Per-listing performance: channels, views, favorites, offers and conversations in the period, units sold, net sales and profit, margin, days to sell and a 0-100 health score, paged and sortable.
[**get_analytic_program**](AnalyticsApi.md#get_analytic_program) | **GET** /v1/analytics/programs/{program} | One program's report over a date range (default the last 30 days, at most 366): KPIs, a daily trend and a breakdown table. Money in integer cents.
[**get_analytic_segment**](AnalyticsApi.md#get_analytic_segment) | **GET** /v1/analytics/segments | Niches along one dimension: listed and active counts, units sold, cohort sell-through, median days to sell, net sales and profit, margin, ROI, ASP and discount from ask; rows under minN are flagged lowSample.
[**get_analytic_segment_by_key**](AnalyticsApi.md#get_analytic_segment_by_key) | **GET** /v1/analytics/segments/{dim}/{key} | One niche in depth: its row, series, by-marketplace split, days-to-sell histogram, price bands, top items and the 10 oldest unsold listings in it.
[**get_analytic_summary**](AnalyticsApi.md#get_analytic_summary) | **GET** /v1/analytics/summary | Headline KPIs for the last N days (the same numbers as the analytics overview).
[**get_analytic_timesery**](AnalyticsApi.md#get_analytic_timesery) | **GET** /v1/analytics/timeseries | Sales and net sales over the last N days, zero-filled, one row per calendar day.
[**get_analytic_today**](AnalyticsApi.md#get_analytic_today) | **GET** /v1/analytics/today | Today's checklist + 14-day activity streak.
[**list_insight_by_platform**](AnalyticsApi.md#list_insight_by_platform) | **GET** /v1/insights/by-platform | Platform velocity + margin insight (90-day window): units, net sales, median days to sell, ASP and net after the real marketplace fees.
[**update_analytic_experiment**](AnalyticsApi.md#update_analytic_experiment) | **PATCH** /v1/analytics/experiments/{id} | Rename an experiment or change its hypothesis, notes or planned end.



## create_analytic_experiment

> crate::models::AnalyticExperimentResponse create_analytic_experiment()
Start a listing experiment. split: 2-4 live platform listings observed over the same window; sequential: one platform listing, before vs after changeAt (optionally applying a new price).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::AnalyticExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_analytic_experiment_declare

> crate::models::AnalyticExperimentResponse create_analytic_experiment_declare(id)
Record the outcome: a winning arm, no difference, or inconclusive. Concludes the experiment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The experiment id (a uuid; any other id is a 404). | [required] |

### Return type

**crate::models::AnalyticExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_analytic_experiment_stop

> crate::models::AnalyticExperimentResponse create_analytic_experiment_stop(id)
Stop an experiment: no more data is counted after now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The experiment id (a uuid; any other id is a 404). | [required] |

### Return type

**crate::models::AnalyticExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_analytic_experiment

> crate::models::DeleteStoryResponse delete_analytic_experiment(id)
Delete an experiment. The listings are untouched.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The experiment id (a uuid; any other id is a 404). | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_bookkeeping

> crate::models::GetAnalyticBookkeepingResponse get_analytic_bookkeeping()
Monthly P&L + per-platform breakdown for a calendar year.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAnalyticBookkeepingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_by_platform

> crate::models::GetAnalyticByPlatformResponse get_analytic_by_platform(days, tz)
Sales, net sales, fees and net profit by platform for the last N days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Calendar days back, today included. Larger values are clamped to 365. |  |[default to 30]
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. |  |

### Return type

**crate::models::GetAnalyticByPlatformResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_compare

> crate::models::GetAnalyticCompareResponse get_analytic_compare(ids, align, platform, tz)
Compare 2-4 of your listings: columns of price, quality, channels, views, favorites, offers and outcome, engagement curves, the fields that differ, and observational tests of favorite rate, view-to-offer rate and views per day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated listing ids, 2 to 4. The first is arm A. | [required] |
**align** | Option<**String**> | age = x is days since first listed (default); calendar = x is the date. |  |
**platform** | Option<**String**> | Restrict engagement to one channel (platform id). |  |
**tz** | Option<**String**> | IANA time zone. Default UTC. |  |

### Return type

**crate::models::GetAnalyticCompareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_dashboard

> crate::models::GetAnalyticDashboardResponse get_analytic_dashboard()
Composite dashboard: KPIs + breakdowns + recent activity.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAnalyticDashboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_experiment

> crate::models::GetAnalyticExperimentResponse get_analytic_experiment(status, page, size)
Your listing A/B experiments, newest first, with status, days running and the current verdict.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Default all. |  |
**page** | Option<**i32**> | 1-indexed page. Default 1. |  |
**size** | Option<**i32**> | Rows per page. Default 20. |  |

### Return type

**crate::models::GetAnalyticExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_experiment_by_id

> crate::models::GetAnalyticExperimentByIdResponse get_analytic_experiment_by_id(id)
One experiment: arms with their windows, exposure and rates, the primary-metric test per arm against A, guardrail tests, the daily series and what to do next.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The experiment id (a uuid). | [required] |

### Return type

**crate::models::GetAnalyticExperimentByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_inventory

> crate::models::GetAnalyticInventoryResponse get_analytic_inventory(range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz, maturity)
Unsold stock: aging buckets with cost, listed value and market value, the 20 listings most in need of attention, shelf value over time, cohort sell-through at the chosen maturity, listed vs sold velocity and data health.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |
**maturity** | Option<**i32**> | Days an item has to sell within to count as sold in the cohort. Default 30. |  |

### Return type

**crate::models::GetAnalyticInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_item

> crate::models::GetAnalyticItemResponse get_analytic_item()
Per-item P&L for sold inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAnalyticItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_item_by_listing_id

> crate::models::GetAnalyticItemByListingIdResponse get_analytic_item_by_listing_id(listing_id, by, tz)
One listing in depth: channels with views, favorites and offers, the merged timeline, engagement series, price history, comps, money waterfall, days on market, offers, messages, experiments and suggestions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing_id** | **String** | A listings.id you own (or an inventory item id with by=inventory). | [required] |
**by** | Option<**String**> | inventory = the id is an inventory item id; the answer is { redirectListingId }. Default listing. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. |  |

### Return type

**crate::models::GetAnalyticItemByListingIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_marketplace

> crate::models::GetAnalyticMarketplaceResponse get_analytic_marketplace(range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz, vs, sort, dir)
Every marketplace side by side: net sales and profit, units, ASP, take rate, margin, shipping subsidy, days to sell, sell-through, cancel and return rates, offers and engagement; with vs=2-3 platforms, a head-to-head over listings live on all of them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |
**vs** | Option<**String**> | Comma-separated 2-3 platform ids for the head-to-head. |  |
**sort** | Option<**String**> | Default netSales. |  |
**dir** | Option<**String**> | Default desc. |  |

### Return type

**crate::models::GetAnalyticMarketplaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_marketplace_by_platform

> crate::models::GetAnalyticMarketplaceByPlatformResponse get_analytic_marketplace_by_platform(platform, range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz)
One marketplace in depth: its row, the money series, fee breakdown, days-to-sell and price-realization histograms, categories, top items, at-risk listings, a day-by-hour sales heatmap and offer stats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | A platform id, e.g. ebay, poshmark. | [required] |
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |

### Return type

**crate::models::GetAnalyticMarketplaceByPlatformResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_overview

> crate::models::GetAnalyticOverviewResponse get_analytic_overview(range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz, stores)
KPIs with deltas (net sales, net profit, units, ASP, margin, sell-through, median days to sell, active listings, take rate), the trend series, by-marketplace split, top categories, brands and items, sell-through cohort, data health and insights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |
**stores** | Option<**String**> | Comma-separated store ids. |  |

### Return type

**crate::models::GetAnalyticOverviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_performance

> crate::models::GetAnalyticPerformanceResponse get_analytic_performance(range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz, status, q, sort, dir, page, size, min_days)
Per-listing performance: channels, views, favorites, offers and conversations in the period, units sold, net sales and profit, margin, days to sell and a 0-100 health score, paged and sortable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |
**status** | Option<**String**> | unsold = listed at least minDays with no completed sale. |  |
**q** | Option<**String**> | Title, SKU or brand contains. |  |
**sort** | Option<**String**> | Default lastActivity. |  |
**dir** | Option<**String**> | Default desc. |  |
**page** | Option<**i32**> | 1-indexed page. Default 1. |  |
**size** | Option<**i32**> | Rows per page. Default 50. |  |
**min_days** | Option<**i32**> | Minimum days listed (for status=unsold). Default 0. |  |

### Return type

**crate::models::GetAnalyticPerformanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_program

> crate::models::GetAnalyticProgramResponse get_analytic_program(program, from, to, by)
One program's report over a date range (default the last 30 days, at most 366): KPIs, a daily trend and a breakdown table. Money in integer cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program** | **String** |  | [required] |
**from** | Option<**String**> | First UTC day (YYYY-MM-DD), inclusive. |  |
**to** | Option<**String**> | Last UTC day (YYYY-MM-DD), inclusive. Default today. |  |
**by** | Option<**String**> | Breakdown dimension, when the program offers one (worlds: showroom, property, event). |  |

### Return type

**crate::models::GetAnalyticProgramResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_segment

> crate::models::GetAnalyticSegmentResponse get_analytic_segment(range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz, dim, sort, dir, min_n, q)
Niches along one dimension: listed and active counts, units sold, cohort sell-through, median days to sell, net sales and profit, margin, ROI, ASP and discount from ask; rows under minN are flagged lowSample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |
**dim** | Option<**String**> | category = main category; subcategory = main > sub; brand; condition; size; source = where you acquired it; department; priceBand = ask in <$10, $10-25, $25-50, $50-100, $100-250, $250+; ageBand = days live at sale or now (<7, 7-30, 30-90, 90+). |  |
**sort** | Option<**String**> | Default netSales. |  |
**dir** | Option<**String**> | Default desc. |  |
**min_n** | Option<**i32**> | Minimum sample for a row to count as a winner or loser. Default 3. |  |
**q** | Option<**String**> | Segment label contains. |  |

### Return type

**crate::models::GetAnalyticSegmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_segment_by_key

> crate::models::GetAnalyticSegmentByKeyResponse get_analytic_segment_by_key(dim, key, range, from, to, compare, cmp_from, cmp_to, labels, platforms, category, brand, tz)
One niche in depth: its row, series, by-marketplace split, days-to-sell histogram, price bands, top items and the 10 oldest unsold listings in it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dim** | **String** | One of category, subcategory, brand, condition, size, source, department, priceBand, ageBand. category = main category; subcategory = main > sub; brand; condition; size; source = where you acquired it; department; priceBand = ask in <$10, $10-25, $25-50, $50-100, $100-250, $250+; ageBand = days live at sale or now (<7, 7-30, 30-90, 90+). | [required] |
**key** | **String** | The segment key from a segments row (URL-encoded). A key with no data in scope is a 404. | [required] |
**range** | Option<**String**> | Reporting window. Default 30d. custom needs from and to. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive, in tz. Only with range=custom. |  |
**compare** | Option<**String**> | Comparison window for deltas: prev (same length just before), yoy, none, custom (cmpFrom/cmpTo). |  |
**cmp_from** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**cmp_to** | Option<**String**> | YYYY-MM-DD. Only with compare=custom. |  |
**labels** | Option<**String**> | Comma-separated inventory labels (any of them). |  |
**platforms** | Option<**String**> | Comma-separated platform ids. |  |
**category** | Option<**String**> | Canonical main-category key. |  |
**brand** | Option<**String**> | Brand name. |  |
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. An invalid name is a 400. |  |

### Return type

**crate::models::GetAnalyticSegmentByKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_summary

> crate::models::GetAnalyticSummaryResponse get_analytic_summary(days, tz)
Headline KPIs for the last N days (the same numbers as the analytics overview).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Calendar days back, today included. Larger values are clamped to 365. |  |[default to 30]
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. |  |

### Return type

**crate::models::GetAnalyticSummaryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_timesery

> crate::models::GetAnalyticTimeseryResponse get_analytic_timesery(days, tz)
Sales and net sales over the last N days, zero-filled, one row per calendar day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Calendar days back, today included. Larger values are clamped to 365. |  |[default to 30]
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. |  |

### Return type

**crate::models::GetAnalyticTimeseryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_analytic_today

> crate::models::GetAnalyticTodayResponse get_analytic_today()
Today's checklist + 14-day activity streak.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAnalyticTodayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_insight_by_platform

> crate::models::ListInsightByPlatformResponse list_insight_by_platform(tz)
Platform velocity + margin insight (90-day window): units, net sales, median days to sell, ASP and net after the real marketplace fees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tz** | Option<**String**> | IANA time zone for day boundaries. Default UTC. |  |

### Return type

**crate::models::ListInsightByPlatformResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_analytic_experiment

> crate::models::AnalyticExperimentResponse update_analytic_experiment(id)
Rename an experiment or change its hypothesis, notes or planned end.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The experiment id (a uuid; any other id is a 404). | [required] |

### Return type

**crate::models::AnalyticExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

