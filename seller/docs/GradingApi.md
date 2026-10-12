# \GradingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_grading_direct**](GradingApi.md#create_grading_direct) | **POST** /v1/grading/direct | Track a card you sent on your own grader account (Direct). Crossly takes no money; it verifies the cert when it comes back.
[**create_grading_run_reserve**](GradingApi.md#create_grading_run_reserve) | **POST** /v1/grading/runs/{id}/reserve | Hold a slot in a Grading Run. Free now; charged on your saved card only if the run locks. Full-access token and Idempotency-Key required.
[**create_grading_submission_cancel**](GradingApi.md#create_grading_submission_cancel) | **POST** /v1/grading/submissions/{id}/cancel | Pull a reserved card out of its run. Only while it is reserved; nothing was charged.
[**create_grading_submission_result**](GradingApi.md#create_grading_submission_result) | **POST** /v1/grading/submissions/{id}/result | Direct only: enter the grade that came back. Writes it to the item and its listings, verified when the cert agrees.
[**create_grading_submission_shipped**](GradingApi.md#create_grading_submission_shipped) | **POST** /v1/grading/submissions/{id}/shipped | Say you mailed a charged card to the hub, with optional tracking.
[**create_grading_verify_cert**](GradingApi.md#create_grading_verify_cert) | **POST** /v1/grading/verify-cert | Look a cert number up with its grader. found=false when the grader has no record.
[**get_grading_worth**](GradingApi.md#get_grading_worth) | **GET** /v1/grading/worth/{inventoryItemId} | Worth grading? Raw value vs what each grade is worth at each grader, against the cost. Never predicts the grade.
[**list_grading_rate_card**](GradingApi.md#list_grading_rate_card) | **GET** /v1/grading/rate-card | Every grader and tier Crossly can send to, with the per-card price in cents (null = paused at the grader).
[**list_grading_runs**](GradingApi.md#list_grading_runs) | **GET** /v1/grading/runs | Grading Runs taking reservations now, soonest cutoff first. meta.open is false while runs are switched off.
[**list_grading_submissions**](GradingApi.md#list_grading_submissions) | **GET** /v1/grading/submissions | Your cards out for grading (runs and Direct), newest first. active=1 keeps only the ones still in flight.



## create_grading_direct

> crate::models::IdResponse create_grading_direct()
Track a card you sent on your own grader account (Direct). Crossly takes no money; it verifies the cert when it comes back.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::IdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_run_reserve

> crate::models::IdResponse create_grading_run_reserve(id)
Hold a slot in a Grading Run. Free now; charged on your saved card only if the run locks. Full-access token and Idempotency-Key required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_submission_cancel

> crate::models::OkResponse create_grading_submission_cancel(id)
Pull a reserved card out of its run. Only while it is reserved; nothing was charged.

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


## create_grading_submission_result

> crate::models::CreateGradingSubmissionResultResponse create_grading_submission_result(id)
Direct only: enter the grade that came back. Writes it to the item and its listings, verified when the cert agrees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateGradingSubmissionResultResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_submission_shipped

> crate::models::OkResponse create_grading_submission_shipped(id)
Say you mailed a charged card to the hub, with optional tracking.

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


## create_grading_verify_cert

> crate::models::CreateGradingVerifyCertResponse create_grading_verify_cert()
Look a cert number up with its grader. found=false when the grader has no record.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateGradingVerifyCertResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_grading_worth

> crate::models::GetGradingWorthResponse get_grading_worth(inventory_item_id)
Worth grading? Raw value vs what each grade is worth at each grader, against the cost. Never predicts the grade.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetGradingWorthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_rate_card

> crate::models::ListGradingRateCardResponse list_grading_rate_card()
Every grader and tier Crossly can send to, with the per-card price in cents (null = paused at the grader).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGradingRateCardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_runs

> crate::models::ListGradingRunsResponse list_grading_runs()
Grading Runs taking reservations now, soonest cutoff first. meta.open is false while runs are switched off.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGradingRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_submissions

> crate::models::ListGradingSubmissionsResponse list_grading_submissions(active)
Your cards out for grading (runs and Direct), newest first. active=1 keeps only the ones still in flight.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active** | Option<**String**> | Only cards that have not finished. |  |

### Return type

**crate::models::ListGradingSubmissionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

