# \WholesaleApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_wholesale_buyer_invite**](WholesaleApi.md#create_wholesale_buyer_invite) | **POST** /v1/wholesale/buyers/invite | Invite a buyer by email to one or more of your catalogs. Binds to their account now if they have one, or when they sign up.
[**create_wholesale_buyer_remove**](WholesaleApi.md#create_wholesale_buyer_remove) | **POST** /v1/wholesale/buyers/remove | Revoke every catalog grant a buyer (or invited email) has with you. Their terms and invoices are kept.
[**create_wholesale_catalog**](WholesaleApi.md#create_wholesale_catalog) | **POST** /v1/wholesale/catalogs | Create a catalog. visibility public | public_prices_private (prices for approved buyers) | private_discoverable (listed, opened on approval) | link (anyone with the URL) | invite (named buyers only; \"private\" is accepted as invite); autoApprove never | verified | all; the cascade terms (minOrderQuantity, casePack, leadTimeDays, verifiedOnly, priceVisibility, maxPerOrder, backorder, restockEta, preset ids) default every item; listed advertises a public catalog in the wholesale directory (never its prices).
[**create_wholesale_catalog_item**](WholesaleApi.md#create_wholesale_catalog_item) | **POST** /v1/wholesale/catalogs/{id}/items | Add your Crossly listings to a catalog (up to 500), each with an optional wholesalePriceCents, minOrderQuantity and casePack. Items already in the catalog have their terms updated.
[**create_wholesale_drop**](WholesaleApi.md#create_wholesale_drop) | **POST** /v1/wholesale/drops | Create a draft trade drop: name, dropAt (ISO; when it opens), audience trade|catalog (+ audienceCatalogId), kind release|prebook. A pre-book needs depositBps (100-9900) and shipBy (YYYY-MM-DD), and takes balanceNetDays (0-90) for the balance invoice. listingIds are your listing ids, each on Crossly. A pre-book makes its listings pre-orders, so buyers pay the deposit at checkout. Schedule it with PATCH status \"scheduled\".
[**create_wholesale_drop_release**](WholesaleApi.md#create_wholesale_drop_release) | **POST** /v1/wholesale/drops/{id}/release | Release a live pre-book: every reserved balance becomes an invoice on account to its buyer (due in the drop balanceNetDays), emailed, and the drop ends. Safe to retry: only balances not yet invoiced are invoiced. Returns { invoiced, balanceCents, invoiceIds }.
[**create_wholesale_freight_request_answer**](WholesaleApi.md#create_wholesale_freight_request_answer) | **POST** /v1/wholesale/freight-requests/{id}/answer | Answer an open freight request with a figure in CENTS (and an optional note).
[**create_wholesale_group**](WholesaleApi.md#create_wholesale_group) | **POST** /v1/wholesale/groups | Create a customer group with a discount in basis points (up to 9000). A buyer in several groups gets the best rate, never a stacked one.
[**create_wholesale_group_member**](WholesaleApi.md#create_wholesale_group_member) | **POST** /v1/wholesale/groups/{id}/members | Add a buyer to a group by account id or by the email of an existing Crossly account.
[**create_wholesale_invoice_credit_note**](WholesaleApi.md#create_wholesale_invoice_credit_note) | **POST** /v1/wholesale/invoices/{id}/credit-notes | Issue a credit note on an on-account invoice (short shipment, damaged, pricing error, returned, goodwill, other): at most the balance due. Numbered <invoice>-CN<n>; settling the rest closes the invoice paid. Not a refund.
[**create_wholesale_invoice_late_fee**](WholesaleApi.md#create_wholesale_invoice_late_fee) | **POST** /v1/wholesale/invoices/{id}/late-fee | Add your late fee (invoice settings lateFeeBps) to an invoice by hand, once. The hourly sweep adds it automatically when an invoice goes overdue.
[**create_wholesale_invoice_mark_paid**](WholesaleApi.md#create_wholesale_invoice_mark_paid) | **POST** /v1/wholesale/invoices/{id}/mark-paid | Record a payment the buyer made to you directly: method ach | check | wire | card | other, a reference, and amountCents (omit for the whole balance). When the invoice is settled, Crossly's fee and the tax it remits are charged to your billing card.
[**create_wholesale_invoice_send**](WholesaleApi.md#create_wholesale_invoice_send) | **POST** /v1/wholesale/invoices/{id}/send | Email the invoice to the buyer again (an overdue one goes as an overdue notice), with the link to pay it.
[**create_wholesale_invoice_void**](WholesaleApi.md#create_wholesale_invoice_void) | **POST** /v1/wholesale/invoices/{id}/void | Void an invoice nobody has paid anything on (e.g. the order was cancelled). It stops counting against the buyer's credit limit.
[**create_wholesale_invoice_write_off**](WholesaleApi.md#create_wholesale_invoice_write_off) | **POST** /v1/wholesale/invoices/{id}/write-off | Write off what is left on an invoice (bad debt): it closes as written_off and stops counting against the buyer's credit.
[**create_wholesale_profile_ships_from_validate**](WholesaleApi.md#create_wholesale_profile_ships_from_validate) | **POST** /v1/wholesale/profile/ships-from/validate | Check a ships-from origin ({ postalCode, country, city?, region? }) with the shipping validator without saving it. 400 invalid_ships_from with the issues; 422 for an embargoed origin.
[**create_wholesale_quote_answer**](WholesaleApi.md#create_wholesale_quote_answer) | **POST** /v1/wholesale/quotes/{id}/answer | Answer (or revise) a quote: a unit price in CENTS per item you quote, an expiresAt (ISO, at most 90 days out) and an optional note. Items left out are not quoted. The buyer is notified and can accept until expiry.
[**create_wholesale_quote_decline**](WholesaleApi.md#create_wholesale_quote_decline) | **POST** /v1/wholesale/quotes/{id}/decline | Decline a quote request, with an optional reason the buyer sees.
[**create_wholesale_request_decide**](WholesaleApi.md#create_wholesale_request_decide) | **POST** /v1/wholesale/requests/{id}/decide | Approve (grants access to that catalog; optional groupId adds the buyer to a customer group) or decline (optional reason) an access request.
[**create_wholesale_tier_bulk**](WholesaleApi.md#create_wholesale_tier_bulk) | **POST** /v1/wholesale/tiers/bulk | One ladder for many listings: breaks as [{ minQuantity, discountBps }] off each listing's own price. Returns how many were updated and which were skipped and why.
[**delete_wholesale_buyer_term**](WholesaleApi.md#delete_wholesale_buyer_term) | **DELETE** /v1/wholesale/buyers/{buyerUserId}/terms | Switch a buyer's account terms off. Invoices already issued are unaffected.
[**delete_wholesale_catalog**](WholesaleApi.md#delete_wholesale_catalog) | **DELETE** /v1/wholesale/catalogs/{id} | Delete a catalog with its items, grants and access requests. Orders and invoices are untouched.
[**delete_wholesale_catalog_item**](WholesaleApi.md#delete_wholesale_catalog_item) | **DELETE** /v1/wholesale/catalogs/{id}/items/{platformListingId} | Take an item out of a catalog. The listing itself is untouched.
[**delete_wholesale_group**](WholesaleApi.md#delete_wholesale_group) | **DELETE** /v1/wholesale/groups/{id} | Delete a customer group. Its members keep any other group or account rate.
[**delete_wholesale_group_member**](WholesaleApi.md#delete_wholesale_group_member) | **DELETE** /v1/wholesale/groups/{id}/members/{buyerUserId} | Take a buyer out of a group.
[**get_wholesale_catalog_line_sheet**](WholesaleApi.md#get_wholesale_catalog_line_sheet) | **GET** /v1/wholesale/catalogs/{id}/line-sheet | The catalog's line sheet: your company details and terms, then every item with retail and trade price, minimum, case pack and quantity breaks.
[**get_wholesale_catalog_share**](WholesaleApi.md#get_wholesale_catalog_share) | **GET** /v1/wholesale/catalogs/{id}/share | The catalog's buyer link (null until you have a shop username), to share or print as a QR code.
[**get_wholesale_category**](WholesaleApi.md#get_wholesale_category) | **GET** /v1/wholesale/categories | The wholesale category taxonomy (id, label, group). A catalog or trade profile takes up to 12 ids, or \"other:<words>\".
[**get_wholesale_drop**](WholesaleApi.md#get_wholesale_drop) | **GET** /v1/wholesale/drops/{id} | One trade drop with its listings and, for a pre-book, every reservation: buyer, item, quantity, deposit, balance, status and the balance invoice once released.
[**get_wholesale_invoice**](WholesaleApi.md#get_wholesale_invoice) | **GET** /v1/wholesale/invoices/{id} | One invoice with its lines, PO reference, payments and whether Crossly dues on it are collected.
[**get_wholesale_overview**](WholesaleApi.md#get_wholesale_overview) | **GET** /v1/wholesale/overview | Your wholesale store at a glance: the setup checklist, 30-day wholesale revenue, open orders, outstanding invoices, active buyers and pending access requests.
[**get_wholesale_profile**](WholesaleApi.md#get_wholesale_profile) | **GET** /v1/wholesale/profile | A store's trade profile (X-Storefront-Id; default store when omitted): company, tagline, logo and banner, contact, ships-from origin, lead time, order minimums (opening, first order, reorder), the default MOQ / case pack / max per order / backorder / price visibility, return, payment and shipping preset ids, invoicing and its settings, buyer requirements and the directory defaults.
[**get_wholesale_quote**](WholesaleApi.md#get_wholesale_quote) | **GET** /v1/wholesale/quotes/{id} | One quote request.
[**get_wholesale_reserve**](WholesaleApi.md#get_wholesale_reserve) | **GET** /v1/wholesale/reserve/{inventoryItemId} | How many units of an inventory item are held back for wholesale, and what retail channels advertise.
[**get_wholesale_term_preview**](WholesaleApi.md#get_wholesale_term_preview) | **GET** /v1/wholesale/terms/preview | The trade terms that apply to a catalog item (?catalogId&platformListingId), a catalog (?catalogId) or the trade profile alone, each with its source (item, catalog, profile, default), plus the return, payment and shipping policies they resolve to.
[**list_wholesale_buyers**](WholesaleApi.md#list_wholesale_buyers) | **GET** /v1/wholesale/buyers | Your trade buyers: catalogs they can open, customer groups, account terms, outstanding balance, wholesale spend, business verification and resale-certificate status (read-only — Crossly reviews those).
[**list_wholesale_catalog_items**](WholesaleApi.md#list_wholesale_catalog_items) | **GET** /v1/wholesale/catalogs/{id}/items | A catalog's items in merchandising order, each with its retail price, trade price, minimum order quantity, case pack and quantity breaks.
[**list_wholesale_catalogs**](WholesaleApi.md#list_wholesale_catalogs) | **GET** /v1/wholesale/catalogs | Your wholesale catalogs with item, buyer-grant and pending-request counts.
[**list_wholesale_drops**](WholesaleApi.md#list_wholesale_drops) | **GET** /v1/wholesale/drops | Your trade drops and pre-books: audience (trade = your approved trade buyers, catalog = whoever can open audienceCatalogId), kind (release | prebook), deposit, ship date, status, and for a pre-book its reservation totals (buyers, units, deposits taken, balances still to invoice).
[**list_wholesale_freight_requests**](WholesaleApi.md#list_wholesale_freight_requests) | **GET** /v1/wholesale/freight-requests | Freight (LTL) quote requests from buyers whose order is over your pallet threshold.
[**list_wholesale_groups**](WholesaleApi.md#list_wholesale_groups) | **GET** /v1/wholesale/groups | Your customer groups with their discount and members.
[**list_wholesale_invoices**](WholesaleApi.md#list_wholesale_invoices) | **GET** /v1/wholesale/invoices | On-account invoices, newest first, with balance due. ?status=unpaid for open, partial and overdue together.
[**list_wholesale_orders**](WholesaleApi.md#list_wholesale_orders) | **GET** /v1/wholesale/orders | Wholesale orders: Crossly orders priced through your trade terms (catalog price, quantity break or account rate) or placed on account. Fulfil them like any order.
[**list_wholesale_quotes**](WholesaleApi.md#list_wholesale_quotes) | **GET** /v1/wholesale/quotes | Quote requests (RFQ) from trade buyers: items with quantity and target price, your quoted prices and expiry. ?status=open lists the ones waiting on you or the buyer.
[**list_wholesale_requests**](WholesaleApi.md#list_wholesale_requests) | **GET** /v1/wholesale/requests | Requests from buyers to open your private or trade-only catalogs, with each buyer's business-verification and tax-exempt status. ?status=all includes decided ones.
[**list_wholesale_setup_steps**](WholesaleApi.md#list_wholesale_setup_steps) | **GET** /v1/wholesale/setup-steps | The wholesale setup checklist for a store: profile, ships-from, payment, returns, a catalog with items, visibility chosen, live. Each step is done because the thing exists.
[**update_wholesale_buyer_term**](WholesaleApi.md#update_wholesale_buyer_term) | **PUT** /v1/wholesale/buyers/{buyerUserId}/terms | Set a buyer's account terms: netDays (0-120) for paying on account, creditLimitCents (the most they may owe you at once; null = no limit) and discountPercentBps (their negotiated rate).
[**update_wholesale_catalog**](WholesaleApi.md#update_wholesale_catalog) | **PATCH** /v1/wholesale/catalogs/{id} | Edit a catalog: name, slug, description, cover image, visibility, opening-order minimum, trade gate, directory listing, active.
[**update_wholesale_catalog_item**](WholesaleApi.md#update_wholesale_catalog_item) | **PATCH** /v1/wholesale/catalogs/{id}/items | Change many catalog items at once: items[] sets each row (trade price, MOQ, case pack, lead time, max per order, backorder, restock date, verified-only, price visibility, preset ids); or platformListingIds + set applies the same values to each, set.pctOffRetailBps pricing each from its retail price.
[**update_wholesale_catalog_item_by_platform_listing_id**](WholesaleApi.md#update_wholesale_catalog_item_by_platform_listing_id) | **PATCH** /v1/wholesale/catalogs/{id}/items/{platformListingId} | Change one item's trade price and its own trade terms in this catalog (MOQ, case pack, lead time, max per order, backorder, restock date, verified-only, price visibility, preset ids). null clears = inherit from the catalog.
[**update_wholesale_catalog_item_order**](WholesaleApi.md#update_wholesale_catalog_item_order) | **PUT** /v1/wholesale/catalogs/{id}/items/order | Set the merchandising order: platform listing ids, first to last. Items left out keep their relative order after these.
[**update_wholesale_drop**](WholesaleApi.md#update_wholesale_drop) | **PATCH** /v1/wholesale/drops/{id} | Edit a trade drop (any create field but kind), schedule it (status \"scheduled\", with dropAt in the future) or cancel it. A live drop can only be cancelled or have endsAt / perCustomerLimit changed.
[**update_wholesale_group**](WholesaleApi.md#update_wholesale_group) | **PATCH** /v1/wholesale/groups/{id} | Rename a group, change its discount, or pause it (active=false).
[**update_wholesale_listing_tier**](WholesaleApi.md#update_wholesale_listing_tier) | **PUT** /v1/wholesale/listings/{platformListingId}/tiers | Replace one listing's quantity breaks: [{ minQuantity ≥ 2, unitPriceCents }], each cheaper than the last and than the list price. An empty list removes them. Applies to every buyer at checkout.
[**update_wholesale_profile**](WholesaleApi.md#update_wholesale_profile) | **PUT** /v1/wholesale/profile | Save a store's trade profile. Omitted fields are unchanged; null clears. shipsFrom is checked by the shipping validator (400 invalid_ships_from). The profile is the bottom of the trade-terms cascade: catalogs and items that leave a term empty inherit it — saving never rewrites catalogs. Preset ids must be your own presets of that kind.
[**update_wholesale_reserve**](WholesaleApi.md#update_wholesale_reserve) | **PUT** /v1/wholesale/reserve/{inventoryItemId} | Reserve N units of an inventory item for wholesale: retail channels advertise the rest and come down when only the reserve is left. 0 clears it.



## create_wholesale_buyer_invite

> crate::models::CreateWholesaleBuyerInviteResponse create_wholesale_buyer_invite()
Invite a buyer by email to one or more of your catalogs. Binds to their account now if they have one, or when they sign up.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleBuyerInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_buyer_remove

> crate::models::CreateWholesaleBuyerRemoveResponse create_wholesale_buyer_remove()
Revoke every catalog grant a buyer (or invited email) has with you. Their terms and invoices are kept.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleBuyerRemoveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_catalog

> crate::models::CreateWholesaleCatalogResponse create_wholesale_catalog()
Create a catalog. visibility public | public_prices_private (prices for approved buyers) | private_discoverable (listed, opened on approval) | link (anyone with the URL) | invite (named buyers only; \"private\" is accepted as invite); autoApprove never | verified | all; the cascade terms (minOrderQuantity, casePack, leadTimeDays, verifiedOnly, priceVisibility, maxPerOrder, backorder, restockEta, preset ids) default every item; listed advertises a public catalog in the wholesale directory (never its prices).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleCatalogResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_catalog_item

> crate::models::CreateWholesaleCatalogItemResponse create_wholesale_catalog_item(id)
Add your Crossly listings to a catalog (up to 500), each with an optional wholesalePriceCents, minOrderQuantity and casePack. Items already in the catalog have their terms updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleCatalogItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_drop

> crate::models::CreateWholesaleDropResponse create_wholesale_drop()
Create a draft trade drop: name, dropAt (ISO; when it opens), audience trade|catalog (+ audienceCatalogId), kind release|prebook. A pre-book needs depositBps (100-9900) and shipBy (YYYY-MM-DD), and takes balanceNetDays (0-90) for the balance invoice. listingIds are your listing ids, each on Crossly. A pre-book makes its listings pre-orders, so buyers pay the deposit at checkout. Schedule it with PATCH status \"scheduled\".

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_drop_release

> crate::models::CreateWholesaleDropReleaseResponse create_wholesale_drop_release(id)
Release a live pre-book: every reserved balance becomes an invoice on account to its buyer (due in the drop balanceNetDays), emailed, and the drop ends. Safe to retry: only balances not yet invoiced are invoiced. Returns { invoiced, balanceCents, invoiceIds }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleDropReleaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_freight_request_answer

> crate::models::CreateWholesaleFreightRequestAnswerResponse create_wholesale_freight_request_answer(id)
Answer an open freight request with a figure in CENTS (and an optional note).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleFreightRequestAnswerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_group

> crate::models::CreateWholesaleGroupResponse create_wholesale_group()
Create a customer group with a discount in basis points (up to 9000). A buyer in several groups gets the best rate, never a stacked one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_group_member

> crate::models::CreateWholesaleGroupMemberResponse create_wholesale_group_member(id)
Add a buyer to a group by account id or by the email of an existing Crossly account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleGroupMemberResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_credit_note

> crate::models::CreateWholesaleInvoiceCreditNoteResponse create_wholesale_invoice_credit_note(id)
Issue a credit note on an on-account invoice (short shipment, damaged, pricing error, returned, goodwill, other): at most the balance due. Numbered <invoice>-CN<n>; settling the rest closes the invoice paid. Not a refund.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleInvoiceCreditNoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_late_fee

> crate::models::WholesaleInvoiceResponse create_wholesale_invoice_late_fee(id)
Add your late fee (invoice settings lateFeeBps) to an invoice by hand, once. The hourly sweep adds it automatically when an invoice goes overdue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_mark_paid

> crate::models::WholesaleInvoiceResponse create_wholesale_invoice_mark_paid(id)
Record a payment the buyer made to you directly: method ach | check | wire | card | other, a reference, and amountCents (omit for the whole balance). When the invoice is settled, Crossly's fee and the tax it remits are charged to your billing card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_send

> crate::models::CreateWholesaleInvoiceSendResponse create_wholesale_invoice_send(id)
Email the invoice to the buyer again (an overdue one goes as an overdue notice), with the link to pay it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleInvoiceSendResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_void

> crate::models::WholesaleInvoiceResponse create_wholesale_invoice_void(id)
Void an invoice nobody has paid anything on (e.g. the order was cancelled). It stops counting against the buyer's credit limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_invoice_write_off

> crate::models::WholesaleInvoiceResponse create_wholesale_invoice_write_off(id)
Write off what is left on an invoice (bad debt): it closes as written_off and stops counting against the buyer's credit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_profile_ships_from_validate

> crate::models::CreateWholesaleProfileShipsFromValidateResponse create_wholesale_profile_ships_from_validate()
Check a ships-from origin ({ postalCode, country, city?, region? }) with the shipping validator without saving it. 400 invalid_ships_from with the issues; 422 for an embargoed origin.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleProfileShipsFromValidateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_quote_answer

> crate::models::WholesaleQuoteResponse create_wholesale_quote_answer(id)
Answer (or revise) a quote: a unit price in CENTS per item you quote, an expiresAt (ISO, at most 90 days out) and an optional note. Items left out are not quoted. The buyer is notified and can accept until expiry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_quote_decline

> crate::models::WholesaleQuoteResponse create_wholesale_quote_decline(id)
Decline a quote request, with an optional reason the buyer sees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_request_decide

> crate::models::CreateWholesaleRequestDecideResponse create_wholesale_request_decide(id)
Approve (grants access to that catalog; optional groupId adds the buyer to a customer group) or decline (optional reason) an access request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleRequestDecideResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wholesale_tier_bulk

> crate::models::CreateWholesaleTierBulkResponse create_wholesale_tier_bulk()
One ladder for many listings: breaks as [{ minQuantity, discountBps }] off each listing's own price. Returns how many were updated and which were skipped and why.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWholesaleTierBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_wholesale_buyer_term

> crate::models::UpdateGroupBuyResponse delete_wholesale_buyer_term(buyer_user_id)
Switch a buyer's account terms off. Invoices already issued are unaffected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buyer_user_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_wholesale_catalog

> crate::models::UpdateGroupBuyResponse delete_wholesale_catalog(id)
Delete a catalog with its items, grants and access requests. Orders and invoices are untouched.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_wholesale_catalog_item

> crate::models::UpdateGroupBuyResponse delete_wholesale_catalog_item(id, platform_listing_id)
Take an item out of a catalog. The listing itself is untouched.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_wholesale_group

> crate::models::UpdateGroupBuyResponse delete_wholesale_group(id)
Delete a customer group. Its members keep any other group or account rate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_wholesale_group_member

> crate::models::UpdateGroupBuyResponse delete_wholesale_group_member(id, buyer_user_id)
Take a buyer out of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**buyer_user_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_catalog_line_sheet

> crate::models::GetWholesaleCatalogLineSheetResponse get_wholesale_catalog_line_sheet(id)
The catalog's line sheet: your company details and terms, then every item with retail and trade price, minimum, case pack and quantity breaks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWholesaleCatalogLineSheetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_catalog_share

> crate::models::GetWholesaleCatalogShareResponse get_wholesale_catalog_share(id)
The catalog's buyer link (null until you have a shop username), to share or print as a QR code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWholesaleCatalogShareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_category

> crate::models::GetWholesaleCategoryResponse get_wholesale_category()
The wholesale category taxonomy (id, label, group). A catalog or trade profile takes up to 12 ids, or \"other:<words>\".

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWholesaleCategoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_drop

> crate::models::GetWholesaleDropResponse get_wholesale_drop(id)
One trade drop with its listings and, for a pre-book, every reservation: buyer, item, quantity, deposit, balance, status and the balance invoice once released.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWholesaleDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_invoice

> crate::models::WholesaleInvoiceResponse get_wholesale_invoice(id)
One invoice with its lines, PO reference, payments and whether Crossly dues on it are collected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_overview

> crate::models::GetWholesaleOverviewResponse get_wholesale_overview()
Your wholesale store at a glance: the setup checklist, 30-day wholesale revenue, open orders, outstanding invoices, active buyers and pending access requests.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWholesaleOverviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_profile

> crate::models::GetWholesaleProfileResponse get_wholesale_profile()
A store's trade profile (X-Storefront-Id; default store when omitted): company, tagline, logo and banner, contact, ships-from origin, lead time, order minimums (opening, first order, reorder), the default MOQ / case pack / max per order / backorder / price visibility, return, payment and shipping preset ids, invoicing and its settings, buyer requirements and the directory defaults.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWholesaleProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_quote

> crate::models::WholesaleQuoteResponse get_wholesale_quote(id)
One quote request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WholesaleQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_reserve

> crate::models::GetWholesaleReserveResponse get_wholesale_reserve(inventory_item_id)
How many units of an inventory item are held back for wholesale, and what retail channels advertise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetWholesaleReserveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wholesale_term_preview

> crate::models::GetWholesaleTermPreviewResponse get_wholesale_term_preview()
The trade terms that apply to a catalog item (?catalogId&platformListingId), a catalog (?catalogId) or the trade profile alone, each with its source (item, catalog, profile, default), plus the return, payment and shipping policies they resolve to.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWholesaleTermPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_buyers

> crate::models::ListWholesaleBuyersResponse list_wholesale_buyers()
Your trade buyers: catalogs they can open, customer groups, account terms, outstanding balance, wholesale spend, business verification and resale-certificate status (read-only — Crossly reviews those).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleBuyersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_catalog_items

> crate::models::ListWholesaleCatalogItemsResponse list_wholesale_catalog_items(id)
A catalog's items in merchandising order, each with its retail price, trade price, minimum order quantity, case pack and quantity breaks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListWholesaleCatalogItemsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_catalogs

> crate::models::ListWholesaleCatalogsResponse list_wholesale_catalogs()
Your wholesale catalogs with item, buyer-grant and pending-request counts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleCatalogsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_drops

> crate::models::ListWholesaleDropsResponse list_wholesale_drops()
Your trade drops and pre-books: audience (trade = your approved trade buyers, catalog = whoever can open audienceCatalogId), kind (release | prebook), deposit, ship date, status, and for a pre-book its reservation totals (buyers, units, deposits taken, balances still to invoice).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleDropsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_freight_requests

> crate::models::ListWholesaleFreightRequestsResponse list_wholesale_freight_requests()
Freight (LTL) quote requests from buyers whose order is over your pallet threshold.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleFreightRequestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_groups

> crate::models::ListWholesaleGroupsResponse list_wholesale_groups()
Your customer groups with their discount and members.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleGroupsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_invoices

> crate::models::ListWholesaleInvoicesResponse list_wholesale_invoices()
On-account invoices, newest first, with balance due. ?status=unpaid for open, partial and overdue together.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleInvoicesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_orders

> crate::models::ListWholesaleOrdersResponse list_wholesale_orders()
Wholesale orders: Crossly orders priced through your trade terms (catalog price, quantity break or account rate) or placed on account. Fulfil them like any order.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_quotes

> crate::models::ListWholesaleQuotesResponse list_wholesale_quotes()
Quote requests (RFQ) from trade buyers: items with quantity and target price, your quoted prices and expiry. ?status=open lists the ones waiting on you or the buyer.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleQuotesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_requests

> crate::models::ListWholesaleRequestsResponse list_wholesale_requests()
Requests from buyers to open your private or trade-only catalogs, with each buyer's business-verification and tax-exempt status. ?status=all includes decided ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleRequestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wholesale_setup_steps

> crate::models::ListWholesaleSetupStepsResponse list_wholesale_setup_steps()
The wholesale setup checklist for a store: profile, ships-from, payment, returns, a catalog with items, visibility chosen, live. Each step is done because the thing exists.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWholesaleSetupStepsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_buyer_term

> crate::models::UpdateWholesaleBuyerTermResponse update_wholesale_buyer_term(buyer_user_id)
Set a buyer's account terms: netDays (0-120) for paying on account, creditLimitCents (the most they may owe you at once; null = no limit) and discountPercentBps (their negotiated rate).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buyer_user_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWholesaleBuyerTermResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_catalog

> crate::models::CreateWholesaleCatalogResponse update_wholesale_catalog(id)
Edit a catalog: name, slug, description, cover image, visibility, opening-order minimum, trade gate, directory listing, active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleCatalogResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_catalog_item

> crate::models::UpdateWholesaleCatalogItemResponse update_wholesale_catalog_item(id)
Change many catalog items at once: items[] sets each row (trade price, MOQ, case pack, lead time, max per order, backorder, restock date, verified-only, price visibility, preset ids); or platformListingIds + set applies the same values to each, set.pctOffRetailBps pricing each from its retail price.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWholesaleCatalogItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_catalog_item_by_platform_listing_id

> crate::models::UpdateGroupBuyResponse update_wholesale_catalog_item_by_platform_listing_id(id, platform_listing_id)
Change one item's trade price and its own trade terms in this catalog (MOQ, case pack, lead time, max per order, backorder, restock date, verified-only, price visibility, preset ids). null clears = inherit from the catalog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_catalog_item_order

> crate::models::ListWholesaleCatalogItemsResponse update_wholesale_catalog_item_order(id)
Set the merchandising order: platform listing ids, first to last. Items left out keep their relative order after these.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListWholesaleCatalogItemsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_drop

> crate::models::CreateWholesaleDropResponse update_wholesale_drop(id)
Edit a trade drop (any create field but kind), schedule it (status \"scheduled\", with dropAt in the future) or cancel it. A live drop can only be cancelled or have endsAt / perCustomerLimit changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_group

> crate::models::CreateWholesaleGroupResponse update_wholesale_group(id)
Rename a group, change its discount, or pause it (active=false).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWholesaleGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_listing_tier

> crate::models::UpdateWholesaleListingTierResponse update_wholesale_listing_tier(platform_listing_id)
Replace one listing's quantity breaks: [{ minQuantity ≥ 2, unitPriceCents }], each cheaper than the last and than the list price. An empty list removes them. Applies to every buyer at checkout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWholesaleListingTierResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_profile

> crate::models::UpdateWholesaleProfileResponse update_wholesale_profile()
Save a store's trade profile. Omitted fields are unchanged; null clears. shipsFrom is checked by the shipping validator (400 invalid_ships_from). The profile is the bottom of the trade-terms cascade: catalogs and items that leave a term empty inherit it — saving never rewrites catalogs. Preset ids must be your own presets of that kind.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateWholesaleProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wholesale_reserve

> crate::models::GetWholesaleReserveResponse update_wholesale_reserve(inventory_item_id)
Reserve N units of an inventory item for wholesale: retail channels advertise the rest and come down when only the reserve is left. 0 clears it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetWholesaleReserveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

