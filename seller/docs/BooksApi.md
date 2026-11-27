# \BooksApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_book_expense**](BooksApi.md#create_book_expense) | **POST** /v1/books/expenses | Log a business expense: date, amountCents, a Schedule C category, vendor, description, receipt link, optionally recurring monthly or yearly.
[**delete_book_expense**](BooksApi.md#delete_book_expense) | **DELETE** /v1/books/expenses/{id} | Delete a logged expense. It leaves the P&L and the exports.
[**get_book**](BooksApi.md#get_book) | **GET** /v1/books | Profit and loss for a year and period on the Schedule C lines: income, COGS, gross profit, expenses (marketplace fees, postage, mileage, your categories), net profit, monthly table, per-marketplace table with 1099-K expectations, Part III COGS, estimated tax and sales tax collected by marketplaces. Money in cents.
[**get_book1099k**](BooksApi.md#get_book1099k) | **GET** /v1/books/1099k | Per marketplace for a year: the 1099-K gross you recorded beside the gross Crossly computed, the difference, and whether a form is expected (federal: over $20,000 and over 200 transactions).
[**get_book_expense**](BooksApi.md#get_book_expense) | **GET** /v1/books/expenses | Your logged business expenses in a year and period (recurring ones expanded), with their Schedule C category. Amounts in cents.
[**get_book_tax_profile**](BooksApi.md#get_book_tax_profile) | **GET** /v1/books/tax-profile | Your tax profile: time zone for books boundaries, filing status, state, set-aside and rate assumptions, accounting method, and business name, EIN and address.
[**get_book_transaction**](BooksApi.md#get_book_transaction) | **GET** /v1/books/transactions | The sale rows behind any P&L line, newest first: gross, refunded, fee and fee source, shipping, COGS, tax collected and net, in cents.
[**update_book1099k**](BooksApi.md#update_book1099k) | **PUT** /v1/books/1099k | Record what a marketplace reported on its 1099-K for a year (gross in cents, transactions, notes). Answers the year's reconciliation, as GET /v1/books/1099k does.
[**update_book_expense**](BooksApi.md#update_book_expense) | **PATCH** /v1/books/expenses/{id} | Change a logged expense.
[**update_book_tax_profile**](BooksApi.md#update_book_tax_profile) | **PUT** /v1/books/tax-profile | Update your tax profile: time zone, filing status, state, set-aside and rate assumptions (basis points), accounting method, business name, EIN and address. Fields left out are unchanged.



## create_book_expense

> crate::models::CreateBookExpenseResponse create_book_expense()
Log a business expense: date, amountCents, a Schedule C category, vendor, description, receipt link, optionally recurring monthly or yearly.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBookExpenseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_book_expense

> crate::models::DeleteStoryResponse delete_book_expense(id)
Delete a logged expense. It leaves the P&L and the exports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The expense id. | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_book

> crate::models::GetBookResponse get_book(year, period, tz)
Profit and loss for a year and period on the Schedule C lines: income, COGS, gross profit, expenses (marketplace fees, postage, mileage, your categories), net profit, monthly table, per-marketplace table with 1099-K expectations, Part III COGS, estimated tax and sales tax collected by marketplaces. Money in cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Tax year. Default the current year. |  |
**period** | Option<**String**> | fy = full year (default), ytd, q1-q4, m01-m12. Boundaries are in your tax-profile time zone. |  |
**tz** | Option<**String**> | IANA zone; default your tax profile zone. |  |

### Return type

**crate::models::GetBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_book1099k

> crate::models::GetBook1099kResponse get_book1099k(year)
Per marketplace for a year: the 1099-K gross you recorded beside the gross Crossly computed, the difference, and whether a form is expected (federal: over $20,000 and over 200 transactions).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Tax year. Default the current year. |  |

### Return type

**crate::models::GetBook1099kResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_book_expense

> crate::models::GetBookExpenseResponse get_book_expense(year, period, category, page, size)
Your logged business expenses in a year and period (recurring ones expanded), with their Schedule C category. Amounts in cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Tax year. Default the current year. |  |
**period** | Option<**String**> | fy = full year (default), ytd, q1-q4, m01-m12. Boundaries are in your tax-profile time zone. |  |
**category** | Option<**String**> | An expense category id, e.g. supplies, postage, software_subscriptions. |  |
**page** | Option<**i32**> | 1-indexed page. Default 1. |  |
**size** | Option<**i32**> | Rows per page, at most 100. Default 25. |  |

### Return type

**crate::models::GetBookExpenseResponse**

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


## get_book_transaction

> crate::models::GetBookTransactionResponse get_book_transaction(year, period, tz, platform, status, page, size)
The sale rows behind any P&L line, newest first: gross, refunded, fee and fee source, shipping, COGS, tax collected and net, in cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Tax year. Default the current year. |  |
**period** | Option<**String**> | fy = full year (default), ytd, q1-q4, m01-m12. Boundaries are in your tax-profile time zone. |  |
**tz** | Option<**String**> | IANA zone; default your tax profile zone. |  |
**platform** | Option<**String**> | One platform id. |  |
**status** | Option<**String**> | completed = counted in income; pending = details not in yet (no money); default all. |  |
**page** | Option<**i32**> | 1-indexed page. Default 1. |  |
**size** | Option<**i32**> | Rows per page, at most 100. Default 50. |  |

### Return type

**crate::models::GetBookTransactionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_book1099k

> crate::models::GetBook1099kResponse update_book1099k()
Record what a marketplace reported on its 1099-K for a year (gross in cents, transactions, notes). Answers the year's reconciliation, as GET /v1/books/1099k does.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBook1099kResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_book_expense

> crate::models::CreateBookExpenseResponse update_book_expense(id)
Change a logged expense.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The expense id. | [required] |

### Return type

**crate::models::CreateBookExpenseResponse**

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

