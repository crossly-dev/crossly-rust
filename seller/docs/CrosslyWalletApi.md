# \CrosslyWalletApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcg_nft_reconcile**](CrosslyWalletApi.md#create_tcg_nft_reconcile) | **POST** /v1/tcg/nfts/reconcile | Re-read your wallets on chain so your card counts follow what you hold (cards you moved away leave; cards you received join).
[**create_wallet_crossly**](CrosslyWalletApi.md#create_wallet_crossly) | **POST** /v1/wallet/crossly | Create your Crossly wallet now (it is otherwise created for you in the background). Idempotent.
[**get_wallet_crossly**](CrosslyWalletApi.md#get_wallet_crossly) | **GET** /v1/wallet/crossly | Your Crossly wallet (self-custody Solana): status, address, whether it is configured, and your card NFT counts by status.
[**list_tcg_nfts**](CrosslyWalletApi.md#list_tcg_nfts) | **GET** /v1/tcg/{game}/nfts | Your card NFTs for a Crossly TCG game: each copy, its mint status, asset id and whether it is soulbound (starter).



## create_tcg_nft_reconcile

> crate::models::CreateTcgNftReconcileResponse create_tcg_nft_reconcile()
Re-read your wallets on chain so your card counts follow what you hold (cards you moved away leave; cards you received join).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateTcgNftReconcileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_wallet_crossly

> crate::models::GetWalletCrosslyResponse create_wallet_crossly()
Create your Crossly wallet now (it is otherwise created for you in the background). Idempotent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWalletCrosslyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_wallet_crossly

> crate::models::GetWalletCrosslyResponse get_wallet_crossly()
Your Crossly wallet (self-custody Solana): status, address, whether it is configured, and your card NFT counts by status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWalletCrosslyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_nfts

> crate::models::ListTcgNftsResponse list_tcg_nfts(game)
Your card NFTs for a Crossly TCG game: each copy, its mint status, asset id and whether it is soulbound (starter).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::ListTcgNftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

