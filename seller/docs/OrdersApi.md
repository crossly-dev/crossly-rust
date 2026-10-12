# \OrdersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_live_shipment_label**](OrdersApi.md#create_live_shipment_label) | **POST** /v1/live-shipments/{id}/label | Buy ONE label for a show box, rated for all its orders packed together; every order takes its tracking and the buyer’s extra postage is refunded.
[**create_live_shipment_ship**](OrdersApi.md#create_live_shipment_ship) | **POST** /v1/live-shipments/{id}/ship | Ship a show box with your own label: every order is marked shipped with this tracking and the buyer’s extra postage is refunded.
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
[**create_order_pickup**](OrdersApi.md#create_order_pickup) | **POST** /v1/orders/pickups | Book a carrier pickup for labelled orders (one carrier per pickup; window at least an hour, up to 14 days ahead).
[**create_order_pickup_label**](OrdersApi.md#create_order_pickup_label) | **POST** /v1/orders/{id}/pickup-label | Buy the carrier label for a Crossly order the buyer collects from a pickup point (debits your shipping wallet). 402 when the wallet is short.
[**create_order_pull_platform_label**](OrdersApi.md#create_order_pull_platform_label) | **POST** /v1/orders/{id}/pull-platform-label | Pull a pre-paid label from cookie platforms (Poshmark/Mercari).
[**create_order_rate**](OrdersApi.md#create_order_rate) | **POST** /v1/orders/{id}/rates | EasyPost rate quote for an order.
[**create_order_refund**](OrdersApi.md#create_order_refund) | **POST** /v1/orders/{id}/refund | Issue a full or partial refund on the platform.
[**create_order_scan_form**](OrdersApi.md#create_order_scan_form) | **POST** /v1/orders/scan-forms | Create SCAN forms (one per carrier account and ship-from address) for labels the carrier has not scanned yet.
[**create_order_tracking**](OrdersApi.md#create_order_tracking) | **POST** /v1/orders/{id}/tracking | Submit tracking number + carrier and notify the platform.
[**create_order_whatnot_label**](OrdersApi.md#create_order_whatnot_label) | **POST** /v1/orders/{id}/whatnot/label | Generate the Whatnot shipping label and record it on the order.
[**create_order_whatnot_package**](OrdersApi.md#create_order_whatnot_package) | **POST** /v1/orders/{id}/whatnot/package | Save a Whatnot shipment package (weight oz, box inches).
[**create_order_whatnot_package_quote**](OrdersApi.md#create_order_whatnot_package_quote) | **POST** /v1/orders/{id}/whatnot/package/quote | Quote what a Whatnot package change costs (cents).
[**create_rental_approve**](OrdersApi.md#create_rental_approve) | **POST** /v1/rentals/{id}/approve | Approve a requested rental (offers with requireApproval): the renter’s held card is charged.
[**create_rental_cancel**](OrdersApi.md#create_rental_cancel) | **POST** /v1/rentals/{id}/cancel | Cancel a booked rental that has not shipped: the renter is refunded in full.
[**create_rental_claim**](OrdersApi.md#create_rental_claim) | **POST** /v1/rentals/{id}/claim | Claim part or all of the deposit (cents, a note, photo URLs): within 3 days of the return, or 14 days after the end if it never came back.
[**create_rental_decline**](OrdersApi.md#create_rental_decline) | **POST** /v1/rentals/{id}/decline | Decline a requested rental: the hold on the renter’s card is released.
[**create_rental_label**](OrdersApi.md#create_rental_label) | **POST** /v1/rentals/{id}/labels | Buy both labels for a booked rental: outbound to the renter, and the return label they use.
[**create_rental_returned**](OrdersApi.md#create_rental_returned) | **POST** /v1/rentals/{id}/returned | Mark a rental back with you. You then have 3 days to claim damage before the deposit goes back.
[**create_rental_shipped**](OrdersApi.md#create_rental_shipped) | **POST** /v1/rentals/{id}/shipped | Mark a booked rental shipped, with at least 2 photos of its condition as it leaves (URLs you uploaded) — the evidence any later claim is judged against.
[**get_order**](OrdersApi.md#get_order) | **GET** /v1/orders/{id} | Get one order.
[**get_order_cancel_eligibility**](OrdersApi.md#get_order_cancel_eligibility) | **GET** /v1/orders/{id}/cancel-eligibility | Whether this order can be cancelled, and which reason codes the marketplace accepts.
[**get_order_count**](OrdersApi.md#get_order_count) | **GET** /v1/orders/counts | Tab badge counts by status.
[**get_order_evidence**](OrdersApi.md#get_order_evidence) | **GET** /v1/orders/{id}/evidence | Everything recorded about how this order was packed and how it arrived.
[**get_order_label_print**](OrdersApi.md#get_order_label_print) | **GET** /v1/orders/{id}/label/print | The order's shipping label as a PDF sized for printing (cropped, upright, fitted).
[**get_order_packing_slip**](OrdersApi.md#get_order_packing_slip) | **GET** /v1/orders/{id}/packing-slip | Single-order packing slip PDF.
[**get_order_proof_of_delivery**](OrdersApi.md#get_order_proof_of_delivery) | **GET** /v1/orders/{id}/proof-of-delivery | Proof-of-delivery evidence from the carrier's scan record (PDF, or JSON).
[**get_order_scan_form**](OrdersApi.md#get_order_scan_form) | **GET** /v1/orders/scan-forms/{id} | One SCAN form; a queued form is re-read from the carrier and its PDF appears once built.
[**get_order_shipment**](OrdersApi.md#get_order_shipment) | **GET** /v1/orders/{id}/shipments | List the parcels an order shipped in.
[**get_order_whatnot_packing_slip**](OrdersApi.md#get_order_whatnot_packing_slip) | **GET** /v1/orders/{id}/whatnot/packing-slip | A fresh link to Whatnot's packing slip PDF.
[**get_order_whatnot_shipment**](OrdersApi.md#get_order_whatnot_shipment) | **GET** /v1/orders/{id}/whatnot/shipment | Whatnot shipment for an order (package, label, tracking). Records a label found on it.
[**list_layaways**](OrdersApi.md#list_layaways) | **GET** /v1/layaways | Your items on layaway (active and paid off). Do not ship an active one: its order moves to needs_fulfillment when paid in full.
[**list_live_shipments**](OrdersApi.md#list_live_shipments) | **GET** /v1/live-shipments | Your show boxes: every buyer’s orders from one live show, to ship together.
[**list_order_pickup_ready**](OrdersApi.md#list_order_pickup_ready) | **GET** /v1/orders/pickup-ready | Orders with a purchased Shippo label a courier can collect or a SCAN form can cover, grouped by carrier.
[**list_order_pickups**](OrdersApi.md#list_order_pickups) | **GET** /v1/orders/pickups | Booked carrier pickups, newest first.
[**list_order_scan_forms**](OrdersApi.md#list_order_scan_forms) | **GET** /v1/orders/scan-forms | SCAN forms, newest first.
[**list_order_units**](OrdersApi.md#list_order_units) | **GET** /v1/orders/{id}/units | List the identified units that shipped on an order.
[**list_orders**](OrdersApi.md#list_orders) | **GET** /v1/orders | List orders.
[**list_rentals**](OrdersApi.md#list_rentals) | **GET** /v1/rentals | Rentals of your items, with the ship-to and the outbound label.
[**update_order**](OrdersApi.md#update_order) | **PATCH** /v1/orders/{id} | Update order (status, notes, tracking).



## create_live_shipment_label

> crate::models::CreateLiveShipmentShipResponse create_live_shipment_label(id)
Buy ONE label for a show box, rated for all its orders packed together; every order takes its tracking and the buyer’s extra postage is refunded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLiveShipmentShipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_shipment_ship

> crate::models::CreateLiveShipmentShipResponse create_live_shipment_ship(id)
Ship a show box with your own label: every order is marked shipped with this tracking and the buyer’s extra postage is refunded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLiveShipmentShipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_bulk_delete

> crate::models::CreateOrderBulkDeleteResponse create_order_bulk_delete()
Bulk soft- or hard-delete orders by id.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_bulk_mark_disputed

> crate::models::CreateInventoryLabelRenameResponse create_order_bulk_mark_disputed()
Bulk mark orders as disputed with a shared reason.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryLabelRenameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_bulk_mark_shipped

> crate::models::CreateOrderBulkMarkShippedResponse create_order_bulk_mark_shipped()
Bulk flip status=shipped, clear the reserved-stock marker, and notify each platform.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderBulkMarkShippedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_cancel

> crate::models::CreateOrderCancelResponse create_order_cancel(id)
Cancel an order on its marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_dispute

> crate::models::CreateOrderDisputeResponse create_order_dispute(id)
Flag an order as disputed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderDisputeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_import

> crate::models::CreateOrderImportResponse create_order_import()
Manually (re-)pull one or more connected platforms' order history for a day-window.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderImportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_label

> crate::models::CreateOrderLabelResponse create_order_label(id)
Purchase EasyPost shipping label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_message

> crate::models::CreateOrderMessageResponse create_order_message(id)
Message the order's buyer on the marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderMessageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_pickup

> crate::models::CreateOrderPickupResponse create_order_pickup()
Book a carrier pickup for labelled orders (one carrier per pickup; window at least an hour, up to 14 days ahead).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderPickupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_pickup_label

> crate::models::CreateOrderPickupLabelResponse create_order_pickup_label(id)
Buy the carrier label for a Crossly order the buyer collects from a pickup point (debits your shipping wallet). 402 when the wallet is short.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderPickupLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_pull_platform_label

> crate::models::CreateOrderPullPlatformLabelResponse create_order_pull_platform_label(id)
Pull a pre-paid label from cookie platforms (Poshmark/Mercari).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderPullPlatformLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_rate

> crate::models::CreateOrderRateResponse create_order_rate(id)
EasyPost rate quote for an order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderRateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_refund

> crate::models::CreateOrderRefundResponse create_order_refund(id)
Issue a full or partial refund on the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderRefundResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_scan_form

> crate::models::CreateOrderScanFormResponse create_order_scan_form()
Create SCAN forms (one per carrier account and ship-from address) for labels the carrier has not scanned yet.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderScanFormResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_tracking

> crate::models::CreateInboxOfferResponse create_order_tracking(id)
Submit tracking number + carrier and notify the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInboxOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_whatnot_label

> crate::models::CreateOrderWhatnotLabelResponse create_order_whatnot_label(id)
Generate the Whatnot shipping label and record it on the order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderWhatnotLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_whatnot_package

> crate::models::CreateOrderWhatnotLabelResponse create_order_whatnot_package(id)
Save a Whatnot shipment package (weight oz, box inches).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderWhatnotLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_whatnot_package_quote

> crate::models::CreateOrderWhatnotPackageQuoteResponse create_order_whatnot_package_quote(id)
Quote what a Whatnot package change costs (cents).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderWhatnotPackageQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_approve

> crate::models::RentalResponse create_rental_approve(id)
Approve a requested rental (offers with requireApproval): the renter’s held card is charged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_cancel

> crate::models::RentalResponse create_rental_cancel(id)
Cancel a booked rental that has not shipped: the renter is refunded in full.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_claim

> crate::models::RentalResponse create_rental_claim(id)
Claim part or all of the deposit (cents, a note, photo URLs): within 3 days of the return, or 14 days after the end if it never came back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_decline

> crate::models::RentalResponse create_rental_decline(id)
Decline a requested rental: the hold on the renter’s card is released.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_label

> crate::models::RentalResponse create_rental_label(id)
Buy both labels for a booked rental: outbound to the renter, and the return label they use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_returned

> crate::models::RentalResponse create_rental_returned(id)
Mark a rental back with you. You then have 3 days to claim damage before the deposit goes back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_shipped

> crate::models::RentalResponse create_rental_shipped(id)
Mark a booked rental shipped, with at least 2 photos of its condition as it leaves (URLs you uploaded) — the evidence any later claim is judged against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::RentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order

> crate::models::ListOrdersItem get_order(id)
Get one order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListOrdersItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_cancel_eligibility

> crate::models::GetOrderCancelEligibilityResponse get_order_cancel_eligibility(id)
Whether this order can be cancelled, and which reason codes the marketplace accepts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderCancelEligibilityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_evidence

> crate::models::GetOrderEvidenceResponse get_order_evidence(id)
Everything recorded about how this order was packed and how it arrived.

Returns the packing capture and its attestation verdict, the seal comparison (dispatch vs arrival), the buyer's arrival-condition state, and the identified units that shipped. Absence is reported as absence — an empty section means nothing was recorded, never that nothing happened.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderEvidenceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_label_print

> std::path::PathBuf get_order_label_print(id, size, flip, parcel_id)
The order's shipping label as a PDF sized for printing (cropped, upright, fitted).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**size** | Option<**String**> | Paper. Default 4x6. |  |
**flip** | Option<**String**> | Turn the label a half turn. |  |
**parcel_id** | Option<**String**> | A specific parcel's label; default the order's primary. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_proof_of_delivery

> crate::models::GetOrderProofOfDeliveryResponse get_order_proof_of_delivery(id)
Proof-of-delivery evidence from the carrier's scan record (PDF, or JSON).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderProofOfDeliveryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_scan_form

> crate::models::GetOrderScanFormResponse get_order_scan_form(id)
One SCAN form; a queued form is re-read from the carrier and its PDF appears once built.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderScanFormResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_shipment

> crate::models::GetOrderShipmentResponse get_order_shipment(id)
List the parcels an order shipped in.

An order can ship in more than one box: eBay allows a second label, Poshmark sells up to ten additional ones, and a bundle can need two. The order record carries only the PRIMARY parcel, so this is the only place the rest are visible. totalLabelCostCents sums every non-voided parcel — reading label cost off the order understates a multi-parcel order by a whole label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderShipmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_whatnot_packing_slip

> crate::models::GetOrderWhatnotPackingSlipResponse get_order_whatnot_packing_slip(id)
A fresh link to Whatnot's packing slip PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderWhatnotPackingSlipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_whatnot_shipment

> crate::models::CreateOrderWhatnotLabelResponse get_order_whatnot_shipment(id)
Whatnot shipment for an order (package, label, tracking). Records a label found on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderWhatnotLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_layaways

> crate::models::ListLayawaysResponse list_layaways()
Your items on layaway (active and paid off). Do not ship an active one: its order moves to needs_fulfillment when paid in full.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLayawaysResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_live_shipments

> crate::models::ListLiveShipmentsResponse list_live_shipments()
Your show boxes: every buyer’s orders from one live show, to ship together.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLiveShipmentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_order_pickup_ready

> crate::models::ListOrderPickupReadyResponse list_order_pickup_ready()
Orders with a purchased Shippo label a courier can collect or a SCAN form can cover, grouped by carrier.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListOrderPickupReadyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_order_pickups

> crate::models::ListOrderPickupsResponse list_order_pickups()
Booked carrier pickups, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListOrderPickupsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_order_scan_forms

> crate::models::ListOrderScanFormsResponse list_order_scan_forms()
SCAN forms, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListOrderScanFormsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_order_units

> crate::models::ListOrderUnitsResponse list_order_units(id)
List the identified units that shipped on an order.

The dispute-time read: what physically went out. An empty list means no identity was recorded, which is not the same as the order having no units.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListOrderUnitsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_orders

> crate::models::ListOrdersResponse list_orders(page, limit, status, platform)
List orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**status** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |

### Return type

**crate::models::ListOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_rentals

> crate::models::ListRentalsResponse list_rentals()
Rentals of your items, with the ship-to and the outbound label.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListRentalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_order

> crate::models::ListOrdersItem update_order(id)
Update order (status, notes, tracking).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListOrdersItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

