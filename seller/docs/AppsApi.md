# \AppsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_app**](AppsApi.md#get_app) | **GET** /v1/apps/{clientId} | Get one listed app by client id or profile slug. 404 if it is not in the directory.
[**get_app_earning**](AppsApi.md#get_app_earning) | **GET** /v1/apps/{id}/earnings | Your OAuth app's revenue-share earnings: summary + ledger. Owner only.
[**get_app_listing_allowance**](AppsApi.md#get_app_listing_allowance) | **GET** /v1/apps/{id}/listing-allowance | Your OAuth app's listing allowance (read-only): whether staff let sellers list above their plan through it, its limitMode (separate pool or shared with the plan), limitPeriod (total or monthly), the per-seller limit and the platform scope. Owner only.
[**list_apps**](AppsApi.md#list_apps) | **GET** /v1/apps | Browse the public Crossly app directories: seller apps, buyer apps or both (search, category, sort, featured, paging).



## get_app

> crate::models::ListAppsItem get_app(client_id)
Get one listed app by client id or profile slug. 404 if it is not in the directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |

### Return type

**crate::models::ListAppsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_app_earning

> crate::models::GetAppEarningResponse get_app_earning(id)
Your OAuth app's revenue-share earnings: summary + ledger. Owner only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetAppEarningResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_app_listing_allowance

> crate::models::GetAppListingAllowanceResponse get_app_listing_allowance(id)
Your OAuth app's listing allowance (read-only): whether staff let sellers list above their plan through it, its limitMode (separate pool or shared with the plan), limitPeriod (total or monthly), the per-seller limit and the platform scope. Owner only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetAppListingAllowanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_apps

> crate::models::ListAppsResponse list_apps(directory, sort, featured, category, q, limit, cursor)
Browse the public Crossly app directories: seller apps, buyer apps or both (search, category, sort, featured, paging).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**featured** | Option<**String**> |  |  |
**category** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**cursor** | Option<**String**> |  |  |

### Return type

**crate::models::ListAppsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

