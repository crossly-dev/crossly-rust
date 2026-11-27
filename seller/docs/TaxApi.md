# \TaxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mileage**](TaxApi.md#create_mileage) | **POST** /v1/mileage | Create a mileage entry.
[**delete_mileage**](TaxApi.md#delete_mileage) | **DELETE** /v1/mileage/{id} | Delete a mileage entry.
[**get_book_tax_profile**](TaxApi.md#get_book_tax_profile) | **GET** /v1/books/tax-profile | Your tax profile: time zone for books boundaries, filing status, state, set-aside and rate assumptions, accounting method, and business name, EIN and address.
[**get_mileage_summary**](TaxApi.md#get_mileage_summary) | **GET** /v1/mileage/summary | Annual mileage totals + IRS deduction. Each trip is priced at the IRS rate on its own date (2026 has two rates, listed in `rates`); `ratePerMile` is the January rate.
[**get_order_sales_tax**](TaxApi.md#get_order_sales_tax) | **GET** /v1/orders/{id}/sales-tax | Who collected sales tax on one Crossly marketplace order, where, and the seller notice.
[**get_tax_sales_tax**](TaxApi.md#get_tax_sales_tax) | **GET** /v1/tax/sales-tax | Crossly marketplace sales Crossly did not collect sales tax on, by jurisdiction.
[**get_tax_schedule_c**](TaxApi.md#get_tax_schedule_c) | **GET** /v1/tax/schedule-c | Schedule C JSON for a tax year (default the current year), from the same P&L as GET /v1/books: line 1 gross receipts before refunds, line 2 refunds, COGS in Part III, completed sales only. GET /v1/books has the full P&L by period.
[**list_mileage**](TaxApi.md#list_mileage) | **GET** /v1/mileage | List mileage entries, newest trip first, paged (limit up to 500). Narrow by year and/or an inclusive from/to date range.
[**list_tax_crossly_collects**](TaxApi.md#list_tax_crossly_collects) | **GET** /v1/tax/crossly-collects | Where Crossly collects sales tax today (its active Stripe Tax registrations).
[**list_tax_sales_tax_not_collected**](TaxApi.md#list_tax_sales_tax_not_collected) | **GET** /v1/tax/sales-tax/not-collected | The individual Crossly sales Crossly did not collect sales tax on, newest first.
[**update_book_tax_profile**](TaxApi.md#update_book_tax_profile) | **PUT** /v1/books/tax-profile | Update your tax profile: time zone, filing status, state, set-aside and rate assumptions (basis points), accounting method, business name, EIN and address. Fields left out are unchanged.
[**update_mileage**](TaxApi.md#update_mileage) | **PATCH** /v1/mileage/{id} | Update a mileage entry.



## create_mileage

> crate::models::CreateMileageResponse create_mileage()
Create a mileage entry.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateMileageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_mileage

> crate::models::OkResponse delete_mileage(id)
Delete a mileage entry.

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


## get_book_tax_profile

> crate::models::GetBookTaxProfileResponse get_book_tax_profile()
Your tax profile: time zone for books boundaries, filing status, state, set-aside and rate assumptions, accounting method, and business name, EIN and address.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBookTaxProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_mileage_summary

> crate::models::GetMileageSummaryResponse get_mileage_summary(year)
Annual mileage totals + IRS deduction. Each trip is priced at the IRS rate on its own date (2026 has two rates, listed in `rates`); `ratePerMile` is the January rate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Calendar tax year. Default the current year. |  |

### Return type

**crate::models::GetMileageSummaryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_sales_tax

> crate::models::GetOrderSalesTaxResponse get_order_sales_tax(id)
Who collected sales tax on one Crossly marketplace order, where, and the seller notice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderSalesTaxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tax_sales_tax

> crate::models::GetTaxSalesTaxResponse get_tax_sales_tax(from, to)
Crossly marketplace sales Crossly did not collect sales tax on, by jurisdiction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | YYYY-MM-DD, inclusive. Default 1 January this year. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive. Default now. |  |

### Return type

**crate::models::GetTaxSalesTaxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tax_schedule_c

> crate::models::GetTaxScheduleCResponse get_tax_schedule_c()
Schedule C JSON for a tax year (default the current year), from the same P&L as GET /v1/books: line 1 gross receipts before refunds, line 2 refunds, COGS in Part III, completed sales only. GET /v1/books has the full P&L by period.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetTaxScheduleCResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_mileage

> crate::models::ListMileageResponse list_mileage(year, from, to, page, limit)
List mileage entries, newest trip first, paged (limit up to 500). Narrow by year and/or an inclusive from/to date range.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Only trips dated in this calendar year. |  |
**from** | Option<**String**> | YYYY-MM-DD, inclusive. Only trips on or after this date. Combines with year. |  |
**to** | Option<**String**> | YYYY-MM-DD, inclusive. Only trips on or before this date; from <= to. |  |
**page** | Option<**i32**> | 1-indexed page. Default 1. |  |
**limit** | Option<**i32**> | Rows per page, default 100, clamped to 500. |  |

### Return type

**crate::models::ListMileageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tax_crossly_collects

> crate::models::ListTaxCrosslyCollectsResponse list_tax_crossly_collects()
Where Crossly collects sales tax today (its active Stripe Tax registrations).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListTaxCrosslyCollectsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tax_sales_tax_not_collected

> crate::models::ListTaxSalesTaxNotCollectedResponse list_tax_sales_tax_not_collected(from, to, page, limit)
The individual Crossly sales Crossly did not collect sales tax on, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListTaxSalesTaxNotCollectedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_book_tax_profile

> crate::models::GetBookTaxProfileResponse update_book_tax_profile()
Update your tax profile: time zone, filing status, state, set-aside and rate assumptions (basis points), accounting method, business name, EIN and address. Fields left out are unchanged.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBookTaxProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_mileage

> crate::models::CreateMileageResponse update_mileage(id)
Update a mileage entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateMileageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

