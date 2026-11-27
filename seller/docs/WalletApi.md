# \WalletApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_wallet_default**](WalletApi.md#delete_wallet_default) | **DELETE** /v1/wallet/defaults | Clear a default (the purpose falls back to your only wallet, or none) or one override (`refKind` + `refKey`).
[**list_wallet_defaults**](WalletApi.md#list_wallet_defaults) | **GET** /v1/wallet/defaults | Your default wallet per purpose (royalties, deploys, drops, game rewards, staking, CBX withdrawals), with per-collection and per-game overrides and what each resolves to today.
[**update_wallet_default**](WalletApi.md#update_wallet_default) | **PUT** /v1/wallet/defaults | Set the default wallet for a purpose, or an override for one collection (`ref: { kind: \"collection\", chain, contract }`) or game (`ref: { kind: \"game\", experienceId }`). The wallet must be yours, verified, and on a network the purpose accepts.



## delete_wallet_default

> crate::models::WalletResponse delete_wallet_default()
Clear a default (the purpose falls back to your only wallet, or none) or one override (`refKind` + `refKey`).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WalletResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wallet_defaults

> crate::models::WalletResponse list_wallet_defaults()
Your default wallet per purpose (royalties, deploys, drops, game rewards, staking, CBX withdrawals), with per-collection and per-game overrides and what each resolves to today.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WalletResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_wallet_default

> crate::models::WalletResponse update_wallet_default()
Set the default wallet for a purpose, or an override for one collection (`ref: { kind: \"collection\", chain, contract }`) or game (`ref: { kind: \"game\", experienceId }`). The wallet must be yours, verified, and on a network the purpose accepts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WalletResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

