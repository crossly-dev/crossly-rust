# \ProductionApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_production_candidate_accept**](ProductionApi.md#create_production_candidate_accept) | **POST** /v1/production/candidates/{id}/accept | Accept a graduated design's production terms (edition size, cost, holder access).
[**create_production_candidate_po**](ProductionApi.md#create_production_candidate_po) | **POST** /v1/production/candidates/{id}/po | Ops: open a purchase order for a candidate. Admin PAT only.
[**create_production_candidate_vote**](ProductionApi.md#create_production_candidate_vote) | **POST** /v1/production/candidates/{id}/vote | Back (support: true) or un-back a design on the production ballot. One vote per person per design; voters must have bought on Crossly.
[**create_production_design_brief**](ProductionApi.md#create_production_design_brief) | **POST** /v1/production/designs/{cosmeticId}/brief | Set a design's production brief (materials, finish, box, reference images).
[**create_production_design_consent**](ProductionApi.md#create_production_design_consent) | **POST** /v1/production/designs/{cosmeticId}/consent | Opt a design in or out of physical production.
[**create_production_design_spec**](ProductionApi.md#create_production_design_spec) | **POST** /v1/production/designs/{cosmeticId}/spec | Set a design's production spec (category, materials, complexity, packaging).
[**create_production_estimate**](ProductionApi.md#create_production_estimate) | **POST** /v1/production/estimate | Estimate a run's landed cost from a spec + edition size (overshoots for safety).
[**create_production_run_purchase**](ProductionApi.md#create_production_run_purchase) | **POST** /v1/production/runs/{id}/purchase | Buy a limited-edition unit of a physical run (holder pricing applies).
[**create_receipt_relist**](ProductionApi.md#create_receipt_relist) | **POST** /v1/receipts/{id}/relist | Sell it again: a draft listing of something you bought on Crossly, carrying its purchase receipt so the next buyer inherits its provenance.
[**get_production_ballot**](ProductionApi.md#get_production_ballot) | **GET** /v1/production/ballot | The ballot: designs waiting for a physical run, most backed first, and whether you may vote.
[**get_production_candidate**](ProductionApi.md#get_production_candidate) | **GET** /v1/production/candidates | Your designs that have graduated toward a physical run.
[**get_production_design_pref**](ProductionApi.md#get_production_design_pref) | **GET** /v1/production/designs/{cosmeticId}/prefs | Read a design's production preferences (spec, brief, opt-out).
[**get_production_drop**](ProductionApi.md#get_production_drop) | **GET** /v1/production/drops | Limited-edition units you own, with fulfillment status.
[**get_production_earning**](ProductionApi.md#get_production_earning) | **GET** /v1/production/earnings | Your CBX earnings from physical production (royalties + balance + lines).
[**get_production_line**](ProductionApi.md#get_production_line) | **GET** /v1/production/lines | Your production lines dashboard (candidate + run + inventory).
[**get_production_po**](ProductionApi.md#get_production_po) | **GET** /v1/production/pos | Ops: list purchase orders (optionally for one candidate). Admin PAT only.
[**get_production_po_reconcile**](ProductionApi.md#get_production_po_reconcile) | **GET** /v1/production/pos/{poId}/reconcile | Ops: reconcile a PO's actuals against the estimate. Admin PAT only.
[**get_production_run**](ProductionApi.md#get_production_run) | **GET** /v1/production/runs | Browse open limited-edition physical runs (graduated designs).
[**get_shop_listing_provenance**](ProductionApi.md#get_shop_listing_provenance) | **GET** /v1/shop/listings/{slug}/provenance | Crossly provenance behind a shop listing: owners on Crossly and since when (never who or for how much). Null when not bought here.
[**update_production_po**](ProductionApi.md#update_production_po) | **PATCH** /v1/production/pos/{poId} | Ops: update a PO (supplier, freight, actuals, QC, status). Admin PAT only.



## create_production_candidate_accept

> crate::models::CreateProductionCandidateAcceptResponse create_production_candidate_accept(id)
Accept a graduated design's production terms (edition size, cost, holder access).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionCandidateAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_candidate_po

> crate::models::UpdateProductionPoResponse create_production_candidate_po(id)
Ops: open a purchase order for a candidate. Admin PAT only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateProductionPoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_candidate_vote

> crate::models::CreateProductionCandidateVoteResponse create_production_candidate_vote(id)
Back (support: true) or un-back a design on the production ballot. One vote per person per design; voters must have bought on Crossly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionCandidateVoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_design_brief

> crate::models::CreateProductionDesignSpecResponse create_production_design_brief(cosmetic_id)
Set a design's production brief (materials, finish, box, reference images).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosmetic_id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionDesignSpecResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_design_consent

> crate::models::CreateProductionDesignSpecResponse create_production_design_consent(cosmetic_id)
Opt a design in or out of physical production.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosmetic_id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionDesignSpecResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_design_spec

> crate::models::CreateProductionDesignSpecResponse create_production_design_spec(cosmetic_id)
Set a design's production spec (category, materials, complexity, packaging).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosmetic_id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionDesignSpecResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_estimate

> crate::models::CreateProductionEstimateResponse create_production_estimate()
Estimate a run's landed cost from a spec + edition size (overshoots for safety).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateProductionEstimateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_production_run_purchase

> crate::models::CreateProductionRunPurchaseResponse create_production_run_purchase(id)
Buy a limited-edition unit of a physical run (holder pricing applies).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateProductionRunPurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_receipt_relist

> crate::models::CreateReceiptRelistResponse create_receipt_relist(id)
Sell it again: a draft listing of something you bought on Crossly, carrying its purchase receipt so the next buyer inherits its provenance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateReceiptRelistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_ballot

> crate::models::GetProductionBallotResponse get_production_ballot()
The ballot: designs waiting for a physical run, most backed first, and whether you may vote.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionBallotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_candidate

> crate::models::GetProductionCandidateResponse get_production_candidate()
Your designs that have graduated toward a physical run.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionCandidateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_design_pref

> crate::models::GetProductionDesignPrefResponse get_production_design_pref(cosmetic_id)
Read a design's production preferences (spec, brief, opt-out).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosmetic_id** | **String** |  | [required] |

### Return type

**crate::models::GetProductionDesignPrefResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_drop

> crate::models::GetProductionDropResponse get_production_drop()
Limited-edition units you own, with fulfillment status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_earning

> crate::models::GetProductionEarningResponse get_production_earning()
Your CBX earnings from physical production (royalties + balance + lines).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionEarningResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_line

> crate::models::GetProductionLineResponse get_production_line()
Your production lines dashboard (candidate + run + inventory).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionLineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_po

> crate::models::GetProductionPoResponse get_production_po(candidate_id)
Ops: list purchase orders (optionally for one candidate). Admin PAT only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**candidate_id** | Option<**String**> |  |  |

### Return type

**crate::models::GetProductionPoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_po_reconcile

> crate::models::GetProductionPoReconcileResponse get_production_po_reconcile(po_id)
Ops: reconcile a PO's actuals against the estimate. Admin PAT only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**po_id** | **String** |  | [required] |

### Return type

**crate::models::GetProductionPoReconcileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_production_run

> crate::models::GetProductionRunResponse get_production_run()
Browse open limited-edition physical runs (graduated designs).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetProductionRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_shop_listing_provenance

> crate::models::GetShopListingProvenanceResponse get_shop_listing_provenance(slug)
Crossly provenance behind a shop listing: owners on Crossly and since when (never who or for how much). Null when not bought here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetShopListingProvenanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_production_po

> crate::models::UpdateProductionPoResponse update_production_po(po_id)
Ops: update a PO (supplier, freight, actuals, QC, status). Admin PAT only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**po_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateProductionPoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

