# \BuyerApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_activity**](BuyerApi.md#create_buyer_activity) | **POST** /v1/buyer/activity | Report an item your user is looking at, and get our answer.
[**create_buyer_auction_bid**](BuyerApi.md#create_buyer_auction_bid) | **POST** /v1/buyer/auctions/{slug}/bids | Place a bid, optionally with a proxy maximum. A WINNING BID IS CHARGED AUTOMATICALLY.
[**create_buyer_auction_pay**](BuyerApi.md#create_buyer_auction_pay) | **POST** /v1/buyer/auctions/{slug}/pay | Pay for an auction you won whose automatic charge did not go through.
[**create_buyer_auction_retract**](BuyerApi.md#create_buyer_auction_retract) | **POST** /v1/buyer/auctions/{slug}/retract | Retract your bids under eBay’s rules — a typo, a changed listing, or an unreachable seller.
[**create_buyer_board**](BuyerApi.md#create_buyer_board) | **POST** /v1/buyer/boards | Create an outfit board.
[**create_buyer_board_item**](BuyerApi.md#create_buyer_board_item) | **POST** /v1/buyer/boards/{id}/items | Add a listing (by slug) to a board, at the next free spot.
[**create_buyer_break_spot_hold**](BuyerApi.md#create_buyer_break_spot_hold) | **POST** /v1/buyer/breaks/{id}/spots/{spotId}/hold | Hold a spot in a pick-your-spot break, then buy it through checkout before the hold lapses.
[**create_buyer_buyback_request**](BuyerApi.md#create_buyer_buyback_request) | **POST** /v1/buyer/buybacks/{id}/request | Ask Crossly to buy an item back under its buyback guarantee.
[**create_buyer_cart_item**](BuyerApi.md#create_buyer_cart_item) | **POST** /v1/buyer/cart/items | Add a listing to your cart.
[**create_buyer_cart_item_save_for_later**](BuyerApi.md#create_buyer_cart_item_save_for_later) | **POST** /v1/buyer/cart/items/{id}/save-for-later | Move a cart line to saved-for-later.
[**create_buyer_cart_optimizer**](BuyerApi.md#create_buyer_cart_optimizer) | **POST** /v1/buyer/cart-optimizer | Cart optimizer: send a shopping list (one item per line, \"2 Charizard Base Set\") and get the cheapest delivered basket across sellers — one pick per line, items + each seller’s shipping with free-shipping thresholds applied. fewestPackages prefers fewer sellers. Nothing is added to the cart.
[**create_buyer_cart_quote**](BuyerApi.md#create_buyer_cart_quote) | **POST** /v1/buyer/cart/quote | Price the cart, delivered — item, shipping, tax, total.
[**create_buyer_cart_saved_move_to_cart**](BuyerApi.md#create_buyer_cart_saved_move_to_cart) | **POST** /v1/buyer/cart/saved/{id}/move-to-cart | Move a saved item back into your cart, at today’s price.
[**create_buyer_case_counter**](BuyerApi.md#create_buyer_case_counter) | **POST** /v1/buyer/cases/{id}/counter | Propose your own refund figure on a case.
[**create_buyer_case_escalate**](BuyerApi.md#create_buyer_case_escalate) | **POST** /v1/buyer/cases/{id}/escalate | Ask Crossly to step in on a case.
[**create_buyer_case_message**](BuyerApi.md#create_buyer_case_message) | **POST** /v1/buyer/cases/{id}/messages | Post a message on a case thread.
[**create_buyer_case_respond**](BuyerApi.md#create_buyer_case_respond) | **POST** /v1/buyer/cases/{id}/respond | Accept or reject the seller’s partial-refund offer on a case.
[**create_buyer_case_withdraw**](BuyerApi.md#create_buyer_case_withdraw) | **POST** /v1/buyer/cases/{id}/withdraw | Withdraw a case you opened.
[**create_buyer_catalog_listing_rental**](BuyerApi.md#create_buyer_catalog_listing_rental) | **POST** /v1/buyer/catalog/listings/{slug}/rental | Book a rental. Returns a Stripe PaymentIntent client secret for rent + shipping both ways + deposit; the dates are held 30 minutes for payment. Crossly buys both labels; the deposit comes back after the item does.
[**create_buyer_catalog_listing_rental_quote**](BuyerApi.md#create_buyer_catalog_listing_rental_quote) | **POST** /v1/buyer/catalog/listings/{slug}/rental/quote | Price a rental: rent (the cheaper of by-the-day and by-the-week), both shipping legs once a ship-to is given, and the deposit. `problem` says why the dates can’t be booked.
[**create_buyer_catalog_listing_video_request**](BuyerApi.md#create_buyer_catalog_listing_video_request) | **POST** /v1/buyer/catalog/listings/{slug}/video-requests | Ask the seller for a video of an item (one open request per item). The seller is notified; their answer appears on the listing for every buyer.
[**create_buyer_conversation_invite**](BuyerApi.md#create_buyer_conversation_invite) | **POST** /v1/buyer/conversations/{id}/invite | Invite a friend into a chat with a seller: a single-use link (7 days) that makes whoever opens it a guest who can read and post. Only the thread’s buyer can invite; at most 5 guests.
[**create_buyer_conversation_join**](BuyerApi.md#create_buyer_conversation_join) | **POST** /v1/buyer/conversations/join/{token} | Join a chat you were invited into, as a guest.
[**create_buyer_feedback**](BuyerApi.md#create_buyer_feedback) | **POST** /v1/buyer/feedback | Leave feedback on up to 25 orders at once, with optional detailed seller ratings.
[**create_buyer_fit_ref**](BuyerApi.md#create_buyer_fit_ref) | **POST** /v1/buyer/fit/refs | Save a reference garment (flat measurements, inches) to compare listings against.
[**create_buyer_following_seller_seen**](BuyerApi.md#create_buyer_following_seller_seen) | **POST** /v1/buyer/following/sellers/seen | Clear the \"new listings\" count on a followed seller, or on all of them.
[**create_buyer_gift_card**](BuyerApi.md#create_buyer_gift_card) | **POST** /v1/buyer/gift-cards | Start a gift card purchase.
[**create_buyer_gift_card_redeem**](BuyerApi.md#create_buyer_gift_card_redeem) | **POST** /v1/buyer/gift-cards/redeem | Redeem a gift card code into your own balance.
[**create_buyer_group_buy_join**](BuyerApi.md#create_buyer_group_buy_join) | **POST** /v1/buyer/group-buys/{id}/join | Join a group buy: a hold at the starting price, captured at the price the group reaches. Checked against this key’s spending controls; send an Idempotency-Key.
[**create_buyer_group_buy_leave**](BuyerApi.md#create_buyer_group_buy_leave) | **POST** /v1/buyer/group-buys/{id}/leave | Leave a group buy and release your hold.
[**create_buyer_hunt**](BuyerApi.md#create_buyer_hunt) | **POST** /v1/buyer/hunts | Start a buyer-agent hunt — over a search (filters), or auto-buy on one listing (listingSlug: buy it once its price, sale applied, is within maxPriceCents). `offer` and `buy` modes also need buyer:checkout:write, and the budget is checked against this key’s spending controls.
[**create_buyer_instant_offer_accept**](BuyerApi.md#create_buyer_instant_offer_accept) | **POST** /v1/buyer/instant-offers/{id}/accept | Accept Crossly’s instant offer on a collection item. Ops then send a prepaid label; the store credit is paid once the item arrives and checks out.
[**create_buyer_layaway_cancel**](BuyerApi.md#create_buyer_layaway_cancel) | **POST** /v1/buyer/layaways/{id}/cancel | Cancel a layaway: everything paid comes back except the layaway fee (10% of the price, at most $15).
[**create_buyer_layaway_pay**](BuyerApi.md#create_buyer_layaway_pay) | **POST** /v1/buyer/layaways/{id}/pay | Pay some (at least $1) or all of what is left now, on the saved card. Paid in full, it ships.
[**create_buyer_listing_audio_summary**](BuyerApi.md#create_buyer_listing_audio_summary) | **POST** /v1/buyer/listings/{slug}/audio-summary | Record the spoken summary of a listing (cached per listing version). 503 naming TTS_API_KEY when no speech provider is configured.
[**create_buyer_listing_split**](BuyerApi.md#create_buyer_listing_split) | **POST** /v1/buyer/listings/{slug}/split | Split a purchase: one pay link per person; nobody is charged until all have paid.
[**create_buyer_listing_subscribe**](BuyerApi.md#create_buyer_listing_subscribe) | **POST** /v1/buyer/listings/{slug}/subscribe | Subscribe & save: a standing order charged to your saved card every interval. Checked against this key’s spending controls; send an Idempotency-Key.
[**create_buyer_listing_try_on**](BuyerApi.md#create_buyer_listing_try_on) | **POST** /v1/buyer/listings/{slug}/try-on | Render an AI try-on of this listing on the buyer’s own photo (base64 `photo`, or `reuseLastPhoto`). `confirmSelfAdult` must be true: the photo is of the buyer and they are 18+. Wearables only; capped per day; 503 naming TRYON_API_KEY when no image model is configured.
[**create_buyer_market_bid**](BuyerApi.md#create_buyer_market_bid) | **POST** /v1/buyer/market/bids | Place a fixed-price market bid. SPENDS MONEY: it fills against the lowest ask at or under your price and charges your saved card (plus the fast-ship premium when fastShip is set and it fills from vault stock). deliverTo vault keeps it in your Crossly vault; gradeWith sends a raw card to a Grading run first. Checked against this key’s spending controls; send an Idempotency-Key.
[**create_buyer_market_watch**](BuyerApi.md#create_buyer_market_watch) | **POST** /v1/buyer/market/watches | Watch a market variant: alert at or below a price, or on any drop.
[**create_buyer_offer**](BuyerApi.md#create_buyer_offer) | **POST** /v1/buyer/offers | Offer a price on a listing.
[**create_buyer_offer_respond**](BuyerApi.md#create_buyer_offer_respond) | **POST** /v1/buyer/offers/{id}/respond | Answer a seller’s counteroffer — accept, decline, or counter back.
[**create_buyer_order_cancel**](BuyerApi.md#create_buyer_order_cancel) | **POST** /v1/buyer/orders/{id}/cancel | Cancel an order before it ships — refunds it in full.
[**create_buyer_order_inr**](BuyerApi.md#create_buyer_order_inr) | **POST** /v1/buyer/orders/{id}/inr | Open an item-not-received case.
[**create_buyer_order_received**](BuyerApi.md#create_buyer_order_received) | **POST** /v1/buyer/orders/{id}/received | Confirm you received an order.
[**create_buyer_order_tip**](BuyerApi.md#create_buyer_order_tip) | **POST** /v1/buyer/orders/{id}/tip | Tip the seller on one of your shipped orders, charged to your saved card. One tip per order; $1–$500; checked against this key’s spending controls.
[**create_buyer_pack_fairness_rotate**](BuyerApi.md#create_buyer_pack_fairness_rotate) | **POST** /v1/buyer/packs/fairness/rotate | Reveal your current server seed and start a new one (optionally with your own client seed). Refused while an opening is awaiting payment.
[**create_buyer_pack_open**](BuyerApi.md#create_buyer_pack_open) | **POST** /v1/buyer/packs/{slug}/open | Open a pack. SPENDS MONEY: charges the pack price to your saved card, then pulls one real item by the published odds (provably fair: see /buyer/packs/fairness) into your Crossly vault. confirmAdult must be true. Checked against this key’s spending controls; send an Idempotency-Key.
[**create_buyer_pack_opening_sell_back**](BuyerApi.md#create_buyer_pack_opening_sell_back) | **POST** /v1/buyer/pack-openings/{id}/sell-back | Sell a pull straight back for Crossly credit at the pack’s buyback rate, within its window, while it is still in your vault.
[**create_buyer_passport_claim**](BuyerApi.md#create_buyer_passport_claim) | **POST** /v1/buyer/passports/{code}/claim | Claim a passport as the item’s current owner (the buyer of its latest sale) — shown as “in its owner’s hands” until it sells again.
[**create_buyer_protection_plan_cancel**](BuyerApi.md#create_buyer_protection_plan_cancel) | **POST** /v1/buyer/protection-plans/{id}/cancel | Cancel a protection plan. The insurer decides what is refundable (pro-rata after the free-look window); that amount goes back to the card.
[**create_buyer_purchase_download**](BuyerApi.md#create_buyer_purchase_download) | **POST** /v1/buyer/purchases/{id}/download | A ten-minute link to a digital purchase’s file; each call uses one of its downloads.
[**create_buyer_purchase_fit**](BuyerApi.md#create_buyer_purchase_fit) | **POST** /v1/buyer/purchases/{id}/fit | Say how a delivered purchase fit: true_to_size, small or large. It teaches your size memory and, pooled with other buyers, how the brand runs.
[**create_buyer_purchase_return**](BuyerApi.md#create_buyer_purchase_return) | **POST** /v1/buyer/purchases/{id}/return | Return a purchase under the seller’s policy, or send back a try-at-home item. With reason doesnt_fit, `fit` (small | large) says which way, and it feeds your size memory.
[**create_buyer_registry**](BuyerApi.md#create_buyer_registry) | **POST** /v1/buyer/registries | Start a gift registry (wedding, baby, birthday, housewarming, graduation, other). Items are added like any wishlist item; set shipAddressId to one of your saved addresses so guests can buy — they never see it.
[**create_buyer_registry_gift_thanked**](BuyerApi.md#create_buyer_registry_gift_thanked) | **POST** /v1/buyer/registries/{id}/gifts/{giftId}/thanked | Tick a gift as thanked (or untick it).
[**create_buyer_rental_arrival_photo**](BuyerApi.md#create_buyer_rental_arrival_photo) | **POST** /v1/buyer/rentals/{id}/arrival-photos | Photograph a rental on arrival (URLs you uploaded): your side of the condition record.
[**create_buyer_rental_cancel**](BuyerApi.md#create_buyer_rental_cancel) | **POST** /v1/buyer/rentals/{id}/cancel | Cancel a rental that has not shipped: everything is refunded.
[**create_buyer_rental_claim_response**](BuyerApi.md#create_buyer_rental_claim_response) | **POST** /v1/buyer/rentals/{id}/claim-response | Answer a deposit claim: accept (the claim is paid, the rest refunded) or dispute (Crossly decides).
[**create_buyer_rental_identity_start**](BuyerApi.md#create_buyer_rental_identity_start) | **POST** /v1/buyer/rentals/identity/start | Start the ID check renting needs (Stripe Identity: photo ID + selfie). Open `url` (hosted) or use `clientSecret` with Stripe.js.
[**create_buyer_return_cancel**](BuyerApi.md#create_buyer_return_cancel) | **POST** /v1/buyer/returns/{id}/cancel | Cancel a return before it ships.
[**create_buyer_return_tracking**](BuyerApi.md#create_buyer_return_tracking) | **POST** /v1/buyer/returns/{id}/tracking | Add tracking to a return you shipped on your own label.
[**create_buyer_seller_club_join**](BuyerApi.md#create_buyer_seller_club_join) | **POST** /v1/buyer/sellers/{sellerUserId}/club/join | Join a seller’s loyalty club (free). Your tier’s discount then applies to their items — the best of it and any other rate you have with them, never stacked.
[**create_buyer_seller_report**](BuyerApi.md#create_buyer_seller_report) | **POST** /v1/buyer/sellers/{username}/report | Report a seller to Crossly’s trust team.
[**create_buyer_split_cancel**](BuyerApi.md#create_buyer_split_cancel) | **POST** /v1/buyer/splits/{id}/cancel | Cancel a split you organised; every held share is released.
[**create_buyer_together**](BuyerApi.md#create_buyer_together) | **POST** /v1/buyer/together | Start a shop-together room.
[**create_buyer_together_item**](BuyerApi.md#create_buyer_together_item) | **POST** /v1/buyer/together/{code}/items | Add a listing to a shop-together board.
[**create_buyer_together_item_vote**](BuyerApi.md#create_buyer_together_item_vote) | **POST** /v1/buyer/together/{code}/items/{id}/vote | Vote an item on a room’s board up (1), down (-1) or clear (0).
[**create_buyer_together_join**](BuyerApi.md#create_buyer_together_join) | **POST** /v1/buyer/together/{code}/join | Join a shop-together room by its code.
[**create_buyer_together_leave**](BuyerApi.md#create_buyer_together_leave) | **POST** /v1/buyer/together/{code}/leave | Leave a shop-together room.
[**create_buyer_together_message**](BuyerApi.md#create_buyer_together_message) | **POST** /v1/buyer/together/{code}/messages | Say something in a shop-together room.
[**create_buyer_total_request**](BuyerApi.md#create_buyer_total_request) | **POST** /v1/buyer/total-requests | Ask a seller for one combined shipping total on everything of theirs in your cart.
[**create_buyer_vault_list**](BuyerApi.md#create_buyer_vault_list) | **POST** /v1/buyer/vault/{id}/list | Sell a vault item on the market at your price. It is already authenticated, so when it sells nothing ships in: the buyer keeps it vaulted or it ships straight to them.
[**create_buyer_vault_unlist**](BuyerApi.md#create_buyer_vault_unlist) | **POST** /v1/buyer/vault/{id}/unlist | Take a vault item off the market; it stays stored.
[**create_buyer_vault_withdraw**](BuyerApi.md#create_buyer_vault_withdraw) | **POST** /v1/buyer/vault/{id}/withdraw | Ship a vault item home, standard or expedited. The shipping, any storage it owes and the early-withdrawal fee (within 90 days of arriving) are charged to your saved card now, all checked against this key’s spending controls; send an Idempotency-Key.
[**create_buyer_wishlist**](BuyerApi.md#create_buyer_wishlist) | **POST** /v1/buyer/wishlists | Create a wishlist.
[**create_buyer_wishlist_invite**](BuyerApi.md#create_buyer_wishlist_invite) | **POST** /v1/buyer/wishlists/{id}/invite | Share a wishlist with a co-editor: a single-use link (14 days). Members add and remove items; only the owner renames, deletes or removes members. At most 20.
[**create_buyer_wishlist_item**](BuyerApi.md#create_buyer_wishlist_item) | **POST** /v1/buyer/wishlists/{id}/items | Add a listing to a wishlist.
[**create_buyer_wishlist_join**](BuyerApi.md#create_buyer_wishlist_join) | **POST** /v1/buyer/wishlists/join/{token} | Join a wishlist you were invited to edit.
[**create_buyer_wishlist_share**](BuyerApi.md#create_buyer_wishlist_share) | **POST** /v1/buyer/wishlists/{id}/share | Share a wishlist — makes it public and returns its link.
[**delete_buyer_board**](BuyerApi.md#delete_buyer_board) | **DELETE** /v1/buyer/boards/{id} | Delete a board.
[**delete_buyer_board_item**](BuyerApi.md#delete_buyer_board_item) | **DELETE** /v1/buyer/boards/{id}/items/{platformListingId} | Take a listing off a board.
[**delete_buyer_break_spot_hold**](BuyerApi.md#delete_buyer_break_spot_hold) | **DELETE** /v1/buyer/breaks/{id}/spots/{spotId}/hold | Release a break spot you hold.
[**delete_buyer_cart_item**](BuyerApi.md#delete_buyer_cart_item) | **DELETE** /v1/buyer/cart/items/{id} | Remove a line from your cart.
[**delete_buyer_cart_saved**](BuyerApi.md#delete_buyer_cart_saved) | **DELETE** /v1/buyer/cart/saved/{id} | Remove an item from saved-for-later. Returns the remaining list.
[**delete_buyer_conversation_guest**](BuyerApi.md#delete_buyer_conversation_guest) | **DELETE** /v1/buyer/conversations/{id}/guests/{userId} | Remove a guest (the thread’s buyer), or leave (a guest, with your own user id).
[**delete_buyer_fit_ref**](BuyerApi.md#delete_buyer_fit_ref) | **DELETE** /v1/buyer/fit/refs/{id} | Delete a reference garment.
[**delete_buyer_market_bid**](BuyerApi.md#delete_buyer_market_bid) | **DELETE** /v1/buyer/market/bids/{id} | Cancel a resting market bid.
[**delete_buyer_market_watch**](BuyerApi.md#delete_buyer_market_watch) | **DELETE** /v1/buyer/market/watches/{id} | Stop watching a market variant.
[**delete_buyer_offer**](BuyerApi.md#delete_buyer_offer) | **DELETE** /v1/buyer/offers/{id} | Withdraw an offer you made that the seller has not answered.
[**delete_buyer_recently_viewed**](BuyerApi.md#delete_buyer_recently_viewed) | **DELETE** /v1/buyer/recently-viewed/{slug} | Remove one listing from your recently viewed.
[**delete_buyer_recently_viewed2**](BuyerApi.md#delete_buyer_recently_viewed2) | **DELETE** /v1/buyer/recently-viewed | Clear your recently viewed list.
[**delete_buyer_registry**](BuyerApi.md#delete_buyer_registry) | **DELETE** /v1/buyer/registries/{id} | Delete a registry (and the wishlist behind it).
[**delete_buyer_seller_club_join**](BuyerApi.md#delete_buyer_seller_club_join) | **DELETE** /v1/buyer/sellers/{sellerUserId}/club/join | Leave a seller’s loyalty club.
[**delete_buyer_together_item**](BuyerApi.md#delete_buyer_together_item) | **DELETE** /v1/buyer/together/{code}/items/{id} | Take an item you added off a room’s board.
[**delete_buyer_total_request**](BuyerApi.md#delete_buyer_total_request) | **DELETE** /v1/buyer/total-requests/{id} | Cancel a total request you made.
[**delete_buyer_try_on**](BuyerApi.md#delete_buyer_try_on) | **DELETE** /v1/buyer/try-ons/{id} | Delete a try-on and its images — or pass `all` to delete every photo and result.
[**delete_buyer_wishlist**](BuyerApi.md#delete_buyer_wishlist) | **DELETE** /v1/buyer/wishlists/{id} | Delete a wishlist and everything on it.
[**delete_buyer_wishlist_item**](BuyerApi.md#delete_buyer_wishlist_item) | **DELETE** /v1/buyer/wishlists/{id}/items/{slug} | Take a listing off a wishlist.
[**delete_buyer_wishlist_member**](BuyerApi.md#delete_buyer_wishlist_member) | **DELETE** /v1/buyer/wishlists/{id}/members/{userId} | Remove a co-editor (the owner), or leave a list shared with you (your own user id).
[**get_buyer_auction_me**](BuyerApi.md#get_buyer_auction_me) | **GET** /v1/buyer/auctions/{slug}/me | Where you stand in one auction — your max, leading, won, must you pay.
[**get_buyer_badge**](BuyerApi.md#get_buyer_badge) | **GET** /v1/buyer/badges | The counts behind the account badge — unread messages, offers waiting, orders to review.
[**get_buyer_board**](BuyerApi.md#get_buyer_board) | **GET** /v1/buyer/boards/{id} | One of your boards with its layout: each item’s x, y and width as fractions of the board, and stacking order.
[**get_buyer_board_shared**](BuyerApi.md#get_buyer_board_shared) | **GET** /v1/buyer/boards/shared/{token} | A shared outfit board, by its link token.
[**get_buyer_break**](BuyerApi.md#get_buyer_break) | **GET** /v1/buyer/breaks/{id} | A card break’s board: spots, holders, and the seed after the draw.
[**get_buyer_case**](BuyerApi.md#get_buyer_case) | **GET** /v1/buyer/cases/{id} | One case — its state, the message thread, and the return parcel if there is one.
[**get_buyer_catalog_listing_my_size**](BuyerApi.md#get_buyer_catalog_listing_my_size) | **GET** /v1/buyer/catalog/listings/{slug}/my-size | This item against your size: the size you keep in this kind of item (and in this brand), whether this one is your size, smaller or larger, a warning if this brand and size ran small or large on you before, and how other buyers say the brand runs. null for an unsized item.
[**get_buyer_catalog_listing_price**](BuyerApi.md#get_buyer_catalog_listing_price) | **GET** /v1/buyer/catalog/listings/{slug}/price | The price today. When the seller has the same item live on another marketplace for less, this IS that price (`priceMatch` says where from), and `lowestAnywhere` lists the marketplaces it is guaranteed not to be cheaper on. `priceCheck` rates it against real sales of the same catalog item in the same condition over the last 180 days (great / good / fair / high, with the median and middle-half range in cents); null until there are 3+ sales from 2+ sellers.
[**get_buyer_catalog_listing_rental**](BuyerApi.md#get_buyer_catalog_listing_rental) | **GET** /v1/buyer/catalog/listings/{slug}/rental | Whether a listing is for rent: day and week rates, the refundable deposit, length limits, the turnaround buffer, and the dates already booked. null when it is not for rent.
[**get_buyer_checkout_usdc**](BuyerApi.md#get_buyer_checkout_usdc) | **GET** /v1/buyer/checkout/usdc | Whether Crossly takes USDC (Stripe stablecoin payments: Solana, Ethereum, Base, Polygon), and its per-order limit. Paying needs the buyer’s wallet, so it happens in the browser checkout, never unattended.
[**get_buyer_collection_offer**](BuyerApi.md#get_buyer_collection_offer) | **GET** /v1/buyer/collection/{id}/offer | What an item in your collection would fetch now: the top open bid on the market for the same variant, condition and grade, and Crossly’s instant offer in store credit when that is switched on (a quote valid for a limited time).
[**get_buyer_collection_set**](BuyerApi.md#get_buyer_collection_set) | **GET** /v1/buyer/collection/sets/{category}/{brand}/{model} | One set: every piece (owned or missing, with its lowest ask on the market), what completing it would cost now, and the leaderboard of collectors who opted in, with your rank.
[**get_buyer_creator**](BuyerApi.md#get_buyer_creator) | **GET** /v1/buyer/creators/{handle} | A creator storefront: profile, affiliate code, and shelves of live picks.
[**get_buyer_fit_vocabulary**](BuyerApi.md#get_buyer_fit_vocabulary) | **GET** /v1/buyer/fit/vocabulary | Garment kinds and the measurements each takes.
[**get_buyer_gift_finder_option**](BuyerApi.md#get_buyer_gift_finder_option) | **GET** /v1/buyer/gift-finder/options | The gift finder’s questions: recipients, interests, budgets.
[**get_buyer_impact**](BuyerApi.md#get_buyer_impact) | **GET** /v1/buyer/impact | The sourced footprint your secondhand purchases avoided.
[**get_buyer_layaway**](BuyerApi.md#get_buyer_layaway) | **GET** /v1/buyer/layaways/{id} | One layaway and its schedule.
[**get_buyer_listing_audio_summary**](BuyerApi.md#get_buyer_listing_audio_summary) | **GET** /v1/buyer/listings/{slug}/audio-summary | A spoken summary of a listing (accessibility): the text, built only from the listing’s own fields and the seller’s description, and the recording of it when one exists.
[**get_buyer_listing_bought_together**](BuyerApi.md#get_buyer_listing_bought_together) | **GET** /v1/buyer/listings/{slug}/bought-together | Frequently bought together / complete the look.
[**get_buyer_listing_break**](BuyerApi.md#get_buyer_listing_break) | **GET** /v1/buyer/listings/{slug}/break | The card break this listing sells spots in, if any; the seed is revealed after the draw.
[**get_buyer_listing_cert**](BuyerApi.md#get_buyer_listing_cert) | **GET** /v1/buyer/listings/{slug}/cert | A graded listing’s cert, checked with the grader: the seller’s claim (grader, grade, cert number), status (verified, mismatch, not_found, or unchecked where the grader has no lookup), population and how many graded higher where the grader publishes them, and the grader’s own cert page.
[**get_buyer_listing_fit**](BuyerApi.md#get_buyer_listing_fit) | **GET** /v1/buyer/listings/{slug}/fit | A listing’s flat measurements, compared with your saved reference garments.
[**get_buyer_listing_group_buy**](BuyerApi.md#get_buyer_listing_group_buy) | **GET** /v1/buyer/listings/{slug}/group-buy | The group buy open on this listing, if any.
[**get_buyer_listing_impact**](BuyerApi.md#get_buyer_listing_impact) | **GET** /v1/buyer/listings/{slug}/impact | The sourced footprint of making a new equivalent; null when no published figure applies.
[**get_buyer_listing_resale_value**](BuyerApi.md#get_buyer_listing_resale_value) | **GET** /v1/buyer/listings/{slug}/resale-value | What the same brand and category resold for on Crossly, and the buyback offer when enabled.
[**get_buyer_listing_subscribe_offer**](BuyerApi.md#get_buyer_listing_subscribe_offer) | **GET** /v1/buyer/listings/{slug}/subscribe-offer | The subscribe & save discount and intervals on a listing; null when the seller has none.
[**get_buyer_listing_try_on**](BuyerApi.md#get_buyer_listing_try_on) | **GET** /v1/buyer/listings/{slug}/try-on | Whether an AI try-on is offered for this listing (a wearable with a photo) and whether the image model is configured.
[**get_buyer_live_stream_auction**](BuyerApi.md#get_buyer_live_stream_auction) | **GET** /v1/buyer/live-streams/{id}/auction | The auction running on a live stream right now, if any. Bid on it by its `slug` at POST /v1/buyer/auctions/:slug/bids.
[**get_buyer_market_purchase_option**](BuyerApi.md#get_buyer_market_purchase_option) | **GET** /v1/buyer/market/purchase-options | Before you bid or buy: whether the lowest ask is vault stock (keep it vaulted, or pay fastShipCents to rush it), and the grading runs a raw card can go straight to (pass gradeWith on the bid).
[**get_buyer_order_eligibility**](BuyerApi.md#get_buyer_order_eligibility) | **GET** /v1/buyer/orders/{id}/eligibility | What you can do with one order right now — cancel, confirm receipt, report not received, leave feedback.
[**get_buyer_order_export**](BuyerApi.md#get_buyer_order_export) | **GET** /v1/buyer/orders/export | Your order history as CSV text (up to 5,000 orders).
[**get_buyer_order_price_protection**](BuyerApi.md#get_buyer_order_price_protection) | **GET** /v1/buyer/orders/{id}/price-protection | Price protection on one order: if the item gets cheaper before `until`, the difference is credited to your Bucks automatically. `creditedCents` is what it has paid so far.
[**get_buyer_pack**](BuyerApi.md#get_buyer_pack) | **GET** /v1/buyer/packs/{slug} | One pack: tiers and odds, items left, expected value, the chase items still in it.
[**get_buyer_pack_fairness**](BuyerApi.md#get_buyer_pack_fairness) | **GET** /v1/buyer/packs/fairness | Your provably-fair seeds: the active server-seed commitment (sha256), your client seed and nonce, and every retired seed in full so past openings can be re-run.
[**get_buyer_party**](BuyerApi.md#get_buyer_party) | **GET** /v1/buyer/parties/{slug} | One shopping party.
[**get_buyer_passport**](BuyerApi.md#get_buyer_passport) | **GET** /v1/buyer/passports/{code} | What a product passport tag opens: the item, whether it was authenticated, how many owners on Crossly and since when — never who or for how much.
[**get_buyer_preference**](BuyerApi.md#get_buyer_preference) | **GET** /v1/buyer/preferences | The shopping profile derived from that activity.
[**get_buyer_price_guide**](BuyerApi.md#get_buyer_price_guide) | **GET** /v1/buyer/price-guide | What matching items actually sold for on Crossly: median, quartiles, by month, and recent sales. Needs q, brand or category.
[**get_buyer_profile**](BuyerApi.md#get_buyer_profile) | **GET** /v1/buyer/profile | Your Crossly shopping profile — name, email, saved address, Bucks balance.
[**get_buyer_purchase_delivery**](BuyerApi.md#get_buyer_purchase_delivery) | **GET** /v1/buyer/purchases/{id}/delivery | Where a purchase is going when not to your door: the pickup point (with its release code) or its ship-together consolidation.
[**get_buyer_purchase_live_moment**](BuyerApi.md#get_buyer_purchase_live_moment) | **GET** /v1/buyer/purchases/{id}/live-moment | The live show a purchase was bought in, with the offset (seconds) into its replay where it sold. null when it was not bought in a show.
[**get_buyer_purchase_return**](BuyerApi.md#get_buyer_purchase_return) | **GET** /v1/buyer/purchases/{id}/return | What a return would refund for each reason, by when, and any return already open.
[**get_buyer_purchase_story**](BuyerApi.md#get_buyer_purchase_story) | **GET** /v1/buyer/purchases/{id}/story | Your owner story on a purchase, passed on to the next owner if you resell it.
[**get_buyer_registry**](BuyerApi.md#get_buyer_registry) | **GET** /v1/buyer/registries/{id} | One of your registries: items with how many are wanted and bought.
[**get_buyer_registry_by_token**](BuyerApi.md#get_buyer_registry_by_token) | **GET** /v1/buyer/registry/{token} | A registry as a guest sees it: items, how many are still wanted, and whether it can ship. Never the address.
[**get_buyer_rental_eligibility**](BuyerApi.md#get_buyer_rental_eligibility) | **GET** /v1/buyer/rentals/eligibility | Can you rent right now: ID-verified (a photo ID and a matching selfie, once), the verified name rentals ship to, and your standing (one rental at a time until your first comes back; none after one that never did).
[**get_buyer_seller_club**](BuyerApi.md#get_buyer_seller_club) | **GET** /v1/buyer/sellers/{sellerUserId}/club | A seller’s loyalty club: its tiers (spend threshold, discount, perks) and your standing. null when the seller has none.
[**get_buyer_seller_trust**](BuyerApi.md#get_buyer_seller_trust) | **GET** /v1/buyer/sellers/{username}/trust | Seller at a glance: member since, completed sales, independent rating, typical handling time, dispute rate, trust tier, badges — and the plain sentences the profile shows.
[**get_buyer_together**](BuyerApi.md#get_buyer_together) | **GET** /v1/buyer/together/{code} | A shop-together room: members, the board with votes, and chat.
[**get_buyer_vault_market_history**](BuyerApi.md#get_buyer_vault_market_history) | **GET** /v1/buyer/vault-markets/history | Your Vault Markets value by day at the marks (oldest first, today live) over `days` (default 90), and today’s change against yesterday’s close.
[**get_buyer_vault_market_index**](BuyerApi.md#get_buyer_vault_market_index) | **GET** /v1/buyer/vault-markets/indexes/{slug} | One index with its published daily closes (oldest first, `days` default 365) and its level now.
[**get_buyer_vault_market_instant_exit**](BuyerApi.md#get_buyer_vault_market_instant_exit) | **GET** /v1/buyer/vault-markets/instant-exit/{id} | The Instant Exit quote for one vault unit you hold, and whether it can be sold now.
[**get_buyer_vault_market_worth**](BuyerApi.md#get_buyer_vault_market_worth) | **GET** /v1/buyer/vault-markets/worth | What your vault is worth now: value at the market prices (marks come only from trades) split into reliable and indicative marks, cost and P&L, today’s change, and an estimate of what the units free to sell would fetch from the bids resting on the book now. Read only.
[**list_buyer_activity**](BuyerApi.md#list_buyer_activity) | **GET** /v1/buyer/activity | What this buyer has compared lately.
[**list_buyer_auction_bids**](BuyerApi.md#list_buyer_auction_bids) | **GET** /v1/buyer/auctions/{slug}/bids | An auction’s bid history, highest first, bidders anonymised.
[**list_buyer_auctions**](BuyerApi.md#list_buyer_auctions) | **GET** /v1/buyer/auctions | Auctions you have bid in — bidding, won or lost (last 100).
[**list_buyer_boards**](BuyerApi.md#list_buyer_boards) | **GET** /v1/buyer/boards | Your outfit boards: collages of listings, each with a share link.
[**list_buyer_breaks**](BuyerApi.md#list_buyer_breaks) | **GET** /v1/buyer/breaks | Card-break spots you hold or bought, with what each pulled.
[**list_buyer_buybacks**](BuyerApi.md#list_buyer_buybacks) | **GET** /v1/buyer/buybacks | Your buyback guarantees and requests.
[**list_buyer_cart**](BuyerApi.md#list_buyer_cart) | **GET** /v1/buyer/cart | What is in your Crossly cart.
[**list_buyer_cart_saved**](BuyerApi.md#list_buyer_cart_saved) | **GET** /v1/buyer/cart/saved | Items you saved for later out of your cart (last 200).
[**list_buyer_cases**](BuyerApi.md#list_buyer_cases) | **GET** /v1/buyer/cases | Resolution center — every case you opened (returns, not received, problems).
[**list_buyer_cashback**](BuyerApi.md#list_buyer_cashback) | **GET** /v1/buyer/cashback | Your Scout cashback — pending, confirmed, paid.
[**list_buyer_catalog_listing_videos**](BuyerApi.md#list_buyer_catalog_listing_videos) | **GET** /v1/buyer/catalog/listings/{slug}/videos | Videos the seller recorded for this item on request, newest first, with the question each one answers.
[**list_buyer_clubs**](BuyerApi.md#list_buyer_clubs) | **GET** /v1/buyer/clubs | Sellers’ loyalty clubs you belong to: your spend with each, your tier and its discount, and the next tier.
[**list_buyer_collection_sets**](BuyerApi.md#list_buyer_collection_sets) | **GET** /v1/buyer/collection/sets | Sets you are collecting — a catalogue brand + model you own at least two pieces of — with owned / total and percent complete, most complete first; and whether you show on set leaderboards.
[**list_buyer_consolidations**](BuyerApi.md#list_buyer_consolidations) | **GET** /v1/buyer/consolidations | Your ship-together orders: parcels at the hub, and the one box out.
[**list_buyer_conversation_guests**](BuyerApi.md#list_buyer_conversation_guests) | **GET** /v1/buyer/conversations/{id}/guests | Guests in a chat.
[**list_buyer_feedback**](BuyerApi.md#list_buyer_feedback) | **GET** /v1/buyer/feedback | Feedback you have left sellers, newest first — with the ids you revise by.
[**list_buyer_feedback_pending**](BuyerApi.md#list_buyer_feedback_pending) | **GET** /v1/buyer/feedback/pending | Orders from the last 60 days that are on their way or arrived and still need your feedback.
[**list_buyer_fit_refs**](BuyerApi.md#list_buyer_fit_refs) | **GET** /v1/buyer/fit/refs | Your saved reference garments for fit comparison.
[**list_buyer_following_sellers**](BuyerApi.md#list_buyer_following_sellers) | **GET** /v1/buyer/following/sellers | Sellers you follow, with how many new listings each has since you last looked.
[**list_buyer_gift_card_credit**](BuyerApi.md#list_buyer_gift_card_credit) | **GET** /v1/buyer/gift-cards/credit | Gift card credit on your account, and when it expires.
[**list_buyer_gift_cards**](BuyerApi.md#list_buyer_gift_cards) | **GET** /v1/buyer/gift-cards | Gift cards you bought, newest first.
[**list_buyer_gift_finder**](BuyerApi.md#list_buyer_gift_finder) | **GET** /v1/buyer/gift-finder | Live listings for a recipient, interest and budget.
[**list_buyer_gift_sent**](BuyerApi.md#list_buyer_gift_sent) | **GET** /v1/buyer/gifts/sent | Items you sent as gifts: whether the teaser went out, whether it arrived and was opened, and any thank-you. Send one with `gift` on checkout.
[**list_buyer_hotspots**](BuyerApi.md#list_buyer_hotspots) | **GET** /v1/buyer/hotspots | Shop-the-look tags on a story or lookbook photo: each tagged listing and where it sits on the image (x, y as fractions).
[**list_buyer_hunts**](BuyerApi.md#list_buyer_hunts) | **GET** /v1/buyer/hunts | Your buyer-agent hunts and what they did.
[**list_buyer_instant_offers**](BuyerApi.md#list_buyer_instant_offers) | **GET** /v1/buyer/instant-offers | Your instant offers: live quotes, accepted ones awaiting a label, approved ones (with the prepaid label) and paid ones.
[**list_buyer_layaways**](BuyerApi.md#list_buyer_layaways) | **GET** /v1/buyer/layaways | Your layaways: price, paid so far, what is left, and the installment schedule. Put an item on layaway at checkout with layawayItemIds.
[**list_buyer_listing_quick_questions**](BuyerApi.md#list_buyer_listing_quick_questions) | **GET** /v1/buyer/listings/{slug}/quick-questions | One-tap questions this listing supports; send one as `quick` on a message.
[**list_buyer_listing_seller_reviews**](BuyerApi.md#list_buyer_listing_seller_reviews) | **GET** /v1/buyer/listings/{slug}/seller-reviews | The seller’s public reviews: summary, text, photos, replies.
[**list_buyer_live_shipments**](BuyerApi.md#list_buyer_live_shipments) | **GET** /v1/buyer/live-shipments | Your one-box-per-show shipments: every order won in a show ships together, and postage is charged once (the rest refunded when it ships).
[**list_buyer_live_stream_lineup**](BuyerApi.md#list_buyer_live_stream_lineup) | **GET** /v1/buyer/live-streams/{id}/lineup | What a live show will sell, in order: each item, its price, and whether it is still available.
[**list_buyer_market_bids**](BuyerApi.md#list_buyer_market_bids) | **GET** /v1/buyer/market/bids | Your resting market bids.
[**list_buyer_market_watches**](BuyerApi.md#list_buyer_market_watches) | **GET** /v1/buyer/market/watches | Market variants you watch, with the price you are waiting for.
[**list_buyer_offers**](BuyerApi.md#list_buyer_offers) | **GET** /v1/buyer/offers | Offers you have made, newest first (last 100).
[**list_buyer_orders**](BuyerApi.md#list_buyer_orders) | **GET** /v1/buyer/orders | What you have bought on Crossly, newest first.
[**list_buyer_pack_openings**](BuyerApi.md#list_buyer_pack_openings) | **GET** /v1/buyer/pack-openings | Your pulls: what you got, its value, sell-back status, and the fairness proof for each (the server seed once rotated).
[**list_buyer_packs**](BuyerApi.md#list_buyer_packs) | **GET** /v1/buyer/packs | Packs you can open: price, every tier with its published odds, what is left in it and its value range, and the expected value of one opening.
[**list_buyer_parties**](BuyerApi.md#list_buyer_parties) | **GET** /v1/buyer/parties | Themed shopping parties, live and upcoming.
[**list_buyer_party_listings**](BuyerApi.md#list_buyer_party_listings) | **GET** /v1/buyer/parties/{slug}/listings | Listings shared to a party, featured first.
[**list_buyer_pickup_points**](BuyerApi.md#list_buyer_pickup_points) | **GET** /v1/buyer/pickup-points | Carrier pickup points (UPS Access Point / FedEx Hold at Location) near an address; pass one as `pickupPoint` to POST /v1/buyer/checkout.
[**list_buyer_protection_plans**](BuyerApi.md#list_buyer_protection_plans) | **GET** /v1/buyer/protection-plans | Your protection plans (bought at checkout with `protection`): status, term, coverage, contract id, and any refund.
[**list_buyer_registries**](BuyerApi.md#list_buyer_registries) | **GET** /v1/buyer/registries | Your gift registries.
[**list_buyer_registry_gifts**](BuyerApi.md#list_buyer_registry_gifts) | **GET** /v1/buyer/registries/{id}/gifts | Your thank-you list: who bought what off the registry, and whether you have thanked them.
[**list_buyer_registry_search**](BuyerApi.md#list_buyer_registry_search) | **GET** /v1/buyer/registries/search | Find a registry by its name or the registrant’s name. Current and recent registries only.
[**list_buyer_rentals**](BuyerApi.md#list_buyer_rentals) | **GET** /v1/buyer/rentals | Your rentals: dates, status (reserved, out, returned, claimed, closed, cancelled), the return label, and what happened to the deposit.
[**list_buyer_size_memory_prompts**](BuyerApi.md#list_buyer_size_memory_prompts) | **GET** /v1/buyer/size-memory/prompts | Delivered orders with a size that you haven’t said how they fit (the last 60 days).
[**list_buyer_size_memory_sizes**](BuyerApi.md#list_buyer_size_memory_sizes) | **GET** /v1/buyer/size-memory/sizes | The sizes learned from what you bought and kept, by kind of item (shoes, tops, bottoms, dresses, outerwear) and department.
[**list_buyer_splits**](BuyerApi.md#list_buyer_splits) | **GET** /v1/buyer/splits | Purchases you split, with each share’s status.
[**list_buyer_subscriptions**](BuyerApi.md#list_buyer_subscriptions) | **GET** /v1/buyer/subscriptions | Your subscribe & save standing orders.
[**list_buyer_together**](BuyerApi.md#list_buyer_together) | **GET** /v1/buyer/together | Shop-together rooms you are in.
[**list_buyer_total_requests**](BuyerApi.md#list_buyer_total_requests) | **GET** /v1/buyer/total-requests | Your combined-shipping total requests, newest per seller.
[**list_buyer_try_ons**](BuyerApi.md#list_buyer_try_ons) | **GET** /v1/buyer/try-ons | Your AI try-ons: the rendered images (labelled AI-generated wherever shown).
[**list_buyer_vault**](BuyerApi.md#list_buyer_vault) | **GET** /v1/buyer/vault | What you own in the Crossly vault: stored (authenticated, on a shelf), listed for sale, or on its way home — with the price to ship one home.
[**list_buyer_vault_market_holdings**](BuyerApi.md#list_buyer_vault_market_holdings) | **GET** /v1/buyer/vault-markets/holdings | Each unit you hold in the vault at its market price, with the mark’s quality (A or B reliable, C indicative, none), cost, P&L and where it stands (stored, listed, lent, pledged, at the grader); `meta` carries the totals and the units you have out on loan.
[**list_buyer_vault_market_indexes**](BuyerApi.md#list_buyer_vault_market_indexes) | **GET** /v1/buyer/vault-markets/indexes | Crossly indexes over vault book keys (category, brand, curated, composite, segment, and your own), each with its level now and change since the last daily close. Index levels are valued only from the book’s marks.
[**list_buyer_vault_market_instant_exit**](BuyerApi.md#list_buyer_vault_market_instant_exit) | **GET** /v1/buyer/vault-markets/instant-exit | Instant Exit quotes for what you hold, one per book key: the best resting bid (yours left out), its share of the mark, the bid levels, units held and which are free to sell, and why a key cannot be sold instantly. A quote, not an order.
[**list_buyer_vault_market_watchlist**](BuyerApi.md#list_buyer_vault_market_watchlist) | **GET** /v1/buyer/vault-markets/watchlist | The book keys you watch, with mark and its quality, 24 h and 7 d change, and the last 30 daily marks.
[**list_buyer_video_requests**](BuyerApi.md#list_buyer_video_requests) | **GET** /v1/buyer/video-requests | Your video requests and their answers.
[**list_buyer_wishlist_items**](BuyerApi.md#list_buyer_wishlist_items) | **GET** /v1/buyer/wishlists/{id}/items | What is on one wishlist.
[**list_buyer_wishlist_members**](BuyerApi.md#list_buyer_wishlist_members) | **GET** /v1/buyer/wishlists/{id}/members | Co-editors of a shared wishlist.
[**list_buyer_wishlists**](BuyerApi.md#list_buyer_wishlists) | **GET** /v1/buyer/wishlists | Your wishlists.
[**update_buyer_board**](BuyerApi.md#update_buyer_board) | **PATCH** /v1/buyer/boards/{id} | Rename a board, change its description, or make it public / private.
[**update_buyer_board_layout**](BuyerApi.md#update_buyer_board_layout) | **PUT** /v1/buyer/boards/{id}/layout | Replace a board’s layout: items by platform listing id or slug, with x, y, w (0–1) and z. At most 40.
[**update_buyer_collection_set_leaderboard**](BuyerApi.md#update_buyer_collection_set_leaderboard) | **PUT** /v1/buyer/collection/sets/leaderboard | Show or hide yourself (by your public handle) on set leaderboards.
[**update_buyer_feedback**](BuyerApi.md#update_buyer_feedback) | **PATCH** /v1/buyer/feedback/{id} | Revise feedback you left, within 60 days.
[**update_buyer_hunt**](BuyerApi.md#update_buyer_hunt) | **PATCH** /v1/buyer/hunts/{id} | Pause, resume or stop a hunt.
[**update_buyer_purchase_story**](BuyerApi.md#update_buyer_purchase_story) | **PUT** /v1/buyer/purchases/{id}/story | Write your owner story (280 characters); it travels with the item if you resell it.
[**update_buyer_registry**](BuyerApi.md#update_buyer_registry) | **PATCH** /v1/buyer/registries/{id} | Edit a registry: title, kind, date, message, ship-to address.
[**update_buyer_registry_item**](BuyerApi.md#update_buyer_registry_item) | **PUT** /v1/buyer/registries/{id}/items/{platformListingId} | How many of an item you want (1–50).
[**update_buyer_subscription**](BuyerApi.md#update_buyer_subscription) | **PATCH** /v1/buyer/subscriptions/{id} | Pause, resume, skip the next delivery, change the interval, or cancel.
[**update_buyer_wishlist**](BuyerApi.md#update_buyer_wishlist) | **PATCH** /v1/buyer/wishlists/{id} | Rename a wishlist, or make it public / private.



## create_buyer_activity

> crate::models::CreateBuyerActivityResponse create_buyer_activity(create_buyer_activity_request)
Report an item your user is looking at, and get our answer.

Records the look and returns whether Crossly has the item and at what price. Send an identifier and a store DOMAIN — a full URL is refused. Nothing about the page itself is stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_activity_request** | **CreateBuyerActivityRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerActivityResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_auction_bid

> crate::models::CreateBuyerAuctionBidResponse create_buyer_auction_bid(slug, create_buyer_auction_bid_request)
Place a bid, optionally with a proxy maximum. A WINNING BID IS CHARGED AUTOMATICALLY.

Spends money: if you win, your saved card is charged when the auction closes, with nobody present. Requires buyer:checkout:write AND this key switched on at PUT /v1/buyer/checkout/controls; the bid’s maximum (or amount) is checked against the key’s per-order and daily ceilings. Amounts are integer cents. `leading: false` means a standing proxy bid outbid you at once. Send an Idempotency-Key: a retried bid is a second bid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The auction listing slug. | [required] |
**create_buyer_auction_bid_request** | **CreateBuyerAuctionBidRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerAuctionBidResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_auction_pay

> crate::models::CreateBuyerAuctionPayResponse create_buyer_auction_pay(slug)
Pay for an auction you won whose automatic charge did not go through.

Returns a Stripe client secret to confirm on the buyer’s device (3-D Secure, a new card). Charges nothing by itself. `paid: true` means there is nothing left to pay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The auction listing slug. | [required] |

### Return type

**crate::models::CreateBuyerAuctionPayResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_auction_retract

> crate::models::CreateBuyerAuctionRetractResponse create_buyer_auction_retract(slug, create_buyer_auction_retract_request)
Retract your bids under eBay’s rules — a typo, a changed listing, or an unreachable seller.

With 12 hours or more left every bid of yours is withdrawn; inside 12 hours only bids placed in the last hour. Proxy answers your bid triggered are withdrawn with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The auction listing slug. | [required] |
**create_buyer_auction_retract_request** | **CreateBuyerAuctionRetractRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerAuctionRetractResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_board

> crate::models::CreateBuyerBoardResponse create_buyer_board()
Create an outfit board.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerBoardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_board_item

> crate::models::BuyerResponse create_buyer_board_item(id)
Add a listing (by slug) to a board, at the next free spot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_break_spot_hold

> crate::models::CreateBuyerBreakSpotHoldResponse create_buyer_break_spot_hold(id, spot_id)
Hold a spot in a pick-your-spot break, then buy it through checkout before the hold lapses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**spot_id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerBreakSpotHoldResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_buyback_request

> crate::models::BuyerResponse create_buyer_buyback_request(id)
Ask Crossly to buy an item back under its buyback guarantee.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_cart_item

> crate::models::CreateBuyerCartItemResponse create_buyer_cart_item(create_buyer_cart_item_request)
Add a listing to your cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_cart_item_request** | **CreateBuyerCartItemRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerCartItemResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_cart_item_save_for_later

> crate::models::BuyerCartResponse create_buyer_cart_item_save_for_later(id)
Move a cart line to saved-for-later.

Returns the whole saved list. A line from an accepted offer cannot be saved — its price only holds in the cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerCartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_cart_optimizer

> crate::models::CreateBuyerCartOptimizerResponse create_buyer_cart_optimizer()
Cart optimizer: send a shopping list (one item per line, \"2 Charizard Base Set\") and get the cheapest delivered basket across sellers — one pick per line, items + each seller’s shipping with free-shipping thresholds applied. fewestPackages prefers fewer sellers. Nothing is added to the cart.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerCartOptimizerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_cart_quote

> crate::models::CreateBuyerCartQuoteResponse create_buyer_cart_quote()
Price the cart, delivered — item, shipping, tax, total.

Runs the real checkout cascade and returns the totals instead of charging. Nothing is purchased. `taxComplete: false` means there is no saved delivery address, so the total is a floor rather than a final figure. `shippingOptions` lists each seller’s shipping services; send `shippingServiceBySeller` to price a different speed (an id the seller does not offer to the destination is a 400). Optional body: `{ \"shippingServiceBySeller\": { \"<sellerUserId>\": \"<serviceId>\" } }` — omitted sellers get the cheapest service; `registryToken` prices it as a gift shipped to that registry. (No body JSON schema: existing callers POST with none.)

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerCartQuoteResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_cart_saved_move_to_cart

> crate::models::BuyerCartResponse create_buyer_cart_saved_move_to_cart(id)
Move a saved item back into your cart, at today’s price.

Refused when the item has sold or delisted. Returns the remaining saved list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerCartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_case_counter

> crate::models::BuyerResponse create_buyer_case_counter(id, create_buyer_case_counter_request)
Propose your own refund figure on a case.

In cents, at most what you paid. The case goes back to the seller for 72 hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**create_buyer_case_counter_request** | **CreateBuyerCaseCounterRequest** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_case_escalate

> crate::models::BuyerResponse create_buyer_case_escalate(id)
Ask Crossly to step in on a case.

Hands the case to Crossly’s reviewers. Refused once it is closed or already escalated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_case_message

> crate::models::BuyerResponse create_buyer_case_message(id, create_buyer_case_message_request)
Post a message on a case thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**create_buyer_case_message_request** | **CreateBuyerCaseMessageRequest** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_case_respond

> crate::models::BuyerResponse create_buyer_case_respond(id, create_buyer_case_respond_request)
Accept or reject the seller’s partial-refund offer on a case.

Accepting refunds the offered amount and closes the case. Rejecting hands it back to the seller for 72 hours. To propose your own figure instead, use the counter endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**create_buyer_case_respond_request** | **CreateBuyerCaseRespondRequest** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_case_withdraw

> crate::models::BuyerResponse create_buyer_case_withdraw(id)
Withdraw a case you opened.

Closes it with no refund and lets the order complete normally. Cannot be reopened — one case per order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_catalog_listing_rental

> crate::models::CreateBuyerCatalogListingRentalResponse create_buyer_catalog_listing_rental(slug)
Book a rental. Returns a Stripe PaymentIntent client secret for rent + shipping both ways + deposit; the dates are held 30 minutes for payment. Crossly buys both labels; the deposit comes back after the item does.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerCatalogListingRentalResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_catalog_listing_rental_quote

> crate::models::CreateBuyerCatalogListingRentalQuoteResponse create_buyer_catalog_listing_rental_quote(slug)
Price a rental: rent (the cheaper of by-the-day and by-the-week), both shipping legs once a ship-to is given, and the deposit. `problem` says why the dates can’t be booked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerCatalogListingRentalQuoteResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_catalog_listing_video_request

> crate::models::CreateBuyerCatalogListingVideoRequestResponse create_buyer_catalog_listing_video_request(slug)
Ask the seller for a video of an item (one open request per item). The seller is notified; their answer appears on the listing for every buyer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerCatalogListingVideoRequestResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_conversation_invite

> crate::models::CreateBuyerWishlistInviteResponse create_buyer_conversation_invite(id)
Invite a friend into a chat with a seller: a single-use link (7 days) that makes whoever opens it a guest who can read and post. Only the thread’s buyer can invite; at most 5 guests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerWishlistInviteResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_conversation_join

> crate::models::CreateBuyerConversationJoinResponse create_buyer_conversation_join(token)
Join a chat you were invited into, as a guest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerConversationJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_feedback

> crate::models::CreateBuyerFeedbackResponse create_buyer_feedback(create_buyer_feedback_request)
Leave feedback on up to 25 orders at once, with optional detailed seller ratings.

Each entry is checked on its own — one refused comment never sinks the others; read `data[].ok`. Orders to review come from GET /v1/buyer/feedback/pending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_feedback_request** | **CreateBuyerFeedbackRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerFeedbackResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_fit_ref

> crate::models::BuyerResponse2 create_buyer_fit_ref()
Save a reference garment (flat measurements, inches) to compare listings against.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse2**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_following_seller_seen

> crate::models::CreateBuyerFollowingSellerSeenResponse create_buyer_following_seller_seen(create_buyer_following_seller_seen_request)
Clear the \"new listings\" count on a followed seller, or on all of them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_following_seller_seen_request** | **CreateBuyerFollowingSellerSeenRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerFollowingSellerSeenResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_gift_card

> crate::models::CreateBuyerGiftCardResponse create_buyer_gift_card(create_buyer_gift_card_request)
Start a gift card purchase.

Creates an unpaid card and a Stripe PaymentIntent, and returns its client secret. Nothing is charged until the PaymentIntent is confirmed; the code is emailed to the recipient once payment clears (at deliverAt, if given) and is never returned here. $10–$500. Cards and their credit expire 5 years after purchase. Idempotency-Key required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_gift_card_request** | **CreateBuyerGiftCardRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerGiftCardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_gift_card_redeem

> crate::models::CreateBuyerGiftCardRedeemResponse create_buyer_gift_card_redeem(create_buyer_gift_card_redeem_request)
Redeem a gift card code into your own balance.

Adds the card’s value to the token owner’s Crossly balance (Crossly Bucks today; CBX granted balance when Crossly issues CBX), spendable at checkout. One redemption per card. The credit keeps the card’s expiry date. Refusals: 404 invalid_code, 409 already_redeemed / not_paid / void / expired.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_gift_card_redeem_request** | **CreateBuyerGiftCardRedeemRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerGiftCardRedeemResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_group_buy_join

> crate::models::CreateBuyerGroupBuyJoinResponse create_buyer_group_buy_join(id)
Join a group buy: a hold at the starting price, captured at the price the group reaches. Checked against this key’s spending controls; send an Idempotency-Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerGroupBuyJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_group_buy_leave

> crate::models::BuyerResponse create_buyer_group_buy_leave(id)
Leave a group buy and release your hold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_hunt

> crate::models::BuyerResponse2 create_buyer_hunt()
Start a buyer-agent hunt — over a search (filters), or auto-buy on one listing (listingSlug: buy it once its price, sale applied, is within maxPriceCents). `offer` and `buy` modes also need buyer:checkout:write, and the budget is checked against this key’s spending controls.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse2**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_instant_offer_accept

> crate::models::BuyerResponse create_buyer_instant_offer_accept(id)
Accept Crossly’s instant offer on a collection item. Ops then send a prepaid label; the store credit is paid once the item arrives and checks out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_layaway_cancel

> crate::models::BuyerLayawayResponse create_buyer_layaway_cancel(id)
Cancel a layaway: everything paid comes back except the layaway fee (10% of the price, at most $15).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerLayawayResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_layaway_pay

> crate::models::BuyerLayawayResponse create_buyer_layaway_pay(id)
Pay some (at least $1) or all of what is left now, on the saved card. Paid in full, it ships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerLayawayResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_listing_audio_summary

> crate::models::GetBuyerListingAudioSummaryResponse create_buyer_listing_audio_summary(slug)
Record the spoken summary of a listing (cached per listing version). 503 naming TTS_API_KEY when no speech provider is configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingAudioSummaryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_listing_split

> crate::models::CreateBuyerListingSplitResponse create_buyer_listing_split(slug)
Split a purchase: one pay link per person; nobody is charged until all have paid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerListingSplitResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_listing_subscribe

> crate::models::BuyerResponse2 create_buyer_listing_subscribe(slug)
Subscribe & save: a standing order charged to your saved card every interval. Checked against this key’s spending controls; send an Idempotency-Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse2**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_listing_try_on

> crate::models::CreateBuyerListingTryOnResponse create_buyer_listing_try_on(slug)
Render an AI try-on of this listing on the buyer’s own photo (base64 `photo`, or `reuseLastPhoto`). `confirmSelfAdult` must be true: the photo is of the buyer and they are 18+. Wearables only; capped per day; 503 naming TRYON_API_KEY when no image model is configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerListingTryOnResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_market_bid

> crate::models::CreateBuyerMarketBidResponse create_buyer_market_bid()
Place a fixed-price market bid. SPENDS MONEY: it fills against the lowest ask at or under your price and charges your saved card (plus the fast-ship premium when fastShip is set and it fills from vault stock). deliverTo vault keeps it in your Crossly vault; gradeWith sends a raw card to a Grading run first. Checked against this key’s spending controls; send an Idempotency-Key.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerMarketBidResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_market_watch

> crate::models::BuyerResponse create_buyer_market_watch()
Watch a market variant: alert at or below a price, or on any drop.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_offer

> crate::models::CreateBuyerOfferResponse create_buyer_offer(create_buyer_offer_request)
Offer a price on a listing.

Sends an offer to the seller. Spends nothing — a seller accepting still leaves you to complete checkout. Offers at or above the asking price are refused; buy it instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_offer_request** | **CreateBuyerOfferRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerOfferResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_offer_respond

> crate::models::CreateBuyerOfferRespondResponse create_buyer_offer_respond(id, create_buyer_offer_respond_request)
Answer a seller’s counteroffer — accept, decline, or counter back.

Only an offer with `awaitingBuyer: true` can be answered. Accepting opens a 48-hour window to pay on Crossly; it charges nothing. A counter must be below the seller’s price (in cents) and becomes a new offer waiting on the seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The offer id. | [required] |
**create_buyer_offer_respond_request** | **CreateBuyerOfferRespondRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerOfferRespondResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_order_cancel

> crate::models::CreateBuyerOrderCancelResponse create_buyer_order_cancel(id)
Cancel an order before it ships — refunds it in full.

Instant while the seller has not bought a label or shipped. Refused with 409 once they have (`details.shopCode`: already_shipped, label_purchased, disputed, already_refunded). Check GET /v1/buyer/orders/{id}/eligibility first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::CreateBuyerOrderCancelResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_order_inr

> crate::models::CreateBuyerOrderInrResponse create_buyer_order_inr(id, create_buyer_order_inr_request)
Open an item-not-received case.

Only after the expected delivery window — a 409 carries `details.reason` and `details.opensAt` when it is too early. The seller then has 72 hours to respond. One case per order, ever.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**create_buyer_order_inr_request** | **CreateBuyerOrderInrRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerOrderInrResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_order_received

> crate::models::CreateBuyerOrderReceivedResponse create_buyer_order_received(id)
Confirm you received an order.

Records receipt and opens feedback. It does NOT release the seller’s money sooner or shorten your buyer protection. Closes the item-not-received entry point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::CreateBuyerOrderReceivedResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_order_tip

> crate::models::CreateBuyerOrderTipResponse create_buyer_order_tip(id)
Tip the seller on one of your shipped orders, charged to your saved card. One tip per order; $1–$500; checked against this key’s spending controls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerOrderTipResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_pack_fairness_rotate

> crate::models::GetBuyerPackFairnessResponse create_buyer_pack_fairness_rotate()
Reveal your current server seed and start a new one (optionally with your own client seed). Refused while an opening is awaiting payment.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerPackFairnessResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_pack_open

> crate::models::CreateBuyerPackOpenResponse create_buyer_pack_open(slug)
Open a pack. SPENDS MONEY: charges the pack price to your saved card, then pulls one real item by the published odds (provably fair: see /buyer/packs/fairness) into your Crossly vault. confirmAdult must be true. Checked against this key’s spending controls; send an Idempotency-Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerPackOpenResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_pack_opening_sell_back

> crate::models::CreateBuyerPackOpeningSellBackResponse create_buyer_pack_opening_sell_back(id)
Sell a pull straight back for Crossly credit at the pack’s buyback rate, within its window, while it is still in your vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerPackOpeningSellBackResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_passport_claim

> crate::models::GetBuyerPassportResponse create_buyer_passport_claim(code)
Claim a passport as the item’s current owner (the buyer of its latest sale) — shown as “in its owner’s hands” until it sells again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPassportResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_protection_plan_cancel

> crate::models::CreateBuyerProtectionPlanCancelResponse create_buyer_protection_plan_cancel(id)
Cancel a protection plan. The insurer decides what is refundable (pro-rata after the free-look window); that amount goes back to the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerProtectionPlanCancelResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_purchase_download

> crate::models::CreateBuyerPurchaseDownloadResponse create_buyer_purchase_download(id)
A ten-minute link to a digital purchase’s file; each call uses one of its downloads.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerPurchaseDownloadResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_purchase_fit

> crate::models::BuyerResponse create_buyer_purchase_fit(id)
Say how a delivered purchase fit: true_to_size, small or large. It teaches your size memory and, pooled with other buyers, how the brand runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_purchase_return

> crate::models::BuyerResponse2 create_buyer_purchase_return(id)
Return a purchase under the seller’s policy, or send back a try-at-home item. With reason doesnt_fit, `fit` (small | large) says which way, and it feeds your size memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse2**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_registry

> crate::models::CreateBuyerRegistryResponse create_buyer_registry()
Start a gift registry (wedding, baby, birthday, housewarming, graduation, other). Items are added like any wishlist item; set shipAddressId to one of your saved addresses so guests can buy — they never see it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerRegistryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_registry_gift_thanked

> crate::models::CreateBuyerRegistryGiftThankedResponse create_buyer_registry_gift_thanked(id, gift_id)
Tick a gift as thanked (or untick it).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**gift_id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerRegistryGiftThankedResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_rental_arrival_photo

> crate::models::CreateBuyerRentalArrivalPhotoResponse create_buyer_rental_arrival_photo(id)
Photograph a rental on arrival (URLs you uploaded): your side of the condition record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerRentalArrivalPhotoResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_rental_cancel

> crate::models::BuyerResponse create_buyer_rental_cancel(id)
Cancel a rental that has not shipped: everything is refunded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_rental_claim_response

> crate::models::BuyerResponse create_buyer_rental_claim_response(id)
Answer a deposit claim: accept (the claim is paid, the rest refunded) or dispute (Crossly decides).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_rental_identity_start

> crate::models::CreateBuyerRentalIdentityStartResponse create_buyer_rental_identity_start()
Start the ID check renting needs (Stripe Identity: photo ID + selfie). Open `url` (hosted) or use `clientSecret` with Stripe.js.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerRentalIdentityStartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_return_cancel

> crate::models::BuyerResponse create_buyer_return_cancel(id)
Cancel a return before it ships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_return_tracking

> crate::models::BuyerResponse create_buyer_return_tracking(id)
Add tracking to a return you shipped on your own label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_seller_club_join

> crate::models::CreateBuyerSellerClubJoinResponse create_buyer_seller_club_join(seller_user_id)
Join a seller’s loyalty club (free). Your tier’s discount then applies to their items — the best of it and any other rate you have with them, never stacked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerSellerClubJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_seller_report

> crate::models::CreateBuyerSellerReportResponse create_buyer_seller_report(username, create_buyer_seller_report_request)
Report a seller to Crossly’s trust team.

A second report while your first is still under review updates that report instead of filing another. `purchaseId` is attached only if it is your order from this seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The seller’s shop handle. | [required] |
**create_buyer_seller_report_request** | **CreateBuyerSellerReportRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerSellerReportResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_split_cancel

> crate::models::BuyerResponse create_buyer_split_cancel(id)
Cancel a split you organised; every held share is released.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together

> crate::models::CreateBuyerTogetherResponse create_buyer_together()
Start a shop-together room.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerTogetherResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together_item

> crate::models::BuyerResponse create_buyer_together_item(code)
Add a listing to a shop-together board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together_item_vote

> crate::models::BuyerResponse create_buyer_together_item_vote(code, id)
Vote an item on a room’s board up (1), down (-1) or clear (0).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together_join

> crate::models::CreateBuyerTogetherResponse create_buyer_together_join(code)
Join a shop-together room by its code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerTogetherResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together_leave

> crate::models::BuyerResponse create_buyer_together_leave(code)
Leave a shop-together room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_together_message

> crate::models::BuyerResponse create_buyer_together_message(code)
Say something in a shop-together room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_total_request

> crate::models::CreateBuyerTotalRequestResponse create_buyer_total_request(create_buyer_total_request_request)
Ask a seller for one combined shipping total on everything of theirs in your cart.

Needs two or more of the seller’s items in your cart. Spends nothing; the quote applies at checkout on Crossly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_total_request_request** | **CreateBuyerTotalRequestRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerTotalRequestResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_vault_list

> crate::models::CreateBuyerVaultListResponse create_buyer_vault_list(id)
Sell a vault item on the market at your price. It is already authenticated, so when it sells nothing ships in: the buyer keeps it vaulted or it ships straight to them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerVaultListResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_vault_unlist

> crate::models::BuyerResponse create_buyer_vault_unlist(id)
Take a vault item off the market; it stays stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_vault_withdraw

> crate::models::CreateBuyerVaultWithdrawResponse create_buyer_vault_withdraw(id)
Ship a vault item home, standard or expedited. The shipping, any storage it owes and the early-withdrawal fee (within 90 days of arriving) are charged to your saved card now, all checked against this key’s spending controls; send an Idempotency-Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerVaultWithdrawResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_wishlist

> crate::models::CreateBuyerWishlistResponse create_buyer_wishlist(create_buyer_wishlist_request)
Create a wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_buyer_wishlist_request** | **CreateBuyerWishlistRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerWishlistResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_wishlist_invite

> crate::models::CreateBuyerWishlistInviteResponse create_buyer_wishlist_invite(id)
Share a wishlist with a co-editor: a single-use link (14 days). Members add and remove items; only the owner renames, deletes or removes members. At most 20.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerWishlistInviteResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_wishlist_item

> crate::models::BuyerResponse2 create_buyer_wishlist_item(id, create_buyer_wishlist_item_request)
Add a listing to a wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_buyer_wishlist_item_request** | **CreateBuyerWishlistItemRequest** |  | [required] |

### Return type

**crate::models::BuyerResponse2**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_wishlist_join

> crate::models::CreateBuyerWishlistJoinResponse create_buyer_wishlist_join(token)
Join a wishlist you were invited to edit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerWishlistJoinResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_wishlist_share

> crate::models::CreateBuyerWishlistShareResponse create_buyer_wishlist_share(id, create_buyer_wishlist_share_request)
Share a wishlist — makes it public and returns its link.

`resetLink: true` issues a new link and kills the old one, for a link that went further than intended. To stop sharing, PATCH the list with isPublic: false.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**create_buyer_wishlist_share_request** | **CreateBuyerWishlistShareRequest** |  | [required] |

### Return type

**crate::models::CreateBuyerWishlistShareResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_board

> crate::models::BuyerResponse delete_buyer_board(id)
Delete a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_board_item

> crate::models::BuyerResponse delete_buyer_board_item(id, platform_listing_id)
Take a listing off a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_break_spot_hold

> crate::models::BuyerResponse delete_buyer_break_spot_hold(id, spot_id)
Release a break spot you hold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**spot_id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_cart_item

> delete_buyer_cart_item(id)
Remove a line from your cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_cart_saved

> crate::models::BuyerCartResponse delete_buyer_cart_saved(id)
Remove an item from saved-for-later. Returns the remaining list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerCartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_conversation_guest

> crate::models::BuyerResponse delete_buyer_conversation_guest(id, user_id)
Remove a guest (the thread’s buyer), or leave (a guest, with your own user id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_fit_ref

> crate::models::BuyerResponse delete_buyer_fit_ref(id)
Delete a reference garment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_market_bid

> crate::models::BuyerResponse delete_buyer_market_bid(id)
Cancel a resting market bid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_market_watch

> crate::models::BuyerResponse delete_buyer_market_watch(id)
Stop watching a market variant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_offer

> crate::models::BuyerResponse delete_buyer_offer(id)
Withdraw an offer you made that the seller has not answered.

Only a pending offer can be withdrawn — an accepted price the seller agreed to cannot be taken back here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The offer id. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_recently_viewed

> crate::models::BuyerResponse delete_buyer_recently_viewed(slug)
Remove one listing from your recently viewed.

Viewing it again brings it back. Sellers’ view counts are unaffected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The listing slug. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_recently_viewed2

> crate::models::BuyerResponse delete_buyer_recently_viewed2()
Clear your recently viewed list.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_registry

> crate::models::BuyerResponse delete_buyer_registry(id)
Delete a registry (and the wishlist behind it).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_seller_club_join

> crate::models::BuyerResponse delete_buyer_seller_club_join(seller_user_id)
Leave a seller’s loyalty club.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_user_id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_together_item

> crate::models::BuyerResponse delete_buyer_together_item(code, id)
Take an item you added off a room’s board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_total_request

> crate::models::BuyerResponse delete_buyer_total_request(id)
Cancel a total request you made.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_try_on

> crate::models::DeleteBuyerTryOnResponse delete_buyer_try_on(id)
Delete a try-on and its images — or pass `all` to delete every photo and result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteBuyerTryOnResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_wishlist

> crate::models::BuyerResponse delete_buyer_wishlist(id)
Delete a wishlist and everything on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_wishlist_item

> crate::models::DeleteBuyerWishlistItemResponse delete_buyer_wishlist_item(id, slug)
Take a listing off a wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The wishlist id. | [required] |
**slug** | **String** | The listing slug. | [required] |

### Return type

**crate::models::DeleteBuyerWishlistItemResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_wishlist_member

> crate::models::BuyerResponse delete_buyer_wishlist_member(id, user_id)
Remove a co-editor (the owner), or leave a list shared with you (your own user id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_auction_me

> crate::models::GetBuyerAuctionMeResponse get_buyer_auction_me(slug)
Where you stand in one auction — your max, leading, won, must you pay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The auction listing slug. | [required] |

### Return type

**crate::models::GetBuyerAuctionMeResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_badge

> crate::models::GetBuyerBadgeResponse get_buyer_badge()
The counts behind the account badge — unread messages, offers waiting, orders to review.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerBadgeResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_board

> crate::models::BuyerBoardResponse get_buyer_board(id)
One of your boards with its layout: each item’s x, y and width as fractions of the board, and stacking order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerBoardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_board_shared

> crate::models::BuyerBoardResponse get_buyer_board_shared(token)
A shared outfit board, by its link token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

**crate::models::BuyerBoardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_break

> crate::models::GetBuyerBreakResponse get_buyer_break(id)
A card break’s board: spots, holders, and the seed after the draw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerBreakResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_case

> crate::models::GetBuyerCaseResponse get_buyer_case(id)
One case — its state, the message thread, and the return parcel if there is one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::GetBuyerCaseResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing_my_size

> crate::models::GetBuyerCatalogListingMySizeResponse get_buyer_catalog_listing_my_size(slug)
This item against your size: the size you keep in this kind of item (and in this brand), whether this one is your size, smaller or larger, a warning if this brand and size ran small or large on you before, and how other buyers say the brand runs. null for an unsized item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCatalogListingMySizeResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing_price

> crate::models::GetBuyerCatalogListingPriceResponse get_buyer_catalog_listing_price(slug)
The price today. When the seller has the same item live on another marketplace for less, this IS that price (`priceMatch` says where from), and `lowestAnywhere` lists the marketplaces it is guaranteed not to be cheaper on. `priceCheck` rates it against real sales of the same catalog item in the same condition over the last 180 days (great / good / fair / high, with the median and middle-half range in cents); null until there are 3+ sales from 2+ sellers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCatalogListingPriceResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_catalog_listing_rental

> crate::models::GetBuyerCatalogListingRentalResponse get_buyer_catalog_listing_rental(slug)
Whether a listing is for rent: day and week rates, the refundable deposit, length limits, the turnaround buffer, and the dates already booked. null when it is not for rent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCatalogListingRentalResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_checkout_usdc

> crate::models::GetBuyerCheckoutUsdcResponse get_buyer_checkout_usdc()
Whether Crossly takes USDC (Stripe stablecoin payments: Solana, Ethereum, Base, Polygon), and its per-order limit. Paying needs the buyer’s wallet, so it happens in the browser checkout, never unattended.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerCheckoutUsdcResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_collection_offer

> crate::models::GetBuyerCollectionOfferResponse get_buyer_collection_offer(id)
What an item in your collection would fetch now: the top open bid on the market for the same variant, condition and grade, and Crossly’s instant offer in store credit when that is switched on (a quote valid for a limited time).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCollectionOfferResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_collection_set

> crate::models::GetBuyerCollectionSetResponse get_buyer_collection_set(category, brand, model)
One set: every piece (owned or missing, with its lowest ask on the market), what completing it would cost now, and the leaderboard of collectors who opted in, with your rank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**brand** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCollectionSetResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_creator

> crate::models::GetBuyerCreatorResponse get_buyer_creator(handle)
A creator storefront: profile, affiliate code, and shelves of live picks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerCreatorResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_fit_vocabulary

> crate::models::GetBuyerFitVocabularyResponse get_buyer_fit_vocabulary()
Garment kinds and the measurements each takes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerFitVocabularyResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_gift_finder_option

> crate::models::GetBuyerGiftFinderOptionResponse get_buyer_gift_finder_option()
The gift finder’s questions: recipients, interests, budgets.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerGiftFinderOptionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_impact

> crate::models::GetBuyerImpactResponse get_buyer_impact()
The sourced footprint your secondhand purchases avoided.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerImpactResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_layaway

> crate::models::BuyerLayawayResponse get_buyer_layaway(id)
One layaway and its schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerLayawayResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_audio_summary

> crate::models::GetBuyerListingAudioSummaryResponse get_buyer_listing_audio_summary(slug)
A spoken summary of a listing (accessibility): the text, built only from the listing’s own fields and the seller’s description, and the recording of it when one exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingAudioSummaryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_bought_together

> crate::models::GetBuyerListingBoughtTogetherResponse get_buyer_listing_bought_together(slug)
Frequently bought together / complete the look.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingBoughtTogetherResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_break

> crate::models::GetBuyerListingBreakResponse get_buyer_listing_break(slug)
The card break this listing sells spots in, if any; the seed is revealed after the draw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingBreakResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_cert

> crate::models::GetBuyerListingCertResponse get_buyer_listing_cert(slug)
A graded listing’s cert, checked with the grader: the seller’s claim (grader, grade, cert number), status (verified, mismatch, not_found, or unchecked where the grader has no lookup), population and how many graded higher where the grader publishes them, and the grader’s own cert page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingCertResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_fit

> crate::models::GetBuyerListingFitResponse get_buyer_listing_fit(slug)
A listing’s flat measurements, compared with your saved reference garments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingFitResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_group_buy

> crate::models::GetBuyerListingGroupBuyResponse get_buyer_listing_group_buy(slug)
The group buy open on this listing, if any.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingGroupBuyResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_impact

> crate::models::GetBuyerListingImpactResponse get_buyer_listing_impact(slug)
The sourced footprint of making a new equivalent; null when no published figure applies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingImpactResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_resale_value

> crate::models::GetBuyerListingResaleValueResponse get_buyer_listing_resale_value(slug)
What the same brand and category resold for on Crossly, and the buyback offer when enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingResaleValueResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_subscribe_offer

> crate::models::GetBuyerListingSubscribeOfferResponse get_buyer_listing_subscribe_offer(slug)
The subscribe & save discount and intervals on a listing; null when the seller has none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingSubscribeOfferResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_listing_try_on

> crate::models::GetBuyerListingTryOnResponse get_buyer_listing_try_on(slug)
Whether an AI try-on is offered for this listing (a wearable with a photo) and whether the image model is configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerListingTryOnResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_live_stream_auction

> crate::models::GetBuyerLiveStreamAuctionResponse get_buyer_live_stream_auction(id)
The auction running on a live stream right now, if any. Bid on it by its `slug` at POST /v1/buyer/auctions/:slug/bids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerLiveStreamAuctionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_market_purchase_option

> crate::models::GetBuyerMarketPurchaseOptionResponse get_buyer_market_purchase_option()
Before you bid or buy: whether the lowest ask is vault stock (keep it vaulted, or pay fastShipCents to rush it), and the grading runs a raw card can go straight to (pass gradeWith on the bid).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerMarketPurchaseOptionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_order_eligibility

> crate::models::GetBuyerOrderEligibilityResponse get_buyer_order_eligibility(id)
What you can do with one order right now — cancel, confirm receipt, report not received, leave feedback.

`inr.eligible` says whether an item-not-received case can be opened; when it cannot, `inr.reason` says why and `inr.opensAt` when it will. The same rules the order page and the action routes enforce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |

### Return type

**crate::models::GetBuyerOrderEligibilityResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_order_export

> crate::models::GetBuyerOrderExportResponse get_buyer_order_export(from, to)
Your order history as CSV text (up to 5,000 orders).

The same CSV the shop downloads, returned as a string in `data.csv` so a program or agent can read it without a file download. Money columns are dollars, as a spreadsheet expects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | First day to include, YYYY-MM-DD. |  |
**to** | Option<**String**> | Last day to include, YYYY-MM-DD (inclusive). |  |

### Return type

**crate::models::GetBuyerOrderExportResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_order_price_protection

> crate::models::GetBuyerOrderPriceProtectionResponse get_buyer_order_price_protection(id)
Price protection on one order: if the item gets cheaper before `until`, the difference is credited to your Bucks automatically. `creditedCents` is what it has paid so far.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerOrderPriceProtectionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_pack

> crate::models::GetBuyerPackResponse get_buyer_pack(slug)
One pack: tiers and odds, items left, expected value, the chase items still in it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPackResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_pack_fairness

> crate::models::GetBuyerPackFairnessResponse get_buyer_pack_fairness()
Your provably-fair seeds: the active server-seed commitment (sha256), your client seed and nonce, and every retired seed in full so past openings can be re-run.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerPackFairnessResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_party

> crate::models::GetBuyerPartyResponse get_buyer_party(slug)
One shopping party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPartyResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_passport

> crate::models::GetBuyerPassportResponse get_buyer_passport(code)
What a product passport tag opens: the item, whether it was authenticated, how many owners on Crossly and since when — never who or for how much.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPassportResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_preference

> crate::models::GetBuyerPreferenceResponse get_buyer_preference()
The shopping profile derived from that activity.

Derived, never declared — there is no preferences form anywhere. `matchRate` is the share of this person's searches Crossly could answer; a low number is an inventory problem, not a personalisation one, which is why it is here.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerPreferenceResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_price_guide

> crate::models::GetBuyerPriceGuideResponse get_buyer_price_guide()
What matching items actually sold for on Crossly: median, quartiles, by month, and recent sales. Needs q, brand or category.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerPriceGuideResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_profile

> crate::models::GetBuyerProfileResponse get_buyer_profile()
Your Crossly shopping profile — name, email, saved address, Bucks balance.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerProfileResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_purchase_delivery

> crate::models::GetBuyerPurchaseDeliveryResponse get_buyer_purchase_delivery(id)
Where a purchase is going when not to your door: the pickup point (with its release code) or its ship-together consolidation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPurchaseDeliveryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_purchase_live_moment

> crate::models::GetBuyerPurchaseLiveMomentResponse get_buyer_purchase_live_moment(id)
The live show a purchase was bought in, with the offset (seconds) into its replay where it sold. null when it was not bought in a show.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPurchaseLiveMomentResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_purchase_return

> crate::models::GetBuyerPurchaseReturnResponse get_buyer_purchase_return(id)
What a return would refund for each reason, by when, and any return already open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPurchaseReturnResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_purchase_story

> crate::models::GetBuyerPurchaseStoryResponse get_buyer_purchase_story(id)
Your owner story on a purchase, passed on to the next owner if you resell it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerPurchaseStoryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_registry

> crate::models::BuyerRegistryResponse get_buyer_registry(id)
One of your registries: items with how many are wanted and bought.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerRegistryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_registry_by_token

> crate::models::BuyerRegistryResponse get_buyer_registry_by_token(token)
A registry as a guest sees it: items, how many are still wanted, and whether it can ship. Never the address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

**crate::models::BuyerRegistryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_rental_eligibility

> crate::models::GetBuyerRentalEligibilityResponse get_buyer_rental_eligibility()
Can you rent right now: ID-verified (a photo ID and a matching selfie, once), the verified name rentals ship to, and your standing (one rental at a time until your first comes back; none after one that never did).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerRentalEligibilityResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_seller_club

> crate::models::GetBuyerSellerClubResponse get_buyer_seller_club(seller_user_id)
A seller’s loyalty club: its tiers (spend threshold, discount, perks) and your standing. null when the seller has none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_user_id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerSellerClubResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_seller_trust

> crate::models::GetBuyerSellerTrustResponse get_buyer_seller_trust(username)
Seller at a glance: member since, completed sales, independent rating, typical handling time, dispute rate, trust tier, badges — and the plain sentences the profile shows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerSellerTrustResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_together

> crate::models::GetBuyerTogetherResponse get_buyer_together(code)
A shop-together room: members, the board with votes, and chat.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerTogetherResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_vault_market_history

> crate::models::GetBuyerVaultMarketHistoryResponse get_buyer_vault_market_history()
Your Vault Markets value by day at the marks (oldest first, today live) over `days` (default 90), and today’s change against yesterday’s close.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerVaultMarketHistoryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_vault_market_index

> crate::models::GetBuyerVaultMarketIndexResponse get_buyer_vault_market_index(slug)
One index with its published daily closes (oldest first, `days` default 365) and its level now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerVaultMarketIndexResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_vault_market_instant_exit

> crate::models::GetBuyerVaultMarketInstantExitResponse get_buyer_vault_market_instant_exit(id)
The Instant Exit quote for one vault unit you hold, and whether it can be sold now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerVaultMarketInstantExitResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_vault_market_worth

> crate::models::GetBuyerVaultMarketWorthResponse get_buyer_vault_market_worth()
What your vault is worth now: value at the market prices (marks come only from trades) split into reliable and indicative marks, cost and P&L, today’s change, and an estimate of what the units free to sell would fetch from the bids resting on the book now. Read only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBuyerVaultMarketWorthResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_activity

> crate::models::ListBuyerActivityResponse list_buyer_activity()
What this buyer has compared lately.

Newest first, and only as far back as the retention window — the link between a person and a comparison is dropped after 180 days, so this thins out rather than growing forever.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerActivityResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_auction_bids

> crate::models::ListBuyerAuctionBidsResponse list_buyer_auction_bids(slug)
An auction’s bid history, highest first, bidders anonymised.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The auction listing slug. | [required] |

### Return type

**crate::models::ListBuyerAuctionBidsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_auctions

> crate::models::ListBuyerAuctionsResponse list_buyer_auctions(tab)
Auctions you have bid in — bidding, won or lost (last 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tab** | Option<**String**> | bidding = still running; won = you are the winner; lost = ended, someone else won. |  |[default to bidding]

### Return type

**crate::models::ListBuyerAuctionsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_boards

> crate::models::ListBuyerBoardsResponse list_buyer_boards()
Your outfit boards: collages of listings, each with a share link.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerBoardsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_breaks

> crate::models::BuyerResponse3 list_buyer_breaks()
Card-break spots you hold or bought, with what each pulled.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_buybacks

> crate::models::BuyerResponse3 list_buyer_buybacks()
Your buyback guarantees and requests.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_cart

> crate::models::ListBuyerCartResponse list_buyer_cart()
What is in your Crossly cart.

Line items with the price captured when each was added. This is NOT a quote — shipping, tax and any discounts are computed at checkout against a delivery address, and the sum of these lines is not what you will be charged.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerCartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_cart_saved

> crate::models::BuyerCartResponse list_buyer_cart_saved()
Items you saved for later out of your cart (last 200).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerCartResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_cases

> crate::models::ListBuyerCasesResponse list_buyer_cases(status)
Resolution center — every case you opened (returns, not received, problems).

`yourTurn: true` means the case is waiting on you. `meta.counts` covers every case, not just this status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | open = still being worked; closed = resolved or withdrawn; all = both. |  |[default to open]

### Return type

**crate::models::ListBuyerCasesResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_cashback

> crate::models::ListBuyerCashbackResponse list_buyer_cashback(status, page, limit)
Your Scout cashback — pending, confirmed, paid.

Newest first. `pending` means an order was reported and the retailer's return window has not closed; nothing is paid until it does. `expired` means a click was never reported as converting, which is the ordinary outcome for most clicks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**crate::models::ListBuyerCashbackResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_catalog_listing_videos

> crate::models::ListBuyerCatalogListingVideosResponse list_buyer_catalog_listing_videos(slug)
Videos the seller recorded for this item on request, newest first, with the question each one answers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerCatalogListingVideosResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_clubs

> crate::models::ListBuyerClubsResponse list_buyer_clubs()
Sellers’ loyalty clubs you belong to: your spend with each, your tier and its discount, and the next tier.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerClubsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_collection_sets

> crate::models::BuyerResponse3 list_buyer_collection_sets()
Sets you are collecting — a catalogue brand + model you own at least two pieces of — with owned / total and percent complete, most complete first; and whether you show on set leaderboards.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_consolidations

> crate::models::BuyerResponse3 list_buyer_consolidations()
Your ship-together orders: parcels at the hub, and the one box out.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_conversation_guests

> crate::models::ListBuyerWishlistMembersResponse list_buyer_conversation_guests(id)
Guests in a chat.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerWishlistMembersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_feedback

> crate::models::ListBuyerFeedbackResponse list_buyer_feedback()
Feedback you have left sellers, newest first — with the ids you revise by.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerFeedbackResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_feedback_pending

> crate::models::ListBuyerFeedbackPendingResponse list_buyer_feedback_pending()
Orders from the last 60 days that are on their way or arrived and still need your feedback.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerFeedbackPendingResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_fit_refs

> crate::models::BuyerResponse3 list_buyer_fit_refs()
Your saved reference garments for fit comparison.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_following_sellers

> crate::models::ListBuyerFollowingSellersResponse list_buyer_following_sellers()
Sellers you follow, with how many new listings each has since you last looked.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerFollowingSellersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_gift_card_credit

> crate::models::ListBuyerGiftCardCreditResponse list_buyer_gift_card_credit()
Gift card credit on your account, and when it expires.

One row per gift card you redeemed. `meta.giftCreditCents` is the unspent, unexpired gift credit in your Crossly Bucks balance; `meta.nextExpiresAt` is the soonest date any of it lapses. Spending at checkout uses gift credit first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerGiftCardCreditResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_gift_cards

> crate::models::ListBuyerGiftCardsResponse list_buyer_gift_cards()
Gift cards you bought, newest first.

Status, recipient and delivery for each card you paid for. The code is masked unless you sent the card to your own email address. Abandoned (unpaid) checkouts are omitted.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerGiftCardsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_gift_finder

> crate::models::ListBuyerGiftFinderResponse list_buyer_gift_finder()
Live listings for a recipient, interest and budget.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerGiftFinderResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_gift_sent

> crate::models::ListBuyerGiftSentResponse list_buyer_gift_sent()
Items you sent as gifts: whether the teaser went out, whether it arrived and was opened, and any thank-you. Send one with `gift` on checkout.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerGiftSentResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_hotspots

> crate::models::ListBuyerHotspotsResponse list_buyer_hotspots()
Shop-the-look tags on a story or lookbook photo: each tagged listing and where it sits on the image (x, y as fractions).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerHotspotsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_hunts

> crate::models::ListBuyerHuntsResponse list_buyer_hunts()
Your buyer-agent hunts and what they did.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerHuntsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_instant_offers

> crate::models::BuyerResponse3 list_buyer_instant_offers()
Your instant offers: live quotes, accepted ones awaiting a label, approved ones (with the prepaid label) and paid ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_layaways

> crate::models::ListBuyerLayawaysResponse list_buyer_layaways()
Your layaways: price, paid so far, what is left, and the installment schedule. Put an item on layaway at checkout with layawayItemIds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerLayawaysResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_listing_quick_questions

> crate::models::ListBuyerListingQuickQuestionsResponse list_buyer_listing_quick_questions(slug)
One-tap questions this listing supports; send one as `quick` on a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerListingQuickQuestionsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_listing_seller_reviews

> crate::models::ListBuyerListingSellerReviewsResponse list_buyer_listing_seller_reviews(slug)
The seller’s public reviews: summary, text, photos, replies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerListingSellerReviewsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_live_shipments

> crate::models::ListBuyerLiveShipmentsResponse list_buyer_live_shipments()
Your one-box-per-show shipments: every order won in a show ships together, and postage is charged once (the rest refunded when it ships).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerLiveShipmentsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_live_stream_lineup

> crate::models::ListBuyerLiveStreamLineupResponse list_buyer_live_stream_lineup(id)
What a live show will sell, in order: each item, its price, and whether it is still available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerLiveStreamLineupResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_market_bids

> crate::models::BuyerResponse3 list_buyer_market_bids()
Your resting market bids.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_market_watches

> crate::models::BuyerResponse3 list_buyer_market_watches()
Market variants you watch, with the price you are waiting for.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_offers

> crate::models::ListBuyerOffersResponse list_buyer_offers(awaiting_buyer)
Offers you have made, newest first (last 100).

`awaitingBuyer: true` is a seller counter waiting on you — answer it with POST /v1/buyer/offers/{id}/respond. `redeemable: true` is an accepted offer you can pay for on Crossly. Amounts are integer cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**awaiting_buyer** | Option<**String**> | true returns only seller counters waiting on you. |  |

### Return type

**crate::models::ListBuyerOffersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_orders

> crate::models::ListBuyerOrdersResponse list_buyer_orders(page, limit)
What you have bought on Crossly, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**crate::models::ListBuyerOrdersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_pack_openings

> crate::models::ListBuyerPackOpeningsResponse list_buyer_pack_openings()
Your pulls: what you got, its value, sell-back status, and the fairness proof for each (the server seed once rotated).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerPackOpeningsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_packs

> crate::models::ListBuyerPacksResponse list_buyer_packs()
Packs you can open: price, every tier with its published odds, what is left in it and its value range, and the expected value of one opening.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerPacksResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_parties

> crate::models::ListBuyerPartiesResponse list_buyer_parties()
Themed shopping parties, live and upcoming.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerPartiesResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_party_listings

> crate::models::BuyerResponse3 list_buyer_party_listings(slug)
Listings shared to a party, featured first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_pickup_points

> crate::models::BuyerResponse3 list_buyer_pickup_points()
Carrier pickup points (UPS Access Point / FedEx Hold at Location) near an address; pass one as `pickupPoint` to POST /v1/buyer/checkout.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_protection_plans

> crate::models::ListBuyerProtectionPlansResponse list_buyer_protection_plans()
Your protection plans (bought at checkout with `protection`): status, term, coverage, contract id, and any refund.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerProtectionPlansResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_registries

> crate::models::ListBuyerRegistriesResponse list_buyer_registries()
Your gift registries.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerRegistriesResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_registry_gifts

> crate::models::ListBuyerRegistryGiftsResponse list_buyer_registry_gifts(id)
Your thank-you list: who bought what off the registry, and whether you have thanked them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerRegistryGiftsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_registry_search

> crate::models::ListBuyerRegistrySearchResponse list_buyer_registry_search()
Find a registry by its name or the registrant’s name. Current and recent registries only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerRegistrySearchResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_rentals

> crate::models::ListBuyerRentalsResponse list_buyer_rentals()
Your rentals: dates, status (reserved, out, returned, claimed, closed, cancelled), the return label, and what happened to the deposit.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerRentalsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_size_memory_prompts

> crate::models::ListBuyerSizeMemoryPromptsResponse list_buyer_size_memory_prompts()
Delivered orders with a size that you haven’t said how they fit (the last 60 days).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerSizeMemoryPromptsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_size_memory_sizes

> crate::models::ListBuyerSizeMemorySizesResponse list_buyer_size_memory_sizes()
The sizes learned from what you bought and kept, by kind of item (shoes, tops, bottoms, dresses, outerwear) and department.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerSizeMemorySizesResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_splits

> crate::models::BuyerResponse3 list_buyer_splits()
Purchases you split, with each share’s status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::BuyerResponse3**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_subscriptions

> crate::models::ListBuyerSubscriptionsResponse list_buyer_subscriptions()
Your subscribe & save standing orders.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerSubscriptionsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_together

> crate::models::ListBuyerTogetherResponse list_buyer_together()
Shop-together rooms you are in.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerTogetherResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_total_requests

> crate::models::ListBuyerTotalRequestsResponse list_buyer_total_requests()
Your combined-shipping total requests, newest per seller.

A seller answers a request by quoting one shipping figure for every item of theirs in your cart. `matchesCart: false` means the cart changed since, so the quote will not apply at checkout.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerTotalRequestsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_try_ons

> crate::models::ListBuyerTryOnsResponse list_buyer_try_ons()
Your AI try-ons: the rendered images (labelled AI-generated wherever shown).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerTryOnsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_vault

> crate::models::ListBuyerVaultResponse list_buyer_vault()
What you own in the Crossly vault: stored (authenticated, on a shelf), listed for sale, or on its way home — with the price to ship one home.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVaultResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_vault_market_holdings

> crate::models::ListBuyerVaultMarketHoldingsResponse list_buyer_vault_market_holdings()
Each unit you hold in the vault at its market price, with the mark’s quality (A or B reliable, C indicative, none), cost, P&L and where it stands (stored, listed, lent, pledged, at the grader); `meta` carries the totals and the units you have out on loan.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVaultMarketHoldingsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_vault_market_indexes

> crate::models::ListBuyerVaultMarketIndexesResponse list_buyer_vault_market_indexes()
Crossly indexes over vault book keys (category, brand, curated, composite, segment, and your own), each with its level now and change since the last daily close. Index levels are valued only from the book’s marks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVaultMarketIndexesResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_vault_market_instant_exit

> crate::models::ListBuyerVaultMarketInstantExitResponse list_buyer_vault_market_instant_exit()
Instant Exit quotes for what you hold, one per book key: the best resting bid (yours left out), its share of the mark, the bid levels, units held and which are free to sell, and why a key cannot be sold instantly. A quote, not an order.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVaultMarketInstantExitResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_vault_market_watchlist

> crate::models::ListBuyerVaultMarketWatchlistResponse list_buyer_vault_market_watchlist()
The book keys you watch, with mark and its quality, 24 h and 7 d change, and the last 30 daily marks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVaultMarketWatchlistResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_video_requests

> crate::models::ListBuyerVideoRequestsResponse list_buyer_video_requests()
Your video requests and their answers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerVideoRequestsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_wishlist_items

> crate::models::ListBuyerWishlistItemsResponse list_buyer_wishlist_items(id)
What is on one wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerWishlistItemsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_wishlist_members

> crate::models::ListBuyerWishlistMembersResponse list_buyer_wishlist_members(id)
Co-editors of a shared wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListBuyerWishlistMembersResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_wishlists

> crate::models::ListBuyerWishlistsResponse list_buyer_wishlists()
Your wishlists.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerWishlistsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_board

> crate::models::BuyerResponse update_buyer_board(id)
Rename a board, change its description, or make it public / private.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_board_layout

> crate::models::BuyerBoardResponse update_buyer_board_layout(id)
Replace a board’s layout: items by platform listing id or slug, with x, y, w (0–1) and z. At most 40.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerBoardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_collection_set_leaderboard

> crate::models::UpdateBuyerCollectionSetLeaderboardResponse update_buyer_collection_set_leaderboard()
Show or hide yourself (by your public handle) on set leaderboards.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateBuyerCollectionSetLeaderboardResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_feedback

> crate::models::BuyerResponse update_buyer_feedback(id, update_buyer_feedback_request)
Revise feedback you left, within 60 days.

Replaces the stars and comment; detailed ratings you omit stay as they were.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**update_buyer_feedback_request** | **UpdateBuyerFeedbackRequest** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_hunt

> crate::models::BuyerResponse update_buyer_hunt(id)
Pause, resume or stop a hunt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_purchase_story

> crate::models::BuyerResponse update_buyer_purchase_story(id)
Write your owner story (280 characters); it travels with the item if you resell it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_registry

> crate::models::BuyerRegistryResponse update_buyer_registry(id)
Edit a registry: title, kind, date, message, ship-to address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerRegistryResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_registry_item

> crate::models::UpdateBuyerRegistryItemResponse update_buyer_registry_item(id, platform_listing_id)
How many of an item you want (1–50).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateBuyerRegistryItemResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_subscription

> crate::models::BuyerResponse update_buyer_subscription(id)
Pause, resume, skip the next delivery, change the interval, or cancel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::BuyerResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_buyer_wishlist

> crate::models::UpdateBuyerWishlistResponse update_buyer_wishlist(id, update_buyer_wishlist_request)
Rename a wishlist, or make it public / private.

A public list can be opened by anyone holding its share link; making it private again stops that link working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id. | [required] |
**update_buyer_wishlist_request** | **UpdateBuyerWishlistRequest** |  | [required] |

### Return type

**crate::models::UpdateBuyerWishlistResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

