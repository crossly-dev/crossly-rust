# \VaultMarketsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vault_lend**](VaultMarketsApi.md#create_vault_lend) | **POST** /v1/vault/{id}/lend | Turn lending on or off for a unit you own in the vault, optionally with your minimum borrow rate (APR in basis points). Returns `{ lendEnabled, minAprBps }`.
[**create_vault_market_account**](VaultMarketsApi.md#create_vault_market_account) | **POST** /v1/vault-markets/account | Open your account at the custodian partner (once). Returns `{ status }`; the custodian verifies it before it is active.
[**create_vault_market_admin_mark**](VaultMarketsApi.md#create_vault_market_admin_mark) | **POST** /v1/vault-markets/admin-marks | Upload your fund administrator's prices (`administrator`, `marks: [{ bookKey, day, cents }]`). Shown beside Crossly's marks and labelled as yours; they never change a Crossly figure.
[**create_vault_market_advance**](VaultMarketsApi.md#create_vault_market_advance) | **POST** /v1/vault-markets/advances | Take an advance against a vault unit that is listed for sale: paid into your custodian account, netted from the sale. Free for 60 days, then the pool rate. Needs a verified ID.
[**create_vault_market_advance_relist**](VaultMarketsApi.md#create_vault_market_advance_relist) | **POST** /v1/vault-markets/advances/{id}/relist | Relist an advance's unit at a new price (never so low its sale could not repay the advance).
[**create_vault_market_agent_draft_cancel**](VaultMarketsApi.md#create_vault_market_agent_draft_cancel) | **POST** /v1/vault-markets/agent/drafts/{id}/cancel | Throw a drafted order away (only while it is still a draft). Returns { ok, draft }.
[**create_vault_market_agent_watch**](VaultMarketsApi.md#create_vault_market_agent_watch) | **POST** /v1/vault-markets/agent/watches | Watch a book key: tell me once when its best bid, lowest ask, last trade or market price crosses a price (cents). Leave out direction for \"crosses\" (whichever way it has to move). Never trades. At most 50 live, each for 90 days.
[**create_vault_market_alert**](VaultMarketsApi.md#create_vault_market_alert) | **POST** /v1/vault-markets/alerts | Alert me when a key's market price goes above or below a price (fires once).
[**create_vault_market_alert_pref_test**](VaultMarketsApi.md#create_vault_market_alert_pref_test) | **POST** /v1/vault-markets/alert-prefs/test | Send yourself one test alert of a `kind` through your own switches. Returns `{ sent, kind, plan }`, where plan says per channel: send, off, quiet (held by quiet hours) or unavailable.
[**create_vault_market_auction_lot**](VaultMarketsApi.md#create_vault_market_auction_lot) | **POST** /v1/vault-markets/auctions/lots | Consign a stored, authenticated unit to the next weekly or premier event, with an optional opening price and reserve. The unit is held until the close. Returns `{ lotId, eventId, endsAt }`.
[**create_vault_market_auction_lot_bid**](VaultMarketsApi.md#create_vault_market_auction_lot_bid) | **POST** /v1/vault-markets/auctions/lots/{id}/bids | Leave or raise your maximum on a lot; the system bids for you up to it. A bid in the last 2 minutes extends the lot by 2 minutes. The winner's saved card is charged at the close through the order book. ID needed above $10,000.
[**create_vault_market_auction_lot_withdraw**](VaultMarketsApi.md#create_vault_market_auction_lot_withdraw) | **POST** /v1/vault-markets/auctions/lots/{id}/withdraw | Withdraw your lot before any bid; the unit goes back on your shelf. Returns `{ ok }`.
[**create_vault_market_block**](VaultMarketsApi.md#create_vault_market_block) | **POST** /v1/vault-markets/blocks | Place a block buy for an entity: one order booked to the first sub-account in `plan` (it pays from its custodian account), split across every sub-account in the plan at the average price, fees included, once it is done and the custodian confirmed each payment. Each share's cash moves between the entity's own custodian accounts; its units move only after that settles. Every sub-account must be one you may trade, and its share must fit its limits.
[**create_vault_market_block_allocate**](VaultMarketsApi.md#create_vault_market_block_allocate) | **POST** /v1/vault-markets/blocks/{orderId}/allocate | Allocate a done block now (the keeper does it too). Returns `{ outcome, block }`: allocated, waiting (a payment or transfer is not settled yet), nothing_filled or none.
[**create_vault_market_bullion_family_peg**](VaultMarketsApi.md#create_vault_market_bullion_family_peg) | **POST** /v1/vault-markets/bullion/family-pegs | Set a family-wide spot-pegged quote: one \"any 1 oz gold coin\" quote resting on each product book of a bullion family at the chosen condition (DS, VNDS or Used). When any leg fills it comes off the others and goes back on only for what is left; two fills at once are settled one at a time against what is left. A bid pays from your custodian account. Replaces your live family quote on the same family, condition and side. ID required.
[**create_vault_market_bullion_peg**](VaultMarketsApi.md#create_vault_market_bullion_peg) | **POST** /v1/vault-markets/bullion/pegs | Set a spot-pegged maker quote on a bullion book key: \"spot plus or minus N cents\" or \"plus or minus N basis points\", with an optional floor and cap. It rests as a pro limit order re-priced every spot tick and is pulled while spot is stale. Replaces your live quote on the same key and side. ID required.
[**create_vault_market_collection_worth_watch**](VaultMarketsApi.md#create_vault_market_collection_worth_watch) | **POST** /v1/vault-markets/collection-worth/watch | Watch every collection item that sits on a vault book key (up to the watchlist cap). Returns `{ watched, skipped }`.
[**create_vault_market_creator_attribution**](VaultMarketsApi.md#create_vault_market_creator_attribution) | **POST** /v1/vault-markets/creators/attribution | Use a creator code: the creator earns a capped share of the fee Crossly keeps on your Vault Markets trades for a limited window (paid by Crossly, never by you). Only for people new to Vault Markets, once, and never your own code.
[**create_vault_market_creator_cohost_invite_respond**](VaultMarketsApi.md#create_vault_market_creator_cohost_invite_respond) | **POST** /v1/vault-markets/creators/cohost-invites/{id}/respond | For creators: accept or decline an invite to co-host a Market Open launch show (`accept` true or false). Accepting needs your vault room verified and adds you to the show as a co-host; your room and the Commons then say you co-host it, with the creator-programme disclosure when it applies.
[**create_vault_market_credit_repay**](VaultMarketsApi.md#create_vault_market_credit_repay) | **POST** /v1/vault-markets/credit/{id}/repay | Repay from your custodian account, interest first. Omit `amountCents` to repay everything; pass `requestId` to make a retry safe. Repaid in full, the items are free.
[**create_vault_market_dcm_link**](VaultMarketsApi.md#create_vault_market_dcm_link) | **POST** /v1/vault-markets/dcm/link | Link a partner exchange account to your account (or to a firm sub-account you manage, `subaccountId`) so Crossly can show its positions: `accountRef` (the account id there) and `code` (the one-time link code it gave you). The partner must confirm the code; the code is never stored. Read-only: Crossly never trades on it.
[**create_vault_market_dcm_unlink**](VaultMarketsApi.md#create_vault_market_dcm_unlink) | **POST** /v1/vault-markets/dcm/unlink | Unlink the partner exchange account from your account (or the firm sub-account `subaccountId`): Crossly stops reading it and its hedges leave portfolio margin.
[**create_vault_market_dealer_apply**](VaultMarketsApi.md#create_vault_market_dealer_apply) | **POST** /v1/vault-markets/dealer/apply | Apply to be a dealer (ID check first). Ops review; approved dealers get raised limits and their bids show as dealer bids.
[**create_vault_market_dealer_bid**](VaultMarketsApi.md#create_vault_market_dealer_bid) | **POST** /v1/vault-markets/dealer/bids | Place a standing dealer bid: a resting GTC/GTD limit buy, flagged as a dealer bid. Approved dealers only.
[**create_vault_market_dealer_pool**](VaultMarketsApi.md#create_vault_market_dealer_pool) | **POST** /v1/vault-markets/dealer/pools | Open a curve pool on a key (approved dealers, ID checked): linear (delta in cents) or exponential (delta in bps), spot, fee, levels per side, units it may sell, and a buy budget from your custodian account. Its ladder goes up at once.
[**create_vault_market_dealer_pool_close**](VaultMarketsApi.md#create_vault_market_dealer_pool_close) | **POST** /v1/vault-markets/dealer/pools/{id}/close | Close a curve pool: its legs come off the book; what filled stays filled.
[**create_vault_market_dealer_pool_preview**](VaultMarketsApi.md#create_vault_market_dealer_pool_preview) | **POST** /v1/vault-markets/dealer/pools/preview | The bids and asks a set of curve settings would show, and why each side stops where it does. Places nothing.
[**create_vault_market_deposit**](VaultMarketsApi.md#create_vault_market_deposit) | **POST** /v1/vault-markets/deposits | Open a ship-in deposit for an ungraded catalogue item (`skuId`, `variantId`, `condition` DS / VNDS / Used). It is inbound until the vault receives and checks it, then stored in your name. Returns `{ vaultItem: { id, status } }`.
[**create_vault_market_donation**](VaultMarketsApi.md#create_vault_market_donation) | **POST** /v1/vault-markets/donations | Give a unit to Crossly Cares (once the charity can receive; otherwise its gate message). Returns `{ id, markCents }`.
[**create_vault_market_entity**](VaultMarketsApi.md#create_vault_market_entity) | **POST** /v1/vault-markets/entities | Open an entity account: you become its admin and the custodian partner starts KYB (its own flow; Crossly keeps only its reference). Needs a verified ID and the custodian (503 names what is missing).
[**create_vault_market_entity_ecp**](VaultMarketsApi.md#create_vault_market_entity_ecp) | **POST** /v1/vault-markets/entities/{id}/ecp | Admin: record the entity's attestation that it is an eligible contract participant. Raising its caps is still an ops decision.
[**create_vault_market_entity_halt**](VaultMarketsApi.md#create_vault_market_entity_halt) | **POST** /v1/vault-markets/entities/{id}/halts | Admin or risk: the firm kill switch. Halts the entity (no `subaccountId`) or one sub-account: open orders on it are cancelled and no new ones are placed until cleared.
[**create_vault_market_entity_halt_clear**](VaultMarketsApi.md#create_vault_market_entity_halt_clear) | **POST** /v1/vault-markets/entities/{id}/halts/{haltId}/clear | Admin or risk: lift a firm halt.
[**create_vault_market_entity_invitation**](VaultMarketsApi.md#create_vault_market_entity_invitation) | **POST** /v1/vault-markets/entities/{id}/invitation | Accept or decline an invitation (`accept`). Trading, admin and risk roles need a verified ID to accept.
[**create_vault_market_entity_kyb**](VaultMarketsApi.md#create_vault_market_entity_kyb) | **POST** /v1/vault-markets/entities/{id}/kyb | Refresh the entity's KYB status from the custodian (and its onboarding link while KYB is open).
[**create_vault_market_entity_member**](VaultMarketsApi.md#create_vault_market_entity_member) | **POST** /v1/vault-markets/entities/{id}/members | Admin: invite a Crossly user by email as admin, trader, risk, ops or viewer (a viewer can never place an order), with optional limits and sub-accounts.
[**create_vault_market_entity_subaccount**](VaultMarketsApi.md#create_vault_market_entity_subaccount) | **POST** /v1/vault-markets/entities/{id}/subaccounts | Admin: open a sub-account once the custodian approved the entity; its limits are held to the entity's caps.
[**create_vault_market_entity_subaccount_close**](VaultMarketsApi.md#create_vault_market_entity_subaccount_close) | **POST** /v1/vault-markets/entities/{id}/subaccounts/{subId}/close | Admin: close a sub-account that holds nothing and has no open orders.
[**create_vault_market_follow_copy_watch_alert_read**](VaultMarketsApi.md#create_vault_market_follow_copy_watch_alert_read) | **POST** /v1/vault-markets/follows/copy-watch/alerts/read | Mark every copy-watch alert read. Returns `{ marked }`.
[**create_vault_market_gift**](VaultMarketsApi.md#create_vault_market_gift) | **POST** /v1/vault-markets/gifts | Give a unit in your vault to another Crossly user (by @username or email). It stays on its shelf; they have 7 days to accept. ID needed above $1,000 at the mark. Returns `{ id, expiresAt }`.
[**create_vault_market_gift_accept**](VaultMarketsApi.md#create_vault_market_gift_accept) | **POST** /v1/vault-markets/gifts/{id}/accept | Accept a gift: the unit becomes yours in place. Returns `{ status }`.
[**create_vault_market_gift_cancel**](VaultMarketsApi.md#create_vault_market_gift_cancel) | **POST** /v1/vault-markets/gifts/{id}/cancel | Cancel a gift you sent before it is accepted. Returns `{ status }`.
[**create_vault_market_gift_decline**](VaultMarketsApi.md#create_vault_market_gift_decline) | **POST** /v1/vault-markets/gifts/{id}/decline | Decline a gift: the unit stays with the giver. Returns `{ status }`.
[**create_vault_market_grade_to_vault**](VaultMarketsApi.md#create_vault_market_grade_to_vault) | **POST** /v1/vault-markets/grade-to-vault | Start Grade to Vault on one item (`source` inventory or vault_unit). With `runId` the grading slot is booked at once (free; your saved card is charged only if the run locks). Nothing is listed until you confirm a price after the grade.
[**create_vault_market_grade_to_vault_book**](VaultMarketsApi.md#create_vault_market_grade_to_vault_book) | **POST** /v1/vault-markets/grade-to-vault/{id}/book | Book (or re-book) the grading slot of a flow still at \"picked\". Answers 503 naming GRADING_HUB_ADDRESS when runs are not open.
[**create_vault_market_grade_to_vault_cancel**](VaultMarketsApi.md#create_vault_market_grade_to_vault_cancel) | **POST** /v1/vault-markets/grade-to-vault/{id}/cancel | Stop the flow before the card goes to the grader (still picked, or its slot only reserved: nothing was charged).
[**create_vault_market_grade_to_vault_keep**](VaultMarketsApi.md#create_vault_market_grade_to_vault_keep) | **POST** /v1/vault-markets/grade-to-vault/{id}/keep | Keep the graded unit in your vault without listing it. The flow ends; list it any time from the vault.
[**create_vault_market_grade_to_vault_list**](VaultMarketsApi.md#create_vault_market_grade_to_vault_list) | **POST** /v1/vault-markets/grade-to-vault/{id}/list | Confirm the price for the grade the unit came back with (`confirm: true`, `gradeKey` must match the unit). Pre-trade risk, the price band and the ID line apply, then one shelf ask is placed on the Crossly order book.
[**create_vault_market_grade_to_vault_shipped**](VaultMarketsApi.md#create_vault_market_grade_to_vault_shipped) | **POST** /v1/vault-markets/grade-to-vault/{id}/shipped | You mailed the card to the grading hub (an inventory card, once its run locked).
[**create_vault_market_grading**](VaultMarketsApi.md#create_vault_market_grading) | **POST** /v1/vault-markets/grading | Send a raw vault unit on the grading hub's shelf to a Grading Run (`runId`). It is held until it comes back, graded, to the same vault item. The slot is free; your saved card is charged only if the run locks.
[**create_vault_market_grading_pull**](VaultMarketsApi.md#create_vault_market_grading_pull) | **POST** /v1/vault-markets/grading/{submissionId}/pull | Pull a vault unit out of its run before the run locks (nothing was charged). It goes back on the shelf.
[**create_vault_market_hall_loan**](VaultMarketsApi.md#create_vault_market_hall_loan) | **POST** /v1/vault-markets/hall/loans | Lend a unit in your Crossly vault to the Hall of Grails: { vaultItemId, credit: 'name' | 'anonymous' }. A standing loan of display rights only: the unit goes on a plinth, under a plate naming you (or \"a collector\"), in every weekly exhibition whose category it fits. It never moves and stays yours to list, sell or withdraw. Earns reputation per exhibition shown; no cash. Resumes a loan you opted out of while it still shows.
[**create_vault_market_hall_loan_credit**](VaultMarketsApi.md#create_vault_market_hall_loan_credit) | **POST** /v1/vault-markets/hall/loans/{id}/credit | Change how the plate on a lent unit's plinth names you, from now: { credit: 'name' } (your display name) or { credit: 'anonymous' } (\"a collector\").
[**create_vault_market_hall_loan_opt_out**](VaultMarketsApi.md#create_vault_market_hall_loan_opt_out) | **POST** /v1/vault-markets/hall/loans/{id}/opt-out | Opt a Hall of Grails loan out, at any time. A unit on a plinth this week finishes this exhibition (status ending, showingUntil); otherwise the loan ends now (status ended).
[**create_vault_market_identity_start**](VaultMarketsApi.md#create_vault_market_identity_start) | **POST** /v1/vault-markets/identity/start | Start (or resume) the one-time ID check the money features and larger orders need. Returns `{ url }` to open, or `{ alreadyVerified: true }`.
[**create_vault_market_index**](VaultMarketsApi.md#create_vault_market_index) | **POST** /v1/vault-markets/indexes | Make an index of your own: 1 to 50 book keys, weighted by mark or equally. Up to 10 per account.
[**create_vault_market_index_buy**](VaultMarketsApi.md#create_vault_market_index_buy) | **POST** /v1/vault-markets/indexes/{slug}/buy | Buy the basket: one limit buy per key in the index for 1-5 whole units each, at most `maxBpsOverMark` over each key's mark, IOC or GTC. Every unit is a real item that lands in your vault. Idempotent on `clientBasketId`; returns a result per key.
[**create_vault_market_index_complaint**](VaultMarketsApi.md#create_vault_market_index_complaint) | **POST** /v1/vault-markets/index-complaints | File a complaint about an index or the method (`slug` optional). Every complaint is answered. Returns `{ id, status, createdAt }`.
[**create_vault_market_index_consultation_comment**](VaultMarketsApi.md#create_vault_market_index_consultation_comment) | **POST** /v1/vault-markets/index-consultations/{id}/comments | Comment on an open consultation (5 to 5,000 characters; up to 5 per person). Comments are published without your name. Returns `{ ok }`.
[**create_vault_market_instant_exit**](VaultMarketsApi.md#create_vault_market_instant_exit) | **POST** /v1/vault-markets/instant-exit | Sell a unit now at the quoted price: a sell IOC at `priceCents` through the pro engine. Refused, with the new quote, if the best bid dropped below it. Crossly never buys or funds it.
[**create_vault_market_joint**](VaultMarketsApi.md#create_vault_market_joint) | **POST** /v1/vault-markets/joint | Open a joint vault (ID first): you hold full rights and accept the current joint terms. `invite` sends invitations (@username or email, each with view, trade or full rights). Returns `{ id, status, invited }`.
[**create_vault_market_joint_close**](VaultMarketsApi.md#create_vault_market_joint_close) | **POST** /v1/vault-markets/joint/{id}/close | Close the joint vault (full rights): open joint orders are cancelled and every unit goes back to its owner. Returns `{ status, unitsReturned }`.
[**create_vault_market_joint_invite**](VaultMarketsApi.md#create_vault_market_joint_invite) | **POST** /v1/vault-markets/joint/{id}/invite | Invite a Crossly user (full rights only). They have 14 days and show ID to join. Returns `{ userId, rights, expiresAt }`.
[**create_vault_market_joint_member_remove**](VaultMarketsApi.md#create_vault_market_joint_member_remove) | **POST** /v1/vault-markets/joint/{id}/members/{userId}/remove | Remove a member (full rights), or leave when userId is you. Their units go back to their own account and their joint orders are cancelled. Returns `{ userId, status, unitsReturned, jointStatus }`.
[**create_vault_market_joint_member_right**](VaultMarketsApi.md#create_vault_market_joint_member_right) | **POST** /v1/vault-markets/joint/{id}/members/{userId}/rights | Set a member rights to view, trade or full (full rights only; the vault always keeps a full member). Returns `{ userId, rights }`.
[**create_vault_market_joint_respond**](VaultMarketsApi.md#create_vault_market_joint_respond) | **POST** /v1/vault-markets/joint/{id}/respond | Accept (ID first; accepts the current joint terms) or decline an invitation. Returns `{ status }`.
[**create_vault_market_joint_term**](VaultMarketsApi.md#create_vault_market_joint_term) | **POST** /v1/vault-markets/joint/{id}/terms | Accept the current joint terms (needed to trade after the terms version changes). Returns `{ termsVersion }`.
[**create_vault_market_joint_trade**](VaultMarketsApi.md#create_vault_market_joint_trade) | **POST** /v1/vault-markets/joint/{id}/trades | Place a limit order for the joint vault (trade or full rights, ID shown). A buy (bookKey) is placed on you, paid from your own Vault Markets account, and what it buys is booked to the joint vault. A sell (vaultItemId of a unit in it) is placed on the unit owner of record, who gets the proceeds; pre-trade checks run against you too. Returns `{ tradeId, side, bookKey, ownerUserId, order }`.
[**create_vault_market_joint_trade_cancel**](VaultMarketsApi.md#create_vault_market_joint_trade_cancel) | **POST** /v1/vault-markets/joint/{id}/trades/{tradeId}/cancel | Cancel a working joint order (trade or full rights); what filled stays filled. Returns `{ tradeId, order }`.
[**create_vault_market_joint_unit**](VaultMarketsApi.md#create_vault_market_joint_unit) | **POST** /v1/vault-markets/joint/{id}/units | Put units you own into the joint vault (each stored and free). They keep you as owner of record and stay on the shelf. Returns `{ added, vaultItemIds }`.
[**create_vault_market_joint_unit_remove**](VaultMarketsApi.md#create_vault_market_joint_unit_remove) | **POST** /v1/vault-markets/joint/{id}/units/remove | Take units out of the joint vault, back to their owner own account (yours at any rights; anyone else with full rights). Returns `{ removed, vaultItemIds }`.
[**create_vault_market_large_trader_report_file**](VaultMarketsApi.md#create_vault_market_large_trader_report_file) | **POST** /v1/vault-markets/large-trader-reports/{id}/file | File (or amend, or refile) a large-trader report: why you hold it (purpose), who you act for, other accounts you control, whether you plan to add, and a statement of 20+ characters. Returns the report.
[**create_vault_market_learn_quiz**](VaultMarketsApi.md#create_vault_market_learn_quiz) | **POST** /v1/vault-markets/learn/{concept}/quiz | Answer a concept's quiz (`answers`: the chosen choice index per question). Graded on the server; 5 tries an hour.
[**create_vault_market_live_stream_featured_lot**](VaultMarketsApi.md#create_vault_market_live_stream_featured_lot) | **POST** /v1/vault-markets/live-streams/{id}/featured-lot | Feature an open lot on your live show (or `lotId: null` to take it off). Viewers hear LIVE_PIN_CHANGED. Returns `{ ok }`.
[**create_vault_market_loan**](VaultMarketsApi.md#create_vault_market_loan) | **POST** /v1/vault-markets/loans | Borrow against stored vault units: they are pledged (no listing, lending or withdrawal) and the market pool lends `amountCents` into your custodian account. Needs a verified ID.
[**create_vault_market_loan_recall**](VaultMarketsApi.md#create_vault_market_loan_recall) | **POST** /v1/vault-markets/loans/{id}/recall | Recall a lent unit: the short has 3 days to return one. Returns `{ recallDueAt }`.
[**create_vault_market_loan_unit**](VaultMarketsApi.md#create_vault_market_loan_unit) | **POST** /v1/vault-markets/loans/{id}/units | Pledge more stored units to a loan (answers a loan call, or makes room).
[**create_vault_market_loan_unit_release**](VaultMarketsApi.md#create_vault_market_loan_unit_release) | **POST** /v1/vault-markets/loans/{id}/units/{vaultItemId}/release | Free one pledged unit, if what is owed still fits inside the other units' limit.
[**create_vault_market_loan_unit_sell**](VaultMarketsApi.md#create_vault_market_loan_unit_sell) | **POST** /v1/vault-markets/loans/{id}/units/{vaultItemId}/sell | Sell a pledged unit to repay its loan: it is listed at `priceCents` (at least the sale floor) and stays pledged until it sells; the proceeds repay the loan first and the rest is yours. Calling it again re-prices it.
[**create_vault_market_locate**](VaultMarketsApi.md#create_vault_market_locate) | **POST** /v1/vault-markets/locates | Hold up to `qty` lendable units of `bookKey` for 15 minutes so a short can open against them (`termDays` 30/90 for a term loan). Needs lending (VAULT_LENDING_TERMS_VERSION) and the ID check. Returns the locate with how many units it holds.
[**create_vault_market_long**](VaultMarketsApi.md#create_vault_market_long) | **POST** /v1/vault-markets/longs | Buy `units` units (default 1; at most 25/10/3/1 by liquidity tier A/B/C/D) into your vault with leverage (up to 2x) from the market staking pool, paying up to `maxCents` each. A partial fill opens with the units bought and the rest is cancelled. Returns `{ positionId, matched, units }`.
[**create_vault_market_midpoint**](VaultMarketsApi.md#create_vault_market_midpoint) | **POST** /v1/vault-markets/midpoint | A hidden block order that trades only at the firm-book midpoint, with another midpoint order of at least your minimum size.
[**create_vault_market_midpoint_cancel**](VaultMarketsApi.md#create_vault_market_midpoint_cancel) | **POST** /v1/vault-markets/midpoint/{id}/cancel | Cancel a midpoint order.
[**create_vault_market_mm_dmm**](VaultMarketsApi.md#create_vault_market_mm_dmm) | **POST** /v1/vault-markets/mm/dmm | Apply to be the designated market maker on a tier B or C key. Ops approves; one per key.
[**create_vault_market_mm_leave**](VaultMarketsApi.md#create_vault_market_mm_leave) | **POST** /v1/vault-markets/mm/{id}/leave | Leave a market-maker programme.
[**create_vault_market_mm_lp**](VaultMarketsApi.md#create_vault_market_mm_lp) | **POST** /v1/vault-markets/mm/lp | Join a key as a Registered Liquidity Provider (ID needed). Meeting coverage earns the LP rates.
[**create_vault_market_mm_mass_cancel**](VaultMarketsApi.md#create_vault_market_mm_mass_cancel) | **POST** /v1/vault-markets/mm/mass-cancel | Cancel every open order (one key, or only quotes) and every resting leg in one statement: nothing fills between the first cancel and the last.
[**create_vault_market_mm_mmp_reset**](VaultMarketsApi.md#create_vault_market_mm_mmp_reset) | **POST** /v1/vault-markets/mm/mmp/reset | End a market-maker protection freeze early.
[**create_vault_market_opening_waitlist**](VaultMarketsApi.md#create_vault_market_opening_waitlist) | **POST** /v1/vault-markets/opening/{slug}/waitlist | Join a key's waitlist, optionally with a friend's invite code (`ref`). Inviting moves you up the queue; nobody is paid. Returns your place and your own code.
[**create_vault_market_p2p_loan_call**](VaultMarketsApi.md#create_vault_market_p2p_loan_call) | **POST** /v1/vault-markets/p2p/loans/{id}/call | Lender: call the loan (once it is a day old). A refinancing auction runs for about 30 hours, the rate a new lender gets climbing to the ceiling; repaid or refinanced, you are paid in full; neither by the end, the unit moves to your vault.
[**create_vault_market_p2p_loan_cancel_call**](VaultMarketsApi.md#create_vault_market_p2p_loan_cancel_call) | **POST** /v1/vault-markets/p2p/loans/{id}/cancel-call | Lender: take back your call before any other lender has taken the loan over. The refinancing auction stops and the loan carries on as before on its terms.
[**create_vault_market_p2p_loan_refi_auction**](VaultMarketsApi.md#create_vault_market_p2p_loan_refi_auction) | **POST** /v1/vault-markets/p2p/loans/{id}/refi-auction | Borrower: look for a cheaper lender. A refinancing auction runs for `hours` (1-72, default 24); the rate a new lender gets climbs from `startAprBps` (default half the loan rate) to just under the loan rate now. The first lender to take it pays your lender off and the loan carries on at that lower rate. Nobody by the end: nothing changes.
[**create_vault_market_p2p_loan_refi_auction_cancel**](VaultMarketsApi.md#create_vault_market_p2p_loan_refi_auction_cancel) | **POST** /v1/vault-markets/p2p/loans/{id}/refi-auction/cancel | Borrower: stop looking for a cheaper lender before anyone has taken the loan over. The loan carries on unchanged.
[**create_vault_market_p2p_loan_refinance**](VaultMarketsApi.md#create_vault_market_p2p_loan_refinance) | **POST** /v1/vault-markets/p2p/loans/{id}/refinance | Take over a loan in a refinancing auction (a lender call or a borrower search for a cheaper lender): you pay the old lender everything owed now (refused above `maxPayoffCents`), which becomes your loan to the same borrower at the auction rate now. Needs a verified ID, an active account and the current lending terms. Pass `requestId` to make a retry safe.
[**create_vault_market_p2p_loan_repay**](VaultMarketsApi.md#create_vault_market_p2p_loan_repay) | **POST** /v1/vault-markets/p2p/loans/{id}/repay | Borrower: repay from your custodian account to the lender. Without `amountCents`, in full (principal plus interest to the second; the unit is free once the custodian has it). With `amountCents` below what is owed, a part payment: interest first, then principal, at least the interest owed now and at least $1; the loan stays open on the principal left. Pass `requestId` to make a retry safe.
[**create_vault_market_p2p_offer**](VaultMarketsApi.md#create_vault_market_p2p_offer) | **POST** /v1/vault-markets/p2p/offers | Offer to lend: `principalCents` per loan at `aprBps` a year against an item (`bookKey`) or one unit (`vaultItemId`), for up to `maxLoans` loans, lasting `expiresInDays` (default 90). Nothing is held: the custodian pays from your own account when a borrower takes it. Needs a verified ID, an active account and the current lending terms.
[**create_vault_market_p2p_offer_cancel**](VaultMarketsApi.md#create_vault_market_p2p_offer_cancel) | **POST** /v1/vault-markets/p2p/offers/{id}/cancel | Withdraw one of your open offers. Loans already taken from it carry on.
[**create_vault_market_p2p_offer_take**](VaultMarketsApi.md#create_vault_market_p2p_offer_take) | **POST** /v1/vault-markets/p2p/offers/{id}/take | Borrow: take an offer against one of your stored vault units (`vaultItemId`). The unit is pledged (no listing, lending or withdrawal) and the principal is paid into your custodian account. Interest accrues by the second; repay any time. Pass `requestId` to make a retry safe.
[**create_vault_market_paper_mission_mark_check**](VaultMarketsApi.md#create_vault_market_paper_mission_mark_check) | **POST** /v1/vault-markets/paper/missions/mark-check | Answer the \"Read the mark\" mission: the quality letter (A, B or C) you read on a market page, checked against the stored mark. One answer per market, 10 a day. Returns `{ bookKey, answer, correct, quality, mission, next }`.
[**create_vault_market_paper_order**](VaultMarketsApi.md#create_vault_market_paper_order) | **POST** /v1/vault-markets/paper/orders | Place a paper order (play money only). It takes visible real liquidity now at book prices with the taker fee simulated; a GTC limit rests the rest with a queue-position estimate, filled later as the real tape reaches it. Sells only paper units you hold (no paper shorts). Never touches the real book. Returns `{ order, fills }`.
[**create_vault_market_paper_order_cancel**](VaultMarketsApi.md#create_vault_market_paper_order_cancel) | **POST** /v1/vault-markets/paper/orders/{id}/cancel | Cancel a working paper order: the play cash or paper units it held come back. Returns `{ order }`.
[**create_vault_market_paper_reset**](VaultMarketsApi.md#create_vault_market_paper_reset) | **POST** /v1/vault-markets/paper/reset | Reset your paper account to $10,000 of play money: working paper orders are cancelled, paper positions cleared, and your leaderboard week starts again. Once a day.
[**create_vault_market_partner_apply**](VaultMarketsApi.md#create_vault_market_partner_apply) | **POST** /v1/vault-markets/partners/apply | Apply as a Vault Drop-off Partner (a card shop that takes walk-ins for the vault). Needs the one-time ID check; ops approve.
[**create_vault_market_partner_batch**](VaultMarketsApi.md#create_vault_market_partner_batch) | **POST** /v1/vault-markets/partners/batches | An approved partner sends walk-ins in under its code: each item (catalogue variant, condition, grade, photos) becomes an inbound vault unit; the box gets one prepaid label. The partner earns a capped share of Crossly's fee on each unit's first sale.
[**create_vault_market_pool_stake**](VaultMarketsApi.md#create_vault_market_pool_stake) | **POST** /v1/vault-markets/pool/stakes | Stake CBX you hold at the custodian into the market pool (base units, as a string). Returns `{ stakeId, shares }`.
[**create_vault_market_pool_stake_unstake**](VaultMarketsApi.md#create_vault_market_pool_stake_unstake) | **POST** /v1/vault-markets/pool/stakes/{id}/unstake | Unstake: paid out after the 7-day cooldown. Returns `{ cooldownUntil }`.
[**create_vault_market_portfolio_margin_enroll**](VaultMarketsApi.md#create_vault_market_portfolio_margin_enroll) | **POST** /v1/vault-markets/portfolio-margin/enroll | Admin: put a firm sub-account under portfolio margin. Needs VAULT_LEVERAGE_VENUE, your ID, an ECP-attested entity that passed KYB, and the minimum equity. Its positions are then margined on the whole account, recalculated every 5 minutes; calls and auctions are unchanged. Audited.
[**create_vault_market_portfolio_margin_leave**](VaultMarketsApi.md#create_vault_market_portfolio_margin_leave) | **POST** /v1/vault-markets/portfolio-margin/leave | Admin: take a firm sub-account off portfolio margin. Each position is then held to its own tier floors again and may be called at once. Audited.
[**create_vault_market_portfolio_margin_what_if**](VaultMarketsApi.md#create_vault_market_portfolio_margin_what_if) | **POST** /v1/vault-markets/portfolio-margin/what-if | The portfolio margin requirement of a firm sub-account before and after a trade (`bookKey`, `side` buy or sell, `units`), at the Crossly mark with fees left out. Changes nothing.
[**create_vault_market_position_cancel**](VaultMarketsApi.md#create_vault_market_position_cancel) | **POST** /v1/vault-markets/positions/{id}/cancel | Cancel the working order: an unfilled opening order cancels the position; an unfilled cover or close leaves it open. Returns `{ ok }`.
[**create_vault_market_position_close**](VaultMarketsApi.md#create_vault_market_position_close) | **POST** /v1/vault-markets/positions/{id}/close | Close a leveraged long: all its units, or `units` of them, each sold for at least `minCents`; each sale repays the pool first.
[**create_vault_market_position_collateral**](VaultMarketsApi.md#create_vault_market_position_collateral) | **POST** /v1/vault-markets/positions/{id}/collateral | Answer a margin call: a short posts more collateral (USD, or CBX at half value); a leveraged long pays down its pool loan. Returns the position.
[**create_vault_market_position_cover**](VaultMarketsApi.md#create_vault_market_position_cover) | **POST** /v1/vault-markets/positions/{id}/cover | Cover a short: all its units, or `units` of them, each buying any unit of the key into the vault for up to `maxCents`; each goes back to its lender.
[**create_vault_market_price_widget**](VaultMarketsApi.md#create_vault_market_price_widget) | **POST** /v1/vault-markets/price-widgets | Make a price widget for one vault market (`kind` mark + `bookKey`) or one public index (`kind` index + `indexSlug`): range, layout (compact or chart), theme (auto, light, dark), optionally your own affiliate code for the link back and the sites allowed to show it (empty = any). Returns the widget with its snippets.
[**create_vault_market_price_widget_preview**](VaultMarketsApi.md#create_vault_market_price_widget_preview) | **POST** /v1/vault-markets/price-widgets/preview | What a widget would show before you save it: `view` (value, move, quality, n, as-of, link back) and `html`, the exact page the iframe serves. Nothing is stored.
[**create_vault_market_pro_bracket**](VaultMarketsApi.md#create_vault_market_pro_bracket) | **POST** /v1/vault-markets/pro/brackets | A buy order with a take-profit and/or stop-loss attached; the exits arm as an OCO pair for what the entry fills.
[**create_vault_market_pro_link_cancel**](VaultMarketsApi.md#create_vault_market_pro_link_cancel) | **POST** /v1/vault-markets/pro/links/{id}/cancel | Cancel a linked order: its open child orders come off; a bracket's entry stays.
[**create_vault_market_pro_oco**](VaultMarketsApi.md#create_vault_market_pro_oco) | **POST** /v1/vault-markets/pro/oco | Two orders on one key, one cancels the other: when one fills or its stop triggers, the other is cancelled (a partial fill shrinks it).
[**create_vault_market_pro_order**](VaultMarketsApi.md#create_vault_market_pro_order) | **POST** /v1/vault-markets/pro/orders | Place an order on a vault book: limit, market (protected, IOC), stop or stop-limit; GTC, GTD, IOC or FOK; post-only; an iceberg `displayQuantity`; a TWAP or VWAP algo over a window. Idempotent on `clientOrderId`. `subaccountId` trades for a firm sub-account you may trade (your role, its KYB, its limits, no firm halt; buys pay from its custodian account and land in it).
[**create_vault_market_pro_order_cancel**](VaultMarketsApi.md#create_vault_market_pro_order_cancel) | **POST** /v1/vault-markets/pro/orders/{id}/cancel | Cancel an order; what filled stays filled.
[**create_vault_market_pro_order_cancel_all**](VaultMarketsApi.md#create_vault_market_pro_order_cancel_all) | **POST** /v1/vault-markets/pro/orders/cancel-all | Cancel every open order, or every one on a book key. Returns `{ cancelled }`.
[**create_vault_market_pro_order_replace**](VaultMarketsApi.md#create_vault_market_pro_order_replace) | **POST** /v1/vault-markets/pro/orders/{id}/replace | Amend an order's price, stop or quantity (cancel-replace: it re-queues at its new price).
[**create_vault_market_pro_quote**](VaultMarketsApi.md#create_vault_market_pro_quote) | **POST** /v1/vault-markets/pro/quotes | Market makers: set a two-sided post-only quote on a book key (replaces the last). With cancel-on-disconnect (default) heartbeat every 30 s or it is pulled.
[**create_vault_market_pro_quote_heartbeat**](VaultMarketsApi.md#create_vault_market_pro_quote_heartbeat) | **POST** /v1/vault-markets/pro/quotes/heartbeat | Keep your quotes alive (cancel-on-disconnect). Returns `{ quotes, at }`.
[**create_vault_market_pro_trailing_stop**](VaultMarketsApi.md#create_vault_market_pro_trailing_stop) | **POST** /v1/vault-markets/pro/trailing-stops | A trailing stop: its stop follows the best price since it was placed, `trailBps` or `trailCents` behind (under the high for a sell, over the low for a buy).
[**create_vault_market_profile_follow**](VaultMarketsApi.md#create_vault_market_profile_follow) | **POST** /v1/vault-markets/profiles/{handle}/follow | Follow someone with a public vault: their new vault buys appear in your feed. No copy trading.
[**create_vault_market_recurring**](VaultMarketsApi.md#create_vault_market_recurring) | **POST** /v1/vault-markets/recurring | Start a recurring buy: one whole unit of a key every week, two weeks or month, at no more than `maxPriceCents` and/or the mark plus `capBpsOverMark`; card (with an optional backup) or custodian; optional auto-lend.
[**create_vault_market_rfq**](VaultMarketsApi.md#create_vault_market_rfq) | **POST** /v1/vault-markets/rfq | Ask for a quote on a block: dealers, the key's DMM and LPs, and holders are told, anonymously. Answers are firm for 60 s. Your limit is never shown.
[**create_vault_market_rfq_accept**](VaultMarketsApi.md#create_vault_market_rfq_accept) | **POST** /v1/vault-markets/rfq/{id}/accept | Take one answer: it fills as a block through the normal fill path (fees by role, self-trade prevention, halts, LULD).
[**create_vault_market_rfq_cancel**](VaultMarketsApi.md#create_vault_market_rfq_cancel) | **POST** /v1/vault-markets/rfq/{id}/cancel | Cancel your open quote request.
[**create_vault_market_rfq_respond**](VaultMarketsApi.md#create_vault_market_rfq_respond) | **POST** /v1/vault-markets/rfq/{id}/respond | Answer a quote request for the full quantity with a firm price, good for 60 s. Selling needs the units free in your vault; buying pays from your Vault Markets custodian account.
[**create_vault_market_rfq_response_withdraw**](VaultMarketsApi.md#create_vault_market_rfq_response_withdraw) | **POST** /v1/vault-markets/rfq/responses/{id}/withdraw | Withdraw your live answer to a quote request.
[**create_vault_market_set_goal**](VaultMarketsApi.md#create_vault_market_set_goal) | **POST** /v1/vault-markets/set-goals | Start a goal on a catalogue set (category, brandSlug, modelSlug from the set search), optionally with a target: one condition, and any grade, ungraded, or one exact grade. One live goal per set.
[**create_vault_market_set_goal_basket**](VaultMarketsApi.md#create_vault_market_set_goal_basket) | **POST** /v1/vault-markets/set-goals/{id}/basket | Place the \"complete my set\" basket bid: one GTC limit buy per missing piece, under a hard budget, paid by one of your saved cards or your custodian account. Every leg passes pre-trade risk and the ID line like any order. Idempotent on clientBasketId. One working basket per goal; its legs are cancelled once the set is complete.
[**create_vault_market_set_goal_basket_cancel**](VaultMarketsApi.md#create_vault_market_set_goal_basket_cancel) | **POST** /v1/vault-markets/set-goals/{id}/basket/cancel | Cancel the goal working basket bid: every working leg comes off the book; what already filled stays in your vault.
[**create_vault_market_share_card**](VaultMarketsApi.md#create_vault_market_share_card) | **POST** /v1/vault-markets/share-cards | Make a verified share card of a position, a vault unit, a public index or your portfolio (`kind`, `refId`). The % on it is the price move since your real fill; amounts stay hidden unless `showAmounts`.
[**create_vault_market_short**](VaultMarketsApi.md#create_vault_market_short) | **POST** /v1/vault-markets/shorts | Short a book key: borrow `units` lent units (default 1; at most 25/10/3/1 by liquidity tier A/B/C/D) and sell each at `limitCents` or better, posting collateral for all of them (50% of the mark each; CBX counts at half, up to 30%). All units are borrowed or none unless `allowPartial`. A partial sale opens with the units sold. Returns `{ positionId, matched, units }`.
[**create_vault_market_supply_batch_label**](VaultMarketsApi.md#create_vault_market_supply_batch_label) | **POST** /v1/vault-markets/supply/batches/{id}/label | Buy a box's prepaid label again after it failed (for example once a ship-from address is saved). A box that has a label keeps it.
[**create_vault_market_supply_invite**](VaultMarketsApi.md#create_vault_market_supply_invite) | **POST** /v1/vault-markets/supply/invites/{id} | Accept or decline a Market Open invite (`accept`).
[**create_vault_market_supply_send**](VaultMarketsApi.md#create_vault_market_supply_send) | **POST** /v1/vault-markets/supply/send | Send inventory items matched to a catalogue variant to the vault as one box: each becomes an inbound vault unit, and the box gets one prepaid inbound label (a PDF to print). Optional: auto-list each unit in the vault book at a share of the mark once stored, and lend when idle. Items that cannot go say why. Crosslisted listings stay live.
[**create_vault_market_term**](VaultMarketsApi.md#create_vault_market_term) | **POST** /v1/vault-markets/terms | Accept the current lending terms (needed to lend or short). Returns `{ termsVersion }`.
[**create_vault_market_tod_claim_object**](VaultMarketsApi.md#create_vault_market_tod_claim_object) | **POST** /v1/vault-markets/tod/claims/{id}/object | Stop a transfer-on-death claim made on your own account. Returns `{ id, status, estateUnlocked }`.
[**create_vault_market_tod_my_claim**](VaultMarketsApi.md#create_vault_market_tod_my_claim) | **POST** /v1/vault-markets/tod/my-claims | Start a claim as a named beneficiary (ID first), naming the account holder by @username or email. It stays a draft until you add documents and submit. Returns `{ id, status, existing }`.
[**create_vault_market_tod_my_claim_document**](VaultMarketsApi.md#create_vault_market_tod_my_claim_document) | **POST** /v1/vault-markets/tod/my-claims/{id}/documents | Add one document to a draft claim (or one ops asked more of): kind, filename and the file as base64 (PDF, JPEG or PNG, up to 8 MB). Kept privately for ops review. Returns `{ id, kind, label, filename, bytes }`.
[**create_vault_market_tod_my_claim_submit**](VaultMarketsApi.md#create_vault_market_tod_my_claim_submit) | **POST** /v1/vault-markets/tod/my-claims/{id}/submit | Send a claim for ops review (it needs a death certificate). The account holder is told and can stop it. Returns `{ id, status }`.
[**create_vault_market_tod_my_claim_withdraw**](VaultMarketsApi.md#create_vault_market_tod_my_claim_withdraw) | **POST** /v1/vault-markets/tod/my-claims/{id}/withdraw | Withdraw a claim before it is released. Returns `{ id, status }`.
[**create_vault_market_tod_term**](VaultMarketsApi.md#create_vault_market_tod_term) | **POST** /v1/vault-markets/tod/terms | Accept the current transfer-on-death terms. Returns `{ termsVersion }`.
[**create_vault_market_unit_evidence_scan**](VaultMarketsApi.md#create_vault_market_unit_evidence_scan) | **POST** /v1/vault-markets/units/{id}/evidence/scans | Add a finished scan of your own (`captureId`, of the same catalogue item) to your vault unit. Shown as added by the owner, not as intake evidence.
[**create_vault_market_unit_sell_now**](VaultMarketsApi.md#create_vault_market_unit_sell_now) | **POST** /v1/vault-markets/units/{id}/sell-now | Sell one unit of this item now into the best bid (Instant Exit: an immediate-or-cancel pro order), never below `minCents`. Returns `{ status: filled | missed, filledCents, quote, order }`.
[**create_vault_market_valuation_statement**](VaultMarketsApi.md#create_vault_market_valuation_statement) | **POST** /v1/vault-markets/valuation-statements | Issue an insurance or appraisal statement for a day: frozen with the sha256 of its record, sealed in the Crossly audit chain, with a public verify link. preparedFor and policyRef are printed on it and never shown on verify. Moves no money.
[**create_vault_market_watchlist**](VaultMarketsApi.md#create_vault_market_watchlist) | **POST** /v1/vault-markets/watchlist | Watch a book key. Returns `{ bookKey, watching }`.
[**create_vault_market_widget_token**](VaultMarketsApi.md#create_vault_market_widget_token) | **POST** /v1/vault-markets/widget/tokens | Mint a widget token for one phone (`deviceName`, `installId`); minting again with the same pair replaces that phone token. The token reads the widget summary and nothing else, and is returned once. At most five phones.
[**create_vault_receipt_token_confirm**](VaultMarketsApi.md#create_vault_receipt_token_confirm) | **POST** /v1/vault/receipt-tokens/{id}/confirm | After your wallet sent the export: read the chain and mark the token live once its one token exists. Pass the `signature` your wallet returned to record it (kept only if it landed).
[**create_vault_receipt_token_export**](VaultMarketsApi.md#create_vault_receipt_token_export) | **POST** /v1/vault/receipt-tokens/export | Send one unit (`vaultItemId`) warehouse receipt to one of your verified Solana wallets (`wallet`) as a token, accepting the current terms (`acceptTerms`). Returns a base64 transaction already signed by the new mint and the receipts authority: your wallet pays the rent and must sign and send it within about two minutes, then call confirm. Nothing is minted until it does. Needs a verified ID; the unit must be stored, unpledged, not lent and not for sale.
[**create_vault_receipt_token_redeem**](VaultMarketsApi.md#create_vault_receipt_token_redeem) | **POST** /v1/vault/receipt-tokens/{id}/redeem | Redeem a receipt token held in one of your verified Solana wallets (`wallet`): returns an unsigned base64 burn your wallet signs and sends (it also closes the token account and returns its rent to you). Once it lands, call redeem/confirm and the unit is yours in the vault. Refused while the token is frozen.
[**create_vault_receipt_token_redeem_confirm**](VaultMarketsApi.md#create_vault_receipt_token_redeem_confirm) | **POST** /v1/vault/receipt-tokens/{id}/redeem/confirm | After your wallet sent the redeem: read the chain; once the token is burned the unit moves into your vault (if it was not yours already). Pass the `signature` to record it.
[**create_vault_receipt_token_wallet_allow**](VaultMarketsApi.md#create_vault_receipt_token_wallet_allow) | **POST** /v1/vault/receipt-tokens/wallets/allow | Put one of your verified Solana wallets (`wallet`) on the receipt-token allow-list so it can hold and receive receipt tokens. Returns a base64 transaction already signed by the receipts authority: the wallet pays the small rent (about 0.001 SOL) and must sign and send it within about two minutes. Needs a verified ID and a wallet that passes the sanctions screen.
[**delete_vault_market_agent_watch**](VaultMarketsApi.md#delete_vault_market_agent_watch) | **DELETE** /v1/vault-markets/agent/watches/{id} | Cancel a live price watch. Returns the watch.
[**delete_vault_market_alert**](VaultMarketsApi.md#delete_vault_market_alert) | **DELETE** /v1/vault-markets/alerts/{id} | Delete a price alert. Returns `{ ok }`.
[**delete_vault_market_bullion_family_peg**](VaultMarketsApi.md#delete_vault_market_bullion_family_peg) | **DELETE** /v1/vault-markets/bullion/family-pegs/{id} | Stop a family-wide quote: every leg order comes off the book; what filled stays filled.
[**delete_vault_market_bullion_peg**](VaultMarketsApi.md#delete_vault_market_bullion_peg) | **DELETE** /v1/vault-markets/bullion/pegs/{id} | Stop a spot-pegged quote: its resting order comes off the book; what filled stays filled.
[**delete_vault_market_dealer_band**](VaultMarketsApi.md#delete_vault_market_dealer_band) | **DELETE** /v1/vault-markets/dealer/bands | Stop a band on a key; its standing bid comes off.
[**delete_vault_market_entity_member**](VaultMarketsApi.md#delete_vault_market_entity_member) | **DELETE** /v1/vault-markets/entities/{id}/members/{memberId} | Admin: remove a member (or leave, yourself). An entity always keeps one admin.
[**delete_vault_market_follow_copy_watch**](VaultMarketsApi.md#delete_vault_market_follow_copy_watch) | **DELETE** /v1/vault-markets/follows/copy-watch/{handle} | Stop copy-watching a collector.
[**delete_vault_market_index**](VaultMarketsApi.md#delete_vault_market_index) | **DELETE** /v1/vault-markets/indexes/{slug} | Delete an index of your own. Returns `{ ok }`.
[**delete_vault_market_locate**](VaultMarketsApi.md#delete_vault_market_locate) | **DELETE** /v1/vault-markets/locates/{id} | Release a live locate before it lapses.
[**delete_vault_market_opening_waitlist**](VaultMarketsApi.md#delete_vault_market_opening_waitlist) | **DELETE** /v1/vault-markets/opening/{slug}/waitlist | Leave a key's waitlist.
[**delete_vault_market_price_widget**](VaultMarketsApi.md#delete_vault_market_price_widget) | **DELETE** /v1/vault-markets/price-widgets/{id} | Delete a widget. Pages that still paste it show that it is no longer available. Returns `{ id, archived }`.
[**delete_vault_market_pro_quote**](VaultMarketsApi.md#delete_vault_market_pro_quote) | **DELETE** /v1/vault-markets/pro/quotes | Pull your quote on a book key, or every quote. Returns `{ pulled }`.
[**delete_vault_market_profile_follow**](VaultMarketsApi.md#delete_vault_market_profile_follow) | **DELETE** /v1/vault-markets/profiles/{handle}/follow | Stop following someone.
[**delete_vault_market_recurring**](VaultMarketsApi.md#delete_vault_market_recurring) | **DELETE** /v1/vault-markets/recurring/{id} | Stop a recurring buy for good; its history stays.
[**delete_vault_market_set_goal**](VaultMarketsApi.md#delete_vault_market_set_goal) | **DELETE** /v1/vault-markets/set-goals/{id} | Remove a set goal. A working basket bid is cancelled first; what filled stays in your vault.
[**delete_vault_market_share_card**](VaultMarketsApi.md#delete_vault_market_share_card) | **DELETE** /v1/vault-markets/share-cards/{id} | Take a share card down: its link and image stop working. Returns `{ ok }`.
[**delete_vault_market_supply_round_up**](VaultMarketsApi.md#delete_vault_market_supply_round_up) | **DELETE** /v1/vault-markets/supply/round-up | Stop the round-up (its history stays; an armed buy is cancelled).
[**delete_vault_market_unit_evidence**](VaultMarketsApi.md#delete_vault_market_unit_evidence) | **DELETE** /v1/vault-markets/units/{id}/evidence/{evidenceId} | Take back a scan you added to your vault unit (intake evidence stays).
[**delete_vault_market_watchlist**](VaultMarketsApi.md#delete_vault_market_watchlist) | **DELETE** /v1/vault-markets/watchlist | Stop watching a book key (`?bookKey=`). Returns `{ bookKey, watching }`.
[**delete_vault_market_widget_token**](VaultMarketsApi.md#delete_vault_market_widget_token) | **DELETE** /v1/vault-markets/widget/tokens | Turn the widget off on every phone: revokes all your widget tokens. Returns `{ revoked }`, how many.
[**delete_vault_market_widget_token_by_id**](VaultMarketsApi.md#delete_vault_market_widget_token_by_id) | **DELETE** /v1/vault-markets/widget/tokens/{id} | Turn the widget off on one phone: revokes that widget token at once. Only a widget token can be revoked here.
[**get_vault_attestation**](VaultMarketsApi.md#get_vault_attestation) | **GET** /v1/vault/attestations | Vault attestations: independent physical counts, the insurance certificate (with a warning when insured value in the vault exceeds its aggregate or per-occurrence limit), SOC 1 Type II and SOC 2 reports. Each carries its signed document.
[**get_vault_market_account**](VaultMarketsApi.md#get_vault_market_account) | **GET** /v1/vault-markets/account | Which Vault Markets registrations are live (and the env each waits on), your custodian account, limits and balances.
[**get_vault_market_adl**](VaultMarketsApi.md#get_vault_market_adl) | **GET** /v1/vault-markets/adl | The insurance fund (booked and free to draw), the most one event may draw before auto-deleveraging (ADL) starts, the loss waterfall in order, and the latest ADL events (no user or position ids). Money in cents.
[**get_vault_market_adl_position**](VaultMarketsApi.md#get_vault_market_adl_position) | **GET** /v1/vault-markets/adl/positions/{id} | One of your positions with its ADL lights (0 once it has ended).
[**get_vault_market_advance_quote**](VaultMarketsApi.md#get_vault_market_advance_quote) | **GET** /v1/vault-markets/advances/quote | What an advance on a vault unit for sale could be: up to half of the lower of its mark and asking price, free for 60 days.
[**get_vault_market_agent_draft**](VaultMarketsApi.md#get_vault_market_agent_draft) | **GET** /v1/vault-markets/agent/drafts/{id} | One drafted order: item, side, units, limit (cents), the book when drafted, status, and the orderBody that places it.
[**get_vault_market_alert_pref**](VaultMarketsApi.md#get_vault_market_alert_pref) | **GET** /v1/vault-markets/alert-prefs | Your vault alert settings: every kind (price, fill, margin_call, adl, p2p_loan, auction, market_open, receipt_token, set_goal, follow, account), every channel (push, telegram, terminal, email, webhook) with whether it is set up, the full on/off grid, the locked switches (a critical kind always pushes) and quiet hours.
[**get_vault_market_auction_lot**](VaultMarketsApi.md#get_vault_market_auction_lot) | **GET** /v1/vault-markets/auctions/lots/{id} | One lot: the price showing, the next minimum bid, whether a reserve exists and is met, the end (extended by late bids), the bid history and your standing.
[**get_vault_market_auction_mine**](VaultMarketsApi.md#get_vault_market_auction_mine) | **GET** /v1/vault-markets/auctions/mine | Lots you consigned (with your reserve) and lots you bid on (with your maximum and whether you lead).
[**get_vault_market_block**](VaultMarketsApi.md#get_vault_market_block) | **GET** /v1/vault-markets/blocks/{orderId} | One block order: its order, plan lines (planned and allocated units, average price, cash share, status).
[**get_vault_market_bullion_for**](VaultMarketsApi.md#get_vault_market_bullion_for) | **GET** /v1/vault-markets/bullion/for | The bullion key families one book key belongs to, with spot for its metal as a labelled reference and its melt value at that spot. Empty families when the item is not keyed bullion.
[**get_vault_market_bullion_key**](VaultMarketsApi.md#get_vault_market_bullion_key) | **GET** /v1/vault-markets/bullion/keys/{slug} | One bullion key family: its catalogue products, every live book key with best bid, best ask, vault units and its mark (from the book only), live pegged quote counts, and spot as a labelled reference.
[**get_vault_market_bullion_peg_preview**](VaultMarketsApi.md#get_vault_market_bullion_peg_preview) | **GET** /v1/vault-markets/bullion/pegs/preview | The price a spot-pegged quote would rest at now (spot plus a premium in cents or basis points, clamped to floor and cap). No price when spot is stale or missing: such a quote waits off the book.
[**get_vault_market_call**](VaultMarketsApi.md#get_vault_market_call) | **GET** /v1/vault-markets/calls/{bookKey} | Recent call-auction uncrosses on a book key: kind (open, reopen, periodic, daily, buy-in, sell-off), clearing price in cents and units.
[**get_vault_market_capacity**](VaultMarketsApi.md#get_vault_market_capacity) | **GET** /v1/vault-markets/capacity | Capacity from the book: `bookKey` (+ `units`) gives ADV, depth within ±5% of the mark, vault stock, lendable units and days to liquidate at 20% of ADV; `index` + `notional` (cents) gives how much of the basket the asks fill now and the days the rest needs.
[**get_vault_market_channel**](VaultMarketsApi.md#get_vault_market_channel) | **GET** /v1/vault-markets/channels | Your Vault Markets channel switches: Telegram alerts and digest, the daily cap on vault nudges by push (default 5), whether a Telegram chat is linked, and how many nudges went out today.
[**get_vault_market_compare**](VaultMarketsApi.md#get_vault_market_compare) | **GET** /v1/vault-markets/compare | Up to 5 book keys and/or index slugs (`items`, comma-separated) as daily series on the same days, each rebased to 100.
[**get_vault_market_confirmation**](VaultMarketsApi.md#get_vault_market_confirmation) | **GET** /v1/vault-markets/confirmations/{tradeId} | One trade confirmation.
[**get_vault_market_confirmation_pdf**](VaultMarketsApi.md#get_vault_market_confirmation_pdf) | **GET** /v1/vault-markets/confirmations/{tradeId}/pdf | One trade confirmation as a PDF.
[**get_vault_market_correlation**](VaultMarketsApi.md#get_vault_market_correlation) | **GET** /v1/vault-markets/correlations | The correlation matrix of the sector indexes' weekly returns over `weeks` (default 52), pairwise-complete, with n per pair; a pair under 26 shared weeks has no number.
[**get_vault_market_creator_attribution**](VaultMarketsApi.md#get_vault_market_creator_attribution) | **GET** /v1/vault-markets/creators/attribution | The creator who brought you to Vault Markets, when one did: when it started and ends, whether it still earns, and the disclosure. Your prices and fees are the same either way.
[**get_vault_market_creator_dashboard**](VaultMarketsApi.md#get_vault_market_creator_dashboard) | **GET** /v1/vault-markets/creators/dashboard | For creators: whether you joined the programme, each of your codes with traders attributed and still earning (counts only), shares accrued, paid and voided (cents) and the latest shares.
[**get_vault_market_creator_programme**](VaultMarketsApi.md#get_vault_market_creator_programme) | **GET** /v1/vault-markets/creators/programme | The Vault Markets creator programme: the share of the fee Crossly keeps that a creator earns, the per-fill and lifetime caps (cents), the attribution window in days, the 2% net floor it sits under, and the disclosure words.
[**get_vault_market_creator_room**](VaultMarketsApi.md#get_vault_market_creator_room) | **GET** /v1/vault-markets/creators/room | For creators: your vault shelf room (applied, verified, rejected or paused) and whether your public vault profile is on, which the room needs.
[**get_vault_market_credit**](VaultMarketsApi.md#get_vault_market_credit) | **GET** /v1/vault-markets/credit/{id} | One loan or advance with its pledged units and its money log (draws, interest, repayments, sale proceeds).
[**get_vault_market_dcm**](VaultMarketsApi.md#get_vault_market_dcm) | **GET** /v1/vault-markets/dcm | The partner exchange (DCM) that lists and clears futures and options on Crossly indexes (needs VAULT_INDEX_DCM + VAULT_INDEX_DCM_API_URL + VAULT_INDEX_DCM_API_KEY), and its contracts as last read: perps, futures and options with the index each settles on, multiplier, tick, expiry and strike. Read-only: Crossly is the licensor and data provider, never the venue.
[**get_vault_market_dcm_contract**](VaultMarketsApi.md#get_vault_market_dcm_contract) | **GET** /v1/vault-markets/dcm/contracts/{symbol} | One partner contract: its spec, its funding prints (newest first; the partner computes and settles funding), annualised funding over the last 21 prints, and the Crossly index level it settles on.
[**get_vault_market_dcm_contract_book**](VaultMarketsApi.md#get_vault_market_dcm_contract_book) | **GET** /v1/vault-markets/dcm/contracts/{symbol}/book | The partner exchange live order book for a contract (cached 5 seconds), with mark, index, last, open interest, 24 h volume, spread in bps and the basis of its mark over the Crossly index in bps. Prices are index points.
[**get_vault_market_dcm_options_input**](VaultMarketsApi.md#get_vault_market_dcm_options_input) | **GET** /v1/vault-markets/dcm/options-inputs/{slug} | Options inputs for an index (indexes only, never a single item): Crossly realised volatility (raw and Geltner-desmoothed, from weekly returns), the forward (the partner future mark, else the Crossly close), Black-76 greeks on a strike x expiry grid and on every option the partner lists on the index. Model inputs, not quotes.
[**get_vault_market_dcm_position**](VaultMarketsApi.md#get_vault_market_dcm_position) | **GET** /v1/vault-markets/dcm/positions | The partner exchange account linked to your account (or to the firm sub-account `subaccountId`) and its positions, read-only (cached 15 seconds); index futures on category indexes count as portfolio-margin hedges for that account: contracts, side, entry and mark price, unrealised P&L and margin in cents, liquidation price and notional in cents. Not linked: `linked: false`.
[**get_vault_market_dealer**](VaultMarketsApi.md#get_vault_market_dealer) | **GET** /v1/vault-markets/dealer | Your dealer status (applied, approved, revoked) and your Instant Exit bands.
[**get_vault_market_dealer_pool**](VaultMarketsApi.md#get_vault_market_dealer_pool) | **GET** /v1/vault-markets/dealer/pools/{id} | One of your curve pools, with its ladder and live legs.
[**get_vault_market_digest_preview**](VaultMarketsApi.md#get_vault_market_digest_preview) | **GET** /v1/vault-markets/digest/preview | This week's vault digest as it would go out: units at the display marks (unmarked units counted, not valued), the like-for-like change from a week ago, offers on your units, watched keys that got a first mark, and one Learn card.
[**get_vault_market_entity**](VaultMarketsApi.md#get_vault_market_entity) | **GET** /v1/vault-markets/entities | The entities (funds, LLCs, trusts, SPVs) you belong to, and invitations waiting for you.
[**get_vault_market_entity_by_id**](VaultMarketsApi.md#get_vault_market_entity_by_id) | **GET** /v1/vault-markets/entities/{id} | One entity: members and roles, sub-accounts with their limits, firm halts, caps.
[**get_vault_market_estimate**](VaultMarketsApi.md#get_vault_market_estimate) | **GET** /v1/vault-markets/estimates/{bookKey} | The Crossly Estimate for a book key — a model, not a price. Method (own trades, index-adjusted last trade, grade ratio), cents with 68% and 95% bands, High/Medium/Low from the walk-forward backtest's FSD, and n. Hidden (`available: false`) under 50 backtested trades or when FSD > 0.30. Never used for margin, loans or settlement.
[**get_vault_market_execution_quality**](VaultMarketsApi.md#get_vault_market_execution_quality) | **GET** /v1/vault-markets/execution-quality | The quarterly execution-quality report (Rule 605 style): fill rates, speed, effective vs quoted spread and shortfall by order type and size, and the busiest keys. `quarter` like 2026-Q4; the current quarter is provisional.
[**get_vault_market_fee_quote**](VaultMarketsApi.md#get_vault_market_fee_quote) | **GET** /v1/vault-markets/fees/quote | Your fee for an order at a price: as maker (the order rests) and as taker (it fills at once), with the programme that sets your rate and any Crossly One trim. A buyer's fee is added to the price; a seller's comes out of the proceeds.
[**get_vault_market_fee_schedule**](VaultMarketsApi.md#get_vault_market_fee_schedule) | **GET** /v1/vault-markets/fees/schedule | The fee schedule by role: maker and taker rates for retail, 30-day volume, Registered LPs and DMMs, the CBX discount, the referral share and the 2% net floor Crossly always keeps.
[**get_vault_market_follow_sharing**](VaultMarketsApi.md#get_vault_market_follow_sharing) | **GET** /v1/vault-markets/follows/sharing | Your sharing switches: the public vault profile, new buys, units sent in, sales, Hall loans, bids and asks for copy-watchers, and the leaderboards. All off until you turn them on.
[**get_vault_market_gem_rate**](VaultMarketsApi.md#get_vault_market_gem_rate) | **GET** /v1/vault-markets/gem-rate/{bookKey} | The Crossly gem rate: of cards graded through Crossly, the share that came back at each grader's gem grade, for this item and its category, with a Wilson 95% band. 10 returns per grader or no number. Not a grader population report.
[**get_vault_market_grade_to_vault**](VaultMarketsApi.md#get_vault_market_grade_to_vault) | **GET** /v1/vault-markets/grade-to-vault/{id} | One Grade to Vault flow, brought up to date with its submission and unit.
[**get_vault_market_grade_to_vault_candidate**](VaultMarketsApi.md#get_vault_market_grade_to_vault_candidate) | **GET** /v1/vault-markets/grade-to-vault/candidates | What you can start Grade to Vault on: raw inventory matched to a catalogue item, and raw vault units on the grading hub shelf; plus whether Grading Runs are open.
[**get_vault_market_grading_signal**](VaultMarketsApi.md#get_vault_market_grading_signal) | **GET** /v1/vault-markets/grading-signal | For a raw book key: the marks of the same item's graded keys, the spread over raw, grading cost per grader, the margin tier grading lifts it to, your raw vault units it pays to grade, and where to act.
[**get_vault_market_hall_loan**](VaultMarketsApi.md#get_vault_market_hall_loan) | **GET** /v1/vault-markets/hall/loans | Your Hall of Grails desk: this week's exhibition, the rules (maxLive loans, shownPerHolder per exhibition, repPoints per exhibition shown), reputation (exhibitions finished), your loans (status active, ending or ended; onShowNow, showingUntil, exhibitions, the next exhibition it fits, the Lent to the Hall badge) and the vault units you could lend (canLend, or the blocker in words, and the next exhibition each fits). Display only.
[**get_vault_market_health**](VaultMarketsApi.md#get_vault_market_health) | **GET** /v1/vault-markets/health | Portfolio health of your vault holdings at Crossly marks: concentration by single item, category, grade and grader (shares, HHI on 0 to 10000, its band and effective number of holdings), value by liquidity tier, the stress grid (each category and everything down 10, 20 and 30 percent), value by mark quality, plain observations, and the held items a what-if can run on. Information only, not advice.
[**get_vault_market_health_what_if**](VaultMarketsApi.md#get_vault_market_health_what_if) | **GET** /v1/vault-markets/health/what-if | What if one held item moved by moveBps (-10000 to 10000 basis points of its mark): the item, your vault total, the item and its category share of the vault, and the single-item HHI, before and after. A scenario on the mark, not a price you could sell at.
[**get_vault_market_home_unit**](VaultMarketsApi.md#get_vault_market_home_unit) | **GET** /v1/vault-markets/home/units | \"Show my vault\": the vault units you hold, the ones standing in your Crossly World home first. Each: vaultItemId, title, grade, imageUrl, bookKey (its market), inHome, visitors (visitors to your home see it). Also your home (slug, privacy, path), max units one home shows, shown, visitorsSee and a note on who sees what. Your home is made on first read. No prices.
[**get_vault_market_index**](VaultMarketsApi.md#get_vault_market_index) | **GET** /v1/vault-markets/indexes/{slug} | One index with its daily closes (oldest first) and its level now.
[**get_vault_market_index_calendar**](VaultMarketsApi.md#get_vault_market_index_calendar) | **GET** /v1/vault-markets/index-calendar | The next four quarterly reviews: reference day (2nd Friday of Mar/Jun/Sep/Dec), announcement day, last close under the old membership and the effective day; June and December also review the Blue-chip 50.
[**get_vault_market_index_consultation**](VaultMarketsApi.md#get_vault_market_index_consultation) | **GET** /v1/vault-markets/index-consultations/{id} | One consultation with its proposal, versions, dates, outcome and the public comments.
[**get_vault_market_index_feed**](VaultMarketsApi.md#get_vault_market_index_feed) | **GET** /v1/vault-markets/index-feed/{slug} | The licensed index feed for one built-in index (needs VAULT_INDEX_DCM + VAULT_INDEX_FEED_SECRET): the level now (intraday from A/B marks, or the last close below 50% constituent coverage), the last close, the frozen 7-day TWAP settlement value and every constituent with its mark source, quality and trade count. Signed: `x-crossly-signature: t=<ms>,v1=<hex HMAC-SHA256(secret, t + \".\" + body)>`. Every read is logged for the licence. Crossly is the licensor and data feed only.
[**get_vault_market_index_feed_funding**](VaultMarketsApi.md#get_vault_market_index_feed_funding) | **GET** /v1/vault-markets/index-feed/{slug}/funding | Reference funding for a perpetual-style future on the index: your premium `premiumBps` (TWAP of (impact mid - index)/index) through F = P + clamp(I - P, ±0.05%) per 8 h, |F| capped at 0.5%/day, with the index level it is relative to. The partner DCM computes and settles funding; this is the reference math.
[**get_vault_market_index_feed_settlement**](VaultMarketsApi.md#get_vault_market_index_feed_settlement) | **GET** /v1/vault-markets/index-feed/{slug}/settlement | The frozen 7-day TWAP settlement series for a licensed index (written once a day, never restated), with the closes behind each value. Signed like the feed.
[**get_vault_market_index_level**](VaultMarketsApi.md#get_vault_market_index_level) | **GET** /v1/vault-markets/indexes/{slug}/levels | An index's daily closes (up to 5 years), oldest first.
[**get_vault_market_index_licence_usage**](VaultMarketsApi.md#get_vault_market_index_licence_usage) | **GET** /v1/vault-markets/index-licence/usage | Your licensed index feed reads over `days`, per index and endpoint (licence-usage report).
[**get_vault_market_index_methodology**](VaultMarketsApi.md#get_vault_market_index_methodology) | **GET** /v1/vault-markets/index-methodology | The index method in machine-readable form: version (semver), inputs, weightings and caps, the family and its screens, the engines and their publication gate, revisions and the settlement series, the calendar, governance and limitations.
[**get_vault_market_instant_exit**](VaultMarketsApi.md#get_vault_market_instant_exit) | **GET** /v1/vault-markets/instant-exit | The Instant Exit quote for one vault unit you hold: the best resting bid on its book key (anyone's, dealers' included), its share of the mark, the bid depth, and whether it can be taken now.
[**get_vault_market_joint**](VaultMarketsApi.md#get_vault_market_joint) | **GET** /v1/vault-markets/joint | Your joint vaults and the invitations waiting for you, with the gate (open, missing env), the joint terms in words and what each right allows. A JointVaultsHome object, not a collection.
[**get_vault_market_joint_by_id**](VaultMarketsApi.md#get_vault_market_joint_by_id) | **GET** /v1/vault-markets/joint/{id} | One joint vault (active members only): members and rights, the units in it at the mark with each owner of record, totals, recent joint trades, and the activity from the audit trail.
[**get_vault_market_key_stat**](VaultMarketsApi.md#get_vault_market_key_stat) | **GET** /v1/vault-markets/key-stats/{bookKey} | Everything derived about one book key in one read: the Crossly Estimate, the liquidity score, risk statistics and the gem rate. A part that cannot be read is `{ error }`.
[**get_vault_market_learn**](VaultMarketsApi.md#get_vault_market_learn) | **GET** /v1/vault-markets/learn | The Learn hub: each concept with whether you passed its quiz. Passing earns cosmetic pins only.
[**get_vault_market_learn_by_concept**](VaultMarketsApi.md#get_vault_market_learn_by_concept) | **GET** /v1/vault-markets/learn/{concept} | One Learn concept: its sections and quiz questions (no answers).
[**get_vault_market_lending_election**](VaultMarketsApi.md#get_vault_market_lending_election) | **GET** /v1/vault-markets/lending/election | Your lender election: `returnInKind` (shorts post 175% collateral, a 14-day buy-in, never cash-settled) and `allowTerm` (your units may go out on 30/90-day fixed-rate term loans, not recallable before term).
[**get_vault_market_lending_programme**](VaultMarketsApi.md#get_vault_market_lending_programme) | **GET** /v1/vault-markets/lending/programme | Your lender programme report: lendable units, units out, utilisation, fee revenue (accrued / paid), term loans and return-in-kind loans in force, and each loan's rate against the key's rate today.
[**get_vault_market_lending_statement**](VaultMarketsApi.md#get_vault_market_lending_statement) | **GET** /v1/vault-markets/lending/statement | Your daily collateral statement as a lender (today live, or `day`): per open loan the mark and its quality, fees owed to you, the short's collateral share at the custodian and its coverage against 105% of mark + fees. A unit without a mark usable for risk shows no coverage number.
[**get_vault_market_liquidity**](VaultMarketsApi.md#get_vault_market_liquidity) | **GET** /v1/vault-markets/liquidity/{bookKey} | The liquidity score (0-100) of a book key and every component behind it: trades in 30/90 days, distinct counterparties, median days between trades, time-weighted quoted spread, two-sided share and depth within 5% (hourly book snapshots), Amihud and Roll. Each component is a percentile within the category. No score under the data threshold.
[**get_vault_market_loan_quote**](VaultMarketsApi.md#get_vault_market_loan_quote) | **GET** /v1/vault-markets/loans/quote | How much you can borrow against vault units (`vaultItemIds`, comma-separated; none = every stored unit you hold): each unit's tier, mark and limit, the pool's rate and yours.
[**get_vault_market_loan_unit_sale_floor**](VaultMarketsApi.md#get_vault_market_loan_unit_sale_floor) | **GET** /v1/vault-markets/loans/{id}/units/{vaultItemId}/sale-floor | The lowest price a pledged unit can be sold at to repay its loan: after the sale (less the vault fee and storage) what is still owed must fit inside the other units' limit.
[**get_vault_market_locate**](VaultMarketsApi.md#get_vault_market_locate) | **GET** /v1/vault-markets/locates | A locate quote: how many units of `key` (a book key) you could borrow now (up to `qty`), the indicative rate (APR bps; `termDays` 30 or 90 for a fixed-rate term loan), and the collateral they need (150%, or 175% for units whose lenders elected return in kind only). Holds nothing; POST to hold.
[**get_vault_market_mark**](VaultMarketsApi.md#get_vault_market_mark) | **GET** /v1/vault-markets/marks/{bookKey} | One book key mark with everything behind it: the price in cents, its quality (A trade-anchored, B firm two-sided, C indicative, none), its confidence in bps and the band in cents, when it was computed, whether risk may act on it (judged for margin), the related key it was implied from (impliedFrom: a grade, the model median of the other sizes, or an index; null when none), every stored input, and the learned relations it may be implied from.
[**get_vault_market_market**](VaultMarketsApi.md#get_vault_market_market) | **GET** /v1/vault-markets/market | One book key (variantId:condition:gradeKey): its mark and history, lending depth and borrow rate, open interest and live auctions.
[**get_vault_market_market_model**](VaultMarketsApi.md#get_vault_market_market_model) | **GET** /v1/vault-markets/market-model/{bookKey} | How a book key trades: its liquidity tier (A continuous, B call auction every 15 minutes, C daily call, D listings only), the call collecting now with its indicative price and volume and nominal close, the next call, the LULD band and any limit state, pause or halt.
[**get_vault_market_mm**](VaultMarketsApi.md#get_vault_market_mm) | **GET** /v1/vault-markets/mm | Your market-maker programmes, the last 30 days of obligations, rebates earned (paid only from fees already taken), and the rules.
[**get_vault_market_mm_key**](VaultMarketsApi.md#get_vault_market_mm_key) | **GET** /v1/vault-markets/mm/key | A key's designated market maker (by the name it chose) and how many LPs quote it.
[**get_vault_market_mm_mmp**](VaultMarketsApi.md#get_vault_market_mm_mmp) | **GET** /v1/vault-markets/mm/mmp | Your market-maker protection: the fill trigger, the freeze, and whether you are frozen now.
[**get_vault_market_mode**](VaultMarketsApi.md#get_vault_market_mode) | **GET** /v1/vault-markets/mode | Your Vault Markets mode: simple (buy, sell and what it is worth) or pro (the full terminal), whether you chose it or it was set for you (new people start in simple; people already trading start in pro), and when.
[**get_vault_market_nav**](VaultMarketsApi.md#get_vault_market_nav) | **GET** /v1/vault-markets/nav | Mark NAV, bid NAV (the resting bids) and liquidation NAV (the bids at 20% of ADV for 10 days), holding by holding, each with its ASC 820 level, mark quality and source, and your administrator's price when uploaded.
[**get_vault_market_nav_file**](VaultMarketsApi.md#get_vault_market_nav_file) | **GET** /v1/vault-markets/nav/file/{format} | The NAV as a csv, pdf or json file.
[**get_vault_market_opening**](VaultMarketsApi.md#get_vault_market_opening) | **GET** /v1/vault-markets/opening/{slug} | One opening key.
[**get_vault_market_oracle**](VaultMarketsApi.md#get_vault_market_oracle) | **GET** /v1/vault-markets/oracle | Crossly as an oracle publisher (needs CROSSLY_ORACLE_PUBLISHER_KEY + CROSSLY_ORACLE_NETWORK): the network (pyth or switchboard), the ed25519 key every value is signed with, each published index feed with its latest attempt, and 24 h counts of published, skipped and failed. Only built-in indexes priced from A/B marks are published; Crossly never reads its own feed back.
[**get_vault_market_oracle_by_slug**](VaultMarketsApi.md#get_vault_market_oracle_by_slug) | **GET** /v1/vault-markets/oracle/{slug} | The latest published oracle value for an index: level, fixed-point price and confidence at the feed exponent, confidence in bps, staleness in seconds, coverage, the exact signed message and its ed25519 signature with the signer key, and whether it is fresh.
[**get_vault_market_order_handling**](VaultMarketsApi.md#get_vault_market_order_handling) | **GET** /v1/vault-markets/order-handling | How orders are handled (the 606/607-style disclosure): one book, self-trade prevention, protected market orders, liquidations by auction, halts, dealers and fees.
[**get_vault_market_p2p_dashboard**](VaultMarketsApi.md#get_vault_market_p2p_dashboard) | **GET** /v1/vault-markets/p2p/dashboard | Peer-to-peer vault loans at a glance: what you owe as a borrower and are owed as a lender (principal + interest to the second), daily interest, called loans, your open offers, the gates and the rules (auction hours, rate ceiling, size bounds, call grace).
[**get_vault_market_p2p_loan**](VaultMarketsApi.md#get_vault_market_p2p_loan) | **GET** /v1/vault-markets/p2p/loans/{id} | One P2P loan with its money log (fund, repay, refinance). A loan in a running refinancing auction can be read by any lender, without the log.
[**get_vault_market_paper**](VaultMarketsApi.md#get_vault_market_paper) | **GET** /v1/vault-markets/paper | Your paper trading account (play money only, never real or convertible; opened at $10,000 the first time): play cash, buying power, paper positions at the real marks, P&L, working paper orders, recent paper fills and this week on the leaderboard.
[**get_vault_market_paper_quote**](VaultMarketsApi.md#get_vault_market_paper_quote) | **GET** /v1/vault-markets/paper/quote | What a paper order would fill against the real book now, without placing it: units fillable inside the limit (a market order uses the 5% protection band around the mark), average price, simulated fee, and the queue ahead of any rest.
[**get_vault_market_partner_me**](VaultMarketsApi.md#get_vault_market_partner_me) | **GET** /v1/vault-markets/partners/me | Your Vault Drop-off Partnership (null when you never applied): status, partner code, fee share and cap, totals and batches.
[**get_vault_market_pool**](VaultMarketsApi.md#get_vault_market_pool) | **GET** /v1/vault-markets/pool | The CBX market staking pool: size, value, what it lends, utilisation, rate, insurance fund.
[**get_vault_market_pop_report**](VaultMarketsApi.md#get_vault_market_pop_report) | **GET** /v1/vault-markets/pop-reports | Grading population report for one market (`bookKey`): focus (on a graded market: its grade, pop = copies graded at exactly that grade, higher = copies graded better by the same grader, total, asOf and source), graders (each grader ladder for the card, best grade first, from one report: the newest, the population feed over a hand entry on the same date), provider.connected, and status (ok, not_synced, no_report or no_source) with explain, which says in plain words what is missing. source is provider (the population feed) or ops (entered by Crossly from the grader published report). Counts only, never a price.
[**get_vault_market_portfolio**](VaultMarketsApi.md#get_vault_market_portfolio) | **GET** /v1/vault-markets/portfolio | Your vault holdings at the market price, with cost and P&L, and units you have out on loan.
[**get_vault_market_portfolio_history**](VaultMarketsApi.md#get_vault_market_portfolio_history) | **GET** /v1/vault-markets/portfolio/history | Your Vault Markets value by day at the marks (vault units, units on loan, positions) and today's return against yesterday's close.
[**get_vault_market_portfolio_margin**](VaultMarketsApi.md#get_vault_market_portfolio_margin) | **GET** /v1/vault-markets/portfolio-margin | Portfolio margin for a firm sub-account (`subaccountId`): eligibility with every reason (ECP, KYB, minimum equity $500k or $100k with evidenced intraday monitoring), whether it is enrolled, the latest requirement, equity, excess and state (healthy, call or buy_in), the breakdown by category and item, the partner index futures counted, and the policy. Before enrolling, a live preview.
[**get_vault_market_position**](VaultMarketsApi.md#get_vault_market_position) | **GET** /v1/vault-markets/positions/{id} | One position with its loans, auctions and custodian instruction log.
[**get_vault_market_position_exit**](VaultMarketsApi.md#get_vault_market_position_exit) | **GET** /v1/vault-markets/positions/{id}/exits | A position's take-profit / stop-loss, if set.
[**get_vault_market_position_risk**](VaultMarketsApi.md#get_vault_market_position_risk) | **GET** /v1/vault-markets/positions/{id}/risk | One position's call and liquidation marks.
[**get_vault_market_press**](VaultMarketsApi.md#get_vault_market_press) | **GET** /v1/vault-markets/press | The press kit: each built-in index with level, quality, n, CSV link and a \"cite as\" line, plus the methodology links.
[**get_vault_market_price_widget**](VaultMarketsApi.md#get_vault_market_price_widget) | **GET** /v1/vault-markets/price-widgets/{id} | One of your price widgets, with its iframe and script-tag snippets and URLs.
[**get_vault_market_pro_depth**](VaultMarketsApi.md#get_vault_market_pro_depth) | **GET** /v1/vault-markets/pro/depth | The L2 depth ladder: price levels and units per side, best first; asks show how many units are authenticated vault stock.
[**get_vault_market_pro_order**](VaultMarketsApi.md#get_vault_market_pro_order) | **GET** /v1/vault-markets/pro/orders/{id} | One order with its full event log.
[**get_vault_market_pro_snapshot**](VaultMarketsApi.md#get_vault_market_pro_snapshot) | **GET** /v1/vault-markets/pro/snapshot | A book key's depth (20 levels a side), last 50 trades, 24 h stats, mark and any halt, in one call.
[**get_vault_market_pro_stat**](VaultMarketsApi.md#get_vault_market_pro_stat) | **GET** /v1/vault-markets/pro/stats | 24 h open, high, low, last, volume and change on a book key.
[**get_vault_market_pro_stream**](VaultMarketsApi.md#get_vault_market_pro_stream) | **GET** /v1/vault-markets/pro/stream | A long-lived text/event-stream for a book key: `snapshot` first, then `depth`, `trades`, `mark` and `halt` events as they change.
[**get_vault_market_profile**](VaultMarketsApi.md#get_vault_market_profile) | **GET** /v1/vault-markets/profile | Your public vault profile switch: on or off, whether followers see new buys, your profile URL and follower count.
[**get_vault_market_profile_by_handle**](VaultMarketsApi.md#get_vault_market_profile_by_handle) | **GET** /v1/vault-markets/profiles/{handle} | Someone's public vault: each market's weight (bps of the marked value), followers, whether you follow, their recent buys (no prices).
[**get_vault_market_proof_of_reserve**](VaultMarketsApi.md#get_vault_market_proof_of_reserve) | **GET** /v1/vault-markets/proof-of-reserves | Proof of reserves: the latest daily Merkle root over every unit in custody (leaf count, stored / listed / grading / withdrawing, pledged, on loan, open loans, receipts, insured value), its Solana memo anchor when VAULT_POR_ANCHOR_KEY is set, the last `days` roots, and the exact hashing method.
[**get_vault_market_proof_of_reserve_by_day**](VaultMarketsApi.md#get_vault_market_proof_of_reserve_by_day) | **GET** /v1/vault-markets/proof-of-reserves/{day} | One day's proof-of-reserves snapshot (root, counts, anchor).
[**get_vault_market_proof_of_reserve_me**](VaultMarketsApi.md#get_vault_market_proof_of_reserve_me) | **GET** /v1/vault-markets/proof-of-reserves/me | Your inclusion proofs for a day (latest by default): each of your units' leaf fields, salt, owner commitment, leaf hash and sibling path. Verify with your OWN user id: commitment = SHA-256(JSON[\"crossly-por-owner-v1\", userId, salt]); then the leaf; then climb the path to the published root.
[**get_vault_market_rate**](VaultMarketsApi.md#get_vault_market_rate) | **GET** /v1/vault-markets/rates | Daily borrow APR (lent units) and pool APR on a book key, with the pool as a whole, oldest first.
[**get_vault_market_report**](VaultMarketsApi.md#get_vault_market_report) | **GET** /v1/vault-markets/report | The quarterly Crossly Vault Report (`quarter` like 2026-q3; default the latest): index levels, top movers, most-lent markets, grade premiums. Book data only.
[**get_vault_market_rfq**](VaultMarketsApi.md#get_vault_market_rfq) | **GET** /v1/vault-markets/rfq/{id} | One quote request: the requester sees every answer (anonymous); a responder sees the request and their own answer.
[**get_vault_market_risk_report**](VaultMarketsApi.md#get_vault_market_risk_report) | **GET** /v1/vault-markets/risk-report | Historical VaR 95/99 1d/10d and Bangia LVaR (model figures: labelled, with n and a band, null below the data threshold), stress scenarios (category −30%, CBX −50%, books thin to 25%, index drawdowns), concentration and counterparty exposure.
[**get_vault_market_risk_stat**](VaultMarketsApi.md#get_vault_market_risk_stat) | **GET** /v1/vault-markets/risk-stats/{bookKey} | Risk statistics for a book key (weekly returns from A-quality marks only) or an index slug (its closes): annualised volatility raw and Geltner-desmoothed (labelled), max drawdown and beta to the Composite. Needs 26 weekly returns, else no numbers.
[**get_vault_market_set_goal**](VaultMarketsApi.md#get_vault_market_set_goal) | **GET** /v1/vault-markets/set-goals/{id} | One set goal: every piece in the set order, held (with the vault unit) or missing (with the cheapest marked book key that meets the target and its mark), the conditions and grades seen, and the working basket bid with its legs.
[**get_vault_market_set_goal_basket_plan**](VaultMarketsApi.md#get_vault_market_set_goal_basket_plan) | **GET** /v1/vault-markets/set-goals/{id}/basket-plan | What a basket bid would place now, without placing it: one leg per missing piece (cheapest first, the mark plus maxBpsOverMark as the limit), the pieces skipped and why, and the most it can cost (limits plus the worst-case buyer fee), never more than budgetCents.
[**get_vault_market_share_card**](VaultMarketsApi.md#get_vault_market_share_card) | **GET** /v1/vault-markets/share-cards/{id} | One share card as the public sees it, with its image and link paths.
[**get_vault_market_statement**](VaultMarketsApi.md#get_vault_market_statement) | **GET** /v1/vault-markets/statements | The last 24 months with whether each statement is issued (frozen, with its sha256).
[**get_vault_market_statement_by_month**](VaultMarketsApi.md#get_vault_market_statement_by_month) | **GET** /v1/vault-markets/statements/{month} | A monthly statement (`month` like 2026-10): units held at month end, loans, pledges, positions, fills, fees, storage, and cash quoted from the custodian and labelled as its record. Issued (frozen) once the month ends.
[**get_vault_market_statement_file**](VaultMarketsApi.md#get_vault_market_statement_file) | **GET** /v1/vault-markets/statements/{month}/file/{format} | A monthly statement as a csv, pdf or json file.
[**get_vault_market_supply**](VaultMarketsApi.md#get_vault_market_supply) | **GET** /v1/vault-markets/supply | A book key's units in the vault and authentications passed, by day (oldest first, today live).
[**get_vault_market_supply_batch**](VaultMarketsApi.md#get_vault_market_supply_batch) | **GET** /v1/vault-markets/supply/batches/{id} | A box sent to the vault: its prepaid label and tracking, or why it has no label.
[**get_vault_market_supply_round_up**](VaultMarketsApi.md#get_vault_market_supply_round_up) | **GET** /v1/vault-markets/supply/round-up | Your seller round-up: the share of each payout that sets a vault buy's budget, the budget so far and what it is doing. `roundUp` is null when none is set.
[**get_vault_market_tax**](VaultMarketsApi.md#get_vault_market_tax) | **GET** /v1/vault-markets/tax | Your tax year on the vault markets: units bought and sold with cost, proceeds, gain and term; lending fees, staking interest (estimate), market-maker rewards; closed positions. Links to the CSV and PDF.
[**get_vault_market_tax_csv**](VaultMarketsApi.md#get_vault_market_tax_csv) | **GET** /v1/vault-markets/tax.csv | The tax year as a CSV: one row per sale, purchase, gift, income line and closed position.
[**get_vault_market_tax_loss_suggestion**](VaultMarketsApi.md#get_vault_market_tax_loss_suggestion) | **GET** /v1/vault-markets/tax/loss-suggestions | Tax-loss suggestions: the units you hold whose mark is below your cost, each with its lot (acquired when and how, cost in cents), short or long term if sold today and the day a short lot turns long, the estimated loss at the mark before selling fees, mark quality, what stops a sale, and a link to the ordinary sell ticket. With totals by term, the gains realised so far this year, and a not-tax-advice disclosure. Never sells.
[**get_vault_market_tax_pdf**](VaultMarketsApi.md#get_vault_market_tax_pdf) | **GET** /v1/vault-markets/tax.pdf | The tax year as a PDF summary.
[**get_vault_market_tca**](VaultMarketsApi.md#get_vault_market_tca) | **GET** /v1/vault-markets/tca | Your pro orders in a window (`from`, `to`; default 90 days) with arrival mark and mid, average fill, implementation shortfall, slippage, fill rate and time to fill.
[**get_vault_market_tca_by_order_id**](VaultMarketsApi.md#get_vault_market_tca_by_order_id) | **GET** /v1/vault-markets/tca/{orderId} | One order's TCA, including slippage against the VWAP of eligible trades while it worked.
[**get_vault_market_tod**](VaultMarketsApi.md#get_vault_market_tod) | **GET** /v1/vault-markets/tod | Your transfer-on-death setup: the gate and terms, whether you accepted them, your beneficiaries and shares, units with their designations and who the plan gives them to today, and open claims on your account (which you can stop). A TodOverview object, not a collection.
[**get_vault_market_tod_my_claim**](VaultMarketsApi.md#get_vault_market_tod_my_claim) | **GET** /v1/vault-markets/tod/my-claims | Claims you filed as a beneficiary: status in words, documents sent (names only), the waiting period, and units moved or held. A TodMyClaims object, not a collection.
[**get_vault_market_unit_ar**](VaultMarketsApi.md#get_vault_market_unit_ar) | **GET** /v1/vault-markets/units/{id}/ar | A vault unit as an AR model, for \"See it in your room\" (the web's /ar/vault-<id> page hands it to Scene Viewer, WebXR or Quick Look): title, model (a path under /models, without .glb, at real size), scanUrl (the unit's own captured mesh, when it has one), imageUrl, art, tint, scale, bookKey, backPath and mine. Your own units always; somebody else's only while they show it to visitors in a home you may enter (else 404).
[**get_vault_market_unit_evidence**](VaultMarketsApi.md#get_vault_market_unit_evidence) | **GET** /v1/vault-markets/units/{id}/evidence | Evidence on a vault unit:its 3D scans (model, texture, listing photos) and its grade report (grader, grade, cert, verified against the grader record, hub intake photos). No owner identity.
[**get_vault_market_unit_quote**](VaultMarketsApi.md#get_vault_market_unit_quote) | **GET** /v1/vault-markets/units/{id}/quote | The market around a vault unit: its mark and source, the best bid that could take it, the Instant Exit price (the best bid) and a pack pull's own sell-back credit.
[**get_vault_market_unit_story**](VaultMarketsApi.md#get_vault_market_unit_story) | **GET** /v1/vault-markets/units/{id}/story | One vault unit's story, oldest first: intake (with scan and grade evidence when it has some), grades, owners, sales at their prices, loans, pledges, Hall of Grails exhibitions, receipt-token exports and redemptions, and each warehouse-receipt hash-chain step (re-verified on read: chain.ok). People are named only while their public vault profile is on; others are 'A collector'.
[**get_vault_market_valuation_statement**](VaultMarketsApi.md#get_vault_market_valuation_statement) | **GET** /v1/vault-markets/valuation-statements/{id} | One statement you issued: every unit, the totals by mark quality, the methodology, its sha256, its audit-chain seal and the verify link.
[**get_vault_market_valuation_statement_pdf**](VaultMarketsApi.md#get_vault_market_valuation_statement_pdf) | **GET** /v1/vault-markets/valuation-statements/{id}/pdf | One statement you issued as a PDF.
[**get_vault_market_valuation_statement_preview**](VaultMarketsApi.md#get_vault_market_valuation_statement_preview) | **GET** /v1/vault-markets/valuation-statements/preview | An insurance or appraisal statement for a day (purpose, asOf YYYY-MM-DD, default today), built but not issued: every unit you held that day at its mark, with quality, last trade, grade and cert, and the total. No id, no hash.
[**get_vault_market_valuation_statement_verify**](VaultMarketsApi.md#get_vault_market_valuation_statement_verify) | **GET** /v1/vault-markets/valuation-statements/{id}/verify | Check any issued statement by its id, as an insurer would: purpose, date, totals by mark quality and the sha256; intact (the stored record still hashes to it), seal.ok (its audit-chain entry still carries it) and, with ?hash=, whether yours matches. Never the holder or the units.
[**get_vault_market_value**](VaultMarketsApi.md#get_vault_market_value) | **GET** /v1/vault-markets/value | A catalogue item's vault book keys (`?skuId=`, optional `variantId`): each key's display mark with quality and n, units in the vault, the Instant Exit price a holder would get now, and whether it can be shipped in (ungraded keys).
[**get_vault_market_vault_suggestion_item**](VaultMarketsApi.md#get_vault_market_vault_suggestion_item) | **GET** /v1/vault-markets/vault-suggestions/items/{id} | One inventory item: its vault suggestion, or in plain words why there is none (`status`: suggested, not_found, no_stock, not_matched, in_vault or no_market).
[**get_vault_market_widget_index**](VaultMarketsApi.md#get_vault_market_widget_index) | **GET** /v1/vault-markets/widgets/index/{slug} | What the <crossly-index> widget shows: level, closes, quality, n, as-of, or `insufficient`.
[**get_vault_market_widget_mark**](VaultMarketsApi.md#get_vault_market_widget_mark) | **GET** /v1/vault-markets/widgets/mark | What the <crossly-mark> widget shows for `bookKey`: mark, quality, trades over 30 days, as-of, sparkline, or `insufficient`.
[**get_vault_market_widget_summary**](VaultMarketsApi.md#get_vault_market_widget_summary) | **GET** /v1/vault-markets/widget/summary | The home-screen widget summary: portfolio value at the mark with its move since the previous marks (cents and bps) and the share resting on reliable A/B marks, the three biggest movers among what you hold (the market movers when you hold nothing; `movers.source` says which), and one watchlist key (`?key=` a book key on your watchlist; else your newest watch) with a 14-day sparkline. Every price is from the Crossly order book and carries its mark quality (A, B, C or none). Cached for five minutes.
[**get_vault_market_world_floor**](VaultMarketsApi.md#get_vault_market_world_floor) | **GET** /v1/vault-markets/world/floor | The Trading Floor in Crossly World: trades (the newest trades on every vault book in the last day, at most 40, newest first: bookKey, name, priceCents, priceText, side -- buy when a buyer took an ask, sell when a seller hit a bid, cross for an auction -- and ageS), tickers (the big board: the most-traded markets, each with its last trade price, the change from the last trade before today in basis points, trades today and the display mark with its quality), bell (Market Open: ringing for ten minutes after a new key opens, with the key and its market link, and what opens next) and trades24h. Every price is a trade on the Crossly order book; it never shows who traded. Nothing can be traded in the room.
[**get_vault_market_world_lounge**](VaultMarketsApi.md#get_vault_market_world_lounge) | **GET** /v1/vault-markets/world/lounge | The Vault Holders Lounge: the rule to join (held value at reliable marks, or a unit count), whether you may join now and what you hold toward it. Anyone may look in; only holders are seen there. No perks, nothing to trade.
[**get_vault_market_world_place_vault_gate**](VaultMarketsApi.md#get_vault_market_world_place_vault_gate) | **GET** /v1/vault-markets/world/places/{place}/vault-gate | The vault-holder door on a Crossly World room or Space, by its presence slug (a room share slug, or space-<slug>): the rule (minValueCents held at reliable A/B marks and/or minUnits units, mode any or all, optionally one product, bullion family or category), the rule in words, and whether you may join now with what you hold toward it. gate is null when the place has no vault door. Anyone may look in; only holders join.
[**get_vault_market_worth**](VaultMarketsApi.md#get_vault_market_worth) | **GET** /v1/vault-markets/worth | What one book key (variantId:condition:gradeKey, `bookKey`) is worth: the mark in cents with its quality (A from trades, B from firm quotes, C indicative, none) and band, what Instant Exit would pay for one unit now from the best resting bid (yours left out), and your units: how many you hold, how many are free to sell, their value at the mark and what selling every free one now would fetch from today’s bids. Plain-words `headline` and `lines`. Read only.
[**get_vault_market_worth_vault**](VaultMarketsApi.md#get_vault_market_worth_vault) | **GET** /v1/vault-markets/worth/vault | What your whole vault is worth now: value at the marks split into reliable and indicative marks, cost and P&L, today’s change, and an estimate of what the units free to sell would fetch from the bids resting on the book now. Read only.
[**get_vault_receipt**](VaultMarketsApi.md#get_vault_receipt) | **GET** /v1/vault/receipts/{receiptNo} | One of your receipts with its full hash chain (issued, transferred, pledged, released, surrendered), re-verified on read: `chain.ok` is false if any event was altered.
[**get_vault_receipt_token**](VaultMarketsApi.md#get_vault_receipt_token) | **GET** /v1/vault/receipt-tokens/{id} | One receipt token (yours, or on a unit you own now) with its chain log: built, minted, frozen, thawed, new holder seen, redeem built, burned, with explorer links.
[**get_vault_receipt_token_by_mint**](VaultMarketsApi.md#get_vault_receipt_token_by_mint) | **GET** /v1/vault/receipt-tokens/by-mint/{mint} | What a Solana mint is: the warehouse receipt number and item it stands for, and whether the token is live, frozen (unit pledged or lent) or burned, and whether the unit is still on a vault shelf. Names no person and no wallet.
[**list_vault_market_accounts**](VaultMarketsApi.md#list_vault_market_accounts) | **GET** /v1/vault-markets/accounts | Every account you can act on: your own, and each sub-account your entity memberships reach, with your role.
[**list_vault_market_adl_events**](VaultMarketsApi.md#list_vault_market_adl_events) | **GET** /v1/vault-markets/adl/events | The ADL events your positions were part of, newest first: role bankrupt (yours went under water) or counterparty (yours was first in line), the price both closed at and the transfer status.
[**list_vault_market_adl_positions**](VaultMarketsApi.md#list_vault_market_adl_positions) | **GET** /v1/vault-markets/adl/positions | Your live positions, each with its place in its item ADL queue: lights 5 (first in line) to 1 (last), 0 when not in the queue (only profitable positions are), ranked by P&L% x effective leverage.
[**list_vault_market_agent_drafts**](VaultMarketsApi.md#list_vault_market_agent_drafts) | **GET** /v1/vault-markets/agent/drafts | Limit orders the shopping agent drafted for you, newest first. A draft places nothing: send its orderBody to POST /api/v1/vault-markets/pro/orders to place it.
[**list_vault_market_agent_watches**](VaultMarketsApi.md#list_vault_market_agent_watches) | **GET** /v1/vault-markets/agent/watches | Your price watches on book keys (best bid, lowest ask, last trade or market price, in cents), live ones first. `?live=true` for live only.
[**list_vault_market_alert_log**](VaultMarketsApi.md#list_vault_market_alert_log) | **GET** /v1/vault-markets/alert-log | What the alerts hub did with your recent vault alerts (`?limit=` 1 to 200, default 50): kind, title, the channels it went to, what was held (channel:reason) and the email digest state.
[**list_vault_market_alerts**](VaultMarketsApi.md#list_vault_market_alerts) | **GET** /v1/vault-markets/alerts | Your price alerts on book keys.
[**list_vault_market_auctions**](VaultMarketsApi.md#list_vault_market_auctions) | **GET** /v1/vault-markets/auctions | Open vault auction events (weekly, ending Sunday; premier, units marked $10,000+) and their lots, soonest ending first.
[**list_vault_market_blocks**](VaultMarketsApi.md#list_vault_market_blocks) | **GET** /v1/vault-markets/blocks | An entity's block orders, newest first, each with its plan and how it was allocated.
[**list_vault_market_bullion_family_pegs**](VaultMarketsApi.md#list_vault_market_bullion_family_pegs) | **GET** /v1/vault-markets/bullion/family-pegs | Your family-wide spot-pegged quotes (live, or all with status=all): one quote resting across a bullion family books at one condition, with units left, the peg, its status and every leg (the pegged quote on each book).
[**list_vault_market_bullion_keys**](VaultMarketsApi.md#list_vault_market_bullion_keys) | **GET** /v1/vault-markets/bullion/keys | Bullion key families (metal, form, size, mint or any mint): members, vault units, best resting bid and ask across their books, and spot as a labelled reference with what one unit of metal is worth at it. Spot is never a Crossly price.
[**list_vault_market_bullion_pegs**](VaultMarketsApi.md#list_vault_market_bullion_pegs) | **GET** /v1/vault-markets/bullion/pegs | Your spot-pegged quotes (live, or all with status=all): side, units, the peg, the order it rests as, the last price and spot it was priced from, and why it is pulled when it is.
[**list_vault_market_collection_worth**](VaultMarketsApi.md#list_vault_market_collection_worth) | **GET** /v1/vault-markets/collection-worth | Your collection valued at the display marks: rows on a vault book key with a mark are summed; rows with no mark or no key are counted, never valued.
[**list_vault_market_confirmations**](VaultMarketsApi.md#list_vault_market_confirmations) | **GET** /v1/vault-markets/confirmations | Trade confirmations for your fills in a window (default 31 days): price, sale fee, maker or taker, order, vault unit and warehouse receipt.
[**list_vault_market_creator_cohost_invites**](VaultMarketsApi.md#list_vault_market_creator_cohost_invites) | **GET** /v1/vault-markets/creators/cohost-invites | For creators: your invites from Crossly to co-host a Market Open launch show (invited, accepted, declined or withdrawn), each with the key, its show and the guidance for co-hosts.
[**list_vault_market_credit**](VaultMarketsApi.md#list_vault_market_credit) | **GET** /v1/vault-markets/credit | Your vault loans and advances (live by default): what is owed, interest accrued, loan-to-value, the liquidation line and the collateral value it is called at.
[**list_vault_market_dealer_bands**](VaultMarketsApi.md#list_vault_market_dealer_bands) | **GET** /v1/vault-markets/dealer/bands | Your Instant Exit bands and whether each is being kept now.
[**list_vault_market_dealer_bids**](VaultMarketsApi.md#list_vault_market_dealer_bids) | **GET** /v1/vault-markets/dealer/bids | Your standing dealer bids: open (default) or all.
[**list_vault_market_dealer_pool_fills**](VaultMarketsApi.md#list_vault_market_dealer_pool_fills) | **GET** /v1/vault-markets/dealer/pools/{id}/fills | A curve pool's fills, newest first: side, price and spot after.
[**list_vault_market_dealer_pools**](VaultMarketsApi.md#list_vault_market_dealer_pools) | **GET** /v1/vault-markets/dealer/pools | Your curve pools: open (default) or all, each with its curve, spot, units, buy budget, running totals, the ladder it shows and its live legs.
[**list_vault_market_digest_history**](VaultMarketsApi.md#list_vault_market_digest_history) | **GET** /v1/vault-markets/digest/history | Past weekly digests: week, status (sent / skipped / failed), the channels used and a summary.
[**list_vault_market_entity_audit**](VaultMarketsApi.md#list_vault_market_entity_audit) | **GET** /v1/vault-markets/entities/{id}/audit | The entity's audit trail, newest first: hash-chained rows with actor, account, API key, IP, session and source.
[**list_vault_market_entity_conversions**](VaultMarketsApi.md#list_vault_market_entity_conversions) | **GET** /v1/vault-markets/entities/{id}/conversions | The custodian's CBX-to-USD reward conversions for an entity that elected USD, newest first, with their status.
[**list_vault_market_estimates_accuracy**](VaultMarketsApi.md#list_vault_market_estimates_accuracy) | **GET** /v1/vault-markets/estimates-accuracy | The Crossly Estimate's track record: walk-forward backtest n, FSD and bias per method, category and liquidity bucket, and whether estimates in that cell are shown.
[**list_vault_market_feed**](VaultMarketsApi.md#list_vault_market_feed) | **GET** /v1/vault-markets/feed | New vault buys by the people you follow who share them, newest first (no prices). Page with `before` (the last item's `at`).
[**list_vault_market_follow_copy_watch**](VaultMarketsApi.md#list_vault_market_follow_copy_watch) | **GET** /v1/vault-markets/follows/copy-watch | The collectors you copy-watch and whether each still shares bids and asks.
[**list_vault_market_follow_copy_watch_alerts**](VaultMarketsApi.md#list_vault_market_follow_copy_watch_alerts) | **GET** /v1/vault-markets/follows/copy-watch/alerts | Your copy-watch alerts, newest first: who, bid or ask, quantity, limit price, market, and whether it still rests. Page with `before`.
[**list_vault_market_follow_feed**](VaultMarketsApi.md#list_vault_market_follow_feed) | **GET** /v1/vault-markets/follows/feed | Public vault activity of the collectors you follow (buys, units sent in, sales, Hall loans), newest first, never a price. Filter with `kinds`; page with `before`.
[**list_vault_market_follow_following**](VaultMarketsApi.md#list_vault_market_follow_following) | **GET** /v1/vault-markets/follows/following | The collectors you follow who have a public vault: what each shares and whether you copy-watch them.
[**list_vault_market_follow_leaderboards**](VaultMarketsApi.md#list_vault_market_follow_leaderboards) | **GET** /v1/vault-markets/follows/leaderboards | An opt-in return leaderboard (`period` 30d, 90d or 365d; `category` all or a category slug), ranked by risk-adjusted return from the book A/B marks. Percent and risk only: no dollars, no volume.
[**list_vault_market_follow_profile_activity**](VaultMarketsApi.md#list_vault_market_follow_profile_activity) | **GET** /v1/vault-markets/follows/profiles/{handle}/activity | One collector public vault activity, newest first, never a price. 404 when their vault is not public.
[**list_vault_market_gifts**](VaultMarketsApi.md#list_vault_market_gifts) | **GET** /v1/vault-markets/gifts | Gifts you sent and received, newest first, with their status.
[**list_vault_market_grade_to_vault**](VaultMarketsApi.md#list_vault_market_grade_to_vault) | **GET** /v1/vault-markets/grade-to-vault | Your Grade to Vault flows, newest first, each brought up to date: its step, the grading submission, the unit, the grade, the market for it, and the ask once listed. `live=1` for those still going.
[**list_vault_market_grading**](VaultMarketsApi.md#list_vault_market_grading) | **GET** /v1/vault-markets/grading | Your vault units sent to grading: where each card is in its run, and the grade once it is back.
[**list_vault_market_hall_lenders**](VaultMarketsApi.md#list_vault_market_hall_lenders) | **GET** /v1/vault-markets/hall/lenders | The vault units lent to this week's Hall of Grails exhibition and on show now, in plinth order: the item (skuId, name, imageUrl, grade), the lender as the plate names them (a display name, or \"a collector\"), the plate lines and how many exhibitions the unit has stood in. meta.exhibition is the week (key, title, endsAt). Names no ids and no amounts.
[**list_vault_market_index_committee_minutes**](VaultMarketsApi.md#list_vault_market_index_committee_minutes) | **GET** /v1/vault-markets/index-committee/minutes | Published Index Committee minutes, newest first: attendees (names and roles), quorum, agenda, minutes and decisions.
[**list_vault_market_index_complaints**](VaultMarketsApi.md#list_vault_market_index_complaints) | **GET** /v1/vault-markets/index-complaints | Your complaints and their answers, newest first.
[**list_vault_market_index_constituents**](VaultMarketsApi.md#list_vault_market_index_constituents) | **GET** /v1/vault-markets/indexes/{slug}/constituents | The book keys in an index with their weight, move since the last close and contribution to the index's move (basis points).
[**list_vault_market_index_consultations**](VaultMarketsApi.md#list_vault_market_index_consultations) | **GET** /v1/vault-markets/index-consultations | Consultations on index method changes (open for at least 30 days), newest first. Filter by `status`.
[**list_vault_market_index_events**](VaultMarketsApi.md#list_vault_market_index_events) | **GET** /v1/vault-markets/index-events | Every index event, newest first: add, delete, rebalance, method, correction, halt; each with when it was announced, when it takes effect, the detail and the reason. Filter by `kind`.
[**list_vault_market_index_events_by_slug**](VaultMarketsApi.md#list_vault_market_index_events_by_slug) | **GET** /v1/vault-markets/indexes/{slug}/events | One index's events, newest first.
[**list_vault_market_index_judgments**](VaultMarketsApi.md#list_vault_market_index_judgments) | **GET** /v1/vault-markets/index-judgments | The expert-judgment log: every Index Committee action (halt, resume, exclude a key, void a trade, restate, retire) with its written reason.
[**list_vault_market_index_level_series**](VaultMarketsApi.md#list_vault_market_index_level_series) | **GET** /v1/vault-markets/indexes/{slug}/level-series | An index's published levels. `engine=chain` (default): daily closes. `engine=tpd|repeat_sales`: weekly (else monthly) model estimates with a 95% band and n; labelled, never a price. `series=settlement` returns each value as first published (never restated). Below the data gate `data` is empty and `notice` explains.
[**list_vault_market_index_licence_mine**](VaultMarketsApi.md#list_vault_market_index_licence_mine) | **GET** /v1/vault-markets/index-licence/mine | Your index feed licences (the indexes each covers, or all), as ops recorded them. No licence: the feed answers 403 not_a_licensee.
[**list_vault_market_index_members**](VaultMarketsApi.md#list_vault_market_index_members) | **GET** /v1/vault-markets/indexes/{slug}/members | The members of an index on a day (`asOf`, default today) with their weights, shares, moves and contributions; for a past day, the inputs stored with that day's close and each member's effective dates.
[**list_vault_market_index_movers**](VaultMarketsApi.md#list_vault_market_index_movers) | **GET** /v1/vault-markets/indexes/movers | The built-in indexes that moved most since their last daily close.
[**list_vault_market_indexes**](VaultMarketsApi.md#list_vault_market_indexes) | **GET** /v1/vault-markets/indexes | Indexes over book keys: per category, per top brand, the Crossly 50 and your own. Each with its level now (base 1000 at inception) and change since the last daily close.
[**list_vault_market_instant_exit_history**](VaultMarketsApi.md#list_vault_market_instant_exit_history) | **GET** /v1/vault-markets/instant-exit/history | Your Instant Exits, newest first: quoted price, mark, and whether it filled.
[**list_vault_market_instant_exit_holdings**](VaultMarketsApi.md#list_vault_market_instant_exit_holdings) | **GET** /v1/vault-markets/instant-exit/holdings | Instant Exit quotes for everything you hold in the vault, one per book key, with units held and which are free to sell.
[**list_vault_market_joint_trades**](VaultMarketsApi.md#list_vault_market_joint_trades) | **GET** /v1/vault-markets/joint/{id}/trades | The joint vault trades, newest first: side, book key, limit, who started it, whose account the order sits on, and the order standing on the book.
[**list_vault_market_large_trader_reports**](VaultMarketsApi.md#list_vault_market_large_trader_reports) | **GET** /v1/vault-markets/large-trader-reports | Your large-trader reports (with any linked accounts, as one party): due first, then filed, then cleared in the last 90 days. Each has `action`: file | amend | refile | none.
[**list_vault_market_lending_statements**](VaultMarketsApi.md#list_vault_market_lending_statements) | **GET** /v1/vault-markets/lending/statements | The days you have stored collateral statements for, newest first, with their summary.
[**list_vault_market_loans**](VaultMarketsApi.md#list_vault_market_loans) | **GET** /v1/vault-markets/loans | Units you lent: rate, fees earned, recall due.
[**list_vault_market_locate_mine**](VaultMarketsApi.md#list_vault_market_locate_mine) | **GET** /v1/vault-markets/locates/mine | Your locates, newest first (held / used / released / expired).
[**list_vault_market_midpoint**](VaultMarketsApi.md#list_vault_market_midpoint) | **GET** /v1/vault-markets/midpoint | Your midpoint block orders.
[**list_vault_market_movers**](VaultMarketsApi.md#list_vault_market_movers) | **GET** /v1/vault-markets/movers | Vault items with the biggest 24 h moves in their market price (from the order book), with units in the vault.
[**list_vault_market_nav_history**](VaultMarketsApi.md#list_vault_market_nav_history) | **GET** /v1/vault-markets/nav/history | Daily NAV snapshots (mark, bid, liquidation, Level 2 / Level 3).
[**list_vault_market_offers**](VaultMarketsApi.md#list_vault_market_offers) | **GET** /v1/vault-markets/offers | The offers inbox: for each book key you hold, the best bid from someone else, units at it, the top bid levels, the mark, and your alert.
[**list_vault_market_opening**](VaultMarketsApi.md#list_vault_market_opening) | **GET** /v1/vault-markets/opening | Keys opening soon on the vault book (and just opened): units on the shelf, dealers quoting, the waitlist, the mark when the book has one, and your place.
[**list_vault_market_oracle_history**](VaultMarketsApi.md#list_vault_market_oracle_history) | **GET** /v1/vault-markets/oracle/{slug}/history | Recent publish attempts for an index over 30 days, newest first: published, skipped (with the reason: too few items priced, stale marks, confidence too wide...) or failed (with the error). Filter with `status`, cap with `limit` (default 100).
[**list_vault_market_p2p_auctions**](VaultMarketsApi.md#list_vault_market_p2p_auctions) | **GET** /v1/vault-markets/p2p/auctions | Running refinancing auctions, soonest to end first: lenders calls (`auction`) and borrowers looking for a cheaper lender (`refiAuction`). The payoff now, the rate a new lender gets now, when it ends, and the item.
[**list_vault_market_p2p_loans**](VaultMarketsApi.md#list_vault_market_p2p_loans) | **GET** /v1/vault-markets/p2p/loans | Your P2P loans as borrower and/or lender (`role`), live by default (`status`): what is owed now, daily interest, the refinancing auction when called, and the item mark for information only (it never triggers anything).
[**list_vault_market_p2p_offer_mine**](VaultMarketsApi.md#list_vault_market_p2p_offer_mine) | **GET** /v1/vault-markets/p2p/offers/mine | Your P2P loan offers, newest first, with how many loans each has funded.
[**list_vault_market_p2p_offers**](VaultMarketsApi.md#list_vault_market_p2p_offers) | **GET** /v1/vault-markets/p2p/offers | The open P2P loan offers book, lowest rate first: cents offered per loan, APR (bps), loans left, and the item (book key) or unit each lends against. `bookKey` narrows to one item. Lender identities are never shown.
[**list_vault_market_p2p_units**](VaultMarketsApi.md#list_vault_market_p2p_units) | **GET** /v1/vault-markets/p2p/units | Your vault units for P2P borrowing: whether each can be pledged now (and why not), the best offers covering it, and its loan when it has one.
[**list_vault_market_paper_fills**](VaultMarketsApi.md#list_vault_market_paper_fills) | **GET** /v1/vault-markets/paper/fills | Your paper fills since your last reset, newest first: price, simulated fee, taker or maker, and whether a visible real order (book) or a real trade (tape) filled it. Play money only.
[**list_vault_market_paper_leaderboard**](VaultMarketsApi.md#list_vault_market_paper_leaderboard) | **GET** /v1/vault-markets/paper/leaderboard | The weekly paper trading leaderboard (opt-in, display names only): best return on play equity this week (or a past `week`, its Monday as YYYY-MM-DD), with your own line. Nothing is won or paid.
[**list_vault_market_paper_missions**](VaultMarketsApi.md#list_vault_market_paper_missions) | **GET** /v1/vault-markets/paper/missions | Your paper-trading missions (play money only): each mission with what to do, your count toward its target, whether it is done and when, plus a suggested market for the \"Read the mark\" mission. Progress is observed by the server from your paper orders and fills, price watches, set goals and graded mark checks. Missions earn world quest progress and cosmetic pins, never money.
[**list_vault_market_paper_orders**](VaultMarketsApi.md#list_vault_market_paper_orders) | **GET** /v1/vault-markets/paper/orders | Your paper orders since your last reset, newest first: working, done (filled, cancelled, expired) or all. Play money only.
[**list_vault_market_partner_accruals**](VaultMarketsApi.md#list_vault_market_partner_accruals) | **GET** /v1/vault-markets/partners/accruals | Your partner fee shares, newest first: accrued on a unit's first sale, paid in Bucks once the sale is final, void if it was refunded.
[**list_vault_market_pool_stakes**](VaultMarketsApi.md#list_vault_market_pool_stakes) | **GET** /v1/vault-markets/pool/stakes | Your stakes in the market pool and what each redeems for now.
[**list_vault_market_portfolio_margin_history**](VaultMarketsApi.md#list_vault_market_portfolio_margin_history) | **GET** /v1/vault-markets/portfolio-margin/history | The 5-minute portfolio margin recalculations for a firm sub-account over the last `hours` (default 24, at most 720), newest first: equity, requirement, excess, scenario and floor figures, gross and state.
[**list_vault_market_position_units**](VaultMarketsApi.md#list_vault_market_position_units) | **GET** /v1/vault-markets/positions/{id}/units | The vault units of one of your positions that you hold (a leveraged long's collateral), each with its story link.
[**list_vault_market_positions**](VaultMarketsApi.md#list_vault_market_positions) | **GET** /v1/vault-markets/positions | Your positions, live (default) or closed.
[**list_vault_market_positions_risk**](VaultMarketsApi.md#list_vault_market_positions_risk) | **GET** /v1/vault-markets/positions-risk | For each live position: its tier, health, margin state, the mark at which it is called and the mark at which it is liquidated, and the distance to it.
[**list_vault_market_price_widgets**](VaultMarketsApi.md#list_vault_market_price_widgets) | **GET** /v1/vault-markets/price-widgets | Your price widgets, newest first, each with its iframe and script-tag snippets. `meta.codes` lists your affiliate codes a widget can link through; `meta.max` is the cap on live widgets.
[**list_vault_market_pro_candles**](VaultMarketsApi.md#list_vault_market_pro_candles) | **GET** /v1/vault-markets/pro/candles | OHLCV candles (1m, 5m, 15m, 1h, 4h, 1d) from the trades on a book key.
[**list_vault_market_pro_events**](VaultMarketsApi.md#list_vault_market_pro_events) | **GET** /v1/vault-markets/pro/events | Your order event log after a sequence number (new, leg, fill, replaced, cancelled, expired, done): a replayable drop copy.
[**list_vault_market_pro_links**](VaultMarketsApi.md#list_vault_market_pro_links) | **GET** /v1/vault-markets/pro/links | Your linked orders (brackets, OCO pairs, trailing stops, position exits): live (default) or done.
[**list_vault_market_pro_orders**](VaultMarketsApi.md#list_vault_market_pro_orders) | **GET** /v1/vault-markets/pro/orders | Your orders: open (default) or done, optionally on one book key, or only those booked to one sub-account (`subaccountId`).
[**list_vault_market_pro_quotes**](VaultMarketsApi.md#list_vault_market_pro_quotes) | **GET** /v1/vault-markets/pro/quotes | Your live quotes.
[**list_vault_market_pro_rewards**](VaultMarketsApi.md#list_vault_market_pro_rewards) | **GET** /v1/vault-markets/pro/rewards | Your daily market-maker liquidity rewards (paid in CBX).
[**list_vault_market_pro_tape**](VaultMarketsApi.md#list_vault_market_pro_tape) | **GET** /v1/vault-markets/pro/tape | The latest trades on a book key, newest first, with which side made and which took.
[**list_vault_market_proof_of_reserve_leaves**](VaultMarketsApi.md#list_vault_market_proof_of_reserve_leaves) | **GET** /v1/vault-markets/proof-of-reserves/{day}/leaves | One day's leaf hashes in tree order, a page at a time: enough to recount the units and rebuild the root yourself. Leaves are salted, so they name no owner.
[**list_vault_market_recurring**](VaultMarketsApi.md#list_vault_market_recurring) | **GET** /v1/vault-markets/recurring | Your recurring buys: key, schedule, price cap, payment, auto-lend, next run.
[**list_vault_market_recurring_runs**](VaultMarketsApi.md#list_vault_market_recurring_runs) | **GET** /v1/vault-markets/recurring/{id}/runs | One recurring buy's runs: filled, skipped (with the reason, e.g. the price was over the cap) or failed.
[**list_vault_market_recurring_runs2**](VaultMarketsApi.md#list_vault_market_recurring_runs2) | **GET** /v1/vault-markets/recurring-runs | Every run of all your recurring buys, newest first.
[**list_vault_market_rfq**](VaultMarketsApi.md#list_vault_market_rfq) | **GET** /v1/vault-markets/rfq | Your quote requests, newest first.
[**list_vault_market_rfq_inbox**](VaultMarketsApi.md#list_vault_market_rfq_inbox) | **GET** /v1/vault-markets/rfq/inbox | Open quote requests you were sent (as a dealer, DMM, LP or holder), with your live answer.
[**list_vault_market_rfq_open**](VaultMarketsApi.md#list_vault_market_rfq_open) | **GET** /v1/vault-markets/rfq/open | Open quote requests on one key (all-to-all: anyone with ID may answer). Anonymous.
[**list_vault_market_set_goal_baskets**](VaultMarketsApi.md#list_vault_market_set_goal_baskets) | **GET** /v1/vault-markets/set-goals/{id}/baskets | A set goal basket bids, newest first, each with its budget, what it committed and spent, and every leg (piece, book key, limit, pro order id, status).
[**list_vault_market_set_goal_sets**](VaultMarketsApi.md#list_vault_market_set_goal_sets) | **GET** /v1/vault-markets/set-goals/sets | Sets to start a goal on. With q (2+ characters): a word-start search of every catalogue set. Without: the sets you hold pieces of in the vault, then the collections you are building. Each with your live goal on it, if any.
[**list_vault_market_set_goals**](VaultMarketsApi.md#list_vault_market_set_goals) | **GET** /v1/vault-markets/set-goals | Your set goals: the set, the target a piece must meet, pieces held of the total (from your vault), and the working basket bid.
[**list_vault_market_share_cards**](VaultMarketsApi.md#list_vault_market_share_cards) | **GET** /v1/vault-markets/share-cards | Your share cards (not taken down), each as the public sees it.
[**list_vault_market_supply_deposits**](VaultMarketsApi.md#list_vault_market_supply_deposits) | **GET** /v1/vault-markets/supply/deposits | Your units sent to the vault, newest first, each with its state and its key's Instant Exit line (\"Instant Exit now: $X (N% of mark)\" or \"No market yet\"). Filter with `inventoryItemIds` (comma-separated).
[**list_vault_market_supply_invites**](VaultMarketsApi.md#list_vault_market_supply_invites) | **GET** /v1/vault-markets/supply/invites | Market Open invites to you as a dealer: keys Crossly is opening and asks you to quote.
[**list_vault_market_valuation_statements**](VaultMarketsApi.md#list_vault_market_valuation_statements) | **GET** /v1/vault-markets/valuation-statements | The insurance and appraisal statements you issued, newest first: purpose, date, units, total at the marks, hash and verify link.
[**list_vault_market_vault_suggestions**](VaultMarketsApi.md#list_vault_market_vault_suggestions) | **GET** /v1/vault-markets/vault-suggestions | What to send to the vault: your inventory items whose book key has a liquid vault market (tier A, B or C), best first. Each has a score (0 to 100) and its reasons, the market (tier, mark and its quality, best bid, best vault ask, spread), Instant Exit now, your list price, days listed without selling, and the vault fee. `meta.summary` counts what was left out and why. Inform only; send with POST /api/v1/vault-markets/supply/send.
[**list_vault_market_watchlist**](VaultMarketsApi.md#list_vault_market_watchlist) | **GET** /v1/vault-markets/watchlist | Your watched book keys with mark, 24 h and 7 d change, and the last 30 daily marks (sparkline).
[**list_vault_market_widget_tokens**](VaultMarketsApi.md#list_vault_market_widget_tokens) | **GET** /v1/vault-markets/widget/tokens | Your live home-screen widget tokens, one per phone: id, name (the phone), public prefix, created and last used. The secret is never returned again.
[**list_vault_market_world_cohosts**](VaultMarketsApi.md#list_vault_market_world_cohosts) | **GET** /v1/vault-markets/world/cohosts | Market Open launch shows a verified creator co-hosts that are on now or opening soon: the key, its phase, the show, the creator (named only while their vault profile is public), their room link and, when the creator is in the creator programme, the disclosure that they may earn a share of the fee Crossly keeps. Never a prompt to trade.
[**list_vault_market_world_creator_rooms**](VaultMarketsApi.md#list_vault_market_world_creator_rooms) | **GET** /v1/vault-markets/world/creator-rooms | Verified creators whose public vault shelf is a world room: title, the creator, the room link and the disclosure. Rooms show each market share of the shelf, never amounts.
[**list_vault_market_world_market_open**](VaultMarketsApi.md#list_vault_market_world_market_open) | **GET** /v1/vault-markets/world/market-open | Market Open events in the Crossly World Commons: keys opening soon and those that opened in the last few hours, live first then soonest, each with its board line, units on the shelf, dealers quoting and the launch show when there is one. Never a prompt to trade.
[**list_vault_receipt_token_wallets**](VaultMarketsApi.md#list_vault_receipt_token_wallets) | **GET** /v1/vault/receipt-tokens/wallets | Your verified Solana wallets and whether each is on the receipt-token allow-list (receipt tokens move only between allow-listed wallets). meta.state is the gate.
[**list_vault_receipt_tokens**](VaultMarketsApi.md#list_vault_receipt_tokens) | **GET** /v1/vault/receipt-tokens | Your vault units with their warehouse receipt and receipt-token state (not exported, waiting for a signature, in a wallet, frozen, redeemed), whether each can be exported now and why not. meta.state is the gate (open, missing envs, program id, authority, terms version, cluster); meta.history is your last 50 exports.
[**list_vault_receipts**](VaultMarketsApi.md#list_vault_receipts) | **GET** /v1/vault/receipts | Your warehouse receipts (documents of title for your vault units), live first. meta.receipts says whether receipts are registered (VAULT_WAREHOUSE_ISSUER) and their terms version.
[**list_vault_unit_list**](VaultMarketsApi.md#list_vault_unit_list) | **GET** /v1/vault/unit-list | The published unit list (bar-list equivalent): every unit on a vault shelf with its warehouse receipt number (when receipts are registered), book key, shelf location code, insured value, status, pledged / on-loan flags and a per-receipt hashed holder (never a name or id). Filter with `bookKey`; pages with `page` / `limit`.
[**update_vault_market_alert_pref**](VaultMarketsApi.md#update_vault_market_alert_pref) | **PUT** /v1/vault-markets/alert-prefs | Change vault alert settings: `matrix` ({ kind: { channel: true|false } }, merged onto what is set), `quietHours` ({ start: \"HH:MM\", end: \"HH:MM\", tz } or null to clear) and `telegram` (the Telegram master switch). Locked switches are ignored. Returns the new settings.
[**update_vault_market_block_allocation**](VaultMarketsApi.md#update_vault_market_block_allocation) | **PUT** /v1/vault-markets/blocks/{orderId}/allocation | Post-trade allocation: set or replace the plan for a buy you placed for one of the entity's sub-accounts (that sub-account leads). Refused once any share has moved.
[**update_vault_market_channel**](VaultMarketsApi.md#update_vault_market_channel) | **PUT** /v1/vault-markets/channels | Change the switches: `telegramAlerts`, `telegramDigest`, `pushCapPerDay` (0 to 20). Margin calls and recalls are never capped. Returns the new view.
[**update_vault_market_dealer_band**](VaultMarketsApi.md#update_vault_market_dealer_band) | **PUT** /v1/vault-markets/dealer/bands | Commit to an Instant Exit band on a key: a standing bid for `units` kept at `bandBps` of the mark, never above `maxPriceCents`, re-quoted as the mark moves.
[**update_vault_market_dealer_pool**](VaultMarketsApi.md#update_vault_market_dealer_pool) | **PATCH** /v1/vault-markets/dealer/pools/{id} | Change a curve pool: spot, step, fee, levels, units, buy budget, floor, ceiling, or pause / resume. Re-quoted at once.
[**update_vault_market_entity_member**](VaultMarketsApi.md#update_vault_market_entity_member) | **PATCH** /v1/vault-markets/entities/{id}/members/{memberId} | Admin: change a member's role, limits or sub-accounts. The last admin cannot be demoted.
[**update_vault_market_entity_subaccount_limit**](VaultMarketsApi.md#update_vault_market_entity_subaccount_limit) | **PUT** /v1/vault-markets/entities/{id}/subaccounts/{subId}/limits | Admin: set a sub-account's limits (per-order notional, units per key, gross, net, leverage), held to the entity's caps. `limits: null` clears them.
[**update_vault_market_entity_usd_election**](VaultMarketsApi.md#update_vault_market_entity_usd_election) | **PUT** /v1/vault-markets/entities/{id}/usd-election | Admin: elect USD for the entity (`usdOnly`). Its sub-accounts then post collateral in USD only and pay every fee in USD, and any CBX reward paid to one of them is converted to USD by the custodian in that account. Audited; it can be turned off again.
[**update_vault_market_follow_copy_watch**](VaultMarketsApi.md#update_vault_market_follow_copy_watch) | **PUT** /v1/vault-markets/follows/copy-watch/{handle} | Copy-watch a collector who shares bids and asks: you get an alert when they rest a new one. It never places an order for you.
[**update_vault_market_follow_sharing**](VaultMarketsApi.md#update_vault_market_follow_sharing) | **PUT** /v1/vault-markets/follows/sharing | Turn sharing switches on or off (`isPublic`, `showBuys`, `deposits`, `sales`, `hallLoans`, `orders`, `leaderboard`). A switch shows only what happens after it goes on; all but the profile itself need the profile public.
[**update_vault_market_home_unit**](VaultMarketsApi.md#update_vault_market_home_unit) | **PUT** /v1/vault-markets/home/units | Show exactly these vault units in your Crossly World home: { units: [{ vaultItemId, visitors }] }. Units left out come out of their cases; units already standing stay where they are; new ones go into a free display-case or shelf slot, else a glass case of their own along a wall. visitors: true lets people who may enter your home see that unit (default false: only you). Returns placed, unplaced (no free case: the home is full), refused (with why) and the desk. Display only: nothing moves.
[**update_vault_market_index**](VaultMarketsApi.md#update_vault_market_index) | **PATCH** /v1/vault-markets/indexes/{slug} | Rename, re-weight, share or change the keys of an index of your own; its level carries on.
[**update_vault_market_lending_election**](VaultMarketsApi.md#update_vault_market_lending_election) | **PUT** /v1/vault-markets/lending/election | Change your lender election (applies to loans opened after the change). Needs lending.
[**update_vault_market_mm_mmp**](VaultMarketsApi.md#update_vault_market_mm_mmp) | **PUT** /v1/vault-markets/mm/mmp | Set market-maker protection: N units or $X filled on your quotes within T seconds pulls every quote and freezes quoting for a set time.
[**update_vault_market_mode**](VaultMarketsApi.md#update_vault_market_mode) | **PUT** /v1/vault-markets/mode | Choose your Vault Markets mode: `mode` simple or pro. Remembered for you on the web and in the app. A display preference only: it places nothing and moves no money.
[**update_vault_market_offer_alert**](VaultMarketsApi.md#update_vault_market_offer_alert) | **PUT** /v1/vault-markets/offers/alert | Set (or clear with null) the bid price that should always notify you on a key you hold, and mute or unmute offer notices on it.
[**update_vault_market_paper_leaderboard**](VaultMarketsApi.md#update_vault_market_paper_leaderboard) | **PUT** /v1/vault-markets/paper/leaderboard | Join or leave the weekly paper leaderboard. Joining needs a display name (2 to 24 letters, numbers, spaces, dots, dashes or underscores), never your real name. Returns `{ optIn, displayName }`.
[**update_vault_market_position_exit**](VaultMarketsApi.md#update_vault_market_position_exit) | **PUT** /v1/vault-markets/positions/{id}/exits | Set a position's take-profit / stop-loss: a long is closed, a short covered, when the last trade reaches one. Both null clears them.
[**update_vault_market_price_widget**](VaultMarketsApi.md#update_vault_market_price_widget) | **PATCH** /v1/vault-markets/price-widgets/{id} | Change a widget: name, range, layout, theme, affiliate code (null for none) or allowed sites. The market or index it shows never changes: make a new widget for another.
[**update_vault_market_profile**](VaultMarketsApi.md#update_vault_market_profile) | **PUT** /v1/vault-markets/profile | Turn your public vault profile on or off (`isPublic`) and whether followers see your new buys (`showBuys`). It shows weights by market, never dollars.
[**update_vault_market_recurring**](VaultMarketsApi.md#update_vault_market_recurring) | **PATCH** /v1/vault-markets/recurring/{id} | Change a recurring buy's schedule, caps, cards or auto-lend, or pause / resume it.
[**update_vault_market_set_goal**](VaultMarketsApi.md#update_vault_market_set_goal) | **PATCH** /v1/vault-markets/set-goals/{id} | Change what a piece must be to count (condition, grade mode, grade). Refused while a basket bid works: cancel it first.
[**update_vault_market_share_card**](VaultMarketsApi.md#update_vault_market_share_card) | **PATCH** /v1/vault-markets/share-cards/{id} | Show or hide the dollar amounts on one of your share cards (`showAmounts`).
[**update_vault_market_supply_deposit**](VaultMarketsApi.md#update_vault_market_supply_deposit) | **PATCH** /v1/vault-markets/supply/deposits/{id} | Change a deposit's toggles: `autoListBps` (list at that share of the mark once stored; null = do not) and `lendWhenIdle`.
[**update_vault_market_supply_round_up**](VaultMarketsApi.md#update_vault_market_supply_round_up) | **PUT** /v1/vault-markets/supply/round-up | Set the round-up: `shareBps` (1%-50%) of each payout released to you adds to a budget; when it covers the cheapest vault unit on `bookKey`, one unit is bought with your saved card, never above the budget.
[**update_vault_market_tod_beneficiary**](VaultMarketsApi.md#update_vault_market_tod_beneficiary) | **PUT** /v1/vault-markets/tod/beneficiaries | Replace your beneficiary list (keep an id to edit one): full name, contact (email or @username), relationship, and share in basis points; shares add up to 10000. Returns `{ beneficiaries, removed }`.
[**update_vault_market_tod_designation**](VaultMarketsApi.md#update_vault_market_tod_designation) | **PUT** /v1/vault-markets/tod/designations | Leave one of your units to one beneficiary (beneficiaryId null clears it; the unit then follows the shares). Returns `{ vaultItemId, beneficiaryId }`.
[**update_vault_market_world_room_vault_gate**](VaultMarketsApi.md#update_vault_market_world_room_vault_gate) | **PUT** /v1/vault-markets/world/rooms/{id}/vault-gate | Set the vault-holder door on your own room (its collection rooms follow): visitors must hold minValueCents (cents, at reliable A/B marks) and/or minUnits units in the Crossly vault, mode any (either) or all (both), optionally only one product (scope kind product, ref the catalogue product id), bullion family (bullion, the family slug) or category (category, the slug). Send null to open it again. Returns `{ gate }`. Moves no money.
[**update_vault_market_world_space_vault_gate**](VaultMarketsApi.md#update_vault_market_world_space_vault_gate) | **PUT** /v1/vault-markets/world/spaces/{slug}/vault-gate | Set the vault-holder door on a Space you host: the same rule as a room (minValueCents at reliable marks and/or minUnits, mode, optional scope). Send null to open it again. Returns `{ gate }`. Moves no money.



## create_vault_lend

> crate::models::CreateVaultLendResponse create_vault_lend(id)
Turn lending on or off for a unit you own in the vault, optionally with your minimum borrow rate (APR in basis points). Returns `{ lendEnabled, minAprBps }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultLendResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_account

> crate::models::StatusResponse create_vault_market_account()
Open your account at the custodian partner (once). Returns `{ status }`; the custodian verifies it before it is active.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_admin_mark

> crate::models::CreateVaultMarketAdminMarkResponse create_vault_market_admin_mark()
Upload your fund administrator's prices (`administrator`, `marks: [{ bookKey, day, cents }]`). Shown beside Crossly's marks and labelled as yours; they never change a Crossly figure.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketAdminMarkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_advance

> crate::models::GetVaultMarketCreditResponse create_vault_market_advance()
Take an advance against a vault unit that is listed for sale: paid into your custodian account, netted from the sale. Free for 60 days, then the pool rate. Needs a verified ID.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_advance_relist

> crate::models::CreateVaultMarketAdvanceRelistResponse create_vault_market_advance_relist(id)
Relist an advance's unit at a new price (never so low its sale could not repay the advance).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketAdvanceRelistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_agent_draft_cancel

> crate::models::CreateVaultMarketAgentDraftCancelResponse create_vault_market_agent_draft_cancel(id)
Throw a drafted order away (only while it is still a draft). Returns { ok, draft }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketAgentDraftCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_agent_watch

> crate::models::CreateVaultMarketAgentWatchResponse create_vault_market_agent_watch()
Watch a book key: tell me once when its best bid, lowest ask, last trade or market price crosses a price (cents). Leave out direction for \"crosses\" (whichever way it has to move). Never trades. At most 50 live, each for 90 days.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketAgentWatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_alert

> crate::models::ListVaultMarketAlertsItem create_vault_market_alert()
Alert me when a key's market price goes above or below a price (fires once).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAlertsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_alert_pref_test

> crate::models::CreateVaultMarketAlertPrefTestResponse create_vault_market_alert_pref_test()
Send yourself one test alert of a `kind` through your own switches. Returns `{ sent, kind, plan }`, where plan says per channel: send, off, quiet (held by quiet hours) or unavailable.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketAlertPrefTestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_auction_lot

> crate::models::CreateVaultMarketAuctionLotResponse create_vault_market_auction_lot()
Consign a stored, authenticated unit to the next weekly or premier event, with an optional opening price and reserve. The unit is held until the close. Returns `{ lotId, eventId, endsAt }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketAuctionLotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_auction_lot_bid

> crate::models::CreateVaultMarketAuctionLotBidResponse create_vault_market_auction_lot_bid(id)
Leave or raise your maximum on a lot; the system bids for you up to it. A bid in the last 2 minutes extends the lot by 2 minutes. The winner's saved card is charged at the close through the order book. ID needed above $10,000.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketAuctionLotBidResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_auction_lot_withdraw

> crate::models::OkResponse create_vault_market_auction_lot_withdraw(id)
Withdraw your lot before any bid; the unit goes back on your shelf. Returns `{ ok }`.

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


## create_vault_market_block

> crate::models::ListVaultMarketBlocksItem create_vault_market_block()
Place a block buy for an entity: one order booked to the first sub-account in `plan` (it pays from its custodian account), split across every sub-account in the plan at the average price, fees included, once it is done and the custodian confirmed each payment. Each share's cash moves between the entity's own custodian accounts; its units move only after that settles. Every sub-account must be one you may trade, and its share must fit its limits.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBlocksItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_block_allocate

> crate::models::CreateVaultMarketBlockAllocateResponse create_vault_market_block_allocate(order_id)
Allocate a done block now (the keeper does it too). Returns `{ outcome, block }`: allocated, waiting (a payment or transfer is not settled yet), nothing_filled or none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketBlockAllocateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_bullion_family_peg

> crate::models::ListVaultMarketBullionFamilyPegsItem create_vault_market_bullion_family_peg()
Set a family-wide spot-pegged quote: one \"any 1 oz gold coin\" quote resting on each product book of a bullion family at the chosen condition (DS, VNDS or Used). When any leg fills it comes off the others and goes back on only for what is left; two fills at once are settled one at a time against what is left. A bid pays from your custodian account. Replaces your live family quote on the same family, condition and side. ID required.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBullionFamilyPegsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_bullion_peg

> crate::models::ListVaultMarketBullionPegsItem create_vault_market_bullion_peg()
Set a spot-pegged maker quote on a bullion book key: \"spot plus or minus N cents\" or \"plus or minus N basis points\", with an optional floor and cap. It rests as a pro limit order re-priced every spot tick and is pulled while spot is stale. Replaces your live quote on the same key and side. ID required.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBullionPegsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_collection_worth_watch

> crate::models::CreateVaultMarketCollectionWorthWatchResponse create_vault_market_collection_worth_watch()
Watch every collection item that sits on a vault book key (up to the watchlist cap). Returns `{ watched, skipped }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketCollectionWorthWatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_creator_attribution

> crate::models::CreateVaultMarketCreatorAttributionResponse create_vault_market_creator_attribution()
Use a creator code: the creator earns a capped share of the fee Crossly keeps on your Vault Markets trades for a limited window (paid by Crossly, never by you). Only for people new to Vault Markets, once, and never your own code.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketCreatorAttributionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_creator_cohost_invite_respond

> crate::models::CreateVaultMarketCreatorCohostInviteRespondResponse create_vault_market_creator_cohost_invite_respond(id)
For creators: accept or decline an invite to co-host a Market Open launch show (`accept` true or false). Accepting needs your vault room verified and adds you to the show as a co-host; your room and the Commons then say you co-host it, with the creator-programme disclosure when it applies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketCreatorCohostInviteRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_credit_repay

> crate::models::CreateVaultMarketCreditRepayResponse create_vault_market_credit_repay(id)
Repay from your custodian account, interest first. Omit `amountCents` to repay everything; pass `requestId` to make a retry safe. Repaid in full, the items are free.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketCreditRepayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dcm_link

> crate::models::CreateVaultMarketDcmLinkResponse create_vault_market_dcm_link()
Link a partner exchange account to your account (or to a firm sub-account you manage, `subaccountId`) so Crossly can show its positions: `accountRef` (the account id there) and `code` (the one-time link code it gave you). The partner must confirm the code; the code is never stored. Read-only: Crossly never trades on it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDcmLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dcm_unlink

> crate::models::CreateVaultMarketDcmUnlinkResponse create_vault_market_dcm_unlink()
Unlink the partner exchange account from your account (or the firm sub-account `subaccountId`): Crossly stops reading it and its hedges leave portfolio margin.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDcmUnlinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dealer_apply

> crate::models::CreateVaultMarketDealerApplyResponse create_vault_market_dealer_apply()
Apply to be a dealer (ID check first). Ops review; approved dealers get raised limits and their bids show as dealer bids.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDealerApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dealer_bid

> crate::models::CreateVaultMarketProOrderResponse create_vault_market_dealer_bid()
Place a standing dealer bid: a resting GTC/GTD limit buy, flagged as a dealer bid. Approved dealers only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dealer_pool

> crate::models::ListVaultMarketDealerPoolsItem create_vault_market_dealer_pool()
Open a curve pool on a key (approved dealers, ID checked): linear (delta in cents) or exponential (delta in bps), spot, fee, levels per side, units it may sell, and a buy budget from your custodian account. Its ladder goes up at once.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerPoolsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dealer_pool_close

> crate::models::ListVaultMarketDealerPoolsItem create_vault_market_dealer_pool_close(id)
Close a curve pool: its legs come off the book; what filled stays filled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketDealerPoolsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_dealer_pool_preview

> crate::models::CreateVaultMarketDealerPoolPreviewResponse create_vault_market_dealer_pool_preview()
The bids and asks a set of curve settings would show, and why each side stops where it does. Places nothing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDealerPoolPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_deposit

> crate::models::CreateVaultMarketDepositResponse create_vault_market_deposit()
Open a ship-in deposit for an ungraded catalogue item (`skuId`, `variantId`, `condition` DS / VNDS / Used). It is inbound until the vault receives and checks it, then stored in your name. Returns `{ vaultItem: { id, status } }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDepositResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_donation

> crate::models::CreateVaultMarketDonationResponse create_vault_market_donation()
Give a unit to Crossly Cares (once the charity can receive; otherwise its gate message). Returns `{ id, markCents }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketDonationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity

> crate::models::CreateVaultMarketEntityResponse create_vault_market_entity()
Open an entity account: you become its admin and the custodian partner starts KYB (its own flow; Crossly keeps only its reference). Needs a verified ID and the custodian (503 names what is missing).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketEntityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_ecp

> crate::models::GetVaultMarketEntityByIdResponse create_vault_market_entity_ecp(id)
Admin: record the entity's attestation that it is an eligible contract participant. Raising its caps is still an ops decision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_halt

> crate::models::CreateVaultMarketEntityHaltResponse create_vault_market_entity_halt(id)
Admin or risk: the firm kill switch. Halts the entity (no `subaccountId`) or one sub-account: open orders on it are cancelled and no new ones are placed until cleared.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketEntityHaltResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_halt_clear

> crate::models::GetVaultMarketEntityByIdResponse create_vault_market_entity_halt_clear(id, halt_id)
Admin or risk: lift a firm halt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**halt_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_invitation

> crate::models::GetVaultMarketEntityResponse create_vault_market_entity_invitation(id)
Accept or decline an invitation (`accept`). Trading, admin and risk roles need a verified ID to accept.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_kyb

> crate::models::CreateVaultMarketEntityKybResponse create_vault_market_entity_kyb(id)
Refresh the entity's KYB status from the custodian (and its onboarding link while KYB is open).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketEntityKybResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_member

> crate::models::GetVaultMarketEntityByIdResponse create_vault_market_entity_member(id)
Admin: invite a Crossly user by email as admin, trader, risk, ops or viewer (a viewer can never place an order), with optional limits and sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_subaccount

> crate::models::GetVaultMarketEntityByIdResponse create_vault_market_entity_subaccount(id)
Admin: open a sub-account once the custodian approved the entity; its limits are held to the entity's caps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_entity_subaccount_close

> crate::models::GetVaultMarketEntityByIdResponse create_vault_market_entity_subaccount_close(id, sub_id)
Admin: close a sub-account that holds nothing and has no open orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_follow_copy_watch_alert_read

> crate::models::CreateVaultMarketFollowCopyWatchAlertReadResponse create_vault_market_follow_copy_watch_alert_read()
Mark every copy-watch alert read. Returns `{ marked }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketFollowCopyWatchAlertReadResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_gift

> crate::models::CreateVaultMarketGiftResponse create_vault_market_gift()
Give a unit in your vault to another Crossly user (by @username or email). It stays on its shelf; they have 7 days to accept. ID needed above $1,000 at the mark. Returns `{ id, expiresAt }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketGiftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_gift_accept

> crate::models::StatusResponse create_vault_market_gift_accept(id)
Accept a gift: the unit becomes yours in place. Returns `{ status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_gift_cancel

> crate::models::StatusResponse create_vault_market_gift_cancel(id)
Cancel a gift you sent before it is accepted. Returns `{ status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_gift_decline

> crate::models::StatusResponse create_vault_market_gift_decline(id)
Decline a gift: the unit stays with the giver. Returns `{ status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault()
Start Grade to Vault on one item (`source` inventory or vault_unit). With `runId` the grading slot is booked at once (free; your saved card is charged only if the run locks). Nothing is listed until you confirm a price after the grade.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault_book

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault_book(id)
Book (or re-book) the grading slot of a flow still at \"picked\". Answers 503 naming GRADING_HUB_ADDRESS when runs are not open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault_cancel

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault_cancel(id)
Stop the flow before the card goes to the grader (still picked, or its slot only reserved: nothing was charged).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault_keep

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault_keep(id)
Keep the graded unit in your vault without listing it. The flow ends; list it any time from the vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault_list

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault_list(id)
Confirm the price for the grade the unit came back with (`confirm: true`, `gradeKey` must match the unit). Pre-trade risk, the price band and the ID line apply, then one shelf ask is placed on the Crossly order book.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grade_to_vault_shipped

> crate::models::ListVaultMarketGradeToVaultItem create_vault_market_grade_to_vault_shipped(id)
You mailed the card to the grading hub (an inventory card, once its run locked).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading

> crate::models::CreateVaultMarketGradingResponse create_vault_market_grading()
Send a raw vault unit on the grading hub's shelf to a Grading Run (`runId`). It is held until it comes back, graded, to the same vault item. The slot is free; your saved card is charged only if the run locks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketGradingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading_pull

> crate::models::StatusResponse create_vault_market_grading_pull(submission_id)
Pull a vault unit out of its run before the run locks (nothing was charged). It goes back on the shelf.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_hall_loan

> crate::models::CreateVaultMarketHallLoanResponse create_vault_market_hall_loan()
Lend a unit in your Crossly vault to the Hall of Grails: { vaultItemId, credit: 'name' | 'anonymous' }. A standing loan of display rights only: the unit goes on a plinth, under a plate naming you (or \"a collector\"), in every weekly exhibition whose category it fits. It never moves and stays yours to list, sell or withdraw. Earns reputation per exhibition shown; no cash. Resumes a loan you opted out of while it still shows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketHallLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_hall_loan_credit

> crate::models::CreateVaultMarketHallLoanCreditResponse create_vault_market_hall_loan_credit(id)
Change how the plate on a lent unit's plinth names you, from now: { credit: 'name' } (your display name) or { credit: 'anonymous' } (\"a collector\").

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketHallLoanCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_hall_loan_opt_out

> crate::models::CreateVaultMarketHallLoanOptOutResponse create_vault_market_hall_loan_opt_out(id)
Opt a Hall of Grails loan out, at any time. A unit on a plinth this week finishes this exhibition (status ending, showingUntil); otherwise the loan ends now (status ended).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketHallLoanOptOutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_identity_start

> crate::models::CreateVaultMarketIdentityStartResponse create_vault_market_identity_start()
Start (or resume) the one-time ID check the money features and larger orders need. Returns `{ url }` to open, or `{ alreadyVerified: true }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketIdentityStartResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_index

> crate::models::GetVaultMarketIndexResponse create_vault_market_index()
Make an index of your own: 1 to 50 book keys, weighted by mark or equally. Up to 10 per account.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketIndexResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_index_buy

> crate::models::CreateVaultMarketIndexBuyResponse create_vault_market_index_buy(slug)
Buy the basket: one limit buy per key in the index for 1-5 whole units each, at most `maxBpsOverMark` over each key's mark, IOC or GTC. Every unit is a real item that lands in your vault. Idempotent on `clientBasketId`; returns a result per key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketIndexBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_index_complaint

> crate::models::CreateVaultMarketIndexComplaintResponse create_vault_market_index_complaint()
File a complaint about an index or the method (`slug` optional). Every complaint is answered. Returns `{ id, status, createdAt }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketIndexComplaintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_index_consultation_comment

> crate::models::OkResponse create_vault_market_index_consultation_comment(id)
Comment on an open consultation (5 to 5,000 characters; up to 5 per person). Comments are published without your name. Returns `{ ok }`.

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


## create_vault_market_instant_exit

> crate::models::CreateVaultMarketInstantExitResponse create_vault_market_instant_exit()
Sell a unit now at the quoted price: a sell IOC at `priceCents` through the pro engine. Refused, with the new quote, if the best bid dropped below it. Crossly never buys or funds it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketInstantExitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint

> crate::models::CreateVaultMarketJointResponse create_vault_market_joint()
Open a joint vault (ID first): you hold full rights and accept the current joint terms. `invite` sends invitations (@username or email, each with view, trade or full rights). Returns `{ id, status, invited }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketJointResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_close

> crate::models::CreateVaultMarketJointCloseResponse create_vault_market_joint_close(id)
Close the joint vault (full rights): open joint orders are cancelled and every unit goes back to its owner. Returns `{ status, unitsReturned }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointCloseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_invite

> crate::models::CreateVaultMarketJointInviteResponse create_vault_market_joint_invite(id)
Invite a Crossly user (full rights only). They have 14 days and show ID to join. Returns `{ userId, rights, expiresAt }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_member_remove

> crate::models::CreateVaultMarketJointMemberRemoveResponse create_vault_market_joint_member_remove(id, user_id)
Remove a member (full rights), or leave when userId is you. Their units go back to their own account and their joint orders are cancelled. Returns `{ userId, status, unitsReturned, jointStatus }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointMemberRemoveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_member_right

> crate::models::CreateVaultMarketJointMemberRightResponse create_vault_market_joint_member_right(id, user_id)
Set a member rights to view, trade or full (full rights only; the vault always keeps a full member). Returns `{ userId, rights }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointMemberRightResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_respond

> crate::models::CreateVaultMarketJointRespondResponse create_vault_market_joint_respond(id)
Accept (ID first; accepts the current joint terms) or decline an invitation. Returns `{ status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_term

> crate::models::TermsVersionResponse create_vault_market_joint_term(id)
Accept the current joint terms (needed to trade after the terms version changes). Returns `{ termsVersion }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TermsVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_trade

> crate::models::CreateVaultMarketJointTradeResponse create_vault_market_joint_trade(id)
Place a limit order for the joint vault (trade or full rights, ID shown). A buy (bookKey) is placed on you, paid from your own Vault Markets account, and what it buys is booked to the joint vault. A sell (vaultItemId of a unit in it) is placed on the unit owner of record, who gets the proceeds; pre-trade checks run against you too. Returns `{ tradeId, side, bookKey, ownerUserId, order }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointTradeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_trade_cancel

> crate::models::CreateVaultMarketJointTradeCancelResponse create_vault_market_joint_trade_cancel(id, trade_id)
Cancel a working joint order (trade or full rights); what filled stays filled. Returns `{ tradeId, order }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**trade_id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointTradeCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_unit

> crate::models::CreateVaultMarketJointUnitResponse create_vault_market_joint_unit(id)
Put units you own into the joint vault (each stored and free). They keep you as owner of record and stay on the shelf. Returns `{ added, vaultItemIds }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointUnitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_joint_unit_remove

> crate::models::CreateVaultMarketJointUnitRemoveResponse create_vault_market_joint_unit_remove(id)
Take units out of the joint vault, back to their owner own account (yours at any rights; anyone else with full rights). Returns `{ removed, vaultItemIds }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketJointUnitRemoveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_large_trader_report_file

> crate::models::ListVaultMarketLargeTraderReportsItem create_vault_market_large_trader_report_file(id)
File (or amend, or refile) a large-trader report: why you hold it (purpose), who you act for, other accounts you control, whether you plan to add, and a statement of 20+ characters. Returns the report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketLargeTraderReportsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_learn_quiz

> crate::models::CreateVaultMarketLearnQuizResponse create_vault_market_learn_quiz(concept)
Answer a concept's quiz (`answers`: the chosen choice index per question). Graded on the server; 5 tries an hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**concept** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketLearnQuizResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_live_stream_featured_lot

> crate::models::OkResponse create_vault_market_live_stream_featured_lot(id)
Feature an open lot on your live show (or `lotId: null` to take it off). Viewers hear LIVE_PIN_CHANGED. Returns `{ ok }`.

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


## create_vault_market_loan

> crate::models::GetVaultMarketCreditResponse create_vault_market_loan()
Borrow against stored vault units: they are pledged (no listing, lending or withdrawal) and the market pool lends `amountCents` into your custodian account. Needs a verified ID.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_loan_recall

> crate::models::CreateVaultMarketLoanRecallResponse create_vault_market_loan_recall(id)
Recall a lent unit: the short has 3 days to return one. Returns `{ recallDueAt }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketLoanRecallResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_loan_unit

> crate::models::GetVaultMarketCreditResponse create_vault_market_loan_unit(id)
Pledge more stored units to a loan (answers a loan call, or makes room).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_loan_unit_release

> crate::models::GetVaultMarketCreditResponse create_vault_market_loan_unit_release(id, vault_item_id)
Free one pledged unit, if what is owed still fits inside the other units' limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**vault_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_loan_unit_sell

> crate::models::CreateVaultMarketLoanUnitSellResponse create_vault_market_loan_unit_sell(id, vault_item_id)
Sell a pledged unit to repay its loan: it is listed at `priceCents` (at least the sale floor) and stays pledged until it sells; the proceeds repay the loan first and the rest is yours. Calling it again re-prices it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**vault_item_id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketLoanUnitSellResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_locate

> crate::models::ListVaultMarketLocateMineItem create_vault_market_locate()
Hold up to `qty` lendable units of `bookKey` for 15 minutes so a short can open against them (`termDays` 30/90 for a term loan). Needs lending (VAULT_LENDING_TERMS_VERSION) and the ID check. Returns the locate with how many units it holds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketLocateMineItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_long

> crate::models::CreateVaultMarketLongResponse create_vault_market_long()
Buy `units` units (default 1; at most 25/10/3/1 by liquidity tier A/B/C/D) into your vault with leverage (up to 2x) from the market staking pool, paying up to `maxCents` each. A partial fill opens with the units bought and the rest is cancelled. Returns `{ positionId, matched, units }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketLongResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_midpoint

> crate::models::ListVaultMarketMidpointItem create_vault_market_midpoint()
A hidden block order that trades only at the firm-book midpoint, with another midpoint order of at least your minimum size.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketMidpointItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_midpoint_cancel

> crate::models::ListVaultMarketMidpointItem create_vault_market_midpoint_cancel(id)
Cancel a midpoint order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketMidpointItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_mm_dmm

> crate::models::CreateVaultMarketMmLpResponse create_vault_market_mm_dmm()
Apply to be the designated market maker on a tier B or C key. Ops approves; one per key.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketMmLpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_mm_leave

> crate::models::CreateVaultMarketMmLpResponse create_vault_market_mm_leave(id)
Leave a market-maker programme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketMmLpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_mm_lp

> crate::models::CreateVaultMarketMmLpResponse create_vault_market_mm_lp()
Join a key as a Registered Liquidity Provider (ID needed). Meeting coverage earns the LP rates.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketMmLpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_mm_mass_cancel

> crate::models::CreateVaultMarketMmMassCancelResponse create_vault_market_mm_mass_cancel()
Cancel every open order (one key, or only quotes) and every resting leg in one statement: nothing fills between the first cancel and the last.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketMmMassCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_mm_mmp_reset

> crate::models::GetVaultMarketMmMmpResponse create_vault_market_mm_mmp_reset()
End a market-maker protection freeze early.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMmMmpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_opening_waitlist

> crate::models::CreateVaultMarketOpeningWaitlistResponse create_vault_market_opening_waitlist(slug)
Join a key's waitlist, optionally with a friend's invite code (`ref`). Inviting moves you up the queue; nobody is paid. Returns your place and your own code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketOpeningWaitlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_call

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_call(id)
Lender: call the loan (once it is a day old). A refinancing auction runs for about 30 hours, the rate a new lender gets climbing to the ceiling; repaid or refinanced, you are paid in full; neither by the end, the unit moves to your vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_cancel_call

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_cancel_call(id)
Lender: take back your call before any other lender has taken the loan over. The refinancing auction stops and the loan carries on as before on its terms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_refi_auction

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_refi_auction(id)
Borrower: look for a cheaper lender. A refinancing auction runs for `hours` (1-72, default 24); the rate a new lender gets climbs from `startAprBps` (default half the loan rate) to just under the loan rate now. The first lender to take it pays your lender off and the loan carries on at that lower rate. Nobody by the end: nothing changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_refi_auction_cancel

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_refi_auction_cancel(id)
Borrower: stop looking for a cheaper lender before anyone has taken the loan over. The loan carries on unchanged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_refinance

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_refinance(id)
Take over a loan in a refinancing auction (a lender call or a borrower search for a cheaper lender): you pay the old lender everything owed now (refused above `maxPayoffCents`), which becomes your loan to the same borrower at the auction rate now. Needs a verified ID, an active account and the current lending terms. Pass `requestId` to make a retry safe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_loan_repay

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_loan_repay(id)
Borrower: repay from your custodian account to the lender. Without `amountCents`, in full (principal plus interest to the second; the unit is free once the custodian has it). With `amountCents` below what is owed, a part payment: interest first, then principal, at least the interest owed now and at least $1; the loan stays open on the principal left. Pass `requestId` to make a retry safe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_offer

> crate::models::ListVaultMarketP2pOffersItem create_vault_market_p2p_offer()
Offer to lend: `principalCents` per loan at `aprBps` a year against an item (`bookKey`) or one unit (`vaultItemId`), for up to `maxLoans` loans, lasting `expiresInDays` (default 90). Nothing is held: the custodian pays from your own account when a borrower takes it. Needs a verified ID, an active account and the current lending terms.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pOffersItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_offer_cancel

> crate::models::ListVaultMarketP2pOffersItem create_vault_market_p2p_offer_cancel(id)
Withdraw one of your open offers. Loans already taken from it carry on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketP2pOffersItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_p2p_offer_take

> crate::models::GetVaultMarketP2pLoanResponse create_vault_market_p2p_offer_take(id)
Borrow: take an offer against one of your stored vault units (`vaultItemId`). The unit is pledged (no listing, lending or withdrawal) and the principal is paid into your custodian account. Interest accrues by the second; repay any time. Pass `requestId` to make a retry safe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_paper_mission_mark_check

> crate::models::CreateVaultMarketPaperMissionMarkCheckResponse create_vault_market_paper_mission_mark_check()
Answer the \"Read the mark\" mission: the quality letter (A, B or C) you read on a market page, checked against the stored mark. One answer per market, 10 a day. Returns `{ bookKey, answer, correct, quality, mission, next }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPaperMissionMarkCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_paper_order

> crate::models::CreateVaultMarketPaperOrderResponse create_vault_market_paper_order()
Place a paper order (play money only). It takes visible real liquidity now at book prices with the taker fee simulated; a GTC limit rests the rest with a queue-position estimate, filled later as the real tape reaches it. Sells only paper units you hold (no paper shorts). Never touches the real book. Returns `{ order, fills }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPaperOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_paper_order_cancel

> crate::models::CreateVaultMarketPaperOrderCancelResponse create_vault_market_paper_order_cancel(id)
Cancel a working paper order: the play cash or paper units it held come back. Returns `{ order }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketPaperOrderCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_paper_reset

> crate::models::CreateVaultMarketPaperResetResponse create_vault_market_paper_reset()
Reset your paper account to $10,000 of play money: working paper orders are cancelled, paper positions cleared, and your leaderboard week starts again. Once a day.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPaperResetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_partner_apply

> crate::models::CreateVaultMarketPartnerApplyResponse create_vault_market_partner_apply()
Apply as a Vault Drop-off Partner (a card shop that takes walk-ins for the vault). Needs the one-time ID check; ops approve.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPartnerApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_partner_batch

> crate::models::CreateVaultMarketPartnerBatchResponse create_vault_market_partner_batch()
An approved partner sends walk-ins in under its code: each item (catalogue variant, condition, grade, photos) becomes an inbound vault unit; the box gets one prepaid label. The partner earns a capped share of Crossly's fee on each unit's first sale.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPartnerBatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pool_stake

> crate::models::CreateVaultMarketPoolStakeResponse create_vault_market_pool_stake()
Stake CBX you hold at the custodian into the market pool (base units, as a string). Returns `{ stakeId, shares }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPoolStakeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pool_stake_unstake

> crate::models::CreateVaultMarketPoolStakeUnstakeResponse create_vault_market_pool_stake_unstake(id)
Unstake: paid out after the 7-day cooldown. Returns `{ cooldownUntil }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketPoolStakeUnstakeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_portfolio_margin_enroll

> crate::models::CreateVaultMarketPortfolioMarginEnrollResponse create_vault_market_portfolio_margin_enroll()
Admin: put a firm sub-account under portfolio margin. Needs VAULT_LEVERAGE_VENUE, your ID, an ECP-attested entity that passed KYB, and the minimum equity. Its positions are then margined on the whole account, recalculated every 5 minutes; calls and auctions are unchanged. Audited.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPortfolioMarginEnrollResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_portfolio_margin_leave

> crate::models::CreateVaultMarketPortfolioMarginLeaveResponse create_vault_market_portfolio_margin_leave()
Admin: take a firm sub-account off portfolio margin. Each position is then held to its own tier floors again and may be called at once. Audited.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPortfolioMarginLeaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_portfolio_margin_what_if

> crate::models::CreateVaultMarketPortfolioMarginWhatIfResponse create_vault_market_portfolio_margin_what_if()
The portfolio margin requirement of a firm sub-account before and after a trade (`bookKey`, `side` buy or sell, `units`), at the Crossly mark with fees left out. Changes nothing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPortfolioMarginWhatIfResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_position_cancel

> crate::models::OkResponse create_vault_market_position_cancel(id)
Cancel the working order: an unfilled opening order cancels the position; an unfilled cover or close leaves it open. Returns `{ ok }`.

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


## create_vault_market_position_close

> crate::models::CreateVaultMarketPositionCloseResponse create_vault_market_position_close(id)
Close a leveraged long: all its units, or `units` of them, each sold for at least `minCents`; each sale repays the pool first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketPositionCloseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_position_collateral

> crate::models::CreateVaultMarketPositionCollateralResponse create_vault_market_position_collateral(id)
Answer a margin call: a short posts more collateral (USD, or CBX at half value); a leveraged long pays down its pool loan. Returns the position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketPositionCollateralResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_position_cover

> crate::models::CreateVaultMarketPositionCloseResponse create_vault_market_position_cover(id)
Cover a short: all its units, or `units` of them, each buying any unit of the key into the vault for up to `maxCents`; each goes back to its lender.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketPositionCloseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_price_widget

> crate::models::ListVaultMarketPriceWidgetsItem create_vault_market_price_widget()
Make a price widget for one vault market (`kind` mark + `bookKey`) or one public index (`kind` index + `indexSlug`): range, layout (compact or chart), theme (auto, light, dark), optionally your own affiliate code for the link back and the sites allowed to show it (empty = any). Returns the widget with its snippets.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPriceWidgetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_price_widget_preview

> crate::models::CreateVaultMarketPriceWidgetPreviewResponse create_vault_market_price_widget_preview()
What a widget would show before you save it: `view` (value, move, quality, n, as-of, link back) and `html`, the exact page the iframe serves. Nothing is stored.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketPriceWidgetPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_bracket

> crate::models::CreateVaultMarketProBracketResponse create_vault_market_pro_bracket()
A buy order with a take-profit and/or stop-loss attached; the exits arm as an OCO pair for what the entry fills.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProBracketResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_link_cancel

> crate::models::ListVaultMarketProLinksItem create_vault_market_pro_link_cancel(id)
Cancel a linked order: its open child orders come off; a bracket's entry stays.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketProLinksItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_oco

> crate::models::CreateVaultMarketProOcoResponse create_vault_market_pro_oco()
Two orders on one key, one cancels the other: when one fills or its stop triggers, the other is cancelled (a partial fill shrinks it).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProOcoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_order

> crate::models::CreateVaultMarketProOrderResponse create_vault_market_pro_order()
Place an order on a vault book: limit, market (protected, IOC), stop or stop-limit; GTC, GTD, IOC or FOK; post-only; an iceberg `displayQuantity`; a TWAP or VWAP algo over a window. Idempotent on `clientOrderId`. `subaccountId` trades for a firm sub-account you may trade (your role, its KYB, its limits, no firm halt; buys pay from its custodian account and land in it).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_order_cancel

> crate::models::CreateVaultMarketProOrderResponse create_vault_market_pro_order_cancel(id)
Cancel an order; what filled stays filled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketProOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_order_cancel_all

> crate::models::CreateVaultMarketProOrderCancelAllResponse create_vault_market_pro_order_cancel_all()
Cancel every open order, or every one on a book key. Returns `{ cancelled }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProOrderCancelAllResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_order_replace

> crate::models::CreateVaultMarketProOrderResponse create_vault_market_pro_order_replace(id)
Amend an order's price, stop or quantity (cancel-replace: it re-queues at its new price).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketProOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_quote

> crate::models::CreateVaultMarketProQuoteResponse create_vault_market_pro_quote()
Market makers: set a two-sided post-only quote on a book key (replaces the last). With cancel-on-disconnect (default) heartbeat every 30 s or it is pulled.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_quote_heartbeat

> crate::models::CreateVaultMarketProQuoteHeartbeatResponse create_vault_market_pro_quote_heartbeat()
Keep your quotes alive (cancel-on-disconnect). Returns `{ quotes, at }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProQuoteHeartbeatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_pro_trailing_stop

> crate::models::CreateVaultMarketProTrailingStopResponse create_vault_market_pro_trailing_stop()
A trailing stop: its stop follows the best price since it was placed, `trailBps` or `trailCents` behind (under the high for a sell, over the low for a buy).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketProTrailingStopResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_profile_follow

> crate::models::CreateVaultMarketProfileFollowResponse create_vault_market_profile_follow(handle)
Follow someone with a public vault: their new vault buys appear in your feed. No copy trading.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketProfileFollowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_recurring

> crate::models::ListVaultMarketRecurringItem create_vault_market_recurring()
Start a recurring buy: one whole unit of a key every week, two weeks or month, at no more than `maxPriceCents` and/or the mark plus `capBpsOverMark`; card (with an optional backup) or custodian; optional auto-lend.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRecurringItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_rfq

> crate::models::CreateVaultMarketRfqResponse create_vault_market_rfq()
Ask for a quote on a block: dealers, the key's DMM and LPs, and holders are told, anonymously. Answers are firm for 60 s. Your limit is never shown.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketRfqResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_rfq_accept

> crate::models::CreateVaultMarketRfqAcceptResponse create_vault_market_rfq_accept(id)
Take one answer: it fills as a block through the normal fill path (fees by role, self-trade prevention, halts, LULD).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketRfqAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_rfq_cancel

> crate::models::ListVaultMarketRfqItem create_vault_market_rfq_cancel(id)
Cancel your open quote request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketRfqItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_rfq_respond

> crate::models::CreateVaultMarketRfqRespondResponse create_vault_market_rfq_respond(id)
Answer a quote request for the full quantity with a firm price, good for 60 s. Selling needs the units free in your vault; buying pays from your Vault Markets custodian account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketRfqRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_rfq_response_withdraw

> crate::models::CreateVaultMarketRfqRespondResponse create_vault_market_rfq_response_withdraw(id)
Withdraw your live answer to a quote request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketRfqRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_set_goal

> crate::models::GetVaultMarketSetGoalResponse create_vault_market_set_goal()
Start a goal on a catalogue set (category, brandSlug, modelSlug from the set search), optionally with a target: one condition, and any grade, ungraded, or one exact grade. One live goal per set.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketSetGoalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_set_goal_basket

> crate::models::ListVaultMarketSetGoalBasketsItem create_vault_market_set_goal_basket(id)
Place the \"complete my set\" basket bid: one GTC limit buy per missing piece, under a hard budget, paid by one of your saved cards or your custodian account. Every leg passes pre-trade risk and the ID line like any order. Idempotent on clientBasketId. One working basket per goal; its legs are cancelled once the set is complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketSetGoalBasketsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_set_goal_basket_cancel

> crate::models::ListVaultMarketSetGoalBasketsItem create_vault_market_set_goal_basket_cancel(id)
Cancel the goal working basket bid: every working leg comes off the book; what already filled stays in your vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketSetGoalBasketsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_share_card

> crate::models::ListVaultMarketShareCardsItem create_vault_market_share_card()
Make a verified share card of a position, a vault unit, a public index or your portfolio (`kind`, `refId`). The % on it is the price move since your real fill; amounts stay hidden unless `showAmounts`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketShareCardsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_short

> crate::models::CreateVaultMarketLongResponse create_vault_market_short()
Short a book key: borrow `units` lent units (default 1; at most 25/10/3/1 by liquidity tier A/B/C/D) and sell each at `limitCents` or better, posting collateral for all of them (50% of the mark each; CBX counts at half, up to 30%). All units are borrowed or none unless `allowPartial`. A partial sale opens with the units sold. Returns `{ positionId, matched, units }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketLongResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_supply_batch_label

> crate::models::GetVaultMarketSupplyBatchResponse create_vault_market_supply_batch_label(id)
Buy a box's prepaid label again after it failed (for example once a ship-from address is saved). A box that has a label keeps it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketSupplyBatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_supply_invite

> crate::models::IdStatusResponse create_vault_market_supply_invite(id)
Accept or decline a Market Open invite (`accept`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_supply_send

> crate::models::CreateVaultMarketSupplySendResponse create_vault_market_supply_send()
Send inventory items matched to a catalogue variant to the vault as one box: each becomes an inbound vault unit, and the box gets one prepaid inbound label (a PDF to print). Optional: auto-list each unit in the vault book at a share of the mark once stored, and lend when idle. Items that cannot go say why. Crosslisted listings stay live.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketSupplySendResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_term

> crate::models::TermsVersionResponse create_vault_market_term()
Accept the current lending terms (needed to lend or short). Returns `{ termsVersion }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::TermsVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_claim_object

> crate::models::CreateVaultMarketTodClaimObjectResponse create_vault_market_tod_claim_object(id)
Stop a transfer-on-death claim made on your own account. Returns `{ id, status, estateUnlocked }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketTodClaimObjectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_my_claim

> crate::models::CreateVaultMarketTodMyClaimResponse create_vault_market_tod_my_claim()
Start a claim as a named beneficiary (ID first), naming the account holder by @username or email. It stays a draft until you add documents and submit. Returns `{ id, status, existing }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketTodMyClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_my_claim_document

> crate::models::CreateVaultMarketTodMyClaimDocumentResponse create_vault_market_tod_my_claim_document(id)
Add one document to a draft claim (or one ops asked more of): kind, filename and the file as base64 (PDF, JPEG or PNG, up to 8 MB). Kept privately for ops review. Returns `{ id, kind, label, filename, bytes }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketTodMyClaimDocumentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_my_claim_submit

> crate::models::IdStatusResponse create_vault_market_tod_my_claim_submit(id)
Send a claim for ops review (it needs a death certificate). The account holder is told and can stop it. Returns `{ id, status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_my_claim_withdraw

> crate::models::IdStatusResponse create_vault_market_tod_my_claim_withdraw(id)
Withdraw a claim before it is released. Returns `{ id, status }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_tod_term

> crate::models::TermsVersionResponse create_vault_market_tod_term()
Accept the current transfer-on-death terms. Returns `{ termsVersion }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::TermsVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_unit_evidence_scan

> crate::models::GetVaultMarketUnitEvidenceResponse create_vault_market_unit_evidence_scan(id)
Add a finished scan of your own (`captureId`, of the same catalogue item) to your vault unit. Shown as added by the owner, not as intake evidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitEvidenceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_unit_sell_now

> crate::models::CreateVaultMarketInstantExitResponse create_vault_market_unit_sell_now(id)
Sell one unit of this item now into the best bid (Instant Exit: an immediate-or-cancel pro order), never below `minCents`. Returns `{ status: filled | missed, filledCents, quote, order }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketInstantExitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_valuation_statement

> crate::models::CreateVaultMarketValuationStatementResponse create_vault_market_valuation_statement()
Issue an insurance or appraisal statement for a day: frozen with the sha256 of its record, sealed in the Crossly audit chain, with a public verify link. preparedFor and policyRef are printed on it and never shown on verify. Moves no money.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketValuationStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_watchlist

> crate::models::CreateVaultMarketWatchlistResponse create_vault_market_watchlist()
Watch a book key. Returns `{ bookKey, watching }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketWatchlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_widget_token

> crate::models::CreateVaultMarketWidgetTokenResponse create_vault_market_widget_token()
Mint a widget token for one phone (`deviceName`, `installId`); minting again with the same pair replaces that phone token. The token reads the widget summary and nothing else, and is returned once. At most five phones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketWidgetTokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_receipt_token_confirm

> crate::models::CreateVaultReceiptTokenConfirmResponse create_vault_receipt_token_confirm(id)
After your wallet sent the export: read the chain and mark the token live once its one token exists. Pass the `signature` your wallet returned to record it (kept only if it landed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultReceiptTokenConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_receipt_token_export

> crate::models::CreateVaultReceiptTokenExportResponse create_vault_receipt_token_export()
Send one unit (`vaultItemId`) warehouse receipt to one of your verified Solana wallets (`wallet`) as a token, accepting the current terms (`acceptTerms`). Returns a base64 transaction already signed by the new mint and the receipts authority: your wallet pays the rent and must sign and send it within about two minutes, then call confirm. Nothing is minted until it does. Needs a verified ID; the unit must be stored, unpledged, not lent and not for sale.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultReceiptTokenExportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_receipt_token_redeem

> crate::models::CreateVaultReceiptTokenRedeemResponse create_vault_receipt_token_redeem(id)
Redeem a receipt token held in one of your verified Solana wallets (`wallet`): returns an unsigned base64 burn your wallet signs and sends (it also closes the token account and returns its rent to you). Once it lands, call redeem/confirm and the unit is yours in the vault. Refused while the token is frozen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultReceiptTokenRedeemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_receipt_token_redeem_confirm

> crate::models::CreateVaultReceiptTokenRedeemConfirmResponse create_vault_receipt_token_redeem_confirm(id)
After your wallet sent the redeem: read the chain; once the token is burned the unit moves into your vault (if it was not yours already). Pass the `signature` to record it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultReceiptTokenRedeemConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_receipt_token_wallet_allow

> crate::models::CreateVaultReceiptTokenWalletAllowResponse create_vault_receipt_token_wallet_allow()
Put one of your verified Solana wallets (`wallet`) on the receipt-token allow-list so it can hold and receive receipt tokens. Returns a base64 transaction already signed by the receipts authority: the wallet pays the small rent (about 0.001 SOL) and must sign and send it within about two minutes. Needs a verified ID and a wallet that passes the sanctions screen.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultReceiptTokenWalletAllowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_agent_watch

> crate::models::ListVaultMarketAgentWatchesItem delete_vault_market_agent_watch(id)
Cancel a live price watch. Returns the watch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketAgentWatchesItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_alert

> crate::models::OkResponse delete_vault_market_alert(id)
Delete a price alert. Returns `{ ok }`.

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


## delete_vault_market_bullion_family_peg

> crate::models::ListVaultMarketBullionFamilyPegsItem delete_vault_market_bullion_family_peg(id)
Stop a family-wide quote: every leg order comes off the book; what filled stays filled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketBullionFamilyPegsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_bullion_peg

> crate::models::ListVaultMarketBullionPegsItem delete_vault_market_bullion_peg(id)
Stop a spot-pegged quote: its resting order comes off the book; what filled stays filled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketBullionPegsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_dealer_band

> crate::models::ListVaultMarketDealerBandsItem delete_vault_market_dealer_band()
Stop a band on a key; its standing bid comes off.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerBandsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_entity_member

> crate::models::OkResponse delete_vault_market_entity_member(id, member_id)
Admin: remove a member (or leave, yourself). An entity always keeps one admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**member_id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_follow_copy_watch

> crate::models::DeleteVaultMarketFollowCopyWatchResponse delete_vault_market_follow_copy_watch(handle)
Stop copy-watching a collector.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketFollowCopyWatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_index

> crate::models::OkResponse delete_vault_market_index(slug)
Delete an index of your own. Returns `{ ok }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_locate

> crate::models::ListVaultMarketLocateMineItem delete_vault_market_locate(id)
Release a live locate before it lapses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketLocateMineItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_opening_waitlist

> crate::models::DeleteVaultMarketOpeningWaitlistResponse delete_vault_market_opening_waitlist(slug)
Leave a key's waitlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketOpeningWaitlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_price_widget

> crate::models::DeleteVaultMarketPriceWidgetResponse delete_vault_market_price_widget(id)
Delete a widget. Pages that still paste it show that it is no longer available. Returns `{ id, archived }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketPriceWidgetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_pro_quote

> crate::models::DeleteVaultMarketProQuoteResponse delete_vault_market_pro_quote()
Pull your quote on a book key, or every quote. Returns `{ pulled }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteVaultMarketProQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_profile_follow

> crate::models::CreateVaultMarketProfileFollowResponse delete_vault_market_profile_follow(handle)
Stop following someone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketProfileFollowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_recurring

> crate::models::ListVaultMarketRecurringItem delete_vault_market_recurring(id)
Stop a recurring buy for good; its history stays.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketRecurringItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_set_goal

> crate::models::DeleteVaultMarketSetGoalResponse delete_vault_market_set_goal(id)
Remove a set goal. A working basket bid is cancelled first; what filled stays in your vault.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketSetGoalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_share_card

> crate::models::OkResponse delete_vault_market_share_card(id)
Take a share card down: its link and image stop working. Returns `{ ok }`.

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


## delete_vault_market_supply_round_up

> crate::models::GetVaultMarketSupplyRoundUpResponse delete_vault_market_supply_round_up()
Stop the round-up (its history stays; an armed buy is cancelled).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketSupplyRoundUpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_unit_evidence

> crate::models::GetVaultMarketUnitEvidenceResponse delete_vault_market_unit_evidence(id, evidence_id)
Take back a scan you added to your vault unit (intake evidence stays).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**evidence_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitEvidenceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_watchlist

> crate::models::CreateVaultMarketWatchlistResponse delete_vault_market_watchlist()
Stop watching a book key (`?bookKey=`). Returns `{ bookKey, watching }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketWatchlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_widget_token

> crate::models::DeleteVaultMarketWidgetTokenResponse delete_vault_market_widget_token()
Turn the widget off on every phone: revokes all your widget tokens. Returns `{ revoked }`, how many.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteVaultMarketWidgetTokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_vault_market_widget_token_by_id

> crate::models::DeleteVaultMarketWidgetTokenByIdResponse delete_vault_market_widget_token_by_id(id)
Turn the widget off on one phone: revokes that widget token at once. Only a widget token can be revoked here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketWidgetTokenByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_attestation

> crate::models::GetVaultAttestationResponse get_vault_attestation()
Vault attestations: independent physical counts, the insurance certificate (with a warning when insured value in the vault exceeds its aggregate or per-occurrence limit), SOC 1 Type II and SOC 2 reports. Each carries its signed document.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultAttestationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_account

> crate::models::GetVaultMarketAccountResponse get_vault_market_account()
Which Vault Markets registrations are live (and the env each waits on), your custodian account, limits and balances.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAccountResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_adl

> crate::models::GetVaultMarketAdlResponse get_vault_market_adl()
The insurance fund (booked and free to draw), the most one event may draw before auto-deleveraging (ADL) starts, the loss waterfall in order, and the latest ADL events (no user or position ids). Money in cents.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAdlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_adl_position

> crate::models::ListVaultMarketAdlPositionsItem get_vault_market_adl_position(id)
One of your positions with its ADL lights (0 once it has ended).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketAdlPositionsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_advance_quote

> crate::models::GetVaultMarketAdvanceQuoteResponse get_vault_market_advance_quote()
What an advance on a vault unit for sale could be: up to half of the lower of its mark and asking price, free for 60 days.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAdvanceQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_agent_draft

> crate::models::ListVaultMarketAgentDraftsItem get_vault_market_agent_draft(id)
One drafted order: item, side, units, limit (cents), the book when drafted, status, and the orderBody that places it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketAgentDraftsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_alert_pref

> crate::models::GetVaultMarketAlertPrefResponse get_vault_market_alert_pref()
Your vault alert settings: every kind (price, fill, margin_call, adl, p2p_loan, auction, market_open, receipt_token, set_goal, follow, account), every channel (push, telegram, terminal, email, webhook) with whether it is set up, the full on/off grid, the locked switches (a critical kind always pushes) and quiet hours.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAlertPrefResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_auction_lot

> crate::models::GetVaultMarketAuctionLotResponse get_vault_market_auction_lot(id)
One lot: the price showing, the next minimum bid, whether a reserve exists and is met, the end (extended by late bids), the bid history and your standing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketAuctionLotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_auction_mine

> crate::models::GetVaultMarketAuctionMineResponse get_vault_market_auction_mine()
Lots you consigned (with your reserve) and lots you bid on (with your maximum and whether you lead).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAuctionMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_block

> crate::models::ListVaultMarketBlocksItem get_vault_market_block(order_id)
One block order: its order, plan lines (planned and allocated units, average price, cash share, status).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketBlocksItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_bullion_for

> crate::models::GetVaultMarketBullionForResponse get_vault_market_bullion_for()
The bullion key families one book key belongs to, with spot for its metal as a labelled reference and its melt value at that spot. Empty families when the item is not keyed bullion.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketBullionForResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_bullion_key

> crate::models::GetVaultMarketBullionKeyResponse get_vault_market_bullion_key(slug)
One bullion key family: its catalogue products, every live book key with best bid, best ask, vault units and its mark (from the book only), live pegged quote counts, and spot as a labelled reference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketBullionKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_bullion_peg_preview

> crate::models::GetVaultMarketBullionPegPreviewResponse get_vault_market_bullion_peg_preview()
The price a spot-pegged quote would rest at now (spot plus a premium in cents or basis points, clamped to floor and cap). No price when spot is stale or missing: such a quote waits off the book.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketBullionPegPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_call

> crate::models::GetVaultMarketCallResponse get_vault_market_call(book_key)
Recent call-auction uncrosses on a book key: kind (open, reopen, periodic, daily, buy-in, sell-off), clearing price in cents and units.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketCallResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_capacity

> crate::models::GetVaultMarketCapacityResponse get_vault_market_capacity()
Capacity from the book: `bookKey` (+ `units`) gives ADV, depth within ±5% of the mark, vault stock, lendable units and days to liquidate at 20% of ADV; `index` + `notional` (cents) gives how much of the basket the asks fill now and the days the rest needs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCapacityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_channel

> crate::models::GetVaultMarketChannelResponse get_vault_market_channel()
Your Vault Markets channel switches: Telegram alerts and digest, the daily cap on vault nudges by push (default 5), whether a Telegram chat is linked, and how many nudges went out today.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketChannelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_compare

> crate::models::GetVaultMarketCompareResponse get_vault_market_compare()
Up to 5 book keys and/or index slugs (`items`, comma-separated) as daily series on the same days, each rebased to 100.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCompareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_confirmation

> crate::models::GetVaultMarketConfirmationResponse get_vault_market_confirmation(trade_id)
One trade confirmation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trade_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketConfirmationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_confirmation_pdf

> std::path::PathBuf get_vault_market_confirmation_pdf(trade_id)
One trade confirmation as a PDF.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trade_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_correlation

> crate::models::GetVaultMarketCorrelationResponse get_vault_market_correlation()
The correlation matrix of the sector indexes' weekly returns over `weeks` (default 52), pairwise-complete, with n per pair; a pair under 26 shared weeks has no number.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCorrelationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_creator_attribution

> crate::models::GetVaultMarketCreatorAttributionResponse get_vault_market_creator_attribution()
The creator who brought you to Vault Markets, when one did: when it started and ends, whether it still earns, and the disclosure. Your prices and fees are the same either way.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreatorAttributionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_creator_dashboard

> crate::models::GetVaultMarketCreatorDashboardResponse get_vault_market_creator_dashboard()
For creators: whether you joined the programme, each of your codes with traders attributed and still earning (counts only), shares accrued, paid and voided (cents) and the latest shares.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreatorDashboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_creator_programme

> crate::models::GetVaultMarketCreatorProgrammeResponse get_vault_market_creator_programme()
The Vault Markets creator programme: the share of the fee Crossly keeps that a creator earns, the per-fill and lifetime caps (cents), the attribution window in days, the 2% net floor it sits under, and the disclosure words.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreatorProgrammeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_creator_room

> crate::models::GetVaultMarketCreatorRoomResponse get_vault_market_creator_room()
For creators: your vault shelf room (applied, verified, rejected or paused) and whether your public vault profile is on, which the room needs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketCreatorRoomResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_credit

> crate::models::GetVaultMarketCreditResponse get_vault_market_credit(id)
One loan or advance with its pledged units and its money log (draws, interest, repayments, sale proceeds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dcm

> crate::models::GetVaultMarketDcmResponse get_vault_market_dcm()
The partner exchange (DCM) that lists and clears futures and options on Crossly indexes (needs VAULT_INDEX_DCM + VAULT_INDEX_DCM_API_URL + VAULT_INDEX_DCM_API_KEY), and its contracts as last read: perps, futures and options with the index each settles on, multiplier, tick, expiry and strike. Read-only: Crossly is the licensor and data provider, never the venue.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketDcmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dcm_contract

> crate::models::GetVaultMarketDcmContractResponse get_vault_market_dcm_contract(symbol)
One partner contract: its spec, its funding prints (newest first; the partner computes and settles funding), annualised funding over the last 21 prints, and the Crossly index level it settles on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketDcmContractResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dcm_contract_book

> crate::models::GetVaultMarketDcmContractBookResponse get_vault_market_dcm_contract_book(symbol)
The partner exchange live order book for a contract (cached 5 seconds), with mark, index, last, open interest, 24 h volume, spread in bps and the basis of its mark over the Crossly index in bps. Prices are index points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketDcmContractBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dcm_options_input

> crate::models::GetVaultMarketDcmOptionsInputResponse get_vault_market_dcm_options_input(slug)
Options inputs for an index (indexes only, never a single item): Crossly realised volatility (raw and Geltner-desmoothed, from weekly returns), the forward (the partner future mark, else the Crossly close), Black-76 greeks on a strike x expiry grid and on every option the partner lists on the index. Model inputs, not quotes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketDcmOptionsInputResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dcm_position

> crate::models::GetVaultMarketDcmPositionResponse get_vault_market_dcm_position()
The partner exchange account linked to your account (or to the firm sub-account `subaccountId`) and its positions, read-only (cached 15 seconds); index futures on category indexes count as portfolio-margin hedges for that account: contracts, side, entry and mark price, unrealised P&L and margin in cents, liquidation price and notional in cents. Not linked: `linked: false`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketDcmPositionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dealer

> crate::models::GetVaultMarketDealerResponse get_vault_market_dealer()
Your dealer status (applied, approved, revoked) and your Instant Exit bands.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketDealerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_dealer_pool

> crate::models::ListVaultMarketDealerPoolsItem get_vault_market_dealer_pool(id)
One of your curve pools, with its ladder and live legs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketDealerPoolsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_digest_preview

> crate::models::GetVaultMarketDigestPreviewResponse get_vault_market_digest_preview()
This week's vault digest as it would go out: units at the display marks (unmarked units counted, not valued), the like-for-like change from a week ago, offers on your units, watched keys that got a first mark, and one Learn card.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketDigestPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_entity

> crate::models::GetVaultMarketEntityResponse get_vault_market_entity()
The entities (funds, LLCs, trusts, SPVs) you belong to, and invitations waiting for you.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketEntityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_entity_by_id

> crate::models::GetVaultMarketEntityByIdResponse get_vault_market_entity_by_id(id)
One entity: members and roles, sub-accounts with their limits, firm halts, caps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_estimate

> crate::models::GetVaultMarketEstimateResponse get_vault_market_estimate(book_key)
The Crossly Estimate for a book key — a model, not a price. Method (own trades, index-adjusted last trade, grade ratio), cents with 68% and 95% bands, High/Medium/Low from the walk-forward backtest's FSD, and n. Hidden (`available: false`) under 50 backtested trades or when FSD > 0.30. Never used for margin, loans or settlement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEstimateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_execution_quality

> crate::models::GetVaultMarketExecutionQualityResponse get_vault_market_execution_quality()
The quarterly execution-quality report (Rule 605 style): fill rates, speed, effective vs quoted spread and shortfall by order type and size, and the busiest keys. `quarter` like 2026-Q4; the current quarter is provisional.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketExecutionQualityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_fee_quote

> crate::models::GetVaultMarketFeeQuoteResponse get_vault_market_fee_quote()
Your fee for an order at a price: as maker (the order rests) and as taker (it fills at once), with the programme that sets your rate and any Crossly One trim. A buyer's fee is added to the price; a seller's comes out of the proceeds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketFeeQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_fee_schedule

> crate::models::GetVaultMarketFeeScheduleResponse get_vault_market_fee_schedule()
The fee schedule by role: maker and taker rates for retail, 30-day volume, Registered LPs and DMMs, the CBX discount, the referral share and the 2% net floor Crossly always keeps.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketFeeScheduleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_follow_sharing

> crate::models::GetVaultMarketFollowSharingResponse get_vault_market_follow_sharing()
Your sharing switches: the public vault profile, new buys, units sent in, sales, Hall loans, bids and asks for copy-watchers, and the leaderboards. All off until you turn them on.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketFollowSharingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_gem_rate

> crate::models::GetVaultMarketGemRateResponse get_vault_market_gem_rate(book_key)
The Crossly gem rate: of cards graded through Crossly, the share that came back at each grader's gem grade, for this item and its category, with a Wilson 95% band. 10 returns per grader or no number. Not a grader population report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketGemRateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_grade_to_vault

> crate::models::ListVaultMarketGradeToVaultItem get_vault_market_grade_to_vault(id)
One Grade to Vault flow, brought up to date with its submission and unit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketGradeToVaultItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_grade_to_vault_candidate

> crate::models::GetVaultMarketGradeToVaultCandidateResponse get_vault_market_grade_to_vault_candidate()
What you can start Grade to Vault on: raw inventory matched to a catalogue item, and raw vault units on the grading hub shelf; plus whether Grading Runs are open.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketGradeToVaultCandidateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_grading_signal

> crate::models::GetVaultMarketGradingSignalResponse get_vault_market_grading_signal()
For a raw book key: the marks of the same item's graded keys, the spread over raw, grading cost per grader, the margin tier grading lifts it to, your raw vault units it pays to grade, and where to act.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketGradingSignalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_hall_loan

> crate::models::GetVaultMarketHallLoanResponse get_vault_market_hall_loan()
Your Hall of Grails desk: this week's exhibition, the rules (maxLive loans, shownPerHolder per exhibition, repPoints per exhibition shown), reputation (exhibitions finished), your loans (status active, ending or ended; onShowNow, showingUntil, exhibitions, the next exhibition it fits, the Lent to the Hall badge) and the vault units you could lend (canLend, or the blocker in words, and the next exhibition each fits). Display only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketHallLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_health

> crate::models::GetVaultMarketHealthResponse get_vault_market_health()
Portfolio health of your vault holdings at Crossly marks: concentration by single item, category, grade and grader (shares, HHI on 0 to 10000, its band and effective number of holdings), value by liquidity tier, the stress grid (each category and everything down 10, 20 and 30 percent), value by mark quality, plain observations, and the held items a what-if can run on. Information only, not advice.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketHealthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_health_what_if

> crate::models::GetVaultMarketHealthWhatIfResponse get_vault_market_health_what_if(book_key, move_bps)
What if one held item moved by moveBps (-10000 to 10000 basis points of its mark): the item, your vault total, the item and its category share of the vault, and the single-item HHI, before and after. A scenario on the mark, not a price you could sell at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |
**move_bps** | **i32** |  | [required] |

### Return type

**crate::models::GetVaultMarketHealthWhatIfResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_home_unit

> crate::models::GetVaultMarketHomeUnitResponse get_vault_market_home_unit()
\"Show my vault\": the vault units you hold, the ones standing in your Crossly World home first. Each: vaultItemId, title, grade, imageUrl, bookKey (its market), inHome, visitors (visitors to your home see it). Also your home (slug, privacy, path), max units one home shows, shown, visitorsSee and a note on who sees what. Your home is made on first read. No prices.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketHomeUnitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index

> crate::models::GetVaultMarketIndexResponse get_vault_market_index(slug)
One index with its daily closes (oldest first) and its level now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_calendar

> crate::models::GetVaultMarketIndexCalendarResponse get_vault_market_index_calendar()
The next four quarterly reviews: reference day (2nd Friday of Mar/Jun/Sep/Dec), announcement day, last close under the old membership and the effective day; June and December also review the Blue-chip 50.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketIndexCalendarResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_consultation

> crate::models::ListVaultMarketIndexConsultationsItem get_vault_market_index_consultation(id)
One consultation with its proposal, versions, dates, outcome and the public comments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketIndexConsultationsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_feed

> crate::models::GetVaultMarketIndexFeedResponse get_vault_market_index_feed(slug)
The licensed index feed for one built-in index (needs VAULT_INDEX_DCM + VAULT_INDEX_FEED_SECRET): the level now (intraday from A/B marks, or the last close below 50% constituent coverage), the last close, the frozen 7-day TWAP settlement value and every constituent with its mark source, quality and trade count. Signed: `x-crossly-signature: t=<ms>,v1=<hex HMAC-SHA256(secret, t + \".\" + body)>`. Every read is logged for the licence. Crossly is the licensor and data feed only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexFeedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_feed_funding

> crate::models::GetVaultMarketIndexFeedFundingResponse get_vault_market_index_feed_funding(slug)
Reference funding for a perpetual-style future on the index: your premium `premiumBps` (TWAP of (impact mid - index)/index) through F = P + clamp(I - P, ±0.05%) per 8 h, |F| capped at 0.5%/day, with the index level it is relative to. The partner DCM computes and settles funding; this is the reference math.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexFeedFundingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_feed_settlement

> crate::models::GetVaultMarketIndexFeedSettlementResponse get_vault_market_index_feed_settlement(slug)
The frozen 7-day TWAP settlement series for a licensed index (written once a day, never restated), with the closes behind each value. Signed like the feed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexFeedSettlementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_level

> crate::models::GetVaultMarketIndexLevelResponse get_vault_market_index_level(slug)
An index's daily closes (up to 5 years), oldest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexLevelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_licence_usage

> crate::models::GetVaultMarketIndexLicenceUsageResponse get_vault_market_index_licence_usage()
Your licensed index feed reads over `days`, per index and endpoint (licence-usage report).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketIndexLicenceUsageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_index_methodology

> crate::models::GetVaultMarketIndexMethodologyResponse get_vault_market_index_methodology()
The index method in machine-readable form: version (semver), inputs, weightings and caps, the family and its screens, the engines and their publication gate, revisions and the settlement series, the calendar, governance and limitations.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketIndexMethodologyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_instant_exit

> crate::models::GetVaultMarketInstantExitResponse get_vault_market_instant_exit()
The Instant Exit quote for one vault unit you hold: the best resting bid on its book key (anyone's, dealers' included), its share of the mark, the bid depth, and whether it can be taken now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketInstantExitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_joint

> crate::models::GetVaultMarketJointResponse get_vault_market_joint()
Your joint vaults and the invitations waiting for you, with the gate (open, missing env), the joint terms in words and what each right allows. A JointVaultsHome object, not a collection.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketJointResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_joint_by_id

> crate::models::GetVaultMarketJointByIdResponse get_vault_market_joint_by_id(id)
One joint vault (active members only): members and rights, the units in it at the mark with each owner of record, totals, recent joint trades, and the activity from the audit trail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketJointByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_key_stat

> crate::models::GetVaultMarketKeyStatResponse get_vault_market_key_stat(book_key)
Everything derived about one book key in one read: the Crossly Estimate, the liquidity score, risk statistics and the gem rate. A part that cannot be read is `{ error }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketKeyStatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_learn

> crate::models::GetVaultMarketLearnResponse get_vault_market_learn()
The Learn hub: each concept with whether you passed its quiz. Passing earns cosmetic pins only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLearnResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_learn_by_concept

> crate::models::GetVaultMarketLearnByConceptResponse get_vault_market_learn_by_concept(concept)
One Learn concept: its sections and quiz questions (no answers).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**concept** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketLearnByConceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_lending_election

> crate::models::GetVaultMarketLendingElectionResponse get_vault_market_lending_election()
Your lender election: `returnInKind` (shorts post 175% collateral, a 14-day buy-in, never cash-settled) and `allowTerm` (your units may go out on 30/90-day fixed-rate term loans, not recallable before term).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLendingElectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_lending_programme

> crate::models::GetVaultMarketLendingProgrammeResponse get_vault_market_lending_programme()
Your lender programme report: lendable units, units out, utilisation, fee revenue (accrued / paid), term loans and return-in-kind loans in force, and each loan's rate against the key's rate today.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLendingProgrammeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_lending_statement

> crate::models::GetVaultMarketLendingStatementResponse get_vault_market_lending_statement()
Your daily collateral statement as a lender (today live, or `day`): per open loan the mark and its quality, fees owed to you, the short's collateral share at the custodian and its coverage against 105% of mark + fees. A unit without a mark usable for risk shows no coverage number.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLendingStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_liquidity

> crate::models::GetVaultMarketLiquidityResponse get_vault_market_liquidity(book_key)
The liquidity score (0-100) of a book key and every component behind it: trades in 30/90 days, distinct counterparties, median days between trades, time-weighted quoted spread, two-sided share and depth within 5% (hourly book snapshots), Amihud and Roll. Each component is a percentile within the category. No score under the data threshold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketLiquidityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_loan_quote

> crate::models::GetVaultMarketLoanQuoteResponse get_vault_market_loan_quote()
How much you can borrow against vault units (`vaultItemIds`, comma-separated; none = every stored unit you hold): each unit's tier, mark and limit, the pool's rate and yours.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLoanQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_loan_unit_sale_floor

> crate::models::GetVaultMarketLoanUnitSaleFloorResponse get_vault_market_loan_unit_sale_floor(id, vault_item_id)
The lowest price a pledged unit can be sold at to repay its loan: after the sale (less the vault fee and storage) what is still owed must fit inside the other units' limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**vault_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketLoanUnitSaleFloorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_locate

> crate::models::GetVaultMarketLocateResponse get_vault_market_locate()
A locate quote: how many units of `key` (a book key) you could borrow now (up to `qty`), the indicative rate (APR bps; `termDays` 30 or 90 for a fixed-rate term loan), and the collateral they need (150%, or 175% for units whose lenders elected return in kind only). Holds nothing; POST to hold.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLocateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_mark

> crate::models::GetVaultMarketMarkResponse get_vault_market_mark(book_key)
One book key mark with everything behind it: the price in cents, its quality (A trade-anchored, B firm two-sided, C indicative, none), its confidence in bps and the band in cents, when it was computed, whether risk may act on it (judged for margin), the related key it was implied from (impliedFrom: a grade, the model median of the other sizes, or an index; null when none), every stored input, and the learned relations it may be implied from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketMarkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_market

> crate::models::GetVaultMarketMarketResponse get_vault_market_market()
One book key (variantId:condition:gradeKey): its mark and history, lending depth and borrow rate, open interest and live auctions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMarketResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_market_model

> crate::models::GetVaultMarketMarketModelResponse get_vault_market_market_model(book_key)
How a book key trades: its liquidity tier (A continuous, B call auction every 15 minutes, C daily call, D listings only), the call collecting now with its indicative price and volume and nominal close, the next call, the LULD band and any limit state, pause or halt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketMarketModelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_mm

> crate::models::GetVaultMarketMmResponse get_vault_market_mm()
Your market-maker programmes, the last 30 days of obligations, rebates earned (paid only from fees already taken), and the rules.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_mm_key

> crate::models::GetVaultMarketMmKeyResponse get_vault_market_mm_key()
A key's designated market maker (by the name it chose) and how many LPs quote it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMmKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_mm_mmp

> crate::models::GetVaultMarketMmMmpResponse get_vault_market_mm_mmp()
Your market-maker protection: the fill trigger, the freeze, and whether you are frozen now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMmMmpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_mode

> crate::models::GetVaultMarketModeResponse get_vault_market_mode()
Your Vault Markets mode: simple (buy, sell and what it is worth) or pro (the full terminal), whether you chose it or it was set for you (new people start in simple; people already trading start in pro), and when.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketModeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_nav

> crate::models::GetVaultMarketNavResponse get_vault_market_nav()
Mark NAV, bid NAV (the resting bids) and liquidation NAV (the bids at 20% of ADV for 10 days), holding by holding, each with its ASC 820 level, mark quality and source, and your administrator's price when uploaded.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketNavResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_nav_file

> String get_vault_market_nav_file(format)
The NAV as a csv, pdf or json file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_opening

> crate::models::ListVaultMarketOpeningItem get_vault_market_opening(slug)
One opening key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketOpeningItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_oracle

> crate::models::GetVaultMarketOracleResponse get_vault_market_oracle()
Crossly as an oracle publisher (needs CROSSLY_ORACLE_PUBLISHER_KEY + CROSSLY_ORACLE_NETWORK): the network (pyth or switchboard), the ed25519 key every value is signed with, each published index feed with its latest attempt, and 24 h counts of published, skipped and failed. Only built-in indexes priced from A/B marks are published; Crossly never reads its own feed back.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketOracleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_oracle_by_slug

> crate::models::GetVaultMarketOracleBySlugResponse get_vault_market_oracle_by_slug(slug)
The latest published oracle value for an index: level, fixed-point price and confidence at the feed exponent, confidence in bps, staleness in seconds, coverage, the exact signed message and its ed25519 signature with the signer key, and whether it is fresh.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketOracleBySlugResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_order_handling

> crate::models::GetVaultMarketOrderHandlingResponse get_vault_market_order_handling()
How orders are handled (the 606/607-style disclosure): one book, self-trade prevention, protected market orders, liquidations by auction, halts, dealers and fees.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketOrderHandlingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_p2p_dashboard

> crate::models::GetVaultMarketP2pDashboardResponse get_vault_market_p2p_dashboard()
Peer-to-peer vault loans at a glance: what you owe as a borrower and are owed as a lender (principal + interest to the second), daily interest, called loans, your open offers, the gates and the rules (auction hours, rate ceiling, size bounds, call grace).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketP2pDashboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_p2p_loan

> crate::models::GetVaultMarketP2pLoanResponse get_vault_market_p2p_loan(id)
One P2P loan with its money log (fund, repay, refinance). A loan in a running refinancing auction can be read by any lender, without the log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketP2pLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_paper

> crate::models::GetVaultMarketPaperResponse get_vault_market_paper()
Your paper trading account (play money only, never real or convertible; opened at $10,000 the first time): play cash, buying power, paper positions at the real marks, P&L, working paper orders, recent paper fills and this week on the leaderboard.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPaperResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_paper_quote

> crate::models::GetVaultMarketPaperQuoteResponse get_vault_market_paper_quote()
What a paper order would fill against the real book now, without placing it: units fillable inside the limit (a market order uses the 5% protection band around the mark), average price, simulated fee, and the queue ahead of any rest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPaperQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_partner_me

> crate::models::GetVaultMarketPartnerMeResponse get_vault_market_partner_me()
Your Vault Drop-off Partnership (null when you never applied): status, partner code, fee share and cap, totals and batches.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPartnerMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pool

> crate::models::GetVaultMarketPoolResponse get_vault_market_pool()
The CBX market staking pool: size, value, what it lends, utilisation, rate, insurance fund.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pop_report

> crate::models::GetVaultMarketPopReportResponse get_vault_market_pop_report(book_key)
Grading population report for one market (`bookKey`): focus (on a graded market: its grade, pop = copies graded at exactly that grade, higher = copies graded better by the same grader, total, asOf and source), graders (each grader ladder for the card, best grade first, from one report: the newest, the population feed over a hand entry on the same date), provider.connected, and status (ok, not_synced, no_report or no_source) with explain, which says in plain words what is missing. source is provider (the population feed) or ops (entered by Crossly from the grader published report). Counts only, never a price.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** | variantId:condition:gradeKey, grade \"-\" when ungraded. | [required] |

### Return type

**crate::models::GetVaultMarketPopReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_portfolio

> crate::models::GetVaultMarketPortfolioResponse get_vault_market_portfolio()
Your vault holdings at the market price, with cost and P&L, and units you have out on loan.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPortfolioResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_portfolio_history

> crate::models::GetVaultMarketPortfolioHistoryResponse get_vault_market_portfolio_history()
Your Vault Markets value by day at the marks (vault units, units on loan, positions) and today's return against yesterday's close.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPortfolioHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_portfolio_margin

> crate::models::GetVaultMarketPortfolioMarginResponse get_vault_market_portfolio_margin()
Portfolio margin for a firm sub-account (`subaccountId`): eligibility with every reason (ECP, KYB, minimum equity $500k or $100k with evidenced intraday monitoring), whether it is enrolled, the latest requirement, equity, excess and state (healthy, call or buy_in), the breakdown by category and item, the partner index futures counted, and the policy. Before enrolling, a live preview.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPortfolioMarginResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_position

> crate::models::GetVaultMarketPositionResponse get_vault_market_position(id)
One position with its loans, auctions and custodian instruction log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketPositionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_position_exit

> crate::models::GetVaultMarketPositionExitResponse get_vault_market_position_exit(id)
A position's take-profit / stop-loss, if set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketPositionExitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_position_risk

> crate::models::ListVaultMarketPositionsRiskItem get_vault_market_position_risk(id)
One position's call and liquidation marks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketPositionsRiskItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_press

> crate::models::GetVaultMarketPressResponse get_vault_market_press()
The press kit: each built-in index with level, quality, n, CSV link and a \"cite as\" line, plus the methodology links.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketPressResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_price_widget

> crate::models::ListVaultMarketPriceWidgetsItem get_vault_market_price_widget(id)
One of your price widgets, with its iframe and script-tag snippets and URLs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketPriceWidgetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pro_depth

> crate::models::GetVaultMarketProDepthResponse get_vault_market_pro_depth()
The L2 depth ladder: price levels and units per side, best first; asks show how many units are authenticated vault stock.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProDepthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pro_order

> crate::models::GetVaultMarketProOrderResponse get_vault_market_pro_order(id)
One order with its full event log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketProOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pro_snapshot

> crate::models::GetVaultMarketProSnapshotResponse get_vault_market_pro_snapshot()
A book key's depth (20 levels a side), last 50 trades, 24 h stats, mark and any halt, in one call.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProSnapshotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pro_stat

> crate::models::GetVaultMarketProStatResponse get_vault_market_pro_stat()
24 h open, high, low, last, volume and change on a book key.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProStatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_pro_stream

> String get_vault_market_pro_stream()
A long-lived text/event-stream for a book key: `snapshot` first, then `depth`, `trades`, `mark` and `halt` events as they change.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_profile

> crate::models::GetVaultMarketProfileResponse get_vault_market_profile()
Your public vault profile switch: on or off, whether followers see new buys, your profile URL and follower count.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_profile_by_handle

> crate::models::GetVaultMarketProfileByHandleResponse get_vault_market_profile_by_handle(handle)
Someone's public vault: each market's weight (bps of the marked value), followers, whether you follow, their recent buys (no prices).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketProfileByHandleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_proof_of_reserve

> crate::models::GetVaultMarketProofOfReserveResponse get_vault_market_proof_of_reserve()
Proof of reserves: the latest daily Merkle root over every unit in custody (leaf count, stored / listed / grading / withdrawing, pledged, on loan, open loans, receipts, insured value), its Solana memo anchor when VAULT_POR_ANCHOR_KEY is set, the last `days` roots, and the exact hashing method.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProofOfReserveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_proof_of_reserve_by_day

> crate::models::GetVaultMarketProofOfReserveByDayResponse get_vault_market_proof_of_reserve_by_day(day)
One day's proof-of-reserves snapshot (root, counts, anchor).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketProofOfReserveByDayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_proof_of_reserve_me

> crate::models::GetVaultMarketProofOfReserveMeResponse get_vault_market_proof_of_reserve_me()
Your inclusion proofs for a day (latest by default): each of your units' leaf fields, salt, owner commitment, leaf hash and sibling path. Verify with your OWN user id: commitment = SHA-256(JSON[\"crossly-por-owner-v1\", userId, salt]); then the leaf; then climb the path to the published root.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProofOfReserveMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_rate

> crate::models::GetVaultMarketRateResponse get_vault_market_rate()
Daily borrow APR (lent units) and pool APR on a book key, with the pool as a whole, oldest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketRateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_report

> crate::models::GetVaultMarketReportResponse get_vault_market_report()
The quarterly Crossly Vault Report (`quarter` like 2026-q3; default the latest): index levels, top movers, most-lent markets, grade premiums. Book data only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_rfq

> crate::models::GetVaultMarketRfqResponse get_vault_market_rfq(id)
One quote request: the requester sees every answer (anonymous); a responder sees the request and their own answer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketRfqResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_risk_report

> crate::models::GetVaultMarketRiskReportResponse get_vault_market_risk_report()
Historical VaR 95/99 1d/10d and Bangia LVaR (model figures: labelled, with n and a band, null below the data threshold), stress scenarios (category −30%, CBX −50%, books thin to 25%, index drawdowns), concentration and counterparty exposure.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketRiskReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_risk_stat

> crate::models::GetVaultMarketRiskStatResponse get_vault_market_risk_stat(book_key)
Risk statistics for a book key (weekly returns from A-quality marks only) or an index slug (its closes): annualised volatility raw and Geltner-desmoothed (labelled), max drawdown and beta to the Composite. Needs 26 weekly returns, else no numbers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_key** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketRiskStatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_set_goal

> crate::models::GetVaultMarketSetGoalResponse get_vault_market_set_goal(id)
One set goal: every piece in the set order, held (with the vault unit) or missing (with the cheapest marked book key that meets the target and its mark), the conditions and grades seen, and the working basket bid with its legs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketSetGoalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_set_goal_basket_plan

> crate::models::GetVaultMarketSetGoalBasketPlanResponse get_vault_market_set_goal_basket_plan(id)
What a basket bid would place now, without placing it: one leg per missing piece (cheapest first, the mark plus maxBpsOverMark as the limit), the pieces skipped and why, and the most it can cost (limits plus the worst-case buyer fee), never more than budgetCents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketSetGoalBasketPlanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_share_card

> crate::models::ListVaultMarketShareCardsItem get_vault_market_share_card(id)
One share card as the public sees it, with its image and link paths.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketShareCardsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_statement

> crate::models::GetVaultMarketStatementResponse get_vault_market_statement()
The last 24 months with whether each statement is issued (frozen, with its sha256).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_statement_by_month

> crate::models::GetVaultMarketStatementByMonthResponse get_vault_market_statement_by_month(month)
A monthly statement (`month` like 2026-10): units held at month end, loans, pledges, positions, fills, fees, storage, and cash quoted from the custodian and labelled as its record. Issued (frozen) once the month ends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketStatementByMonthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_statement_file

> String get_vault_market_statement_file(month, format)
A monthly statement as a csv, pdf or json file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |
**format** | **String** |  | [required] |

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_supply

> crate::models::GetVaultMarketSupplyResponse get_vault_market_supply()
A book key's units in the vault and authentications passed, by day (oldest first, today live).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketSupplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_supply_batch

> crate::models::GetVaultMarketSupplyBatchResponse get_vault_market_supply_batch(id)
A box sent to the vault: its prepaid label and tracking, or why it has no label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketSupplyBatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_supply_round_up

> crate::models::GetVaultMarketSupplyRoundUpResponse get_vault_market_supply_round_up()
Your seller round-up: the share of each payout that sets a vault buy's budget, the budget so far and what it is doing. `roundUp` is null when none is set.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketSupplyRoundUpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tax

> crate::models::GetVaultMarketTaxResponse get_vault_market_tax()
Your tax year on the vault markets: units bought and sold with cost, proceeds, gain and term; lending fees, staking interest (estimate), market-maker rewards; closed positions. Links to the CSV and PDF.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketTaxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tax_csv

> String get_vault_market_tax_csv()
The tax year as a CSV: one row per sale, purchase, gift, income line and closed position.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tax_loss_suggestion

> crate::models::GetVaultMarketTaxLossSuggestionResponse get_vault_market_tax_loss_suggestion()
Tax-loss suggestions: the units you hold whose mark is below your cost, each with its lot (acquired when and how, cost in cents), short or long term if sold today and the day a short lot turns long, the estimated loss at the mark before selling fees, mark quality, what stops a sale, and a link to the ordinary sell ticket. With totals by term, the gains realised so far this year, and a not-tax-advice disclosure. Never sells.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketTaxLossSuggestionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tax_pdf

> std::path::PathBuf get_vault_market_tax_pdf()
The tax year as a PDF summary.

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


## get_vault_market_tca

> crate::models::GetVaultMarketTcaResponse get_vault_market_tca()
Your pro orders in a window (`from`, `to`; default 90 days) with arrival mark and mid, average fill, implementation shortfall, slippage, fill rate and time to fill.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketTcaResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tca_by_order_id

> crate::models::GetVaultMarketTcaByOrderIdResponse get_vault_market_tca_by_order_id(order_id)
One order's TCA, including slippage against the VWAP of eligible trades while it worked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketTcaByOrderIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tod

> crate::models::GetVaultMarketTodResponse get_vault_market_tod()
Your transfer-on-death setup: the gate and terms, whether you accepted them, your beneficiaries and shares, units with their designations and who the plan gives them to today, and open claims on your account (which you can stop). A TodOverview object, not a collection.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketTodResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_tod_my_claim

> crate::models::GetVaultMarketTodMyClaimResponse get_vault_market_tod_my_claim()
Claims you filed as a beneficiary: status in words, documents sent (names only), the waiting period, and units moved or held. A TodMyClaims object, not a collection.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketTodMyClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_unit_ar

> crate::models::GetVaultMarketUnitArResponse get_vault_market_unit_ar(id)
A vault unit as an AR model, for \"See it in your room\" (the web's /ar/vault-<id> page hands it to Scene Viewer, WebXR or Quick Look): title, model (a path under /models, without .glb, at real size), scanUrl (the unit's own captured mesh, when it has one), imageUrl, art, tint, scale, bookKey, backPath and mine. Your own units always; somebody else's only while they show it to visitors in a home you may enter (else 404).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitArResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_unit_evidence

> crate::models::GetVaultMarketUnitEvidenceResponse get_vault_market_unit_evidence(id)
Evidence on a vault unit:its 3D scans (model, texture, listing photos) and its grade report (grader, grade, cert, verified against the grader record, hub intake photos). No owner identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitEvidenceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_unit_quote

> crate::models::GetVaultMarketUnitQuoteResponse get_vault_market_unit_quote(id)
The market around a vault unit: its mark and source, the best bid that could take it, the Instant Exit price (the best bid) and a pack pull's own sell-back credit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_unit_story

> crate::models::GetVaultMarketUnitStoryResponse get_vault_market_unit_story(id)
One vault unit's story, oldest first: intake (with scan and grade evidence when it has some), grades, owners, sales at their prices, loans, pledges, Hall of Grails exhibitions, receipt-token exports and redemptions, and each warehouse-receipt hash-chain step (re-verified on read: chain.ok). People are named only while their public vault profile is on; others are 'A collector'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketUnitStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_valuation_statement

> crate::models::GetVaultMarketValuationStatementResponse get_vault_market_valuation_statement(id)
One statement you issued: every unit, the totals by mark quality, the methodology, its sha256, its audit-chain seal and the verify link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketValuationStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_valuation_statement_pdf

> std::path::PathBuf get_vault_market_valuation_statement_pdf(id)
One statement you issued as a PDF.

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


## get_vault_market_valuation_statement_preview

> crate::models::GetVaultMarketValuationStatementPreviewResponse get_vault_market_valuation_statement_preview()
An insurance or appraisal statement for a day (purpose, asOf YYYY-MM-DD, default today), built but not issued: every unit you held that day at its mark, with quality, last trade, grade and cert, and the total. No id, no hash.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketValuationStatementPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_valuation_statement_verify

> crate::models::GetVaultMarketValuationStatementVerifyResponse get_vault_market_valuation_statement_verify(id)
Check any issued statement by its id, as an insurer would: purpose, date, totals by mark quality and the sha256; intact (the stored record still hashes to it), seal.ok (its audit-chain entry still carries it) and, with ?hash=, whether yours matches. Never the holder or the units.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketValuationStatementVerifyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_value

> crate::models::GetVaultMarketValueResponse get_vault_market_value()
A catalogue item's vault book keys (`?skuId=`, optional `variantId`): each key's display mark with quality and n, units in the vault, the Instant Exit price a holder would get now, and whether it can be shipped in (ungraded keys).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketValueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_vault_suggestion_item

> crate::models::GetVaultMarketVaultSuggestionItemResponse get_vault_market_vault_suggestion_item(id)
One inventory item: its vault suggestion, or in plain words why there is none (`status`: suggested, not_found, no_stock, not_matched, in_vault or no_market).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketVaultSuggestionItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_widget_index

> crate::models::GetVaultMarketWidgetIndexResponse get_vault_market_widget_index(slug)
What the <crossly-index> widget shows: level, closes, quality, n, as-of, or `insufficient`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketWidgetIndexResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_widget_mark

> crate::models::GetVaultMarketWidgetMarkResponse get_vault_market_widget_mark()
What the <crossly-mark> widget shows for `bookKey`: mark, quality, trades over 30 days, as-of, sparkline, or `insufficient`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWidgetMarkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_widget_summary

> crate::models::GetVaultMarketWidgetSummaryResponse get_vault_market_widget_summary()
The home-screen widget summary: portfolio value at the mark with its move since the previous marks (cents and bps) and the share resting on reliable A/B marks, the three biggest movers among what you hold (the market movers when you hold nothing; `movers.source` says which), and one watchlist key (`?key=` a book key on your watchlist; else your newest watch) with a 14-day sparkline. Every price is from the Crossly order book and carries its mark quality (A, B, C or none). Cached for five minutes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWidgetSummaryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_world_floor

> crate::models::GetVaultMarketWorldFloorResponse get_vault_market_world_floor()
The Trading Floor in Crossly World: trades (the newest trades on every vault book in the last day, at most 40, newest first: bookKey, name, priceCents, priceText, side -- buy when a buyer took an ask, sell when a seller hit a bid, cross for an auction -- and ageS), tickers (the big board: the most-traded markets, each with its last trade price, the change from the last trade before today in basis points, trades today and the display mark with its quality), bell (Market Open: ringing for ten minutes after a new key opens, with the key and its market link, and what opens next) and trades24h. Every price is a trade on the Crossly order book; it never shows who traded. Nothing can be traded in the room.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWorldFloorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_world_lounge

> crate::models::GetVaultMarketWorldLoungeResponse get_vault_market_world_lounge()
The Vault Holders Lounge: the rule to join (held value at reliable marks, or a unit count), whether you may join now and what you hold toward it. Anyone may look in; only holders are seen there. No perks, nothing to trade.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWorldLoungeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_world_place_vault_gate

> crate::models::GetVaultMarketWorldPlaceVaultGateResponse get_vault_market_world_place_vault_gate(place)
The vault-holder door on a Crossly World room or Space, by its presence slug (a room share slug, or space-<slug>): the rule (minValueCents held at reliable A/B marks and/or minUnits units, mode any or all, optionally one product, bullion family or category), the rule in words, and whether you may join now with what you hold toward it. gate is null when the place has no vault door. Anyone may look in; only holders join.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketWorldPlaceVaultGateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_worth

> crate::models::GetVaultMarketWorthResponse get_vault_market_worth()
What one book key (variantId:condition:gradeKey, `bookKey`) is worth: the mark in cents with its quality (A from trades, B from firm quotes, C indicative, none) and band, what Instant Exit would pay for one unit now from the best resting bid (yours left out), and your units: how many you hold, how many are free to sell, their value at the mark and what selling every free one now would fetch from today’s bids. Plain-words `headline` and `lines`. Read only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWorthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_worth_vault

> crate::models::GetVaultMarketWorthVaultResponse get_vault_market_worth_vault()
What your whole vault is worth now: value at the marks split into reliable and indicative marks, cost and P&L, today’s change, and an estimate of what the units free to sell would fetch from the bids resting on the book now. Read only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketWorthVaultResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_receipt

> crate::models::GetVaultReceiptResponse get_vault_receipt(receipt_no)
One of your receipts with its full hash chain (issued, transferred, pledged, released, surrendered), re-verified on read: `chain.ok` is false if any event was altered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**receipt_no** | **String** |  | [required] |

### Return type

**crate::models::GetVaultReceiptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_receipt_token

> crate::models::GetVaultReceiptTokenResponse get_vault_receipt_token(id)
One receipt token (yours, or on a unit you own now) with its chain log: built, minted, frozen, thawed, new holder seen, redeem built, burned, with explorer links.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultReceiptTokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_receipt_token_by_mint

> crate::models::GetVaultReceiptTokenByMintResponse get_vault_receipt_token_by_mint(mint)
What a Solana mint is: the warehouse receipt number and item it stands for, and whether the token is live, frozen (unit pledged or lent) or burned, and whether the unit is still on a vault shelf. Names no person and no wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint** | **String** |  | [required] |

### Return type

**crate::models::GetVaultReceiptTokenByMintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_accounts

> crate::models::ListVaultMarketAccountsResponse list_vault_market_accounts()
Every account you can act on: your own, and each sub-account your entity memberships reach, with your role.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAccountsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_adl_events

> crate::models::ListVaultMarketAdlEventsResponse list_vault_market_adl_events()
The ADL events your positions were part of, newest first: role bankrupt (yours went under water) or counterparty (yours was first in line), the price both closed at and the transfer status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAdlEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_adl_positions

> crate::models::ListVaultMarketAdlPositionsResponse list_vault_market_adl_positions()
Your live positions, each with its place in its item ADL queue: lights 5 (first in line) to 1 (last), 0 when not in the queue (only profitable positions are), ranked by P&L% x effective leverage.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAdlPositionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_agent_drafts

> crate::models::ListVaultMarketAgentDraftsResponse list_vault_market_agent_drafts()
Limit orders the shopping agent drafted for you, newest first. A draft places nothing: send its orderBody to POST /api/v1/vault-markets/pro/orders to place it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAgentDraftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_agent_watches

> crate::models::ListVaultMarketAgentWatchesResponse list_vault_market_agent_watches()
Your price watches on book keys (best bid, lowest ask, last trade or market price, in cents), live ones first. `?live=true` for live only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAgentWatchesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_alert_log

> crate::models::ListVaultMarketAlertLogResponse list_vault_market_alert_log()
What the alerts hub did with your recent vault alerts (`?limit=` 1 to 200, default 50): kind, title, the channels it went to, what was held (channel:reason) and the email digest state.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAlertLogResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_alerts

> crate::models::ListVaultMarketAlertsResponse list_vault_market_alerts()
Your price alerts on book keys.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAlertsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_auctions

> crate::models::ListVaultMarketAuctionsResponse list_vault_market_auctions()
Open vault auction events (weekly, ending Sunday; premier, units marked $10,000+) and their lots, soonest ending first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketAuctionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_blocks

> crate::models::ListVaultMarketBlocksResponse list_vault_market_blocks()
An entity's block orders, newest first, each with its plan and how it was allocated.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBlocksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_bullion_family_pegs

> crate::models::ListVaultMarketBullionFamilyPegsResponse list_vault_market_bullion_family_pegs()
Your family-wide spot-pegged quotes (live, or all with status=all): one quote resting across a bullion family books at one condition, with units left, the peg, its status and every leg (the pegged quote on each book).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBullionFamilyPegsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_bullion_keys

> crate::models::ListVaultMarketBullionKeysResponse list_vault_market_bullion_keys()
Bullion key families (metal, form, size, mint or any mint): members, vault units, best resting bid and ask across their books, and spot as a labelled reference with what one unit of metal is worth at it. Spot is never a Crossly price.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBullionKeysResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_bullion_pegs

> crate::models::ListVaultMarketBullionPegsResponse list_vault_market_bullion_pegs()
Your spot-pegged quotes (live, or all with status=all): side, units, the peg, the order it rests as, the last price and spot it was priced from, and why it is pulled when it is.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketBullionPegsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_collection_worth

> crate::models::ListVaultMarketCollectionWorthResponse list_vault_market_collection_worth()
Your collection valued at the display marks: rows on a vault book key with a mark are summed; rows with no mark or no key are counted, never valued.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketCollectionWorthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_confirmations

> crate::models::ListVaultMarketConfirmationsResponse list_vault_market_confirmations()
Trade confirmations for your fills in a window (default 31 days): price, sale fee, maker or taker, order, vault unit and warehouse receipt.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketConfirmationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_creator_cohost_invites

> crate::models::ListVaultMarketCreatorCohostInvitesResponse list_vault_market_creator_cohost_invites()
For creators: your invites from Crossly to co-host a Market Open launch show (invited, accepted, declined or withdrawn), each with the key, its show and the guidance for co-hosts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketCreatorCohostInvitesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_credit

> crate::models::ListVaultMarketCreditResponse list_vault_market_credit()
Your vault loans and advances (live by default): what is owed, interest accrued, loan-to-value, the liquidation line and the collateral value it is called at.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketCreditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_dealer_bands

> crate::models::ListVaultMarketDealerBandsResponse list_vault_market_dealer_bands()
Your Instant Exit bands and whether each is being kept now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerBandsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_dealer_bids

> crate::models::ListVaultMarketDealerBidsResponse list_vault_market_dealer_bids()
Your standing dealer bids: open (default) or all.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerBidsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_dealer_pool_fills

> crate::models::ListVaultMarketDealerPoolFillsResponse list_vault_market_dealer_pool_fills(id)
A curve pool's fills, newest first: side, price and spot after.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketDealerPoolFillsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_dealer_pools

> crate::models::ListVaultMarketDealerPoolsResponse list_vault_market_dealer_pools()
Your curve pools: open (default) or all, each with its curve, spot, units, buy budget, running totals, the ladder it shows and its live legs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerPoolsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_digest_history

> crate::models::ListVaultMarketDigestHistoryResponse list_vault_market_digest_history()
Past weekly digests: week, status (sent / skipped / failed), the channels used and a summary.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDigestHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_entity_audit

> crate::models::ListVaultMarketEntityAuditResponse list_vault_market_entity_audit(id)
The entity's audit trail, newest first: hash-chained rows with actor, account, API key, IP, session and source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketEntityAuditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_entity_conversions

> crate::models::ListVaultMarketEntityConversionsResponse list_vault_market_entity_conversions(id)
The custodian's CBX-to-USD reward conversions for an entity that elected USD, newest first, with their status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketEntityConversionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_estimates_accuracy

> crate::models::ListVaultMarketEstimatesAccuracyResponse list_vault_market_estimates_accuracy()
The Crossly Estimate's track record: walk-forward backtest n, FSD and bias per method, category and liquidity bucket, and whether estimates in that cell are shown.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketEstimatesAccuracyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_feed

> crate::models::ListVaultMarketFeedResponse list_vault_market_feed()
New vault buys by the people you follow who share them, newest first (no prices). Page with `before` (the last item's `at`).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFeedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_copy_watch

> crate::models::ListVaultMarketFollowCopyWatchResponse list_vault_market_follow_copy_watch()
The collectors you copy-watch and whether each still shares bids and asks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFollowCopyWatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_copy_watch_alerts

> crate::models::ListVaultMarketFollowCopyWatchAlertsResponse list_vault_market_follow_copy_watch_alerts()
Your copy-watch alerts, newest first: who, bid or ask, quantity, limit price, market, and whether it still rests. Page with `before`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFollowCopyWatchAlertsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_feed

> crate::models::ListVaultMarketFollowFeedResponse list_vault_market_follow_feed()
Public vault activity of the collectors you follow (buys, units sent in, sales, Hall loans), newest first, never a price. Filter with `kinds`; page with `before`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFollowFeedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_following

> crate::models::ListVaultMarketFollowFollowingResponse list_vault_market_follow_following()
The collectors you follow who have a public vault: what each shares and whether you copy-watch them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFollowFollowingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_leaderboards

> crate::models::ListVaultMarketFollowLeaderboardsResponse list_vault_market_follow_leaderboards()
An opt-in return leaderboard (`period` 30d, 90d or 365d; `category` all or a category slug), ranked by risk-adjusted return from the book A/B marks. Percent and risk only: no dollars, no volume.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketFollowLeaderboardsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_follow_profile_activity

> crate::models::ListVaultMarketFollowFeedResponse list_vault_market_follow_profile_activity(handle)
One collector public vault activity, newest first, never a price. 404 when their vault is not public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketFollowFeedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_gifts

> crate::models::ListVaultMarketGiftsResponse list_vault_market_gifts()
Gifts you sent and received, newest first, with their status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGiftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_grade_to_vault

> crate::models::ListVaultMarketGradeToVaultResponse list_vault_market_grade_to_vault()
Your Grade to Vault flows, newest first, each brought up to date: its step, the grading submission, the unit, the grade, the market for it, and the ask once listed. `live=1` for those still going.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGradeToVaultResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_grading

> crate::models::ListVaultMarketGradingResponse list_vault_market_grading()
Your vault units sent to grading: where each card is in its run, and the grade once it is back.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGradingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_hall_lenders

> crate::models::ListVaultMarketHallLendersResponse list_vault_market_hall_lenders()
The vault units lent to this week's Hall of Grails exhibition and on show now, in plinth order: the item (skuId, name, imageUrl, grade), the lender as the plate names them (a display name, or \"a collector\"), the plate lines and how many exhibitions the unit has stood in. meta.exhibition is the week (key, title, endsAt). Names no ids and no amounts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketHallLendersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_committee_minutes

> crate::models::ListVaultMarketIndexCommitteeMinutesResponse list_vault_market_index_committee_minutes()
Published Index Committee minutes, newest first: attendees (names and roles), quorum, agenda, minutes and decisions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexCommitteeMinutesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_complaints

> crate::models::ListVaultMarketIndexComplaintsResponse list_vault_market_index_complaints()
Your complaints and their answers, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexComplaintsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_constituents

> crate::models::ListVaultMarketIndexMembersResponse list_vault_market_index_constituents(slug)
The book keys in an index with their weight, move since the last close and contribution to the index's move (basis points).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketIndexMembersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_consultations

> crate::models::ListVaultMarketIndexConsultationsResponse list_vault_market_index_consultations()
Consultations on index method changes (open for at least 30 days), newest first. Filter by `status`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexConsultationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_events

> crate::models::ListVaultMarketIndexEventsResponse list_vault_market_index_events()
Every index event, newest first: add, delete, rebalance, method, correction, halt; each with when it was announced, when it takes effect, the detail and the reason. Filter by `kind`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_events_by_slug

> crate::models::ListVaultMarketIndexEventsResponse list_vault_market_index_events_by_slug(slug)
One index's events, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketIndexEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_judgments

> crate::models::ListVaultMarketIndexJudgmentsResponse list_vault_market_index_judgments()
The expert-judgment log: every Index Committee action (halt, resume, exclude a key, void a trade, restate, retire) with its written reason.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexJudgmentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_level_series

> crate::models::ListVaultMarketIndexLevelSeriesResponse list_vault_market_index_level_series(slug)
An index's published levels. `engine=chain` (default): daily closes. `engine=tpd|repeat_sales`: weekly (else monthly) model estimates with a 95% band and n; labelled, never a price. `series=settlement` returns each value as first published (never restated). Below the data gate `data` is empty and `notice` explains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketIndexLevelSeriesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_licence_mine

> crate::models::ListVaultMarketIndexLicenceMineResponse list_vault_market_index_licence_mine()
Your index feed licences (the indexes each covers, or all), as ops recorded them. No licence: the feed answers 403 not_a_licensee.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexLicenceMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_members

> crate::models::ListVaultMarketIndexMembersResponse list_vault_market_index_members(slug)
The members of an index on a day (`asOf`, default today) with their weights, shares, moves and contributions; for a past day, the inputs stored with that day's close and each member's effective dates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketIndexMembersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_index_movers

> crate::models::ListVaultMarketIndexesResponse list_vault_market_index_movers()
The built-in indexes that moved most since their last daily close.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_indexes

> crate::models::ListVaultMarketIndexesResponse list_vault_market_indexes()
Indexes over book keys: per category, per top brand, the Crossly 50 and your own. Each with its level now (base 1000 at inception) and change since the last daily close.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketIndexesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_instant_exit_history

> crate::models::ListVaultMarketInstantExitHistoryResponse list_vault_market_instant_exit_history()
Your Instant Exits, newest first: quoted price, mark, and whether it filled.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketInstantExitHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_instant_exit_holdings

> crate::models::ListVaultMarketInstantExitHoldingsResponse list_vault_market_instant_exit_holdings()
Instant Exit quotes for everything you hold in the vault, one per book key, with units held and which are free to sell.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketInstantExitHoldingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_joint_trades

> crate::models::ListVaultMarketJointTradesResponse list_vault_market_joint_trades(id)
The joint vault trades, newest first: side, book key, limit, who started it, whose account the order sits on, and the order standing on the book.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketJointTradesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_large_trader_reports

> crate::models::ListVaultMarketLargeTraderReportsResponse list_vault_market_large_trader_reports()
Your large-trader reports (with any linked accounts, as one party): due first, then filed, then cleared in the last 90 days. Each has `action`: file | amend | refile | none.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketLargeTraderReportsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_lending_statements

> crate::models::ListVaultMarketLendingStatementsResponse list_vault_market_lending_statements()
The days you have stored collateral statements for, newest first, with their summary.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketLendingStatementsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_loans

> crate::models::ListVaultMarketLoansResponse list_vault_market_loans()
Units you lent: rate, fees earned, recall due.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketLoansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_locate_mine

> crate::models::ListVaultMarketLocateMineResponse list_vault_market_locate_mine()
Your locates, newest first (held / used / released / expired).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketLocateMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_midpoint

> crate::models::ListVaultMarketMidpointResponse list_vault_market_midpoint()
Your midpoint block orders.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketMidpointResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_movers

> crate::models::ListVaultMarketMoversResponse list_vault_market_movers()
Vault items with the biggest 24 h moves in their market price (from the order book), with units in the vault.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketMoversResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_nav_history

> crate::models::ListVaultMarketNavHistoryResponse list_vault_market_nav_history()
Daily NAV snapshots (mark, bid, liquidation, Level 2 / Level 3).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketNavHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_offers

> crate::models::ListVaultMarketOffersResponse list_vault_market_offers()
The offers inbox: for each book key you hold, the best bid from someone else, units at it, the top bid levels, the mark, and your alert.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_opening

> crate::models::ListVaultMarketOpeningResponse list_vault_market_opening()
Keys opening soon on the vault book (and just opened): units on the shelf, dealers quoting, the waitlist, the mark when the book has one, and your place.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketOpeningResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_oracle_history

> crate::models::ListVaultMarketOracleHistoryResponse list_vault_market_oracle_history(slug)
Recent publish attempts for an index over 30 days, newest first: published, skipped (with the reason: too few items priced, stale marks, confidence too wide...) or failed (with the error). Filter with `status`, cap with `limit` (default 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketOracleHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_p2p_auctions

> crate::models::ListVaultMarketP2pLoansResponse list_vault_market_p2p_auctions()
Running refinancing auctions, soonest to end first: lenders calls (`auction`) and borrowers looking for a cheaper lender (`refiAuction`). The payoff now, the rate a new lender gets now, when it ends, and the item.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pLoansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_p2p_loans

> crate::models::ListVaultMarketP2pLoansResponse list_vault_market_p2p_loans()
Your P2P loans as borrower and/or lender (`role`), live by default (`status`): what is owed now, daily interest, the refinancing auction when called, and the item mark for information only (it never triggers anything).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pLoansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_p2p_offer_mine

> crate::models::ListVaultMarketP2pOffersResponse list_vault_market_p2p_offer_mine()
Your P2P loan offers, newest first, with how many loans each has funded.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_p2p_offers

> crate::models::ListVaultMarketP2pOffersResponse list_vault_market_p2p_offers()
The open P2P loan offers book, lowest rate first: cents offered per loan, APR (bps), loans left, and the item (book key) or unit each lends against. `bookKey` narrows to one item. Lender identities are never shown.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_p2p_units

> crate::models::ListVaultMarketP2pUnitsResponse list_vault_market_p2p_units()
Your vault units for P2P borrowing: whether each can be pledged now (and why not), the best offers covering it, and its loan when it has one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketP2pUnitsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_paper_fills

> crate::models::ListVaultMarketPaperFillsResponse list_vault_market_paper_fills()
Your paper fills since your last reset, newest first: price, simulated fee, taker or maker, and whether a visible real order (book) or a real trade (tape) filled it. Play money only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPaperFillsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_paper_leaderboard

> crate::models::ListVaultMarketPaperLeaderboardResponse list_vault_market_paper_leaderboard()
The weekly paper trading leaderboard (opt-in, display names only): best return on play equity this week (or a past `week`, its Monday as YYYY-MM-DD), with your own line. Nothing is won or paid.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPaperLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_paper_missions

> crate::models::ListVaultMarketPaperMissionsResponse list_vault_market_paper_missions()
Your paper-trading missions (play money only): each mission with what to do, your count toward its target, whether it is done and when, plus a suggested market for the \"Read the mark\" mission. Progress is observed by the server from your paper orders and fills, price watches, set goals and graded mark checks. Missions earn world quest progress and cosmetic pins, never money.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPaperMissionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_paper_orders

> crate::models::ListVaultMarketPaperOrdersResponse list_vault_market_paper_orders()
Your paper orders since your last reset, newest first: working, done (filled, cancelled, expired) or all. Play money only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPaperOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_partner_accruals

> crate::models::ListVaultMarketPartnerAccrualsResponse list_vault_market_partner_accruals()
Your partner fee shares, newest first: accrued on a unit's first sale, paid in Bucks once the sale is final, void if it was refunded.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPartnerAccrualsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pool_stakes

> crate::models::ListVaultMarketPoolStakesResponse list_vault_market_pool_stakes()
Your stakes in the market pool and what each redeems for now.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPoolStakesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_portfolio_margin_history

> crate::models::ListVaultMarketPortfolioMarginHistoryResponse list_vault_market_portfolio_margin_history()
The 5-minute portfolio margin recalculations for a firm sub-account over the last `hours` (default 24, at most 720), newest first: equity, requirement, excess, scenario and floor figures, gross and state.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPortfolioMarginHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_position_units

> crate::models::ListVaultMarketPositionUnitsResponse list_vault_market_position_units(id)
The vault units of one of your positions that you hold (a leveraged long's collateral), each with its story link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketPositionUnitsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_positions

> crate::models::ListVaultMarketPositionsResponse list_vault_market_positions()
Your positions, live (default) or closed.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPositionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_positions_risk

> crate::models::ListVaultMarketPositionsRiskResponse list_vault_market_positions_risk()
For each live position: its tier, health, margin state, the mark at which it is called and the mark at which it is liquidated, and the distance to it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPositionsRiskResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_price_widgets

> crate::models::ListVaultMarketPriceWidgetsResponse list_vault_market_price_widgets()
Your price widgets, newest first, each with its iframe and script-tag snippets. `meta.codes` lists your affiliate codes a widget can link through; `meta.max` is the cap on live widgets.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketPriceWidgetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_candles

> crate::models::ListVaultMarketProCandlesResponse list_vault_market_pro_candles()
OHLCV candles (1m, 5m, 15m, 1h, 4h, 1d) from the trades on a book key.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProCandlesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_events

> crate::models::ListVaultMarketProEventsResponse list_vault_market_pro_events()
Your order event log after a sequence number (new, leg, fill, replaced, cancelled, expired, done): a replayable drop copy.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_links

> crate::models::ListVaultMarketProLinksResponse list_vault_market_pro_links()
Your linked orders (brackets, OCO pairs, trailing stops, position exits): live (default) or done.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProLinksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_orders

> crate::models::ListVaultMarketProOrdersResponse list_vault_market_pro_orders()
Your orders: open (default) or done, optionally on one book key, or only those booked to one sub-account (`subaccountId`).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_quotes

> crate::models::ListVaultMarketProQuotesResponse list_vault_market_pro_quotes()
Your live quotes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProQuotesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_rewards

> crate::models::ListVaultMarketProRewardsResponse list_vault_market_pro_rewards()
Your daily market-maker liquidity rewards (paid in CBX).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProRewardsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_pro_tape

> crate::models::ListVaultMarketProTapeResponse list_vault_market_pro_tape()
The latest trades on a book key, newest first, with which side made and which took.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketProTapeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_proof_of_reserve_leaves

> crate::models::ListVaultMarketProofOfReserveLeavesResponse list_vault_market_proof_of_reserve_leaves(day)
One day's leaf hashes in tree order, a page at a time: enough to recount the units and rebuild the root yourself. Leaves are salted, so they name no owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketProofOfReserveLeavesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_recurring

> crate::models::ListVaultMarketRecurringResponse list_vault_market_recurring()
Your recurring buys: key, schedule, price cap, payment, auto-lend, next run.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRecurringResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_recurring_runs

> crate::models::ListVaultMarketRecurringRunsResponse list_vault_market_recurring_runs(id)
One recurring buy's runs: filled, skipped (with the reason, e.g. the price was over the cap) or failed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketRecurringRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_recurring_runs2

> crate::models::ListVaultMarketRecurringRunsResponse list_vault_market_recurring_runs2()
Every run of all your recurring buys, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRecurringRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_rfq

> crate::models::ListVaultMarketRfqResponse list_vault_market_rfq()
Your quote requests, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRfqResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_rfq_inbox

> crate::models::ListVaultMarketRfqInboxResponse list_vault_market_rfq_inbox()
Open quote requests you were sent (as a dealer, DMM, LP or holder), with your live answer.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRfqInboxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_rfq_open

> crate::models::ListVaultMarketRfqResponse list_vault_market_rfq_open()
Open quote requests on one key (all-to-all: anyone with ID may answer). Anonymous.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketRfqResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_set_goal_baskets

> crate::models::ListVaultMarketSetGoalBasketsResponse list_vault_market_set_goal_baskets(id)
A set goal basket bids, newest first, each with its budget, what it committed and spent, and every leg (piece, book key, limit, pro order id, status).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketSetGoalBasketsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_set_goal_sets

> crate::models::ListVaultMarketSetGoalSetsResponse list_vault_market_set_goal_sets()
Sets to start a goal on. With q (2+ characters): a word-start search of every catalogue set. Without: the sets you hold pieces of in the vault, then the collections you are building. Each with your live goal on it, if any.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketSetGoalSetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_set_goals

> crate::models::ListVaultMarketSetGoalsResponse list_vault_market_set_goals()
Your set goals: the set, the target a piece must meet, pieces held of the total (from your vault), and the working basket bid.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketSetGoalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_share_cards

> crate::models::ListVaultMarketShareCardsResponse list_vault_market_share_cards()
Your share cards (not taken down), each as the public sees it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketShareCardsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_supply_deposits

> crate::models::ListVaultMarketSupplyDepositsResponse list_vault_market_supply_deposits()
Your units sent to the vault, newest first, each with its state and its key's Instant Exit line (\"Instant Exit now: $X (N% of mark)\" or \"No market yet\"). Filter with `inventoryItemIds` (comma-separated).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketSupplyDepositsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_supply_invites

> crate::models::ListVaultMarketSupplyInvitesResponse list_vault_market_supply_invites()
Market Open invites to you as a dealer: keys Crossly is opening and asks you to quote.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketSupplyInvitesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_valuation_statements

> crate::models::ListVaultMarketValuationStatementsResponse list_vault_market_valuation_statements()
The insurance and appraisal statements you issued, newest first: purpose, date, units, total at the marks, hash and verify link.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketValuationStatementsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_vault_suggestions

> crate::models::ListVaultMarketVaultSuggestionsResponse list_vault_market_vault_suggestions()
What to send to the vault: your inventory items whose book key has a liquid vault market (tier A, B or C), best first. Each has a score (0 to 100) and its reasons, the market (tier, mark and its quality, best bid, best vault ask, spread), Instant Exit now, your list price, days listed without selling, and the vault fee. `meta.summary` counts what was left out and why. Inform only; send with POST /api/v1/vault-markets/supply/send.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketVaultSuggestionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_watchlist

> crate::models::ListVaultMarketWatchlistResponse list_vault_market_watchlist()
Your watched book keys with mark, 24 h and 7 d change, and the last 30 daily marks (sparkline).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketWatchlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_widget_tokens

> crate::models::ListVaultMarketWidgetTokensResponse list_vault_market_widget_tokens()
Your live home-screen widget tokens, one per phone: id, name (the phone), public prefix, created and last used. The secret is never returned again.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketWidgetTokensResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_world_cohosts

> crate::models::ListVaultMarketWorldCohostsResponse list_vault_market_world_cohosts()
Market Open launch shows a verified creator co-hosts that are on now or opening soon: the key, its phase, the show, the creator (named only while their vault profile is public), their room link and, when the creator is in the creator programme, the disclosure that they may earn a share of the fee Crossly keeps. Never a prompt to trade.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketWorldCohostsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_world_creator_rooms

> crate::models::ListVaultMarketWorldCreatorRoomsResponse list_vault_market_world_creator_rooms()
Verified creators whose public vault shelf is a world room: title, the creator, the room link and the disclosure. Rooms show each market share of the shelf, never amounts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketWorldCreatorRoomsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_world_market_open

> crate::models::ListVaultMarketWorldMarketOpenResponse list_vault_market_world_market_open()
Market Open events in the Crossly World Commons: keys opening soon and those that opened in the last few hours, live first then soonest, each with its board line, units on the shelf, dealers quoting and the launch show when there is one. Never a prompt to trade.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketWorldMarketOpenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_receipt_token_wallets

> crate::models::ListVaultReceiptTokenWalletsResponse list_vault_receipt_token_wallets()
Your verified Solana wallets and whether each is on the receipt-token allow-list (receipt tokens move only between allow-listed wallets). meta.state is the gate.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultReceiptTokenWalletsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_receipt_tokens

> crate::models::ListVaultReceiptTokensResponse list_vault_receipt_tokens()
Your vault units with their warehouse receipt and receipt-token state (not exported, waiting for a signature, in a wallet, frozen, redeemed), whether each can be exported now and why not. meta.state is the gate (open, missing envs, program id, authority, terms version, cluster); meta.history is your last 50 exports.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultReceiptTokensResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_receipts

> crate::models::ListVaultReceiptsResponse list_vault_receipts()
Your warehouse receipts (documents of title for your vault units), live first. meta.receipts says whether receipts are registered (VAULT_WAREHOUSE_ISSUER) and their terms version.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultReceiptsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_unit_list

> crate::models::ListVaultUnitListResponse list_vault_unit_list()
The published unit list (bar-list equivalent): every unit on a vault shelf with its warehouse receipt number (when receipts are registered), book key, shelf location code, insured value, status, pledged / on-loan flags and a per-receipt hashed holder (never a name or id). Filter with `bookKey`; pages with `page` / `limit`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultUnitListResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_alert_pref

> crate::models::GetVaultMarketAlertPrefResponse update_vault_market_alert_pref()
Change vault alert settings: `matrix` ({ kind: { channel: true|false } }, merged onto what is set), `quietHours` ({ start: \"HH:MM\", end: \"HH:MM\", tz } or null to clear) and `telegram` (the Telegram master switch). Locked switches are ignored. Returns the new settings.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketAlertPrefResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_block_allocation

> crate::models::ListVaultMarketBlocksItem update_vault_market_block_allocation(order_id)
Post-trade allocation: set or replace the plan for a buy you placed for one of the entity's sub-accounts (that sub-account leads). Refused once any share has moved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketBlocksItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_channel

> crate::models::GetVaultMarketChannelResponse update_vault_market_channel()
Change the switches: `telegramAlerts`, `telegramDigest`, `pushCapPerDay` (0 to 20). Margin calls and recalls are never capped. Returns the new view.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketChannelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_dealer_band

> crate::models::ListVaultMarketDealerBandsItem update_vault_market_dealer_band()
Commit to an Instant Exit band on a key: a standing bid for `units` kept at `bandBps` of the mark, never above `maxPriceCents`, re-quoted as the mark moves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketDealerBandsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_dealer_pool

> crate::models::ListVaultMarketDealerPoolsItem update_vault_market_dealer_pool(id)
Change a curve pool: spot, step, fee, levels, units, buy budget, floor, ceiling, or pause / resume. Re-quoted at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketDealerPoolsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_entity_member

> crate::models::GetVaultMarketEntityByIdResponse update_vault_market_entity_member(id, member_id)
Admin: change a member's role, limits or sub-accounts. The last admin cannot be demoted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**member_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_entity_subaccount_limit

> crate::models::GetVaultMarketEntityByIdResponse update_vault_market_entity_subaccount_limit(id, sub_id)
Admin: set a sub-account's limits (per-order notional, units per key, gross, net, leverage), held to the entity's caps. `limits: null` clears them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**sub_id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_entity_usd_election

> crate::models::GetVaultMarketEntityByIdResponse update_vault_market_entity_usd_election(id)
Admin: elect USD for the entity (`usdOnly`). Its sub-accounts then post collateral in USD only and pay every fee in USD, and any CBX reward paid to one of them is converted to USD by the custodian in that account. Audited; it can be turned off again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketEntityByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_follow_copy_watch

> crate::models::DeleteVaultMarketFollowCopyWatchResponse update_vault_market_follow_copy_watch(handle)
Copy-watch a collector who shares bids and asks: you get an alert when they rest a new one. It never places an order for you.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

**crate::models::DeleteVaultMarketFollowCopyWatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_follow_sharing

> crate::models::GetVaultMarketFollowSharingResponse update_vault_market_follow_sharing()
Turn sharing switches on or off (`isPublic`, `showBuys`, `deposits`, `sales`, `hallLoans`, `orders`, `leaderboard`). A switch shows only what happens after it goes on; all but the profile itself need the profile public.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketFollowSharingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_home_unit

> crate::models::UpdateVaultMarketHomeUnitResponse update_vault_market_home_unit()
Show exactly these vault units in your Crossly World home: { units: [{ vaultItemId, visitors }] }. Units left out come out of their cases; units already standing stay where they are; new ones go into a free display-case or shelf slot, else a glass case of their own along a wall. visitors: true lets people who may enter your home see that unit (default false: only you). Returns placed, unplaced (no free case: the home is full), refused (with why) and the desk. Display only: nothing moves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateVaultMarketHomeUnitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_index

> crate::models::GetVaultMarketIndexResponse update_vault_market_index(slug)
Rename, re-weight, share or change the keys of an index of your own; its level carries on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketIndexResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_lending_election

> crate::models::GetVaultMarketLendingElectionResponse update_vault_market_lending_election()
Change your lender election (applies to loans opened after the change). Needs lending.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketLendingElectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_mm_mmp

> crate::models::GetVaultMarketMmMmpResponse update_vault_market_mm_mmp()
Set market-maker protection: N units or $X filled on your quotes within T seconds pulls every quote and freezes quoting for a set time.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketMmMmpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_mode

> crate::models::GetVaultMarketModeResponse update_vault_market_mode()
Choose your Vault Markets mode: `mode` simple or pro. Remembered for you on the web and in the app. A display preference only: it places nothing and moves no money.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketModeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_offer_alert

> crate::models::UpdateVaultMarketOfferAlertResponse update_vault_market_offer_alert()
Set (or clear with null) the bid price that should always notify you on a key you hold, and mute or unmute offer notices on it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateVaultMarketOfferAlertResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_paper_leaderboard

> crate::models::UpdateVaultMarketPaperLeaderboardResponse update_vault_market_paper_leaderboard()
Join or leave the weekly paper leaderboard. Joining needs a display name (2 to 24 letters, numbers, spaces, dots, dashes or underscores), never your real name. Returns `{ optIn, displayName }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateVaultMarketPaperLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_position_exit

> crate::models::UpdateVaultMarketPositionExitResponse update_vault_market_position_exit(id)
Set a position's take-profit / stop-loss: a long is closed, a short covered, when the last trade reaches one. Both null clears them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateVaultMarketPositionExitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_price_widget

> crate::models::ListVaultMarketPriceWidgetsItem update_vault_market_price_widget(id)
Change a widget: name, range, layout, theme, affiliate code (null for none) or allowed sites. The market or index it shows never changes: make a new widget for another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketPriceWidgetsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_profile

> crate::models::GetVaultMarketProfileResponse update_vault_market_profile()
Turn your public vault profile on or off (`isPublic`) and whether followers see your new buys (`showBuys`). It shows weights by market, never dollars.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_recurring

> crate::models::ListVaultMarketRecurringItem update_vault_market_recurring(id)
Change a recurring buy's schedule, caps, cards or auto-lend, or pause / resume it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketRecurringItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_set_goal

> crate::models::GetVaultMarketSetGoalResponse update_vault_market_set_goal(id)
Change what a piece must be to count (condition, grade mode, grade). Refused while a basket bid works: cancel it first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketSetGoalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_share_card

> crate::models::ListVaultMarketShareCardsItem update_vault_market_share_card(id)
Show or hide the dollar amounts on one of your share cards (`showAmounts`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListVaultMarketShareCardsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_supply_deposit

> crate::models::UpdateVaultMarketSupplyDepositResponse update_vault_market_supply_deposit(id)
Change a deposit's toggles: `autoListBps` (list at that share of the mark once stored; null = do not) and `lendWhenIdle`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateVaultMarketSupplyDepositResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_supply_round_up

> crate::models::GetVaultMarketSupplyRoundUpResponse update_vault_market_supply_round_up()
Set the round-up: `shareBps` (1%-50%) of each payout released to you adds to a budget; when it covers the cheapest vault unit on `bookKey`, one unit is bought with your saved card, never above the budget.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetVaultMarketSupplyRoundUpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_tod_beneficiary

> crate::models::UpdateVaultMarketTodBeneficiaryResponse update_vault_market_tod_beneficiary()
Replace your beneficiary list (keep an id to edit one): full name, contact (email or @username), relationship, and share in basis points; shares add up to 10000. Returns `{ beneficiaries, removed }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateVaultMarketTodBeneficiaryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_tod_designation

> crate::models::UpdateVaultMarketTodDesignationResponse update_vault_market_tod_designation()
Leave one of your units to one beneficiary (beneficiaryId null clears it; the unit then follows the shares). Returns `{ vaultItemId, beneficiaryId }`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateVaultMarketTodDesignationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_world_room_vault_gate

> crate::models::UpdateVaultMarketWorldRoomVaultGateResponse update_vault_market_world_room_vault_gate(id)
Set the vault-holder door on your own room (its collection rooms follow): visitors must hold minValueCents (cents, at reliable A/B marks) and/or minUnits units in the Crossly vault, mode any (either) or all (both), optionally only one product (scope kind product, ref the catalogue product id), bullion family (bullion, the family slug) or category (category, the slug). Send null to open it again. Returns `{ gate }`. Moves no money.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateVaultMarketWorldRoomVaultGateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_vault_market_world_space_vault_gate

> crate::models::UpdateVaultMarketWorldRoomVaultGateResponse update_vault_market_world_space_vault_gate(slug)
Set the vault-holder door on a Space you host: the same rule as a room (minValueCents at reliable marks and/or minUnits, mode, optional scope). Send null to open it again. Returns `{ gate }`. Moves no money.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::UpdateVaultMarketWorldRoomVaultGateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

