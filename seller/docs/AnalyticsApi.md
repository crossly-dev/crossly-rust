# \AnalyticsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_analytic_bookkeeping**](AnalyticsApi.md#get_analytic_bookkeeping) | **GET** /v1/analytics/bookkeeping | Monthly P&L + per-platform breakdown for a calendar year.
[**get_analytic_by_platform**](AnalyticsApi.md#get_analytic_by_platform) | **GET** /v1/analytics/by-platform | Sales + revenue grouped by platform for the last N days.
[**get_analytic_dashboard**](AnalyticsApi.md#get_analytic_dashboard) | **GET** /v1/analytics/dashboard | Composite dashboard: KPIs + breakdowns + recent activity.
[**get_analytic_item**](AnalyticsApi.md#get_analytic_item) | **GET** /v1/analytics/items | Per-item P&L for sold inventory.
[**get_analytic_summary**](AnalyticsApi.md#get_analytic_summary) | **GET** /v1/analytics/summary | Headline KPIs for the last N days.
[**get_analytic_timesery**](AnalyticsApi.md#get_analytic_timesery) | **GET** /v1/analytics/timeseries | Daily sales + revenue series for the last N days.
[**get_analytic_today**](AnalyticsApi.md#get_analytic_today) | **GET** /v1/analytics/today | Today's checklist + 14-day activity streak.
[**list_insight_by_platform**](AnalyticsApi.md#list_insight_by_platform) | **GET** /v1/insights/by-platform | Platform velocity + margin insight (90-day window).



## get_analytic_bookkeeping

> crate::models::GetAnalyticBookkeepingResponse get_analytic_bookkeeping()
Monthly P&L + per-platform breakdown for a calendar year.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticBookkeepingResponse**](GetAnalyticBookkeepingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_by_platform

> crate::models::GetAnalyticByPlatformResponse get_analytic_by_platform()
Sales + revenue grouped by platform for the last N days.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticByPlatformResponse**](GetAnalyticByPlatformResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_dashboard

> crate::models::GetAnalyticDashboardResponse get_analytic_dashboard()
Composite dashboard: KPIs + breakdowns + recent activity.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticDashboardResponse**](GetAnalyticDashboardResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_item

> crate::models::GetAnalyticItemResponse get_analytic_item()
Per-item P&L for sold inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticItemResponse**](GetAnalyticItemResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_summary

> crate::models::GetAnalyticSummaryResponse get_analytic_summary(days)
Headline KPIs for the last N days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |[default to 30]

### Return type

[**crate::models::GetAnalyticSummaryResponse**](GetAnalyticSummaryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_timesery

> crate::models::GetAnalyticTimeseryResponse get_analytic_timesery()
Daily sales + revenue series for the last N days.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticTimeseryResponse**](GetAnalyticTimeseryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytic_today

> crate::models::GetAnalyticTodayResponse get_analytic_today()
Today's checklist + 14-day activity streak.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAnalyticTodayResponse**](GetAnalyticTodayResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_insight_by_platform

> crate::models::V1List list_insight_by_platform()
Platform velocity + margin insight (90-day window).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

