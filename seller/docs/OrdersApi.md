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
[**create_order_cancel**](OrdersApi.md#create_order_cancel) | **POST** /v1/orders/{id}/cancel | Cancel an order before it ships. Crossly-marketplace orders refund the buyer (also needs orders:refund) and restock; crossposted orders go to the marketplace, or fail not_supported_on_platform (recordOnly: true records a cancel made on the marketplace). Fires order.cancelled.
[**create_order_correct_tracking**](OrdersApi.md#create_order_correct_tracking) | **POST** /v1/orders/{id}/correct-tracking | Correct a wrong tracking number the marketplace already holds.
[**create_order_dispute**](OrdersApi.md#create_order_dispute) | **POST** /v1/orders/{id}/dispute | Flag an order as disputed.
[**create_order_import**](OrdersApi.md#create_order_import) | **POST** /v1/orders/import | Manually (re-)pull one or more connected platforms' order history for a day-window.
[**create_order_label**](OrdersApi.md#create_order_label) | **POST** /v1/orders/{id}/label | Purchase a Crossly Shipping label. An order that already has a label answers 409 label_exists unless allowSecondLabel is true.
[**create_order_label_void**](OrdersApi.md#create_order_label_void) | **POST** /v1/orders/{id}/label/void | Void a Crossly Shipping label and refund it to the shipping wallet.
[**create_order_link_listing**](OrdersApi.md#create_order_link_listing) | **POST** /v1/orders/{id}/link-listing | Link an unmatched sale to one of your listings (marks it sold, delists it elsewhere).
[**create_order_message**](OrdersApi.md#create_order_message) | **POST** /v1/orders/{id}/message | Message the order's buyer on the marketplace.
[**create_order_order_chat**](OrdersApi.md#create_order_order_chat) | **POST** /v1/orders/{id}/order-chat | Send a message to the order's buyer chat on the marketplace. Mirrored into the Crossly inbox.
[**create_order_pickup**](OrdersApi.md#create_order_pickup) | **POST** /v1/orders/pickups | Book a carrier pickup for labelled orders (one carrier per pickup; window at least an hour, up to 14 days ahead).
[**create_order_pickup_label**](OrdersApi.md#create_order_pickup_label) | **POST** /v1/orders/{id}/pickup-label | Buy the carrier label for a Crossly order the buyer collects from a pickup point (debits your shipping wallet). 402 when the wallet is short. An order that already has a label answers 409 label_exists unless allowSecondLabel is true.
[**create_order_platform_label_package**](OrdersApi.md#create_order_platform_label_package) | **POST** /v1/orders/{id}/platform-label/package | Save the package (weight oz, box inches) the marketplace prices the label from.
[**create_order_platform_label_package_quote**](OrdersApi.md#create_order_platform_label_package_quote) | **POST** /v1/orders/{id}/platform-label/package/quote | Quote what a change to the marketplace shipment's package costs (cents).
[**create_order_platform_label_pull**](OrdersApi.md#create_order_platform_label_pull) | **POST** /v1/orders/{id}/platform-label/pull | Pull the marketplace's prepaid label into Crossly (every platform with a label descriptor).
[**create_order_platform_label_purchase**](OrdersApi.md#create_order_platform_label_purchase) | **POST** /v1/orders/{id}/platform-label/purchase | Buy a replacement / extra marketplace label, or have the marketplace generate it, and record it.
[**create_order_platform_label_quote**](OrdersApi.md#create_order_platform_label_quote) | **POST** /v1/orders/{id}/platform-label/quote | Quote what buying a marketplace label costs, before you are charged.
[**create_order_pull_platform_label**](OrdersApi.md#create_order_pull_platform_label) | **POST** /v1/orders/{id}/pull-platform-label | Pull a pre-paid label from cookie platforms (Poshmark/Mercari).
[**create_order_rate**](OrdersApi.md#create_order_rate) | **POST** /v1/orders/{id}/rates | Crossly Shipping rate quote for an order.
[**create_order_refund**](OrdersApi.md#create_order_refund) | **POST** /v1/orders/{id}/refund | DEPRECATED: use POST /v1/orders/{id}/refunds. Full or partial refund (amount in dollars). Same idempotency and cap as /refunds: the same request twice refunds once (Idempotency-Key, else the body), never past what the buyer paid.
[**create_order_refund_by_id**](OrdersApi.md#create_order_refund_by_id) | **POST** /v1/orders/{id}/refunds | Refund an order, in full or in part. `amount` is dollars; omit it for everything left (or the line items' value). Crossly-marketplace orders refund the buyer's card; crossposted orders refund through the marketplace, or fail not_supported_on_platform. Never refunds past the order (refund_exceeds_remaining). The same request twice refunds once: send an Idempotency-Key (without one, the body is the key). A partial refund keeps the order's status. What the platform takes (full/partial, its own reason keys, a comment) is GET /v1/orders/{id}/refund-options; a request outside it fails partial_refund_unsupported, invalid_refund_reason or refund_exceeds_item_amount. Fires order.refunded.
[**create_order_refund_resolve**](OrdersApi.md#create_order_refund_resolve) | **POST** /v1/orders/{id}/refunds/{refundId}/resolve | Settle an `unverified` refund (one Crossly could not confirm with the marketplace after its request was interrupted) once you have checked the marketplace: succeeded (with the amount actually refunded, dollars) or failed (nothing moved; the same Idempotency-Key can then try again). Crossly-marketplace refunds are confirmed by support instead.
[**create_order_scan_form**](OrdersApi.md#create_order_scan_form) | **POST** /v1/orders/scan-forms | Create SCAN forms (one per carrier account and ship-from address) for labels the carrier has not scanned yet.
[**create_order_tracking**](OrdersApi.md#create_order_tracking) | **POST** /v1/orders/{id}/tracking | Submit tracking number + carrier and notify the platform.
[**create_order_whatnot_label**](OrdersApi.md#create_order_whatnot_label) | **POST** /v1/orders/{id}/whatnot/label | Generate the Whatnot shipping label and record it on the order.
[**create_order_whatnot_package**](OrdersApi.md#create_order_whatnot_package) | **POST** /v1/orders/{id}/whatnot/package | Save a Whatnot shipment package (weight oz, box inches).
[**create_order_whatnot_package_quote**](OrdersApi.md#create_order_whatnot_package_quote) | **POST** /v1/orders/{id}/whatnot/package/quote | Quote what a Whatnot package change costs (cents).
[**create_rental_approve**](OrdersApi.md#create_rental_approve) | **POST** /v1/rentals/{id}/approve | Approve a requested rental (offers with requireApproval): the renter’s held card is charged.
[**create_rental_cancel**](OrdersApi.md#create_rental_cancel) | **POST** /v1/rentals/{id}/cancel | Cancel a booked rental that has not shipped: the renter is refunded in full.
[**create_rental_claim**](OrdersApi.md#create_rental_claim) | **POST** /v1/rentals/{id}/claim | Claim part or all of the deposit (cents, a note, photo URLs): within 3 days of the return, or 14 days after the end if it never came back.
[**create_rental_decline**](OrdersApi.md#create_rental_decline) | **POST** /v1/rentals/{id}/decline | Decline a requested rental: the hold on the renter’s card is released.
[**create_rental_delivered**](OrdersApi.md#create_rental_delivered) | **POST** /v1/rentals/{id}/delivered | Mark a rental delivered to the renter.
[**create_rental_extension_decide**](OrdersApi.md#create_rental_extension_decide) | **POST** /v1/rentals/{id}/extensions/{extensionId}/decide | Approve or decline a renter’s request for more days.
[**create_rental_inspect**](OrdersApi.md#create_rental_inspect) | **POST** /v1/rentals/{id}/inspect | Record the return inspection: the checklist, photos, and optionally release the deposit.
[**create_rental_label**](OrdersApi.md#create_rental_label) | **POST** /v1/rentals/{id}/labels | Buy both labels for a booked rental: outbound to the renter, and the return label they use.
[**create_rental_not_returned**](OrdersApi.md#create_rental_not_returned) | **POST** /v1/rentals/{id}/not-returned | Report a rental not returned (14 days past due): the deposit goes to you.
[**create_rental_release_deposit**](OrdersApi.md#create_rental_release_deposit) | **POST** /v1/rentals/{id}/release-deposit | Release the deposit now, without a claim.
[**create_rental_returned**](OrdersApi.md#create_rental_returned) | **POST** /v1/rentals/{id}/returned | Mark a rental back with you. You then have 3 days to claim damage before the deposit goes back.
[**create_rental_shipped**](OrdersApi.md#create_rental_shipped) | **POST** /v1/rentals/{id}/shipped | Mark a booked rental shipped, with at least 2 photos of its condition as it leaves (URLs you uploaded) — the evidence any later claim is judged against.
[**get_order**](OrdersApi.md#get_order) | **GET** /v1/orders/{id} | Get one order, with its sale price, fees, SKU and line items.
[**get_order_cancel_eligibility**](OrdersApi.md#get_order_cancel_eligibility) | **GET** /v1/orders/{id}/cancel-eligibility | Whether this order can be cancelled, and which reason codes the marketplace accepts.
[**get_order_count**](OrdersApi.md#get_order_count) | **GET** /v1/orders/counts | Tab badge counts by status.
[**get_order_delivery_photo**](OrdersApi.md#get_order_delivery_photo) | **GET** /v1/orders/{id}/delivery-photo | The courier's delivery photo for this order, pulled automatically, with where it came from.
[**get_order_evidence**](OrdersApi.md#get_order_evidence) | **GET** /v1/orders/{id}/evidence | Everything recorded about how this order was packed and how it arrived.
[**get_order_label_print**](OrdersApi.md#get_order_label_print) | **GET** /v1/orders/{id}/label/print | The order's shipping label as a PDF sized for printing (cropped, upright, fitted).
[**get_order_order_chat**](OrdersApi.md#get_order_order_chat) | **GET** /v1/orders/{id}/order-chat | The order's buyer chat thread on the marketplace (separate from listing comments): { orderId, platform, messages: [{ id, sender (buyer|seller|platform), body, sentAt, attachments? }], canSend }, oldest message first.
[**get_order_packing_slip**](OrdersApi.md#get_order_packing_slip) | **GET** /v1/orders/{id}/packing-slip | Single-order packing slip PDF.
[**get_order_platform_label_option**](OrdersApi.md#get_order_platform_label_option) | **GET** /v1/orders/{id}/platform-label/options | What buying a marketplace label offers for this order (weight tiers, mailbox state).
[**get_order_platform_label_packing_slip**](OrdersApi.md#get_order_platform_label_packing_slip) | **GET** /v1/orders/{id}/platform-label/packing-slip | A fresh link to the marketplace's own packing slip PDF.
[**get_order_platform_label_state**](OrdersApi.md#get_order_platform_label_state) | **GET** /v1/orders/{id}/platform-label/state | The marketplace's shipment for an order (package, label, tracking). Records a label found on it.
[**get_order_platform_label_url**](OrdersApi.md#get_order_platform_label_url) | **GET** /v1/orders/{id}/platform-label/url | A fresh, short-lived link to the marketplace's label sheet. Never stored.
[**get_order_proof_of_delivery**](OrdersApi.md#get_order_proof_of_delivery) | **GET** /v1/orders/{id}/proof-of-delivery | Proof-of-delivery evidence from the carrier's scan record (PDF, or JSON).
[**get_order_refund_option**](OrdersApi.md#get_order_refund_option) | **GET** /v1/orders/{id}/refund-options | What a refund on this order may be, from the platform's declared capability: whether Crossly can refund it at all, full and/or partial, what a partial is measured against (total = what the buyer paid, item = the item price only), whether it may precede shipment, the platform's own reason keys (when listed, a refund's reason must be one of them), whether a comment is sent, whether the refund IS cancelling the sale (cancelsSale: Mercari, full only), whether the window closes when the order completes (untilCompleted; windowClosed = it has) and a one-line note on what the platform does not allow.
[**get_order_sales_tax**](OrdersApi.md#get_order_sales_tax) | **GET** /v1/orders/{id}/sales-tax | Who collected sales tax on one Crossly marketplace order, where, and the seller notice.
[**get_order_scan_form**](OrdersApi.md#get_order_scan_form) | **GET** /v1/orders/scan-forms/{id} | One SCAN form; a queued form is re-read from the carrier and its PDF appears once built.
[**get_order_shipment**](OrdersApi.md#get_order_shipment) | **GET** /v1/orders/{id}/shipments | List the parcels an order shipped in.
[**get_order_whatnot_packing_slip**](OrdersApi.md#get_order_whatnot_packing_slip) | **GET** /v1/orders/{id}/whatnot/packing-slip | A fresh link to Whatnot's packing slip PDF.
[**get_order_whatnot_shipment**](OrdersApi.md#get_order_whatnot_shipment) | **GET** /v1/orders/{id}/whatnot/shipment | Whatnot shipment for an order (package, label, tracking). Records a label found on it.
[**get_rental**](OrdersApi.md#get_rental) | **GET** /v1/rentals/{id} | One rental: its timeline, both sides’ condition photos, payments, extensions and what can be done next.
[**get_rental_calendar**](OrdersApi.md#get_rental_calendar) | **GET** /v1/rentals/calendar | Every rentable item’s bookings (per unit), turnaround buffers and blackouts between two dates (≤ 93 days).
[**list_layaways**](OrdersApi.md#list_layaways) | **GET** /v1/layaways | Your items on layaway (active and paid off). Do not ship an active one: its order moves to needs_fulfillment when paid in full.
[**list_live_shipments**](OrdersApi.md#list_live_shipments) | **GET** /v1/live-shipments | Your show boxes: every buyer’s orders from one live show, to ship together.
[**list_order_pickup_ready**](OrdersApi.md#list_order_pickup_ready) | **GET** /v1/orders/pickup-ready | Orders with a purchased Crossly Shipping label a courier can collect or a SCAN form can cover, grouped by carrier.
[**list_order_pickups**](OrdersApi.md#list_order_pickups) | **GET** /v1/orders/pickups | Booked carrier pickups, newest first.
[**list_order_refunds**](OrdersApi.md#list_order_refunds) | **GET** /v1/orders/{id}/refunds | Every refund requested on this order through the API or a cancel, newest first (amounts in dollars).
[**list_order_scan_forms**](OrdersApi.md#list_order_scan_forms) | **GET** /v1/orders/scan-forms | SCAN forms, newest first.
[**list_order_units**](OrdersApi.md#list_order_units) | **GET** /v1/orders/{id}/units | List the identified units that shipped on an order.
[**list_orders**](OrdersApi.md#list_orders) | **GET** /v1/orders | List orders, each with its sale price, fees, SKU and line items.
[**list_rental_history**](OrdersApi.md#list_rental_history) | **GET** /v1/rentals/history | Active rentals and history, filtered by view, status, dates, item or a search.
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

> crate::models::UpdatedResponse create_order_bulk_mark_disputed()
Bulk mark orders as disputed with a shared reason.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdatedResponse**

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
Cancel an order before it ships. Crossly-marketplace orders refund the buyer (also needs orders:refund) and restock; crossposted orders go to the marketplace, or fail not_supported_on_platform (recordOnly: true records a cancel made on the marketplace). Fires order.cancelled.

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


## create_order_correct_tracking

> crate::models::CreateOrderCorrectTrackingResponse create_order_correct_tracking(id)
Correct a wrong tracking number the marketplace already holds.

Replaces the order's tracking number with `trackingNumber`, on Crossly and on the marketplace, where the platform can change tracking it already has (its trackingCorrection capability): `correction` is replace (overwritten), remove_then_submit (the old number removed, then the new one sent) or submit (the platform held none). Refused with validation_error where the platform cannot change it (update it there), when the order has no tracking, its tracking came with a label (void the label instead), or it is closed or delivered; conflict while the previous tracking update is still in flight.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderCorrectTrackingResponse**

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
Purchase a Crossly Shipping label. An order that already has a label answers 409 label_exists unless allowSecondLabel is true.

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


## create_order_label_void

> crate::models::CreateOrderLabelVoidResponse create_order_label_void(id)
Void a Crossly Shipping label and refund it to the shipping wallet.

Voids the order's unused label, owes the wallet charge back in full (markup included) and sets the order back to needs_fulfillment. The refund is held as pending (`refundPendingCents`; `refundedCents` stays 0) and is credited to the shipping wallet when Crossly releases held credits. Only while the carrier has not scanned the parcel and within 30 days of purchase. `tracking.outcome` says what happens to tracking already sent to the marketplace: not_submitted, removal_queued, replace_on_next_label, or update_manually (with `platformOrderUrl`). Repeating the call returns the same result with `alreadyVoided: true` and refunds nothing again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderLabelVoidResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_link_listing

> crate::models::CreateOrderLinkListingResponse create_order_link_listing(id)
Link an unmatched sale to one of your listings (marks it sold, delists it elsewhere).

For an order whose listing is an auto-created external-sale stub. Body: { listingId, platformListingId? }. Moves the sale onto the listing's row on the order's platform (or moves the stub's row under the listing when it has none), decrements its stock like a detected sale, archives the stub, and delists the item from every other marketplace once no unit is left. 409 when the order is already linked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderLinkListingResponse**

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


## create_order_order_chat

> crate::models::OkResponse create_order_order_chat(id, create_order_order_chat_request)
Send a message to the order's buyer chat on the marketplace. Mirrored into the Crossly inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_order_order_chat_request** | **CreateOrderOrderChatRequest** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
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
Buy the carrier label for a Crossly order the buyer collects from a pickup point (debits your shipping wallet). 402 when the wallet is short. An order that already has a label answers 409 label_exists unless allowSecondLabel is true.

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


## create_order_platform_label_package

> crate::models::GetOrderWhatnotShipmentResponse create_order_platform_label_package(id)
Save the package (weight oz, box inches) the marketplace prices the label from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderWhatnotShipmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_platform_label_package_quote

> crate::models::CreateOrderWhatnotPackageQuoteResponse create_order_platform_label_package_quote(id)
Quote what a change to the marketplace shipment's package costs (cents).

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


## create_order_platform_label_pull

> crate::models::CreateOrderPlatformLabelPullResponse create_order_platform_label_pull(id)
Pull the marketplace's prepaid label into Crossly (every platform with a label descriptor).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderPlatformLabelPullResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_platform_label_purchase

> crate::models::CreateOrderPlatformLabelPurchaseResponse create_order_platform_label_purchase(id)
Buy a replacement / extra marketplace label, or have the marketplace generate it, and record it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderPlatformLabelPurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_platform_label_quote

> crate::models::CreateOrderPlatformLabelQuoteResponse create_order_platform_label_quote(id)
Quote what buying a marketplace label costs, before you are charged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderPlatformLabelQuoteResponse**

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
Crossly Shipping rate quote for an order.

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
DEPRECATED: use POST /v1/orders/{id}/refunds. Full or partial refund (amount in dollars). Same idempotency and cap as /refunds: the same request twice refunds once (Idempotency-Key, else the body), never past what the buyer paid.

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


## create_order_refund_by_id

> crate::models::CreateOrderRefundByIdResponse create_order_refund_by_id(id)
Refund an order, in full or in part. `amount` is dollars; omit it for everything left (or the line items' value). Crossly-marketplace orders refund the buyer's card; crossposted orders refund through the marketplace, or fail not_supported_on_platform. Never refunds past the order (refund_exceeds_remaining). The same request twice refunds once: send an Idempotency-Key (without one, the body is the key). A partial refund keeps the order's status. What the platform takes (full/partial, its own reason keys, a comment) is GET /v1/orders/{id}/refund-options; a request outside it fails partial_refund_unsupported, invalid_refund_reason or refund_exceeds_item_amount. Fires order.refunded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderRefundByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_refund_resolve

> crate::models::ListOrderRefundsItem create_order_refund_resolve(id, refund_id)
Settle an `unverified` refund (one Crossly could not confirm with the marketplace after its request was interrupted) once you have checked the marketplace: succeeded (with the amount actually refunded, dollars) or failed (nothing moved; the same Idempotency-Key can then try again). Crossly-marketplace refunds are confirmed by support instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**refund_id** | **String** |  | [required] |

### Return type

**crate::models::ListOrderRefundsItem**

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

> crate::models::CreateOrderTrackingResponse create_order_tracking(id)
Submit tracking number + carrier and notify the platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderTrackingResponse**

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

> crate::models::GetOrderWhatnotShipmentResponse create_order_whatnot_package(id)
Save a Whatnot shipment package (weight oz, box inches).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderWhatnotShipmentResponse**

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


## create_rental_delivered

> crate::models::RentalResponse create_rental_delivered(id)
Mark a rental delivered to the renter.

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


## create_rental_extension_decide

> crate::models::CreateRentalExtensionDecideResponse create_rental_extension_decide(id, extension_id)
Approve or decline a renter’s request for more days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**extension_id** | **String** |  | [required] |

### Return type

**crate::models::CreateRentalExtensionDecideResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_inspect

> crate::models::RentalResponse create_rental_inspect(id)
Record the return inspection: the checklist, photos, and optionally release the deposit.

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


## create_rental_not_returned

> crate::models::RentalResponse create_rental_not_returned(id)
Report a rental not returned (14 days past due): the deposit goes to you.

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


## create_rental_release_deposit

> crate::models::RentalResponse create_rental_release_deposit(id)
Release the deposit now, without a claim.

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

> crate::models::GetOrderResponse get_order(id)
Get one order, with its sale price, fees, SKU and line items.

Each order carries its sale exactly as the order.created webhook sent it: salePrice, platformFee, taxCollected and shippingCharged in DOLLARS as JSON numbers (not decimal strings), feeSource (\"exact\" = the fee the marketplace charged, \"estimated\"), sku (the SKU of the listing), quantity (units), and lineItems: one entry per order row {lineKey, listingId, inventoryItemId, platformListingId (the MARKETPLACE listing id), sku (variant SKU, else the listing SKU), variantId, quantity, unitPrice (dollars per unit, rounded to the cent), total (dollars)}. A multi-item marketplace order is one order row per line sharing platformOrderId, each with its own lineKey. detailsPending: true (the sale was recorded before its price and buyer loaded), an order with no sale, or a sale with no listing behind it has null money and lineItems [] (sku and quantity are still given when known). channel is the business channel that sold it (retail, wholesale, auction, live, world, pos), purchaseOrderRef the buyer's PO number, and trade the on-account invoice behind a wholesale order {invoiceId, invoiceNumber, invoiceStatus (open, partial, overdue, paid, void), netDays, dueAt, totalCents, amountPaidCents, balanceCents} or null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderResponse**

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


## get_order_delivery_photo

> crate::models::GetOrderDeliveryPhotoResponse get_order_delivery_photo(id)
The courier's delivery photo for this order, pulled automatically, with where it came from.

Crossly pulls the doorstep photo from whoever has it (the carrier API, e.g. UPS; Trackified when its tracking carries one; the marketplace when its payload does) and stores it on the order. state is found | none | not_delivered; source / sourceLabel say where it came from (\"From UPS\", \"Added by you\"); checked lists who was asked when there is none. Carriers often attach the photo hours after delivery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderDeliveryPhotoResponse**

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


## get_order_order_chat

> crate::models::GetOrderOrderChatResponse get_order_order_chat(id)
The order's buyer chat thread on the marketplace (separate from listing comments): { orderId, platform, messages: [{ id, sender (buyer|seller|platform), body, sentAt, attachments? }], canSend }, oldest message first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderOrderChatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## get_order_platform_label_option

> crate::models::GetOrderPlatformLabelOptionResponse get_order_platform_label_option(id)
What buying a marketplace label offers for this order (weight tiers, mailbox state).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderPlatformLabelOptionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_platform_label_packing_slip

> crate::models::UrlExpiresInSecondsResponse get_order_platform_label_packing_slip(id)
A fresh link to the marketplace's own packing slip PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UrlExpiresInSecondsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_platform_label_state

> crate::models::GetOrderWhatnotShipmentResponse get_order_platform_label_state(id)
The marketplace's shipment for an order (package, label, tracking). Records a label found on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderWhatnotShipmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_platform_label_url

> crate::models::UrlExpiresInSecondsResponse get_order_platform_label_url(id)
A fresh, short-lived link to the marketplace's label sheet. Never stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UrlExpiresInSecondsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## get_order_refund_option

> crate::models::GetOrderRefundOptionResponse get_order_refund_option(id)
What a refund on this order may be, from the platform's declared capability: whether Crossly can refund it at all, full and/or partial, what a partial is measured against (total = what the buyer paid, item = the item price only), whether it may precede shipment, the platform's own reason keys (when listed, a refund's reason must be one of them), whether a comment is sent, whether the refund IS cancelling the sale (cancelsSale: Mercari, full only), whether the window closes when the order completes (untilCompleted; windowClosed = it has) and a one-line note on what the platform does not allow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderRefundOptionResponse**

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

> crate::models::UrlExpiresInSecondsResponse get_order_whatnot_packing_slip(id)
A fresh link to Whatnot's packing slip PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UrlExpiresInSecondsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_order_whatnot_shipment

> crate::models::GetOrderWhatnotShipmentResponse get_order_whatnot_shipment(id)
Whatnot shipment for an order (package, label, tracking). Records a label found on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetOrderWhatnotShipmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_rental

> crate::models::GetRentalResponse get_rental(id)
One rental: its timeline, both sides’ condition photos, payments, extensions and what can be done next.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetRentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_rental_calendar

> crate::models::GetRentalCalendarResponse get_rental_calendar()
Every rentable item’s bookings (per unit), turnaround buffers and blackouts between two dates (≤ 93 days).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetRentalCalendarResponse**

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
Orders with a purchased Crossly Shipping label a courier can collect or a SCAN form can cover, grouped by carrier.

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


## list_order_refunds

> crate::models::ListOrderRefundsResponse list_order_refunds(id)
Every refund requested on this order through the API or a cancel, newest first (amounts in dollars).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListOrderRefundsResponse**

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

> crate::models::ListOrdersResponse list_orders(page, limit, status, platform, channel, details, unlinked, updated_since)
List orders, each with its sale price, fees, SKU and line items.

Each order carries its sale exactly as the order.created webhook sent it: salePrice, platformFee, taxCollected and shippingCharged in DOLLARS as JSON numbers (not decimal strings), feeSource (\"exact\" = the fee the marketplace charged, \"estimated\"), sku (the SKU of the listing), quantity (units), and lineItems: one entry per order row {lineKey, listingId, inventoryItemId, platformListingId (the MARKETPLACE listing id), sku (variant SKU, else the listing SKU), variantId, quantity, unitPrice (dollars per unit, rounded to the cent), total (dollars)}. A multi-item marketplace order is one order row per line sharing platformOrderId, each with its own lineKey. detailsPending: true (the sale was recorded before its price and buyer loaded), an order with no sale, or a sale with no listing behind it has null money and lineItems [] (sku and quantity are still given when known). channel is the business channel that sold it (retail, wholesale, auction, live, world, pos), purchaseOrderRef the buyer's PO number, and trade the on-account invoice behind a wholesale order {invoiceId, invoiceNumber, invoiceStatus (open, partial, overdue, paid, void), netDays, dueAt, totalCents, amountPaidCents, balanceCents} or null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**status** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |
**channel** | Option<**String**> | Only orders sold through this business channel (e.g. wholesale). |  |
**details** | Option<**String**> | pending = only orders whose sale details (price, buyer) have not loaded yet. |  |
**unlinked** | Option<**String**> | 1 or true = only orders whose sale is not linked to any listing (an orphan sale). |  |
**updated_since** | Option<**String**> | Only orders created or changed at or after this time (ISO 8601): poll with the time of your last call to catch new orders and status/tracking changes. |  |

### Return type

**crate::models::ListOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_rental_history

> crate::models::ListRentalHistoryResponse list_rental_history()
Active rentals and history, filtered by view, status, dates, item or a search.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListRentalHistoryResponse**

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

> crate::models::UpdateOrderResponse update_order(id)
Update order (status, notes, tracking).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

