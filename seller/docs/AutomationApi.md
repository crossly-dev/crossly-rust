# \AutomationApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_automation_rule**](AutomationApi.md#create_automation_rule) | **POST** /v1/automation/rules | Create an automation rule.
[**create_automation_rule_import**](AutomationApi.md#create_automation_rule_import) | **POST** /v1/automation/rules/import | Import one or more rules from recipe JSON (single or bundle).
[**create_automation_rule_run_now**](AutomationApi.md#create_automation_rule_run_now) | **POST** /v1/automation/rules/{id}/run-now | Fire an automation rule immediately.
[**create_automation_rule_toggle**](AutomationApi.md#create_automation_rule_toggle) | **POST** /v1/automation/rules/{id}/toggle | Flip an automation rule between active and inactive.
[**create_automation_rule_validate_recipe**](AutomationApi.md#create_automation_rule_validate_recipe) | **POST** /v1/automation/rules/validate-recipe | Dry-run validate one or more recipes against the live catalog.
[**delete_automation_rule**](AutomationApi.md#delete_automation_rule) | **DELETE** /v1/automation/rules/{id} | Delete an automation rule.
[**get_automation_catalog**](AutomationApi.md#get_automation_catalog) | **GET** /v1/automation/catalog | Supported triggerType / actionType / conditionType values for automation rules.
[**get_automation_rule**](AutomationApi.md#get_automation_rule) | **GET** /v1/automation/rules/{id} | Get a single automation rule.
[**get_automation_rule_export**](AutomationApi.md#get_automation_rule_export) | **GET** /v1/automation/rules/export | Export the user's full rule library as a portable recipe bundle.
[**get_automation_rule_export_by_id**](AutomationApi.md#get_automation_rule_export_by_id) | **GET** /v1/automation/rules/{id}/export | Export a single automation rule as a portable recipe.
[**list_automation_rules**](AutomationApi.md#list_automation_rules) | **GET** /v1/automation/rules | List automation rules.
[**list_automation_runs**](AutomationApi.md#list_automation_runs) | **GET** /v1/automation/runs | Per-fire history for automation rules and workflow chain runs.
[**update_automation_rule**](AutomationApi.md#update_automation_rule) | **PUT** /v1/automation/rules/{id} | Update an automation rule (full replace).



## create_automation_rule

> crate::models::CreateAutomationRuleResponse create_automation_rule()
Create an automation rule.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAutomationRuleResponse**](CreateAutomationRuleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_automation_rule_import

> crate::models::CreateAutomationRuleImportResponse create_automation_rule_import(activate)
Import one or more rules from recipe JSON (single or bundle).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::CreateAutomationRuleImportResponse**](CreateAutomationRuleImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_automation_rule_run_now

> crate::models::CreateAutomationRuleRunNowResponse create_automation_rule_run_now(id)
Fire an automation rule immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateAutomationRuleRunNowResponse**](CreateAutomationRuleRunNowResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_automation_rule_toggle

> crate::models::CreateAutomationRuleToggleResponse create_automation_rule_toggle(id)
Flip an automation rule between active and inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateAutomationRuleToggleResponse**](CreateAutomationRuleToggleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_automation_rule_validate_recipe

> crate::models::CreateAutomationRuleValidateRecipeResponse create_automation_rule_validate_recipe()
Dry-run validate one or more recipes against the live catalog.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAutomationRuleValidateRecipeResponse**](CreateAutomationRuleValidateRecipeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_automation_rule

> crate::models::DeleteAutomationRuleResponse delete_automation_rule(id)
Delete an automation rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteAutomationRuleResponse**](DeleteAutomationRuleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automation_catalog

> crate::models::GetAutomationCatalogResponse get_automation_catalog()
Supported triggerType / actionType / conditionType values for automation rules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAutomationCatalogResponse**](GetAutomationCatalogResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automation_rule

> crate::models::GetAutomationRuleResponse get_automation_rule(id)
Get a single automation rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetAutomationRuleResponse**](GetAutomationRuleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automation_rule_export

> crate::models::GetAutomationRuleExportResponse get_automation_rule_export()
Export the user's full rule library as a portable recipe bundle.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAutomationRuleExportResponse**](GetAutomationRuleExportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_automation_rule_export_by_id

> crate::models::GetAutomationRuleExportByIdResponse get_automation_rule_export_by_id(id)
Export a single automation rule as a portable recipe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetAutomationRuleExportByIdResponse**](GetAutomationRuleExportByIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_automation_rules

> crate::models::V1List list_automation_rules()
List automation rules.

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


## list_automation_runs

> crate::models::V1List list_automation_runs(rule_id, chain_id, limit)
Per-fire history for automation rules and workflow chain runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | Option<**String**> |  |  |
**chain_id** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_automation_rule

> crate::models::UpdateAutomationRuleResponse update_automation_rule(id)
Update an automation rule (full replace).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateAutomationRuleResponse**](UpdateAutomationRuleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

