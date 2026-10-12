# \Web3Api

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_web3_alert**](Web3Api.md#create_web3_alert) | **POST** /v1/web3/alerts | Set a price alert on a token, or on a whole collection without tokenId: you are notified when one is listed at or below `price` (the chain's native coin). Setting it again moves the price.
[**create_web3_auction**](Web3Api.md#create_web3_auction) | **POST** /v1/web3/auctions | Start an English auction on a token in your linked wallet: `reserve` is the opening bid in the chain's native coin, `hours` its length (1–720). Bids must rise 5%; a late bid extends the end by 10 minutes.
[**create_web3_auction_cancel**](Web3Api.md#create_web3_auction_cancel) | **POST** /v1/web3/auctions/{id}/cancel | Call off your auction while it has no bids.
[**create_web3_creator_claim**](Web3Api.md#create_web3_creator_claim) | **POST** /v1/web3/creator/claims | Claim an EVM collection as its creator: its owner() must be one of your verified wallets. Returns `{ chain, contract }`.
[**create_web3_crosslist**](Web3Api.md#create_web3_crosslist) | **POST** /v1/web3/crosslist | Submit signed Seaport listings for several marketplaces at once (crossly, opensea); results per marketplace.
[**create_web3_drop_remind**](Web3Api.md#create_web3_drop_remind) | **POST** /v1/web3/drops/{id}/remind | Turn the 15-minute reminder for a drop on, or off if it already is. Returns `{ remindMe }`.
[**create_web3_favorite**](Web3Api.md#create_web3_favorite) | **POST** /v1/web3/favorites | Favourite an NFT, or unfavourite it if it already is. Returns `{ favorited }`.
[**create_web3_launch**](Web3Api.md#create_web3_launch) | **POST** /v1/web3/launches | Draft a launchpad collection: name, symbol, supply, mint price (wei of the chain’s native currency), schedule, optional holder phase and royalty. Crossly takes 5% of each mint inside the contract.
[**create_web3_launch_airdrop**](Web3Api.md#create_web3_launch_airdrop) | **POST** /v1/web3/launches/{id}/airdrop | Plan an airdrop (one free NFT each, up to 500) to `addresses` and/or the holders of `holdersOf`. Returns the transactions for the owner wallet; no payment moves.
[**create_web3_launch_art**](Web3Api.md#create_web3_launch_art) | **POST** /v1/web3/launches/{id}/art | Append per-token art (image URL, name, traits), up to the supply, before the reveal. Returns `{ count }`.
[**create_web3_launch_confirm_deploy**](Web3Api.md#create_web3_launch_confirm_deploy) | **POST** /v1/web3/launches/{id}/confirm-deploy | Confirm a sent deploy transaction: the contract must be the Crossly launch contract with this draft’s settings; it then goes live.
[**create_web3_launch_follow_creator**](Web3Api.md#create_web3_launch_follow_creator) | **POST** /v1/web3/launches/{id}/follow-creator | Follow the creator of a launch: their launches are announced to you, and again when each opens. Returns `{ following }`.
[**create_web3_launch_reveal**](Web3Api.md#create_web3_launch_reveal) | **POST** /v1/web3/launches/{id}/reveal | Reveal the art: one random offset maps every token to its asset for good. Returns `{ revealedAt }`.
[**create_web3_order**](Web3Api.md#create_web3_order) | **POST** /v1/web3/orders | Place a signed Seaport 1.6 order (listing, offer or collection offer) in the Crossly book.
[**create_web3_order_cancel**](Web3Api.md#create_web3_order_cancel) | **POST** /v1/web3/orders/{chain}/{hash}/cancel | Take one of your orders out of the Crossly book.
[**create_web3_order_fulfillment_batch**](Web3Api.md#create_web3_order_fulfillment_batch) | **POST** /v1/web3/orders/fulfillment-batch | Fulfilments for sweeping several listings in one Seaport transaction; unfillable ones are skipped with a reason.
[**create_web3_phygital**](Web3Api.md#create_web3_phygital) | **POST** /v1/web3/phygitals | Make an NFT you hold redeemable for one of your physical inventory items.
[**create_web3_phygital_redeem**](Web3Api.md#create_web3_phygital_redeem) | **POST** /v1/web3/phygitals/{id}/redeem | Redeem a phygital you hold (checked on-chain) for its physical item, with a shipping address. Once per phygital.
[**create_web3_phygital_redemption_ship**](Web3Api.md#create_web3_phygital_redemption_ship) | **POST** /v1/web3/phygitals/redemptions/{id}/ship | Mark a redemption shipped with its carrier and tracking number; the redeemer is notified.
[**create_web3_rarible_draft**](Web3Api.md#create_web3_rarible_draft) | **POST** /v1/web3/rarible/draft | Draft a Rarible V2 listing for an NFT in your linked wallet: the EIP-712 order to sign and whether Rarible's transfer proxy is approved.
[**create_web3_rarible_listing**](Web3Api.md#create_web3_rarible_listing) | **POST** /v1/web3/rarible/listings | Submit a signed Rarible V2 listing: the signature is verified, then the order is posted to Rarible and recorded.
[**create_web3_solana_list_tx**](Web3Api.md#create_web3_solana_list_tx) | **POST** /v1/web3/solana/{marketplace}/list-tx | Build an unsigned listing transaction on the Crossly book, Tensor or Magic Eden for a Solana NFT in your linked wallet.
[**create_web3_solana_listing**](Web3Api.md#create_web3_solana_listing) | **POST** /v1/web3/solana/{marketplace}/listings | Record a Crossly-book, Tensor or Magic Eden listing your wallet sent (read back from the chain first).
[**create_web3_solana_offer**](Web3Api.md#create_web3_solana_offer) | **POST** /v1/web3/solana/offers | Record a sent bid; it is recorded only once its trade state exists on chain.
[**create_web3_solana_offer_accept_tx**](Web3Api.md#create_web3_solana_offer_accept_tx) | **POST** /v1/web3/solana/offers/{hash}/accept-tx | Build the sale into a bid from your linked wallet that holds the NFT (sell + execute_sale, one transaction).
[**create_web3_solana_offer_cancel_tx**](Web3Api.md#create_web3_solana_offer_cancel_tx) | **POST** /v1/web3/solana/offers/{hash}/cancel-tx | Build the cancel of your bid: the SOL your other bids do not need comes back.
[**create_web3_solana_offer_confirm**](Web3Api.md#create_web3_solana_offer_confirm) | **POST** /v1/web3/solana/offers/{hash}/confirm | Apply a sent accept or cancel once the chain shows the bid closed.
[**create_web3_solana_offer_tx**](Web3Api.md#create_web3_solana_offer_tx) | **POST** /v1/web3/solana/offers/tx | Build a bid on a Solana NFT on Crossly's book for your linked wallet to sign; the SOL goes into your Auction House escrow.
[**create_web3_solana_order_buy_tx**](Web3Api.md#create_web3_solana_order_buy_tx) | **POST** /v1/web3/solana/orders/{hash}/buy-tx | Build the unsigned transaction that buys a Solana listing (price capped at the listed price).
[**create_web3_solana_order_confirm**](Web3Api.md#create_web3_solana_order_confirm) | **POST** /v1/web3/solana/orders/{hash}/confirm | Apply a sent Solana buy or delist to the book (the transaction is read back from the chain).
[**create_web3_solana_order_delist_tx**](Web3Api.md#create_web3_solana_order_delist_tx) | **POST** /v1/web3/solana/orders/{hash}/delist-tx | Build the unsigned transaction that takes your Solana listing down.
[**create_web3_solana_tensor_bid_accept_tx**](Web3Api.md#create_web3_solana_tensor_bid_accept_tx) | **POST** /v1/web3/solana/tensor-bids/accept-tx | Build the transaction that sells your NFT into a Tensor bid, never below `minLamports`.
[**create_web3_solana_tensor_bid_cancel_tx**](Web3Api.md#create_web3_solana_tensor_bid_cancel_tx) | **POST** /v1/web3/solana/tensor-bids/cancel-tx | Build the transaction that cancels your Tensor bid.
[**create_web3_solana_tensor_bid_place_tx**](Web3Api.md#create_web3_solana_tensor_bid_place_tx) | **POST** /v1/web3/solana/tensor-bids/place-tx | Build a Tensor collection bid (or trait bid with `trait`) for your Solana wallet to sign. Price in lamports.
[**create_web3_token_refresh**](Web3Api.md#create_web3_token_refresh) | **POST** /v1/web3/tokens/{chain}/{contract}/{tokenId}/refresh | Re-read a token's metadata from the chain's provider (name, traits, image — a new image is copied to Crossly's CDN). Once per token per 10 minutes.
[**create_web3_transaction**](Web3Api.md#create_web3_transaction) | **POST** /v1/web3/transactions | Apply a Seaport transaction's fills and cancels to the Crossly book (read from the receipt).
[**create_web3_wallet_challenge**](Web3Api.md#create_web3_wallet_challenge) | **POST** /v1/web3/wallets/challenge | Start linking a wallet: returns the message the wallet must sign.
[**create_web3_wallet_sync**](Web3Api.md#create_web3_wallet_sync) | **POST** /v1/web3/wallets/{id}/sync | Import a linked wallet's NFTs into inventory; archives ones that have left the wallet.
[**create_web3_wallet_verify**](Web3Api.md#create_web3_wallet_verify) | **POST** /v1/web3/wallets/verify | Finish linking a wallet with its signature over the challenge message.
[**delete_web3_alert**](Web3Api.md#delete_web3_alert) | **DELETE** /v1/web3/alerts/{id} | Delete a price alert.
[**delete_web3_holder_perk**](Web3Api.md#delete_web3_holder_perk) | **DELETE** /v1/web3/holder-perks/{id} | Remove a holder perk. Returns `{ ok }`.
[**delete_web3_launch**](Web3Api.md#delete_web3_launch) | **DELETE** /v1/web3/launches/{id} | Delete a launch draft that never went live. Returns `{ ok }`.
[**delete_web3_launch_follow_creator**](Web3Api.md#delete_web3_launch_follow_creator) | **DELETE** /v1/web3/launches/{id}/follow-creator | Stop following the creator of a launch. Returns `{ following }`.
[**delete_web3_marketplace_key**](Web3Api.md#delete_web3_marketplace_key) | **DELETE** /v1/web3/marketplace-keys/{marketplace} | Remove your own key for a marketplace; Crossly's key is used again.
[**delete_web3_wallet**](Web3Api.md#delete_web3_wallet) | **DELETE** /v1/web3/wallets/{id} | Unlink a wallet. Its NFTs stay in inventory but stop syncing.
[**get_web3_auction**](Web3Api.md#get_web3_auction) | **GET** /v1/web3/auctions/{chain}/{contract}/{tokenId} | A token's live (or latest) auction: reserve, end, top bid, bid count and the minimum next bid.
[**get_web3_browse**](Web3Api.md#get_web3_browse) | **GET** /v1/web3/browse | The Web3 directory's categories and chains, with how many collections each holds.
[**get_web3_collection**](Web3Api.md#get_web3_collection) | **GET** /v1/web3/collections/{chain}/{contract} | A collection: metadata and Crossly stats (floor, listed, volume, best collection offer).
[**get_web3_collection_depth**](Web3Api.md#get_web3_collection_depth) | **GET** /v1/web3/collections/{chain}/{contract}/depth | A collection's Crossly order book by price level — asks cheapest first, bids dearest first, with running totals — and the floor of every trait value.
[**get_web3_collection_insight**](Web3Api.md#get_web3_collection_insight) | **GET** /v1/web3/collections/{chain}/{contract}/insights | On-chain insights for a collection: holders, trait counts across the whole collection, recent transfers and a 30-day activity series.
[**get_web3_collection_item**](Web3Api.md#get_web3_collection_item) | **GET** /v1/web3/collections/{chain}/{contract}/items | A collection's NFTs: filter by traits (repeat trait=Type:Value; OR within a type, AND across), search by name or token id, sort by id, rarity or Crossly price. Whole-collection once indexed.
[**get_web3_collection_trait**](Web3Api.md#get_web3_collection_trait) | **GET** /v1/web3/collections/{chain}/{contract}/traits | Every trait type and value in a collection, with how many tokens have each.
[**get_web3_collection_trait_token**](Web3Api.md#get_web3_collection_trait_token) | **GET** /v1/web3/collections/{chain}/{contract}/trait-tokens | Token ids in a collection with one trait — what a trait offer commits to.
[**get_web3_collector**](Web3Api.md#get_web3_collector) | **GET** /v1/web3/collector | Your collector perks: how many different NFTs you bought through Crossly, your Collector pin tier, relist rewards paid, and each NFT you still hold with the Bought on Crossly mark.
[**get_web3_creator**](Web3Api.md#get_web3_creator) | **GET** /v1/web3/creator/{chain}/{contract} | A collection’s creator dashboard: sales by marketplace and day, estimated royalties, holders, and for a launch the mint revenue and what is waiting to be withdrawn (wei of the chain’s native currency).
[**get_web3_crosslist_quote**](Web3Api.md#get_web3_crosslist_quote) | **GET** /v1/web3/crosslist/quote | Each marketplace's terms for listing a token: Crossly fee, OpenSea required fees and zone, Tensor availability.
[**get_web3_featured**](Web3Api.md#get_web3_featured) | **GET** /v1/web3/featured | The NFT home's featured collections: chosen by Crossly, else the hottest.
[**get_web3_inventory**](Web3Api.md#get_web3_inventory) | **GET** /v1/web3/inventory/{itemId} | The NFT behind an inventory item: its token, wallet, and open listings on every marketplace (null if it is not an NFT).
[**get_web3_launch_allowlist_root_tx**](Web3Api.md#get_web3_launch_allowlist_root_tx) | **GET** /v1/web3/launches/{id}/allowlist/root-tx | The setAllowlistRoot transaction for the allowlist Crossly holds (EVM, live): `{ chain, from, to, data }`.
[**get_web3_launch_deploy_tx**](Web3Api.md#get_web3_launch_deploy_tx) | **GET** /v1/web3/launches/{id}/deploy-tx | The contract-creation transaction for the draft: send it from the owner wallet, then confirm it. Returns `{ chain, from, data }`.
[**get_web3_launch_tool**](Web3Api.md#get_web3_launch_tool) | **GET** /v1/web3/launches/{id}/tools | A launch’s creator tools: allowlist size and root (and whether the contract has it yet), uploaded art and reveal, supply minted.
[**get_web3_order_fulfillment**](Web3Api.md#get_web3_order_fulfillment) | **GET** /v1/web3/orders/{chain}/{hash}/fulfillment | The signed order a taker passes to Seaport, after re-checking it can still fill.
[**get_web3_phygital**](Web3Api.md#get_web3_phygital) | **GET** /v1/web3/phygitals/{chain}/{contract}/{tokenId} | A token's phygital: the physical item it can be redeemed for, and whether it has been.
[**get_web3_phygital2**](Web3Api.md#get_web3_phygital2) | **GET** /v1/web3/phygitals | Phygitals you linked (with each redemption and its shipping address) and ones you redeemed.
[**get_web3_portfolio**](Web3Api.md#get_web3_portfolio) | **GET** /v1/web3/portfolio | Your NFTs with value (Crossly floor, else recent sales) and cost (what a linked wallet paid), realised sales, and USD totals at each day’s coin price.
[**get_web3_portfolio_tax**](Web3Api.md#get_web3_portfolio_tax) | **GET** /v1/web3/portfolio/tax | One calendar year of NFT disposals from your linked wallets as CSV text: acquired, sold, proceeds, cost basis, gain (coin and USD).
[**get_web3_profile**](Web3Api.md#get_web3_profile) | **GET** /v1/web3/profiles/{address} | A wallet's public profile: NFTs held, open Crossly listings, and activity.
[**get_web3_resolve**](Web3Api.md#get_web3_resolve) | **GET** /v1/web3/resolve/{chain}/{slug} | A collection's chain and contract from its readable URL slug (e.g. eth / boredapeyachtclub).
[**get_web3_solana_collection_tensor_bid**](Web3Api.md#get_web3_solana_collection_tensor_bid) | **GET** /v1/web3/solana/collections/{collection}/tensor-bids | A Solana collection’s Tensor bids (collection-wide and by trait), best first; price in lamports. Needs your Tensor key.
[**get_web3_token**](Web3Api.md#get_web3_token) | **GET** /v1/web3/tokens/{chain}/{contract}/{tokenId} | One NFT: metadata, traits, owner, open listings and offers, activity.
[**list_web3_activity**](Web3Api.md#list_web3_activity) | **GET** /v1/web3/activity | Listings, offers, sales and cancels in the Crossly book, newest first.
[**list_web3_alerts**](Web3Api.md#list_web3_alerts) | **GET** /v1/web3/alerts | Your NFT price alerts, with the cheapest open Crossly listing each one covers now.
[**list_web3_approvals**](Web3Api.md#list_web3_approvals) | **GET** /v1/web3/approvals | Your collections the OpenSea conduit is approved to move, per linked wallet.
[**list_web3_auctions**](Web3Api.md#list_web3_auctions) | **GET** /v1/web3/auctions | Live English auctions on the Crossly book, ending soonest first, each with its top bid and the least the next bid can be.
[**list_web3_browse_collections**](Web3Api.md#list_web3_browse_collections) | **GET** /v1/web3/browse/collections | Browse NFT collections by chain, category and name; sort hot (Crossly sales, listings, favourites), new, volume, floor or name.
[**list_web3_browse_tokens**](Web3Api.md#list_web3_browse_tokens) | **GET** /v1/web3/browse/tokens | Hot NFTs: what is for sale on Crossly first, then tokens from the hottest collections.
[**list_web3_bundles**](Web3Api.md#list_web3_bundles) | **GET** /v1/web3/bundles | Open bundle listings (2–30 NFTs for one price), newest first, by chain or by a contract any item is from.
[**list_web3_collection_history**](Web3Api.md#list_web3_collection_history) | **GET** /v1/web3/collections/{chain}/{contract}/history | Daily floor, sales, volume, holders and listed count for a collection, oldest first (up to 365 days).
[**list_web3_collections**](Web3Api.md#list_web3_collections) | **GET** /v1/web3/collections | Collections ranked by 30-day Crossly volume, with floor and listed count.
[**list_web3_creator_collections**](Web3Api.md#list_web3_creator_collections) | **GET** /v1/web3/creator/collections | Collections you are the creator of: your launches, and contracts you claimed by owning them.
[**list_web3_drops**](Web3Api.md#list_web3_drops) | **GET** /v1/web3/drops | Upcoming and live NFT drops (curated mints on the creator's own page), soonest first, with whether you asked to be reminded.
[**list_web3_holder_perks**](Web3Api.md#list_web3_holder_perks) | **GET** /v1/web3/holder-perks | Your holder perks: percent off your Crossly shop for holders of an NFT collection, best first.
[**list_web3_launches**](Web3Api.md#list_web3_launches) | **GET** /v1/web3/launches | Your launchpad collections (drafts and live), newest first, with minted count and phase.
[**list_web3_listings**](Web3Api.md#list_web3_listings) | **GET** /v1/web3/listings | Open NFT listings in the Crossly book — the cheapest per token.
[**list_web3_marketplace_keys**](Web3Api.md#list_web3_marketplace_keys) | **GET** /v1/web3/marketplace-keys | Your own marketplace API keys (last four characters only) and whether Crossly's key covers each marketplace.
[**list_web3_nfts**](Web3Api.md#list_web3_nfts) | **GET** /v1/web3/nfts | List the NFTs in your inventory, imported from your linked wallets.
[**list_web3_orders**](Web3Api.md#list_web3_orders) | **GET** /v1/web3/orders | Your orders in the Crossly book (listings and offers), any status.
[**list_web3_purchases**](Web3Api.md#list_web3_purchases) | **GET** /v1/web3/purchases | NFTs your linked wallets bought — on Crossly or any marketplace the chain feed saw — newest first, with price and whether you still hold each.
[**list_web3_rankings**](Web3Api.md#list_web3_rankings) | **GET** /v1/web3/rankings | Collections by sales volume over 24h, 7d or 30d, in one coin: the ETH chains together, or Polygon / Solana on their own.
[**list_web3_reserved**](Web3Api.md#list_web3_reserved) | **GET** /v1/web3/reserved | Private listings reserved for one of your linked wallets — only you can see or buy them.
[**list_web3_sales**](Web3Api.md#list_web3_sales) | **GET** /v1/web3/sales | The latest NFT sales across the directory, from the chain (Seaport and Blur) and the Crossly book, newest first; page with `before`.
[**list_web3_search**](Web3Api.md#list_web3_search) | **GET** /v1/web3/search | Find collections by contract address (every chain) or by name.
[**list_web3_search_tokens**](Web3Api.md#list_web3_search_tokens) | **GET** /v1/web3/search/tokens | Find tokens by collection name and number: 'pudgy 42', 'Bored Ape #1234'.
[**list_web3_wallets**](Web3Api.md#list_web3_wallets) | **GET** /v1/web3/wallets | List your verified, linked wallets (EVM and Solana).
[**list_web3_watchlist**](Web3Api.md#list_web3_watchlist) | **GET** /v1/web3/watchlist | Your favourited NFTs, with the cheapest open Crossly listing and the last sale of each.
[**update_web3_holder_perk**](Web3Api.md#update_web3_holder_perk) | **PUT** /v1/web3/holder-perks | Set the percent off (100–5000 bps) holders of a collection get across your Crossly shop; one perk per collection.
[**update_web3_launch_allowlist**](Web3Api.md#update_web3_launch_allowlist) | **PUT** /v1/web3/launches/{id}/allowlist | Replace the early-phase allowlist (up to 10,000 wallets). EVM live launches get back `rootTx`, the setAllowlistRoot transaction for the owner wallet.
[**update_web3_marketplace_key**](Web3Api.md#update_web3_marketplace_key) | **PUT** /v1/web3/marketplace-keys/{marketplace} | Save your own API key for a marketplace (checked against the marketplace first where it can be); stored encrypted.



## create_web3_alert

> crate::models::IdResponse create_web3_alert()
Set a price alert on a token, or on a whole collection without tokenId: you are notified when one is listed at or below `price` (the chain's native coin). Setting it again moves the price.

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


## create_web3_auction

> crate::models::CreateWeb3AuctionResponse create_web3_auction()
Start an English auction on a token in your linked wallet: `reserve` is the opening bid in the chain's native coin, `hours` its length (1–720). Bids must rise 5%; a late bid extends the end by 10 minutes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3AuctionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_auction_cancel

> crate::models::OkResponse create_web3_auction_cancel(id)
Call off your auction while it has no bids.

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


## create_web3_creator_claim

> crate::models::CreateWeb3CreatorClaimResponse create_web3_creator_claim()
Claim an EVM collection as its creator: its owner() must be one of your verified wallets. Returns `{ chain, contract }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3CreatorClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_crosslist

> crate::models::CreateWeb3CrosslistResponse create_web3_crosslist()
Submit signed Seaport listings for several marketplaces at once (crossly, opensea); results per marketplace.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3CrosslistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_drop_remind

> crate::models::CreateWeb3DropRemindResponse create_web3_drop_remind(id)
Turn the 15-minute reminder for a drop on, or off if it already is. Returns `{ remindMe }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3DropRemindResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_favorite

> crate::models::CreateWeb3FavoriteResponse create_web3_favorite()
Favourite an NFT, or unfavourite it if it already is. Returns `{ favorited }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3FavoriteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch

> crate::models::ListWeb3LaunchesItem create_web3_launch()
Draft a launchpad collection: name, symbol, supply, mint price (wei of the chain’s native currency), schedule, optional holder phase and royalty. Crossly takes 5% of each mint inside the contract.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3LaunchesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch_airdrop

> crate::models::CreateWeb3LaunchAirdropResponse create_web3_launch_airdrop(id)
Plan an airdrop (one free NFT each, up to 500) to `addresses` and/or the holders of `holdersOf`. Returns the transactions for the owner wallet; no payment moves.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3LaunchAirdropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch_art

> crate::models::CreateWeb3LaunchArtResponse create_web3_launch_art(id)
Append per-token art (image URL, name, traits), up to the supply, before the reveal. Returns `{ count }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3LaunchArtResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch_confirm_deploy

> crate::models::ListWeb3LaunchesItem create_web3_launch_confirm_deploy(id)
Confirm a sent deploy transaction: the contract must be the Crossly launch contract with this draft’s settings; it then goes live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListWeb3LaunchesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch_follow_creator

> crate::models::CreateWeb3LaunchFollowCreatorResponse create_web3_launch_follow_creator(id)
Follow the creator of a launch: their launches are announced to you, and again when each opens. Returns `{ following }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3LaunchFollowCreatorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_launch_reveal

> crate::models::CreateWeb3LaunchRevealResponse create_web3_launch_reveal(id)
Reveal the art: one random offset maps every token to its asset for good. Returns `{ revealedAt }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3LaunchRevealResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_order

> crate::models::CreateWeb3OrderResponse create_web3_order()
Place a signed Seaport 1.6 order (listing, offer or collection offer) in the Crossly book.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3OrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_order_cancel

> crate::models::OkResponse create_web3_order_cancel(chain, hash)
Take one of your orders out of the Crossly book.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**hash** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_order_fulfillment_batch

> crate::models::CreateWeb3OrderFulfillmentBatchResponse create_web3_order_fulfillment_batch()
Fulfilments for sweeping several listings in one Seaport transaction; unfillable ones are skipped with a reason.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3OrderFulfillmentBatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_phygital

> crate::models::CreateWeb3PhygitalResponse create_web3_phygital()
Make an NFT you hold redeemable for one of your physical inventory items.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3PhygitalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_phygital_redeem

> crate::models::CreateWeb3PhygitalRedeemResponse create_web3_phygital_redeem(id)
Redeem a phygital you hold (checked on-chain) for its physical item, with a shipping address. Once per phygital.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3PhygitalRedeemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_phygital_redemption_ship

> crate::models::OkResponse create_web3_phygital_redemption_ship(id)
Mark a redemption shipped with its carrier and tracking number; the redeemer is notified.

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


## create_web3_rarible_draft

> crate::models::CreateWeb3RaribleDraftResponse create_web3_rarible_draft()
Draft a Rarible V2 listing for an NFT in your linked wallet: the EIP-712 order to sign and whether Rarible's transfer proxy is approved.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3RaribleDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_rarible_listing

> crate::models::OrderHashResponse create_web3_rarible_listing()
Submit a signed Rarible V2 listing: the signature is verified, then the order is posted to Rarible and recorded.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OrderHashResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_list_tx

> crate::models::CreateWeb3SolanaListTxResponse create_web3_solana_list_tx(marketplace)
Build an unsigned listing transaction on the Crossly book, Tensor or Magic Eden for a Solana NFT in your linked wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaListTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_listing

> crate::models::OrderHashResponse create_web3_solana_listing(marketplace)
Record a Crossly-book, Tensor or Magic Eden listing your wallet sent (read back from the chain first).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace** | **String** |  | [required] |

### Return type

**crate::models::OrderHashResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_offer

> crate::models::OrderHashResponse create_web3_solana_offer()
Record a sent bid; it is recorded only once its trade state exists on chain.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OrderHashResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_offer_accept_tx

> crate::models::CreateWeb3SolanaListTxResponse create_web3_solana_offer_accept_tx(hash)
Build the sale into a bid from your linked wallet that holds the NFT (sell + execute_sale, one transaction).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaListTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_offer_cancel_tx

> crate::models::CreateWeb3SolanaListTxResponse create_web3_solana_offer_cancel_tx(hash)
Build the cancel of your bid: the SOL your other bids do not need comes back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaListTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_offer_confirm

> crate::models::CreateWeb3SolanaOfferConfirmResponse create_web3_solana_offer_confirm(hash)
Apply a sent accept or cancel once the chain shows the bid closed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaOfferConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_offer_tx

> crate::models::CreateWeb3SolanaOfferTxResponse create_web3_solana_offer_tx()
Build a bid on a Solana NFT on Crossly's book for your linked wallet to sign; the SOL goes into your Auction House escrow.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3SolanaOfferTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_order_buy_tx

> crate::models::CreateWeb3SolanaOrderBuyTxResponse create_web3_solana_order_buy_tx(hash)
Build the unsigned transaction that buys a Solana listing (price capped at the listed price).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaOrderBuyTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_order_confirm

> crate::models::CreateWeb3SolanaOfferConfirmResponse create_web3_solana_order_confirm(hash)
Apply a sent Solana buy or delist to the book (the transaction is read back from the chain).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaOfferConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_order_delist_tx

> crate::models::CreateWeb3SolanaListTxResponse create_web3_solana_order_delist_tx(hash)
Build the unsigned transaction that takes your Solana listing down.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3SolanaListTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_tensor_bid_accept_tx

> crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse create_web3_solana_tensor_bid_accept_tx()
Build the transaction that sells your NFT into a Tensor bid, never below `minLamports`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_tensor_bid_cancel_tx

> crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse create_web3_solana_tensor_bid_cancel_tx()
Build the transaction that cancels your Tensor bid.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_solana_tensor_bid_place_tx

> crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse create_web3_solana_tensor_bid_place_tx()
Build a Tensor collection bid (or trait bid with `trait`) for your Solana wallet to sign. Price in lamports.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3SolanaTensorBidPlaceTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_token_refresh

> crate::models::CreateWeb3TokenRefreshResponse create_web3_token_refresh(chain, contract, token_id)
Re-read a token's metadata from the chain's provider (name, traits, image — a new image is copied to Crossly's CDN). Once per token per 10 minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3TokenRefreshResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_transaction

> crate::models::CreateWeb3TransactionResponse create_web3_transaction()
Apply a Seaport transaction's fills and cancels to the Crossly book (read from the receipt).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3TransactionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_wallet_challenge

> crate::models::CreateWeb3WalletChallengeResponse create_web3_wallet_challenge()
Start linking a wallet: returns the message the wallet must sign.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3WalletChallengeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_wallet_sync

> crate::models::CreateWeb3WalletSyncResponse create_web3_wallet_sync(id)
Import a linked wallet's NFTs into inventory; archives ones that have left the wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3WalletSyncResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_web3_wallet_verify

> crate::models::CreateWeb3WalletVerifyResponse create_web3_wallet_verify()
Finish linking a wallet with its signature over the challenge message.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3WalletVerifyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_web3_alert

> crate::models::OkResponse delete_web3_alert(id)
Delete a price alert.

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


## delete_web3_holder_perk

> crate::models::OkResponse delete_web3_holder_perk(id)
Remove a holder perk. Returns `{ ok }`.

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


## delete_web3_launch

> crate::models::OkResponse delete_web3_launch(id)
Delete a launch draft that never went live. Returns `{ ok }`.

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


## delete_web3_launch_follow_creator

> crate::models::CreateWeb3LaunchFollowCreatorResponse delete_web3_launch_follow_creator(id)
Stop following the creator of a launch. Returns `{ following }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWeb3LaunchFollowCreatorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_web3_marketplace_key

> delete_web3_marketplace_key(marketplace)
Remove your own key for a marketplace; Crossly's key is used again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_web3_wallet

> crate::models::OkResponse delete_web3_wallet(id)
Unlink a wallet. Its NFTs stay in inventory but stop syncing.

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


## get_web3_auction

> crate::models::GetWeb3AuctionResponse get_web3_auction(chain, contract, token_id)
A token's live (or latest) auction: reserve, end, top bid, bid count and the minimum next bid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3AuctionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_browse

> crate::models::GetWeb3BrowseResponse get_web3_browse()
The Web3 directory's categories and chains, with how many collections each holds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3BrowseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection

> crate::models::GetWeb3CollectionResponse get_web3_collection(chain, contract)
A collection: metadata and Crossly stats (floor, listed, volume, best collection offer).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection_depth

> crate::models::GetWeb3CollectionDepthResponse get_web3_collection_depth(chain, contract)
A collection's Crossly order book by price level — asks cheapest first, bids dearest first, with running totals — and the floor of every trait value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionDepthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection_insight

> crate::models::GetWeb3CollectionInsightResponse get_web3_collection_insight(chain, contract)
On-chain insights for a collection: holders, trait counts across the whole collection, recent transfers and a 30-day activity series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionInsightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection_item

> crate::models::GetWeb3CollectionItemResponse get_web3_collection_item(chain, contract)
A collection's NFTs: filter by traits (repeat trait=Type:Value; OR within a type, AND across), search by name or token id, sort by id, rarity or Crossly price. Whole-collection once indexed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection_trait

> crate::models::GetWeb3CollectionTraitResponse get_web3_collection_trait(chain, contract)
Every trait type and value in a collection, with how many tokens have each.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionTraitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collection_trait_token

> crate::models::GetWeb3CollectionTraitTokenResponse get_web3_collection_trait_token(chain, contract)
Token ids in a collection with one trait — what a trait offer commits to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CollectionTraitTokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_collector

> crate::models::GetWeb3CollectorResponse get_web3_collector()
Your collector perks: how many different NFTs you bought through Crossly, your Collector pin tier, relist rewards paid, and each NFT you still hold with the Bought on Crossly mark.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3CollectorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_creator

> crate::models::GetWeb3CreatorResponse get_web3_creator(chain, contract)
A collection’s creator dashboard: sales by marketplace and day, estimated royalties, holders, and for a launch the mint revenue and what is waiting to be withdrawn (wei of the chain’s native currency).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3CreatorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_crosslist_quote

> crate::models::GetWeb3CrosslistQuoteResponse get_web3_crosslist_quote()
Each marketplace's terms for listing a token: Crossly fee, OpenSea required fees and zone, Tensor availability.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3CrosslistQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_featured

> crate::models::GetWeb3FeaturedResponse get_web3_featured()
The NFT home's featured collections: chosen by Crossly, else the hottest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3FeaturedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_inventory

> crate::models::GetWeb3InventoryResponse get_web3_inventory(item_id)
The NFT behind an inventory item: its token, wallet, and open listings on every marketplace (null if it is not an NFT).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3InventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_launch_allowlist_root_tx

> crate::models::GetWeb3LaunchAllowlistRootTxResponse get_web3_launch_allowlist_root_tx(id)
The setAllowlistRoot transaction for the allowlist Crossly holds (EVM, live): `{ chain, from, to, data }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3LaunchAllowlistRootTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_launch_deploy_tx

> crate::models::GetWeb3LaunchDeployTxResponse get_web3_launch_deploy_tx(id)
The contract-creation transaction for the draft: send it from the owner wallet, then confirm it. Returns `{ chain, from, data }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3LaunchDeployTxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_launch_tool

> crate::models::GetWeb3LaunchToolResponse get_web3_launch_tool(id)
A launch’s creator tools: allowlist size and root (and whether the contract has it yet), uploaded art and reveal, supply minted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3LaunchToolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_order_fulfillment

> crate::models::GetWeb3OrderFulfillmentResponse get_web3_order_fulfillment(chain, hash)
The signed order a taker passes to Seaport, after re-checking it can still fill.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**hash** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3OrderFulfillmentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_phygital

> crate::models::GetWeb3PhygitalResponse get_web3_phygital(chain, contract, token_id)
A token's phygital: the physical item it can be redeemed for, and whether it has been.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3PhygitalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_phygital2

> crate::models::GetWeb3Phygital2Response get_web3_phygital2()
Phygitals you linked (with each redemption and its shipping address) and ones you redeemed.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3Phygital2Response**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_portfolio

> crate::models::GetWeb3PortfolioResponse get_web3_portfolio()
Your NFTs with value (Crossly floor, else recent sales) and cost (what a linked wallet paid), realised sales, and USD totals at each day’s coin price.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3PortfolioResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_portfolio_tax

> crate::models::GetWeb3PortfolioTaxResponse get_web3_portfolio_tax()
One calendar year of NFT disposals from your linked wallets as CSV text: acquired, sold, proceeds, cost basis, gain (coin and USD).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWeb3PortfolioTaxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_profile

> crate::models::GetWeb3ProfileResponse get_web3_profile(address)
A wallet's public profile: NFTs held, open Crossly listings, and activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3ProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_resolve

> crate::models::GetWeb3ResolveResponse get_web3_resolve(chain, slug)
A collection's chain and contract from its readable URL slug (e.g. eth / boredapeyachtclub).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3ResolveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_solana_collection_tensor_bid

> crate::models::GetWeb3SolanaCollectionTensorBidResponse get_web3_solana_collection_tensor_bid(collection)
A Solana collection’s Tensor bids (collection-wide and by trait), best first; price in lamports. Needs your Tensor key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3SolanaCollectionTensorBidResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_web3_token

> crate::models::GetWeb3TokenResponse get_web3_token(chain, contract, token_id)
One NFT: metadata, traits, owner, open listings and offers, activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

**crate::models::GetWeb3TokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_activity

> crate::models::ListWeb3ActivityResponse list_web3_activity()
Listings, offers, sales and cancels in the Crossly book, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3ActivityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_alerts

> crate::models::ListWeb3AlertsResponse list_web3_alerts()
Your NFT price alerts, with the cheapest open Crossly listing each one covers now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3AlertsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_approvals

> crate::models::ListWeb3ApprovalsResponse list_web3_approvals()
Your collections the OpenSea conduit is approved to move, per linked wallet.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3ApprovalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_auctions

> crate::models::ListWeb3AuctionsResponse list_web3_auctions()
Live English auctions on the Crossly book, ending soonest first, each with its top bid and the least the next bid can be.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3AuctionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_browse_collections

> crate::models::ListWeb3BrowseCollectionsResponse list_web3_browse_collections()
Browse NFT collections by chain, category and name; sort hot (Crossly sales, listings, favourites), new, volume, floor or name.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3BrowseCollectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_browse_tokens

> crate::models::ListWeb3BrowseTokensResponse list_web3_browse_tokens()
Hot NFTs: what is for sale on Crossly first, then tokens from the hottest collections.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3BrowseTokensResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_bundles

> crate::models::ListWeb3BundlesResponse list_web3_bundles()
Open bundle listings (2–30 NFTs for one price), newest first, by chain or by a contract any item is from.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3BundlesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_collection_history

> crate::models::ListWeb3CollectionHistoryResponse list_web3_collection_history(chain, contract)
Daily floor, sales, volume, holders and listed count for a collection, oldest first (up to 365 days).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::ListWeb3CollectionHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_collections

> crate::models::ListWeb3CollectionsResponse list_web3_collections()
Collections ranked by 30-day Crossly volume, with floor and listed count.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3CollectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_creator_collections

> crate::models::ListWeb3CreatorCollectionsResponse list_web3_creator_collections()
Collections you are the creator of: your launches, and contracts you claimed by owning them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3CreatorCollectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_drops

> crate::models::ListWeb3DropsResponse list_web3_drops()
Upcoming and live NFT drops (curated mints on the creator's own page), soonest first, with whether you asked to be reminded.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3DropsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_holder_perks

> crate::models::ListWeb3HolderPerksResponse list_web3_holder_perks()
Your holder perks: percent off your Crossly shop for holders of an NFT collection, best first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3HolderPerksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_launches

> crate::models::ListWeb3LaunchesResponse list_web3_launches()
Your launchpad collections (drafts and live), newest first, with minted count and phase.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3LaunchesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_listings

> crate::models::ListWeb3ListingsResponse list_web3_listings()
Open NFT listings in the Crossly book — the cheapest per token.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3ListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_marketplace_keys

> crate::models::ListWeb3MarketplaceKeysResponse list_web3_marketplace_keys()
Your own marketplace API keys (last four characters only) and whether Crossly's key covers each marketplace.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3MarketplaceKeysResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_nfts

> crate::models::ListWeb3NftsResponse list_web3_nfts(chain, wallet_id, page, limit)
List the NFTs in your inventory, imported from your linked wallets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | Option<**String**> |  |  |
**wallet_id** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListWeb3NftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_orders

> crate::models::ListWeb3OrdersResponse list_web3_orders()
Your orders in the Crossly book (listings and offers), any status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3OrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_purchases

> crate::models::ListWeb3PurchasesResponse list_web3_purchases()
NFTs your linked wallets bought — on Crossly or any marketplace the chain feed saw — newest first, with price and whether you still hold each.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3PurchasesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_rankings

> crate::models::ListWeb3RankingsResponse list_web3_rankings()
Collections by sales volume over 24h, 7d or 30d, in one coin: the ETH chains together, or Polygon / Solana on their own.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3RankingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_reserved

> crate::models::ListWeb3BundlesResponse list_web3_reserved()
Private listings reserved for one of your linked wallets — only you can see or buy them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3BundlesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_sales

> crate::models::ListWeb3SalesResponse list_web3_sales()
The latest NFT sales across the directory, from the chain (Seaport and Blur) and the Crossly book, newest first; page with `before`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3SalesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_search

> crate::models::ListWeb3SearchResponse list_web3_search()
Find collections by contract address (every chain) or by name.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3SearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_search_tokens

> crate::models::ListWeb3SearchTokensResponse list_web3_search_tokens()
Find tokens by collection name and number: 'pudgy 42', 'Bored Ape #1234'.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3SearchTokensResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_wallets

> crate::models::ListWeb3WalletsResponse list_web3_wallets()
List your verified, linked wallets (EVM and Solana).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3WalletsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_web3_watchlist

> crate::models::ListWeb3WatchlistResponse list_web3_watchlist()
Your favourited NFTs, with the cheapest open Crossly listing and the last sale of each.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3WatchlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_web3_holder_perk

> crate::models::ListWeb3HolderPerksItem update_web3_holder_perk()
Set the percent off (100–5000 bps) holders of a collection get across your Crossly shop; one perk per collection.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWeb3HolderPerksItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_web3_launch_allowlist

> crate::models::UpdateWeb3LaunchAllowlistResponse update_web3_launch_allowlist(id)
Replace the early-phase allowlist (up to 10,000 wallets). EVM live launches get back `rootTx`, the setAllowlistRoot transaction for the owner wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWeb3LaunchAllowlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_web3_marketplace_key

> crate::models::ListWeb3MarketplaceKeysItem update_web3_marketplace_key(marketplace)
Save your own API key for a marketplace (checked against the marketplace first where it can be); stored encrypted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace** | **String** |  | [required] |

### Return type

**crate::models::ListWeb3MarketplaceKeysItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

