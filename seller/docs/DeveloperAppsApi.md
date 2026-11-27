# \DeveloperAppsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_developer_app_certification_submit**](DeveloperAppsApi.md#create_developer_app_certification_submit) | **POST** /v1/developer/apps/{appId}/certification/submit | Send the application to Crossly review. Needs a passing conformance run on this client version and report-menu evidence.
[**create_developer_app_certification_withdraw**](DeveloperAppsApi.md#create_developer_app_certification_withdraw) | **POST** /v1/developer/apps/{appId}/certification/withdraw | Take the application out of review to edit it.
[**create_developer_app_client_version**](DeveloperAppsApi.md#create_developer_app_client_version) | **POST** /v1/developer/apps/{appId}/client-versions | Register a new release of your certified client and start its conformance run. A pass on the same major version carries the certification automatically; a new major version goes to Crossly review; a failure keeps your previous certified version.
[**create_developer_app_client_version_run**](DeveloperAppsApi.md#create_developer_app_client_version_run) | **POST** /v1/developer/apps/{appId}/client-versions/{versionId}/run | Start a new conformance run for a release whose run failed, expired or was aborted.
[**create_developer_app_conformance_run**](DeveloperAppsApi.md#create_developer_app_conformance_run) | **POST** /v1/developer/apps/{appId}/conformance-runs | Start a conformance run: a test experience only your app, signed in as you, can join. Join it with POST /v1/worlds/experiences/{experienceId}/join and follow the instruction.
[**create_developer_app_conformance_run_abort**](DeveloperAppsApi.md#create_developer_app_conformance_run_abort) | **POST** /v1/developer/apps/{appId}/conformance-runs/{runId}/abort | Abort an open run.
[**create_developer_app_verification_check_domain**](DeveloperAppsApi.md#create_developer_app_verification_check_domain) | **POST** /v1/developer/apps/{appId}/verification/check-domain | Look up the DNS TXT record crossly-app-verification=<token> on your domain (or _crossly.<domain>) now.
[**create_developer_app_verification_submit**](DeveloperAppsApi.md#create_developer_app_verification_submit) | **POST** /v1/developer/apps/{appId}/verification/submit | Send the verification to Crossly review (each scope is decided separately).
[**create_developer_app_verification_withdraw**](DeveloperAppsApi.md#create_developer_app_verification_withdraw) | **POST** /v1/developer/apps/{appId}/verification/withdraw | Take the verification out of review to edit it.
[**get_developer_app_certification**](DeveloperAppsApi.md#get_developer_app_certification) | **GET** /v1/developer/apps/{appId}/certification | The Crossly Carnival client certification application (null until you create it): status, client, evidence, the passing run and what still blocks submission.
[**get_developer_app_client_version**](DeveloperAppsApi.md#get_developer_app_client_version) | **GET** /v1/developer/apps/{appId}/client-versions/{versionId} | One client release and its re-certification.
[**get_developer_app_conformance_run**](DeveloperAppsApi.md#get_developer_app_conformance_run) | **GET** /v1/developer/apps/{appId}/conformance-runs/{runId} | A run's step, instruction and every check with its verdict.
[**get_developer_app_health**](DeveloperAppsApi.md#get_developer_app_health) | **GET** /v1/developer/apps/{appId}/health | Your app's API health: write-request counts by status class (24 h, 7 d) across every seller using it, top error codes with advice, a retry-storm flag, the last 50 failed requests (message and correlation id only on requests made with your own account), and from Atlas: how many people authorized the app and with which scopes (counts, never who) and the token endpoint's OAuth errors by code (e.g. invalid_grant) with the latest one.
[**get_developer_app_profile**](DeveloperAppsApi.md#get_developer_app_profile) | **GET** /v1/developer/apps/{appId}/profile | Your app's public directory profile: every editable field, the listing status, what still needs review, and `preview` — the profile exactly as /seller-apps or /buyer-apps will show it.
[**get_developer_app_review**](DeveloperAppsApi.md#get_developer_app_review) | **GET** /v1/developer/apps/{appId}/review | Your app's Crossly Carnival client certification, verification and review history in one read.
[**get_developer_app_verification**](DeveloperAppsApi.md#get_developer_app_verification) | **GET** /v1/developer/apps/{appId}/verification | The app verification for worlds:publish / worlds:monetize (null until created): domain proof, policies, justifications and per-scope decisions.
[**list_developer_app_client_versions**](DeveloperAppsApi.md#list_developer_app_client_versions) | **GET** /v1/developer/apps/{appId}/client-versions | Releases of your certified client, newest first: the re-certification status (awaiting_run, running, carried, ops_review, approved, rejected, failed, superseded), the rule that decided it and its checks.
[**list_developer_app_conformance_runs**](DeveloperAppsApi.md#list_developer_app_conformance_runs) | **GET** /v1/developer/apps/{appId}/conformance-runs | Your app's conformance runs, newest first.
[**update_developer_app_certification**](DeveloperAppsApi.md#update_developer_app_certification) | **PUT** /v1/developer/apps/{appId}/certification | Create or edit the application: client name, platform, version, build and source URLs, description and evidence (report menu, chat, Policy UI). Changing the client clears the passing run.
[**update_developer_app_listing**](DeveloperAppsApi.md#update_developer_app_listing) | **PUT** /v1/developer/apps/{appId}/listing | Turn \"Listed publicly\" on or off. On: an app Crossly approved whose name, scopes and audience are unchanged is listed at once; anything else goes to review (pending_review). Off: unlisted immediately.
[**update_developer_app_profile**](DeveloperAppsApi.md#update_developer_app_profile) | **PATCH** /v1/developer/apps/{appId}/profile | Edit your app's directory profile: name, slug, tagline, description, longDescription (markdown), category, audience (auto|seller|buyer|both), logoUrl, bannerUrl (4:1, 1600×400), screenshots (≤6), demoVideoUrl, homepageUrl, installUrl, support/privacy/terms links, docsLinks [{label,url}] and pricingNote. Absent keys are kept, null clears. Images not on Crossly's CDN are copied onto it. Renaming a listed app or changing its audience sends it back to review.
[**update_developer_app_verification**](DeveloperAppsApi.md#update_developer_app_verification) | **PUT** /v1/developer/apps/{appId}/verification | Create or edit the verification: domain, requested scopes with a justification each, privacy policy, terms, support email, demo.



## create_developer_app_certification_submit

> crate::models::DeveloperAppCertificationResponse create_developer_app_certification_submit(app_id)
Send the application to Crossly review. Needs a passing conformance run on this client version and report-menu evidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppCertificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_certification_withdraw

> crate::models::DeveloperAppCertificationResponse create_developer_app_certification_withdraw(app_id)
Take the application out of review to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppCertificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_client_version

> crate::models::CreateDeveloperAppClientVersionResponse create_developer_app_client_version(app_id)
Register a new release of your certified client and start its conformance run. A pass on the same major version carries the certification automatically; a new major version goes to Crossly review; a failure keeps your previous certified version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::CreateDeveloperAppClientVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_client_version_run

> crate::models::CreateDeveloperAppClientVersionResponse create_developer_app_client_version_run(app_id, version_id)
Start a new conformance run for a release whose run failed, expired or was aborted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

**crate::models::CreateDeveloperAppClientVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_conformance_run

> crate::models::GetDeveloperAppConformanceRunResponse create_developer_app_conformance_run(app_id)
Start a conformance run: a test experience only your app, signed in as you, can join. Join it with POST /v1/worlds/experiences/{experienceId}/join and follow the instruction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppConformanceRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_conformance_run_abort

> crate::models::CreateDeveloperAppConformanceRunAbortResponse create_developer_app_conformance_run_abort(app_id, run_id)
Abort an open run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**run_id** | **String** |  | [required] |

### Return type

**crate::models::CreateDeveloperAppConformanceRunAbortResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_verification_check_domain

> crate::models::DeveloperAppVerificationResponse create_developer_app_verification_check_domain(app_id)
Look up the DNS TXT record crossly-app-verification=<token> on your domain (or _crossly.<domain>) now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_verification_submit

> crate::models::DeveloperAppVerificationResponse create_developer_app_verification_submit(app_id)
Send the verification to Crossly review (each scope is decided separately).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_developer_app_verification_withdraw

> crate::models::DeveloperAppVerificationResponse create_developer_app_verification_withdraw(app_id)
Take the verification out of review to edit it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_certification

> crate::models::GetDeveloperAppCertificationResponse get_developer_app_certification(app_id)
The Crossly Carnival client certification application (null until you create it): status, client, evidence, the passing run and what still blocks submission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppCertificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_client_version

> crate::models::GetDeveloperAppClientVersionResponse get_developer_app_client_version(app_id, version_id)
One client release and its re-certification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppClientVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_conformance_run

> crate::models::GetDeveloperAppConformanceRunResponse get_developer_app_conformance_run(app_id, run_id)
A run's step, instruction and every check with its verdict.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**run_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppConformanceRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_health

> crate::models::GetDeveloperAppHealthResponse get_developer_app_health(app_id)
Your app's API health: write-request counts by status class (24 h, 7 d) across every seller using it, top error codes with advice, a retry-storm flag, the last 50 failed requests (message and correlation id only on requests made with your own account), and from Atlas: how many people authorized the app and with which scopes (counts, never who) and the token endpoint's OAuth errors by code (e.g. invalid_grant) with the latest one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppHealthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_profile

> crate::models::GetDeveloperAppProfileResponse get_developer_app_profile(app_id)
Your app's public directory profile: every editable field, the listing status, what still needs review, and `preview` — the profile exactly as /seller-apps or /buyer-apps will show it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_review

> crate::models::GetDeveloperAppReviewResponse get_developer_app_review(app_id)
Your app's Crossly Carnival client certification, verification and review history in one read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_developer_app_verification

> crate::models::GetDeveloperAppVerificationResponse get_developer_app_verification(app_id)
The app verification for worlds:publish / worlds:monetize (null until created): domain proof, policies, justifications and per-scope decisions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::GetDeveloperAppVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_developer_app_client_versions

> crate::models::ListDeveloperAppClientVersionsResponse list_developer_app_client_versions(app_id)
Releases of your certified client, newest first: the re-certification status (awaiting_run, running, carried, ops_review, approved, rejected, failed, superseded), the rule that decided it and its checks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::ListDeveloperAppClientVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_developer_app_conformance_runs

> crate::models::ListDeveloperAppConformanceRunsResponse list_developer_app_conformance_runs(app_id)
Your app's conformance runs, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::ListDeveloperAppConformanceRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_developer_app_certification

> crate::models::DeveloperAppCertificationResponse update_developer_app_certification(app_id)
Create or edit the application: client name, platform, version, build and source URLs, description and evidence (report menu, chat, Policy UI). Changing the client clears the passing run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppCertificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_developer_app_listing

> crate::models::UpdateDeveloperAppListingResponse update_developer_app_listing(app_id)
Turn \"Listed publicly\" on or off. On: an app Crossly approved whose name, scopes and audience are unchanged is listed at once; anything else goes to review (pending_review). Off: unlisted immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateDeveloperAppListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_developer_app_profile

> crate::models::UpdateDeveloperAppProfileResponse update_developer_app_profile(app_id)
Edit your app's directory profile: name, slug, tagline, description, longDescription (markdown), category, audience (auto|seller|buyer|both), logoUrl, bannerUrl (4:1, 1600×400), screenshots (≤6), demoVideoUrl, homepageUrl, installUrl, support/privacy/terms links, docsLinks [{label,url}] and pricingNote. Absent keys are kept, null clears. Images not on Crossly's CDN are copied onto it. Renaming a listed app or changing its audience sends it back to review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateDeveloperAppProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_developer_app_verification

> crate::models::DeveloperAppVerificationResponse update_developer_app_verification(app_id)
Create or edit the verification: domain, requested scopes with a justification each, privacy policy, terms, support email, demo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

**crate::models::DeveloperAppVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

