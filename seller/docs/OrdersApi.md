# \OrdersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_order_bulk_delete**](OrdersApi.md#create_order_bulk_delete) | **POST** /v1/orders/bulk-delete | Bulk soft- or hard-delete orders by id.
[**create_order_bulk_export**](OrdersApi.md#create_order_bulk_export) | **POST** /v1/orders/bulk-export | Bulk export selected orders as CSV.
[**create_order_bulk_mark_disputed**](OrdersApi.md#create_order_bulk_mark_disputed) | **POST** /v1/orders/bulk-mark-disputed | Bulk mark orders as disputed with a shared reason.
[**create_order_bulk_mark_shipped**](OrdersApi.md#create_order_bulk_mark_shipped) | **POST** /v1/orders/bulk-mark-shipped | Bulk flip status=shipped, clear the reserved-stock marker, and notify each platform.
[**create_order_bulk_packing_slip**](OrdersApi.md#create_order_bulk_packing_slip) | **POST** /v1/orders/bulk-packing-slips | Bulk packing slips PDF.
[**create_order_cancel**](OrdersApi.md#create_order_cancel) | **POST** /v1/orders/{id}/cancel | Cancel an order on its marketplace.
[**create_order_dispute**](OrdersApi.md#create_order_dispute) | **POST** /v1/orders/{id}/dispute | Flag an order as disputed.
[**create_order_import**](OrdersApi.md#create_order_import) | **POST** /v1/orders/import | Manually (re-)pull one or more connected platforms' order history for a day-window.
[**create_order_label**](OrdersApi.md#create_order_label) | **POST** /v1/orders/{id}/label | Purchase EasyPost shipping label.
[**create_order_message**](OrdersApi.md#create_order_message) | **POST** /v1/orders/{id}/message | Message the order's buyer on the marketplace.
[**create_order_pull_platform_label**](OrdersApi.md#create_order_pull_platform_label) | **POST** /v1/orders/{id}/pull-platform-label | Pull a pre-paid label from cookie platforms (Poshmark/Mercari).
[**create_order_rate**](OrdersApi.md#create_order_rate) | **POST** /v1/orders/{id}/rates | EasyPost rate quote for an order.
[**create_order_refund**](OrdersApi.md#create_order_refund) | **POST** /v1/orders/{id}/refund | Issue a full or partial refund on the platform.
[**create_order_tracking**](OrdersApi.md#create_order_tracking) | **POST** /v1/orders/{id}/tracking | Submit tracking number + carrier and notify the platform.
[**get_order**](OrdersApi.md#get_order) | **GET** /v1/orders/{id} | Get one order.
[**get_order_cancel_eligibility**](OrdersApi.md#get_order_cancel_eligibility) | **GET** /v1/orders/{id}/cancel-eligibility | Whether this order can be cancelled, and which reason codes the marketplace accepts.
[**get_order_count**](OrdersApi.md#get_order_count) | **GET** /v1/orders/counts | Tab badge counts by status.
[**get_order_evidence**](OrdersApi.md#get_order_evidence) | **GET** /v1/orders/{id}/evidence | Everything recorded about how this order was packed and how it arrived.
[**get_order_packing_slip**](OrdersApi.md#get_order_packing_slip) | **GET** /v1/orders/{id}/packing-slip | Single-order packing slip PDF.
[**get_order_proof_of_delivery**](OrdersApi.md#get_order_proof_of_delivery) | **GET** /v1/orders/{id}/proof-of-delivery | Proof-of-delivery evidence from the carrier's scan record (PDF, or JSON).
[**get_order_shipment**](OrdersApi.md#get_order_shipment) | **GET** /v1/orders/{id}/shipments | List the parcels an order shipped in.
[**list_order_units**](OrdersApi.md#list_order_units) | **GET** /v1/orders/{id}/units | List the identified units that shipped on an order.
[**list_orders**](OrdersApi.md#list_orders) | **GET** /v1/orders | List orders.
[**update_order**](OrdersApi.md#update_order) | **PATCH** /v1/orders/{id} | Update order (status, notes, tracking).



## create_order_bulk_delete

> crate::models::CreateOrderBulkDeleteResponse create_order_bulk_delete()
Bulk soft- or hard-delete orders by id.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateOrderBulkDeleteResponse**](CreateOrderBulkDeleteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_bulk_export

> String create_order_bulk_export()
Bulk export selected orders as CSV.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_bulk_mark_disputed

> crate::models::CreateOrderBulkMarkDisputedResponse create_order_bulk_mark_disputed()
Bulk mark orders as disputed with a shared reason.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateOrderBulkMarkDisputedResponse**](CreateOrderBulkMarkDisputedResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_bulk_mark_shipped

> crate::models::CreateOrderBulkMarkShippedResponse create_order_bulk_mark_shipped()
Bulk flip status=shipped, clear the reserved-stock marker, and notify each platform.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateOrderBulkMarkShippedResponse**](CreateOrderBulkMarkShippedResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_bulk_packing_slip

> std::path::PathBuf create_order_bulk_packing_slip()
Bulk packing slips PDF.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_cancel

> crate::models::CreateOrderCancelResponse create_order_cancel(id)
Cancel an order on its marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderCancelResponse**](CreateOrderCancelResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_dispute

> crate::models::CreateOrderDisputeResponse create_order_dispute(id)
Flag an order as disputed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderDisputeResponse**](CreateOrderDisputeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_import

> crate::models::CreateOrderImportResponse create_order_import()
Manually (re-)pull one or more connected platforms' order history for a day-window.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateOrderImportResponse**](CreateOrderImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_label

> crate::models::CreateOrderLabelResponse create_order_label(id)
Purchase EasyPost shipping label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderLabelResponse**](CreateOrderLabelResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_message

> crate::models::CreateOrderMessageResponse create_order_message(id)
Message the order's buyer on the marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderMessageResponse**](CreateOrderMessageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_pull_platform_label

> crate::models::CreateOrderPullPlatformLabelResponse create_order_pull_platform_label(id)
Pull a pre-paid label from cookie platforms (Poshmark/Mercari).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderPullPlatformLabelResponse**](CreateOrderPullPlatformLabelResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_rate

> crate::models::CreateOrderRateResponse create_order_rate(id)
EasyPost rate quote for an order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderRateResponse**](CreateOrderRateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_refund

> crate::models::CreateOrderRefundResponse create_order_refund(id)
Issue a full or partial refund on the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderRefundResponse**](CreateOrderRefundResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_tracking

> crate::models::CreateOrderTrackingResponse create_order_tracking(id)
Submit tracking number + carrier and notify the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateOrderTrackingResponse**](CreateOrderTrackingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> crate::models::GetOrderResponse get_order(id)
Get one order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetOrderResponse**](GetOrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_cancel_eligibility

> crate::models::GetOrderCancelEligibilityResponse get_order_cancel_eligibility(id)
Whether this order can be cancelled, and which reason codes the marketplace accepts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetOrderCancelEligibilityResponse**](GetOrderCancelEligibilityResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_count

> ::std::collections::HashMap<String, f32> get_order_count()
Tab badge counts by status.

### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, f32>**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_evidence

> crate::models::GetOrderEvidenceResponse get_order_evidence(id)
Everything recorded about how this order was packed and how it arrived.

Returns the packing capture and its attestation verdict, the seal comparison (dispatch vs arrival), the buyer's arrival-condition state, and the identified units that shipped. Absence is reported as absence — an empty section means nothing was recorded, never that nothing happened.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetOrderEvidenceResponse**](GetOrderEvidenceResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_packing_slip

> std::path::PathBuf get_order_packing_slip(id)
Single-order packing slip PDF.

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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_proof_of_delivery

> crate::models::GetOrderProofOfDeliveryResponse get_order_proof_of_delivery(id)
Proof-of-delivery evidence from the carrier's scan record (PDF, or JSON).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetOrderProofOfDeliveryResponse**](GetOrderProofOfDeliveryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_shipment

> crate::models::GetOrderShipmentResponse get_order_shipment(id)
List the parcels an order shipped in.

An order can ship in more than one box: eBay allows a second label, Poshmark sells up to ten additional ones, and a bundle can need two. The order record carries only the PRIMARY parcel, so this is the only place the rest are visible. totalLabelCostCents sums every non-voided parcel — reading label cost off the order understates a multi-parcel order by a whole label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetOrderShipmentResponse**](GetOrderShipmentResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_order_units

> crate::models::V1List list_order_units(id)
List the identified units that shipped on an order.

The dispute-time read: what physically went out. An empty list means no identity was recorded, which is not the same as the order having no units.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_orders

> crate::models::V1List list_orders(page, limit, status, platform)
List orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**status** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_order

> crate::models::UpdateOrderResponse update_order(id)
Update order (status, notes, tracking).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateOrderResponse**](UpdateOrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

