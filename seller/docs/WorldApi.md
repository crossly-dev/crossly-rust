# \WorldApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_spatial_invite_accept**](WorldApi.md#create_spatial_invite_accept) | **POST** /v1/spatial/invites/accept | Redeem an invite code (dashes optional); returns the room it opens.
[**create_spatial_property_lease_withdraw**](WorldApi.md#create_spatial_property_lease_withdraw) | **POST** /v1/spatial/property/leases/{id}/withdraw | Withdraw a lease offer nobody has accepted yet.
[**create_spatial_scene**](WorldApi.md#create_spatial_scene) | **POST** /v1/spatial/scenes | Start a 3D room: a collection (your items on shelves) or a warehouse (racks you arrange). Private until you make it public.
[**create_spatial_scene_grant**](WorldApi.md#create_spatial_scene_grant) | **POST** /v1/spatial/scenes/{id}/grants | Let a specific user into one of your rooms (visitor or staff), no code needed.
[**create_spatial_scene_invite**](WorldApi.md#create_spatial_scene_invite) | **POST** /v1/spatial/scenes/{id}/invites | Mint an invite code to one of your rooms (visitor or staff, optional expiry in hours). The code is in this response only: just its hash is stored.
[**create_spatial_scene_node**](WorldApi.md#create_spatial_scene_node) | **POST** /v1/spatial/scenes/{id}/nodes | Add a unit (rack, shelf, bin) to one of your warehouse rooms.
[**create_spatial_scene_place**](WorldApi.md#create_spatial_scene_place) | **POST** /v1/spatial/scenes/{id}/place | Pin an inventory item to a slot in one of your rooms: by warehouse nodeId, or by a collection container key and index.
[**create_spatial_scene_rename**](WorldApi.md#create_spatial_scene_rename) | **POST** /v1/spatial/scenes/{id}/rename | Rename one of your rooms.
[**create_spatial_scene_shop_place**](WorldApi.md#create_spatial_scene_shop_place) | **POST** /v1/spatial/scenes/{id}/shop/place | Place an inventory item in a section and display of your shop room.
[**create_spatial_scene_unpin**](WorldApi.md#create_spatial_scene_unpin) | **POST** /v1/spatial/scenes/{id}/unpin | Clear every pin in one of your rooms, so it arranges itself again.
[**create_spatial_scene_visibility**](WorldApi.md#create_spatial_scene_visibility) | **POST** /v1/spatial/scenes/{id}/visibility | Make one of your rooms private, unlisted (anyone with the link) or public (in the lobby and searchable).
[**create_spatial_scene_warehouse_stock**](WorldApi.md#create_spatial_scene_warehouse_stock) | **POST** /v1/spatial/scenes/{id}/warehouse/stock | Put an inventory item (or unit) on a rack in one of your warehouse rooms.
[**create_spatial_shop_neighbourhood**](WorldApi.md#create_spatial_shop_neighbourhood) | **POST** /v1/spatial/shop/neighbourhood | Move your shop to another neighbourhood (by slug).
[**create_spatial_street_apply**](WorldApi.md#create_spatial_street_apply) | **POST** /v1/spatial/streets/{slug}/apply | Ask to open your shop on a street whose owner reviews applications.
[**create_spatial_street_invite**](WorldApi.md#create_spatial_street_invite) | **POST** /v1/spatial/streets/{slug}/invite | Invite a shop (by its slug) to open on a street you own.
[**create_world_avatar_equip**](WorldApi.md#create_world_avatar_equip) | **POST** /v1/world/avatar/equip | Wear something you already own -- exactly ONE of { cosmeticId } (a cosmetic you hold), { receiptId } (a Crossly purchase from your closet) or { inventoryItemId } (your own stock). The slot comes from the thing itself; slot is only a fallback for an item with no catalog shape. Replaces whatever was in that slot. Moves no money. Answers the slot and your loadout after the change.
[**create_world_avatar_unequip**](WorldApi.md#create_world_avatar_unequip) | **POST** /v1/world/avatar/unequip | Take off whatever is in one slot. Idempotent: an empty slot answers removed:false. Answers your loadout after the change.
[**create_world_avatar_wearable**](WorldApi.md#create_world_avatar_wearable) | **POST** /v1/world/avatar/wearables | Author a wearable for a body slot (a mesh profile and params, an asking price in COSM base units as a string). A submission: inactive until an operator approves it.
[**create_world_contest**](WorldApi.md#create_world_contest) | **POST** /v1/world/contests | Set up a skill contest on a game placed in your place: title, game (its instance id), entry { mode, feeCents? }, prizes by place, funder, window, attempts. Paid entry needs the operator to allow it where it runs.
[**create_world_contest_entry_settle_payment**](WorldApi.md#create_world_contest_entry_settle_payment) | **POST** /v1/world/contests/entries/{id}/settle-payment | Read the fee of a paid entry back from Stripe and settle it (safe to repeat).
[**create_world_contest_join**](WorldApi.md#create_world_contest_join) | **POST** /v1/world/contests/{id}/join | Enter a contest. Free and sponsored contests take you in at once; a paid one answers a client secret to pay the entry fee.
[**create_world_convention**](WorldApi.md#create_world_convention) | **POST** /v1/world/conventions | Host a convention in one of your rooms: title, dates, tables and how sellers get one (invite, free, a fee in cents, or a revenue share in basis points).
[**create_world_convention_accept**](WorldApi.md#create_world_convention_accept) | **POST** /v1/world/conventions/{id}/accept | Accept: your own invitation (no body), or, as host, an application (username).
[**create_world_convention_apply**](WorldApi.md#create_world_convention_apply) | **POST** /v1/world/conventions/{id}/apply | Ask for a table at a convention.
[**create_world_convention_invite**](WorldApi.md#create_world_convention_invite) | **POST** /v1/world/conventions/{id}/invite | Invite a seller (by shop handle) to a convention you host.
[**create_world_convention_paid**](WorldApi.md#create_world_convention_paid) | **POST** /v1/world/conventions/{id}/paid | As host, record that a seller (username) paid their table fee off-platform. Moves no money.
[**create_world_convention_withdraw**](WorldApi.md#create_world_convention_withdraw) | **POST** /v1/world/conventions/{id}/withdraw | Withdraw your table or decline your invitation; as host, decline someone (username).
[**create_world_crew**](WorldApi.md#create_world_crew) | **POST** /v1/world/crews | Start a crew you own: { name (2-32 chars), tag (2-5 letters or digits, shown on members' nameplates) }. Answers its id and join code. 409 when you are already in a crew.
[**create_world_crew_chat**](WorldApi.md#create_world_crew_chat) | **POST** /v1/world/crew/chat | Say a line to your crew (body: text, up to 400 characters). Returns data.messages, the crew chat ring (the last 50 lines, kept in memory only), and data.said when your line went out; a blocked or too-quick line is dropped silently. 404 when you are in no crew.
[**create_world_crew_join**](WorldApi.md#create_world_crew_join) | **POST** /v1/world/crews/join | Join a crew by the code a member gave you. 404 for an unknown code, 409 when you are already in a crew or it is full (30).
[**create_world_crew_leave**](WorldApi.md#create_world_crew_leave) | **POST** /v1/world/crews/leave | Leave your crew. If you own it, the crew is disbanded. Idempotent: leaving when in none succeeds.
[**create_world_decor_design**](WorldApi.md#create_world_decor_design) | **POST** /v1/world/decor/designs | Author a decor design (what it unlocks, a mesh profile and params, an asking price in COSM base units as a string). Inactive until an operator approves it.
[**create_world_decor_design_submit**](WorldApi.md#create_world_decor_design_submit) | **POST** /v1/world/decor/designs/{id}/submit | Send one of your decor designs to the review queue.
[**create_world_decor_retract**](WorldApi.md#create_world_decor_retract) | **POST** /v1/world/decor/retract | Take a placed decor piece back into your inventory; answers the inventory after.
[**create_world_dm**](WorldApi.md#create_world_dm) | **POST** /v1/world/dms/{userId} | Send a direct message to a friend (a mutual follow; never across a block), up to 400 characters, through the world chat guard. 403 when they are not a friend, 429 when sent too quickly.
[**create_world_follow**](WorldApi.md#create_world_follow) | **POST** /v1/world/follows/{userId} | Follow somebody. When they follow you back you are friends (friends-here, invites, messages). Idempotent.
[**create_world_friend_invite**](WorldApi.md#create_world_friend_invite) | **POST** /v1/world/friends/{userId}/invite | \"Join me\": send one friend (a mutual follow) a notification with a link to the PUBLIC room you name by slug. 404 when they are not a friend or the room is not public; 429 when you invited them in the last minute.
[**create_world_game_package**](WorldApi.md#create_world_game_package) | **POST** /v1/world/games/packages | Submit a game package version for review: files as { path, base64 } (game.json, thumb.png, logic/, scene/, audio/, images/). Checked like the CLI, assets quarantined, the entry document dry-run, then reviewed by a person.
[**create_world_game_package_check**](WorldApi.md#create_world_game_package_check) | **POST** /v1/world/games/packages/check | Validate a game package exactly as a submission is checked (the shared intake report, your version still free) without storing, uploading or queuing anything: world-kit validate for an agent. Same files body as submit.
[**create_world_game_rating**](WorldApi.md#create_world_game_rating) | **POST** /v1/world/games/{id}/rating | Rate a game you played, 1 to 5 stars (rating again replaces it). Only players the runner recorded in that game can rate it.
[**create_world_game_report**](WorldApi.md#create_world_game_report) | **POST** /v1/world/games/{id}/report | Report a game: reason broken | unfair | offensive | scam | other, optional detail. Staff review it; the creator is told the reason, never who.
[**create_world_game_store_checkout**](WorldApi.md#create_world_game_store_checkout) | **POST** /v1/world/games/{id}/store/{item}/checkout | Buy an item or pass from a game store: answers a client secret for the payment; the item is yours once it succeeds.
[**create_world_grail_match**](WorldApi.md#create_world_grail_match) | **POST** /v1/world/grails/match | Grail glints: send the ids on show somewhere (listingIds -- listings.id, inventoryIds, skuIds -- catalogue market SKUs; up to 400 each) and get back data.listingIds / data.inventoryIds / data.skuIds: the ones that are things you already want -- an open bid, a watched or wishlisted listing (or one selling the same SKU), a market watch, or a card your deck is missing. Read-only.
[**create_world_hall_loan**](WorldApi.md#create_world_hall_loan) | **POST** /v1/world/hall/loans | Lend a piece from your vault to this week's Hall of Grails exhibition: { vaultItemId, credit: 'name' | 'anonymous' }. It shows under your plaque until the exhibition ends; it stays in your vault (still listable and withdrawable), and stops showing if it leaves.
[**create_world_home_ask**](WorldApi.md#create_world_home_ask) | **POST** /v1/world/homes/{slug}/ask | Ask a home owner whether they would sell a piece you saw there (an item or a decor piece).
[**create_world_home_guestbook**](WorldApi.md#create_world_home_guestbook) | **POST** /v1/world/homes/{slug}/guestbook | Sign a home guestbook (one short line).
[**create_world_home_guestbook_report**](WorldApi.md#create_world_home_guestbook_report) | **POST** /v1/world/homes/{slug}/guestbook/{id}/report | Report a guestbook entry to moderation.
[**create_world_hunt_start**](WorldApi.md#create_world_hunt_start) | **POST** /v1/world/hunt/start | Start the Great Hunt this season by reading its first riddle (the Copper key's). Idempotent: starting again answers your progress unchanged. Answers the same shape as GET /v1/world/hunt. 409 when no season is running.
[**create_world_interactive_asset**](WorldApi.md#create_world_interactive_asset) | **POST** /v1/world/interactive/assets | Upload one asset for your designs: kind glb (4 MB), wav (4 MB) or png (1 MB), the file as base64. Answers its sha256 and the path to reference from MML.
[**create_world_interactive_design**](WorldApi.md#create_world_interactive_design) | **POST** /v1/world/interactive/designs | Create an interactive design: name, footprint (s, m, l or f), optional tint and description, an asking price in COSM base units (a string, six decimals) and optional max supply. Inactive until ops approve a submitted version.
[**create_world_interactive_design_check**](WorldApi.md#create_world_interactive_design_check) | **POST** /v1/world/interactive/designs/{id}/check | Check the draft: static rules, that every referenced upload exists, and a sandboxed dry run. Budgeted per person.
[**create_world_interactive_design_preview**](WorldApi.md#create_world_interactive_design_preview) | **POST** /v1/world/interactive/designs/{id}/preview | A private preview token for the draft (author only) and the socket path that runs it. Budgeted per person.
[**create_world_interactive_design_submit**](WorldApi.md#create_world_interactive_design_submit) | **POST** /v1/world/interactive/designs/{id}/submit | Freeze the draft as a new version and send it to ops review. Up to 10 a day.
[**create_world_interactive_reset**](WorldApi.md#create_world_interactive_reset) | **POST** /v1/world/interactive/reset | Wipe the stored state of an interactive piece standing in a place you own (a home or your boutique).
[**create_world_job_appraisal_answer**](WorldApi.md#create_world_job_appraisal_answer) | **POST** /v1/world/jobs/appraisals/{id}/answers | Answer a price check: your estimate of the item value in CENTS and an optional note. Moves no money; the asker may accept it.
[**create_world_job_shift_claim**](WorldApi.md#create_world_job_shift_claim) | **POST** /v1/world/jobs/shifts/{id}/claim | Claim an open shift. You work it by being in the place while it runs.
[**create_world_job_shift_release**](WorldApi.md#create_world_job_shift_release) | **POST** /v1/world/jobs/shifts/{id}/release | Give a shift you claimed back to the board.
[**create_world_outfit**](WorldApi.md#create_world_outfit) | **POST** /v1/world/outfits | Save what you are wearing now as a named look: { name (1-32 chars), appearance? (skin, hair, hairColour, eyes, build -- unknown keys fall back to the default) }. Each worn thing is recorded as the door it goes back on by; one that no longer traces to something you own is listed in skipped and not saved. 409 when you already keep 6.
[**create_world_outfit_apply**](WorldApi.md#create_world_outfit_apply) | **POST** /v1/world/outfits/{id}/apply | Put a saved look on. Every item goes through the same equip as POST /v1/world/avatar/equip, so ownership still holds: something you no longer own is listed in skipped with the reason and not worn. Slots the look left empty are emptied (cleared). Answers the appearance of the look (for your character -- appearance lives on your device), applied, skipped and cleared. Moves no money.
[**create_world_party**](WorldApi.md#create_world_party) | **POST** /v1/world/parties | Start a party with you as leader (up to 8 members).
[**create_world_party_chat**](WorldApi.md#create_world_party_chat) | **POST** /v1/world/parties/{id}/chat | Say something in your party chat (up to 2000 characters).
[**create_world_party_invite**](WorldApi.md#create_world_party_invite) | **POST** /v1/world/parties/{id}/invite | Invite a friend (userId) to your party.
[**create_world_party_invite_accept**](WorldApi.md#create_world_party_invite_accept) | **POST** /v1/world/parties/invites/{inviteId}/accept | Accept a party invite.
[**create_world_party_invite_decline**](WorldApi.md#create_world_party_invite_decline) | **POST** /v1/world/parties/invites/{inviteId}/decline | Decline a party invite.
[**create_world_party_kick**](WorldApi.md#create_world_party_kick) | **POST** /v1/world/parties/{id}/kick | As leader, remove a member (userId).
[**create_world_party_leader**](WorldApi.md#create_world_party_leader) | **POST** /v1/world/parties/{id}/leader | As leader, pass the lead to a member (userId).
[**create_world_party_leave**](WorldApi.md#create_world_party_leave) | **POST** /v1/world/parties/{id}/leave | Leave your party (the lead passes on if you led it).
[**create_world_party_wishlist**](WorldApi.md#create_world_party_wishlist) | **POST** /v1/world/parties/{id}/wishlist | Add a listing (listingId, or its shop slug) to the party wishlist; answers the wishlist.
[**create_world_party_wishlist_vote**](WorldApi.md#create_world_party_wishlist_vote) | **POST** /v1/world/parties/{id}/wishlist/{listingId}/vote | Thumb a wishlist listing up or down (null clears your vote); answers the wishlist.
[**create_world_place_item_want**](WorldApi.md#create_world_place_item_want) | **POST** /v1/world/places/{slug}/items/{itemId}/want | Tell a room owner you would buy one of their items (once per person per item; they see the count). Moves no money.
[**create_world_space**](WorldApi.md#create_world_space) | **POST** /v1/world/spaces | Open a Space (sellers only, at most two): slug, name, venue, optional membership and about.
[**create_world_space_event**](WorldApi.md#create_world_space_event) | **POST** /v1/world/spaces/{slug}/events | As host or co-host, put an event on: title, kind, optional about, ISO start and end with offset.
[**create_world_space_event_rsvp**](WorldApi.md#create_world_space_event_rsvp) | **POST** /v1/world/spaces/{slug}/events/{id}/rsvp | Say whether you are going to an event.
[**create_world_space_invite**](WorldApi.md#create_world_space_invite) | **POST** /v1/world/spaces/{slug}/invite | Invite somebody (userId) to a Space.
[**create_world_space_join**](WorldApi.md#create_world_space_join) | **POST** /v1/world/spaces/{slug}/join | Join a Space, or ask to when it is request-only.
[**create_world_space_leave**](WorldApi.md#create_world_space_leave) | **POST** /v1/world/spaces/{slug}/leave | Leave a Space.
[**create_world_space_request_by_user_id**](WorldApi.md#create_world_space_request_by_user_id) | **POST** /v1/world/spaces/{slug}/requests/{userId} | As host or co-host, accept or turn down a request to join.
[**create_world_space_role**](WorldApi.md#create_world_space_role) | **POST** /v1/world/spaces/{slug}/role | As host, make a member a co-host or back to a member.
[**delete_spatial_scene_grant**](WorldApi.md#delete_spatial_scene_grant) | **DELETE** /v1/spatial/scenes/{id}/grants/{grantId} | Take somebody's access to your room away, or kill an invite nobody has used yet.
[**delete_spatial_scene_node**](WorldApi.md#delete_spatial_scene_node) | **DELETE** /v1/spatial/scenes/{id}/nodes | Remove units (by id, up to 500) from one of your warehouse rooms.
[**delete_spatial_scene_warehouse_stock**](WorldApi.md#delete_spatial_scene_warehouse_stock) | **DELETE** /v1/spatial/scenes/{id}/warehouse/stock | Take an inventory item (or unit) off its rack.
[**delete_spatial_street_request**](WorldApi.md#delete_spatial_street_request) | **DELETE** /v1/spatial/streets/requests/{id} | Withdraw a street request you made that has not been founded yet.
[**delete_world_block**](WorldApi.md#delete_world_block) | **DELETE** /v1/world/blocks/{id} | Unblock somebody (by the block id from GET /v1/world/blocks).
[**delete_world_collection_capability**](WorldApi.md#delete_world_collection_capability) | **DELETE** /v1/world/collections/{chain}/{contract}/capabilities | Withdraw your declaration for a collection.
[**delete_world_follow**](WorldApi.md#delete_world_follow) | **DELETE** /v1/world/follows/{userId} | Unfollow somebody. Idempotent.
[**delete_world_hall_loan**](WorldApi.md#delete_world_hall_loan) | **DELETE** /v1/world/hall/loans/{id} | Take one of your loans off show in the Hall of Grails.
[**delete_world_home_guestbook**](WorldApi.md#delete_world_home_guestbook) | **DELETE** /v1/world/homes/{slug}/guestbook/{id} | As the home owner, hide a guestbook entry.
[**delete_world_interactive_design**](WorldApi.md#delete_world_interactive_design) | **DELETE** /v1/world/interactive/designs/{id} | Delete a draft design, or archive a submitted one (off sale; owned copies keep working).
[**delete_world_job_shift**](WorldApi.md#delete_world_job_shift) | **DELETE** /v1/world/jobs/shifts/{id} | Cancel a shift you posted that nobody has worked yet.
[**delete_world_outfit**](WorldApi.md#delete_world_outfit) | **DELETE** /v1/world/outfits/{id} | Delete a saved look. 404 when it is not yours.
[**delete_world_party_wishlist**](WorldApi.md#delete_world_party_wishlist) | **DELETE** /v1/world/parties/{id}/wishlist/{listingId} | Take a listing you added off the party wishlist; answers the wishlist.
[**delete_world_place_ban**](WorldApi.md#delete_world_place_ban) | **DELETE** /v1/world/places/{slug}/bans/{userId} | Lift a ban from a place you own; they can walk in again right away.
[**delete_world_space**](WorldApi.md#delete_world_space) | **DELETE** /v1/world/spaces/{slug} | As host, close a Space.
[**delete_world_space_event**](WorldApi.md#delete_world_space_event) | **DELETE** /v1/world/spaces/{slug}/events/{id} | As host or co-host, cancel an event.
[**get_spatial_scene_attention**](WorldApi.md#get_spatial_scene_attention) | **GET** /v1/spatial/scenes/{sceneId}/attention | What visitors looked at in one of your rooms: viewers, looks, inspects and dwell per item over the last days (1-35, default 30).
[**get_spatial_scene_window**](WorldApi.md#get_spatial_scene_window) | **GET** /v1/spatial/scenes/{id}/window | The listings you chose for the street window of your shop, in slot order (left, middle, right; empty = newest first), and `choices`: your listings that can go in it, newest first.
[**get_spatial_shop_neighbourhood**](WorldApi.md#get_spatial_shop_neighbourhood) | **GET** /v1/spatial/shop/neighbourhood | Your shop's neighbourhood, and whether and where it may move.
[**get_spatial_space**](WorldApi.md#get_spatial_space) | **GET** /v1/spatial/spaces | The room catalogue: every space profile (what it holds, how it is sorted, its containers) and the warehouse units.
[**get_spatial_street_admission**](WorldApi.md#get_spatial_street_admission) | **GET** /v1/spatial/streets/admissions | Street applications and invitations: yours, and those waiting on streets you own.
[**get_spatial_street_decor**](WorldApi.md#get_spatial_street_decor) | **GET** /v1/spatial/streets/{slug}/decor | The decor on a street you may dress, and what you can place.
[**get_world_arcade**](WorldApi.md#get_world_arcade) | **GET** /v1/world/arcade | The Arcade in Crossly World: data.cabinets (slot, where it stands, host -- piece, price_guess or empty --, game, title, blurb, the piece it hosts and where, record, packageId and thumbnail for a creator game from the Game Directory, players) and data.scores (the High Score wall: per game a title, kind -- leaderboard, record or top -- and up to five ASCII lines; a name shows only for somebody who chose to be seen by name, everyone else is \"A player\"), plus creatorGames (creator games with a cabinet now). Games are played in the world.
[**get_world_aurora**](WorldApi.md#get_world_aurora) | **GET** /v1/world/aurora | The Sales Aurora over the Commons in Crossly World: every real sale on Crossly in the last data.windowDays (30) days. data.total (sales in the window), data.families (key, label, colour, count per colour family: cards, coins, jewellery, electronics, media, fashion, toys, other) and data.sales (newest first, at most 48: family, ageHours, fresh -- sold in the last ten minutes, which shimmers). Aggregate only: no item, price, seller or platform. data.updatedAt is when it was read.
[**get_world_auto_shop**](WorldApi.md#get_world_auto_shop) | **GET** /v1/world/auto-shop | Whether Crossly keeps a world shop for you automatically: enabled, and the shop (id, name, visibility, autoCreated, address, label) or null. Change it with PATCH /v1/world/auto-shop.
[**get_world_avatar_identity**](WorldApi.md#get_world_avatar_identity) | **GET** /v1/world/avatar/identity | Whether others see your name in the world (named), or not (anonymous); null if you have never chosen.
[**get_world_avatar_loadout**](WorldApi.md#get_world_avatar_loadout) | **GET** /v1/world/avatar/loadout | What you are wearing: the rig, its slots (avatar_head, avatar_body, avatar_outer -- a jacket over the top, avatar_legs, avatar_feet, avatar_back) and each worn entry -- a cosmetic or a real item -- with its verified-purchase mark. Carries no price, receipt id, seller or order.
[**get_world_avatar_loadout_by_user_id**](WorldApi.md#get_world_avatar_loadout_by_user_id) | **GET** /v1/world/avatar/loadout/{userId} | What somebody else is wearing, with the verified-purchase mark on each real item -- public on purpose, since a check only its owner can see is not a check. No price, receipt id, seller or order.
[**get_world_avatar_look**](WorldApi.md#get_world_avatar_look) | **GET** /v1/world/avatar/look | Your world look on the account: the handle you chose (null if anonymous) and your appearance; null if you never made one.
[**get_world_avatar_nft_badge**](WorldApi.md#get_world_avatar_nft_badge) | **GET** /v1/world/avatar/nft-badge | The NFT you wear on your nameplate in the world, or null.
[**get_world_capability**](WorldApi.md#get_world_capability) | **GET** /v1/world/capabilities | What your held NFTs unlock in Crossly World: each token with the capabilities its collection declared (wear, display, emote, gate, ...). Holding is your verified wallets (their synced tokens); the capabilities of a transferred token go within five minutes.
[**get_world_capability_catalogue**](WorldApi.md#get_world_capability_catalogue) | **GET** /v1/world/capabilities/catalogue | The capability kinds of spec crossly.nft-capabilities/1: what each does, whether only the creator of the collection may declare it, whether it needs review, and which clients honour it.
[**get_world_capability_wear**](WorldApi.md#get_world_capability_wear) | **GET** /v1/world/capabilities/wear | What you wear from your NFTs and COSM copies: your pick per avatar slot (head, body, legs, feet, back) and what the room sees.
[**get_world_chest**](WorldApi.md#get_world_chest) | **GET** /v1/world/chests | The Commons chest: data.window (this 20-minute window, UTC), spot ([x, z] on the plaza), endsAt (when the next chest comes) and mine (openedThis, and opened -- how many windows you have opened one in; the Chest opener pin comes at 10). A chest is opened by walking up to it in Crossly World, never over the API.
[**get_world_collection_capability**](WorldApi.md#get_world_collection_capability) | **GET** /v1/world/collections/{chain}/{contract}/capabilities | The live capability declaration of a collection (404 when it has none).
[**get_world_contest_mine**](WorldApi.md#get_world_contest_mine) | **GET** /v1/world/contests/mine | Your contest entries (best score, attempts used, payment state) and prizes (granted, or owed until their gate opens).
[**get_world_convention**](WorldApi.md#get_world_convention) | **GET** /v1/world/conventions/{id} | One convention. 404 for an unknown or malformed id alike.
[**get_world_cosm_balance**](WorldApi.md#get_world_cosm_balance) | **GET** /v1/world/cosm/balance | Your COSM balance: baseUnits (a STRING -- COSM has 6 decimals and overflows a JSON number), decimals, available (false = COSM is not provisioned on this deployment, not a zero balance).
[**get_world_crew**](WorldApi.md#get_world_crew) | **GET** /v1/world/crew | Your crew: data.crew is null when you are in none, else its name, tag, join code and whether you own it; data.members lists up to 30 members, each with the public room they are in right now when they chose to be seen by name (else null).
[**get_world_crew_chat**](WorldApi.md#get_world_crew_chat) | **GET** /v1/world/crew/chat | Your crew chat: data.messages is the last 50 lines (from, name, text, at), kept in memory only, so it starts empty after a restart. 404 when you are in no crew.
[**get_world_crew_leaderboard**](WorldApi.md#get_world_crew_leaderboard) | **GET** /v1/world/crew/leaderboards | The game leaderboards (Price Guess, hunts, speed trades; this week and all time) ranked among your crew only, each member by the name the crew sees. 404 when you are in no crew.
[**get_world_dm**](WorldApi.md#get_world_dm) | **GET** /v1/world/dms/{userId} | Your direct-message thread with one friend, oldest first (the last 100 lines: id, from, text, at, mine). Reading it marks their lines read.
[**get_world_emote_showable**](WorldApi.md#get_world_emote_showable) | **GET** /v1/world/emotes/showable | The things you own that a show-off emote can hold up.
[**get_world_firefly**](WorldApi.md#get_world_firefly) | **GET** /v1/world/fireflies | Fireflies on the Commons lawn: data.day (today, UTC), today (fireflies everyone has caught today) and mine (today -- your catches today, up to perDay -- total, all your catches; the Firefly Catcher pin comes at 30 -- and perDay). Fireflies are caught by walking up to a lit one in Crossly World after dark, never over the API, and nothing is spent.
[**get_world_fit_stat**](WorldApi.md#get_world_fit_stat) | **GET** /v1/world/fits/stats | Your walking storefront, last 7 days: distinct people who tapped \"Shop this\" or \"Offer\" on what you wore, and how many of each. Counts only; nobody is identified.
[**get_world_fit_value**](WorldApi.md#get_world_fit_value) | **GET** /v1/world/fit-value | What your outfit is worth on the market: cents (sum of each catalogued piece's order-book price -- last sale, else lowest ask), pieces worn, how many were priced, and share (whether you show it on your card).
[**get_world_game**](WorldApi.md#get_world_game) | **GET** /v1/world/games/{id} | One live game: its manifest, its store items and which you own.
[**get_world_game_leaderboard**](WorldApi.md#get_world_game_leaderboard) | **GET** /v1/world/games/{id}/leaderboard | A live game leaderboard, this week and all time, by its manifest scoring (wins, best points, or best time). Only people who chose to be named show by name. scoring is null for a game that keeps no board.
[**get_world_game_package**](WorldApi.md#get_world_game_package) | **GET** /v1/world/games/packages/{id} | One of your game packages with every version, its review state and decision note.
[**get_world_game_template**](WorldApi.md#get_world_game_template) | **GET** /v1/world/games/templates/{kind} | A fresh game package from a template (table, arena, object; collection is an NFT capability declaration), as { path, base64 } files: the world-kit init, for an agent with no local kit. ?id=my-game&name=My Game.
[**get_world_hall**](WorldApi.md#get_world_hall) | **GET** /v1/world/hall | The Hall of Grails in Crossly World: data.exhibition (key, title, blurb, endsAt -- a new one every Monday UTC -- and next, the coming one's title) and data.exhibits, up to twelve real catalogue items in slot order (skuId, name, brand, imageUrl, thumbUrl, retailCents, lowestAskCents, lastSaleCents: the live order book, null when the market has none). The first stands on the centre plinth. Buy, bid or watch an exhibit through the market endpoints with its skuId.
[**get_world_hall_loan**](WorldApi.md#get_world_hall_loan) | **GET** /v1/world/hall/loans | Lend to the Hall: this week's exhibition (key, title, categorySlug, endsAt), your loans on show (id, vaultItemId, skuId, name, credit, since) and the vault pieces you can lend (eligible: vaultItemId, skuId, name, imageUrl, grade) -- held in your Crossly vault, not pledged, in the exhibition's category. perMember is how many you may lend each week.
[**get_world_home_placeable**](WorldApi.md#get_world_home_placeable) | **GET** /v1/world/home/placeable | What you can place in your home: your items and your furniture.
[**get_world_hunt**](WorldApi.md#get_world_hunt) | **GET** /v1/world/hunt | Your Great Hunt: data.season (id, name, startsAt, endsAt, the title and how many earn it, live), started (you read the first clue), held (keys held, 0-3, always in the order copper, jade, crystal), keys (each with its name, colour, its riddle once it is yours to read, and when you picked it up), and cleared (your rank through the Gate this season, when, and whether you earned the title) or null. Keys are picked up by walking to them in Crossly World; nothing here takes one.
[**get_world_hunt_scoreboard**](WorldApi.md#get_world_hunt_scoreboard) | **GET** /v1/world/hunt/scoreboard | The Great Hunt Scoreboard, as the monolith in the Commons shows it: data.season (the running season, or the last one between seasons), finishers (the first ten through the Gate in order: rank, name, short, named, at, titled -- a name shows only for somebody who chose to be seen by name in the world, everyone else is \"A player\") and total (everybody through the Gate this season).
[**get_world_interactive_design**](WorldApi.md#get_world_interactive_design) | **GET** /v1/world/interactive/designs/{id} | One of your interactive designs: its draft source, the last static check, and every submitted version with its review state.
[**get_world_job**](WorldApi.md#get_world_job) | **GET** /v1/world/jobs | The jobs board: open staff shifts you could claim, your claimed shifts and open price checks (appraisals), as the world shows them.
[**get_world_kite**](WorldApi.md#get_world_kite) | **GET** /v1/world/kites | Kites flown on the Commons lawn: data.day (today, UTC), today (kites everyone has flown today) and mine (today -- yours today, up to perDay -- perDay, and days -- the days you have flown one on; the Kite flyer pin comes at 5). Kites are flown by standing out on the lawn in Crossly World by day, never over the API, and nothing is spent.
[**get_world_lantern**](WorldApi.md#get_world_lantern) | **GET** /v1/world/lanterns | Sky lanterns in the Commons: data.day (today, UTC), today (lanterns everyone has released today) and mine (today -- yours today, up to perDay -- perDay, and days -- the days you have released one on; the Lantern lighter pin comes at 5). Lanterns are released at the lantern stand in Crossly World, never over the API, and nothing is spent.
[**get_world_party_chat**](WorldApi.md#get_world_party_chat) | **GET** /v1/world/parties/{id}/chat | Your party chat: the recent lines. 404 when this is not your party.
[**get_world_place_board**](WorldApi.md#get_world_place_board) | **GET** /v1/world/places/{slug}/boards | The Trade Floor's wall of wants, its authentication desk and its Vault Markets ticker (vaultMovers: bookKey, name, label, units, markCents, change24hBps; only the trade-floor place has them).
[**get_world_place_collection**](WorldApi.md#get_world_place_collection) | **GET** /v1/world/places/{slug}/collections/{category}/{brand}/{model} | One collection room: every piece of the set, owned or missing. 404 for an unknown room or set.
[**get_world_place_drop**](WorldApi.md#get_world_place_drop) | **GET** /v1/world/places/{slug}/drop | The owner's next drop (name, cover, time, live) and how many people are waiting in the room.
[**get_world_place_item_interest**](WorldApi.md#get_world_place_item_interest) | **GET** /v1/world/places/{slug}/items/{itemId}/interest | How many people looked at one item in this room today.
[**get_world_place_live**](WorldApi.md#get_world_place_live) | **GET** /v1/world/places/{slug}/live | The owner's live sale: their stream and the lot on the block, or null.
[**get_world_place_seller**](WorldApi.md#get_world_place_seller) | **GET** /v1/world/places/{slug}/seller | The shop whose public room this is (username and name), or null.
[**get_world_place_shop_pet**](WorldApi.md#get_world_place_shop_pet) | **GET** /v1/world/places/{slug}/shop-pet | The shop's pet kind, or null.
[**get_world_space**](WorldApi.md#get_world_space) | **GET** /v1/world/spaces/{slug} | One Space: its layout, host, events and your place in it.
[**get_world_space_can_host**](WorldApi.md#get_world_space_can_host) | **GET** /v1/world/spaces/can-host | Whether you may open a Space: sellers only, at most two active.
[**get_world_trial_leaderboard**](WorldApi.md#get_world_trial_leaderboard) | **GET** /v1/world/trials/{course}/leaderboard | A Time Trials course's boards, as the board beside it in the Commons shows them: data.course, week, weekly (this week's ten fastest: rank, name, short, named, ms, at) and allTime (the ten fastest ever, each person once). A name shows only for somebody who chose to be seen by name in the world; everyone else is \"A player\". 404 for an unknown course; course 1 is commons-loop.
[**get_world_trial_mine**](WorldApi.md#get_world_trial_mine) | **GET** /v1/world/trials/mine | Your Hoverboard Time Trials bests: data.week (its Monday, YYYY-MM-DD, UTC) and courses, each with course (id, name, parMs, rings, lengthM), week (your best lap this week in ms, your place on the weekly board, laps finished, and the split at each ring) or null, and allTime (your best ever in ms, your place on the all-time board, when) or null. Laps are timed by Crossly from where it saw you ride in the world; nothing here writes one.
[**get_world_wallet**](WorldApi.md#get_world_wallet) | **GET** /v1/world/wallet | Every balance you hold in Crossly World, read-only: bucksCents (Crossly Bucks, cents); cbx (available, decimals, and the three CBX balances kept apart as base-unit STRINGS -- granted (spend here only), earned (withdrawable once matured), connected (a delegation ceiling on your own wallet, not cash on hand) -- plus pendingCents of cashback not yet matured); cosm (available, decimals, baseUnits string). available:false means that currency is not provisioned on this deployment, not that you hold none.
[**get_world_wish**](WorldApi.md#get_world_wish) | **GET** /v1/world/wishes | The Wishing Fountain in the Commons: data.day (today, UTC), today (wishes everyone has made today) and mine (madeToday, days -- how many days you have wished on, the Wishmaker pin comes at 7 -- and nextAt, when tomorrow's wish opens). One wish a day; a wish is made by tossing a coin at the fountain in Crossly World, never over the API, and nothing is spent.
[**list_spatial_scene_grants**](WorldApi.md#list_spatial_scene_grants) | **GET** /v1/spatial/scenes/{id}/grants | Everyone who can currently walk into one of your rooms, and unused invites. Never returns codes.
[**list_spatial_scene_warehouse_unplaced**](WorldApi.md#list_spatial_scene_warehouse_unplaced) | **GET** /v1/spatial/scenes/{id}/warehouse/unplaced | Stock not yet on a shelf in a warehouse room you can arrange (q filters by title).
[**list_spatial_shared_with_me**](WorldApi.md#list_spatial_shared_with_me) | **GET** /v1/spatial/shared-with-me | Rooms other people let you into.
[**list_spatial_street_leases**](WorldApi.md#list_spatial_street_leases) | **GET** /v1/spatial/streets/{slug}/leases | Plots on a street offered for lease: weekly rent in CENTS and terms.
[**list_world_avatar_closet**](WorldApi.md#list_world_avatar_closet) | **GET** /v1/world/avatar/closet | Everything you can verifiably wear (your Crossly purchase receipts), newest first: receiptId, item title, brand, size, image, catalog sku/variant, quantity, purchasedAt. Pass a receiptId to POST /v1/world/avatar/equip to put one on. limit 1-200, offset for paging.
[**list_world_avatar_stock**](WorldApi.md#list_world_avatar_stock) | **GET** /v1/world/avatar/stock | Items of your OWN inventory you could wear into a room as a walking storefront: inventoryItemId, title, image and the slot it would go on. Pass an inventoryItemId to POST /v1/world/avatar/equip.
[**list_world_avatar_wearables**](WorldApi.md#list_world_avatar_wearables) | **GET** /v1/world/avatar/wearables | Wearables you have authored as a designer: id, sku, name, slot, priceBaseUnits (COSM base units, a string), maxSupply, mintedCount, active (false until reviewed). meta.available is false when COSM is not provisioned here.
[**list_world_best_dressed**](WorldApi.md#list_world_best_dressed) | **GET** /v1/world/best-dressed | This week's best-dressed board (Monday-based), top ten: name, profile username and loves. Only people still choosing to be seen by name.
[**list_world_capability_declarations**](WorldApi.md#list_world_capability_declarations) | **GET** /v1/world/capabilities/declarations | Your capability declarations, in every review state (pending, live, rejected with the note from the reviewer).
[**list_world_contest_standings**](WorldApi.md#list_world_contest_standings) | **GET** /v1/world/contests/{id}/standings | The standings of a contest now: best score first (or lowest, for a time game), ties to whoever got there first.
[**list_world_contests**](WorldApi.md#list_world_contests) | **GET** /v1/world/contests | Open skill contests on Crossly World games, soonest ending first: entry mode (free, sponsored, paid) and prizes.
[**list_world_convention_exhibitors**](WorldApi.md#list_world_convention_exhibitors) | **GET** /v1/world/conventions/{id}/exhibitors | The host roster: who applied, who was invited, who is coming (by shop handle). Host only; 404 otherwise.
[**list_world_convention_mine**](WorldApi.md#list_world_convention_mine) | **GET** /v1/world/conventions/mine | Conventions you host and conventions you are exhibiting at.
[**list_world_conventions**](WorldApi.md#list_world_conventions) | **GET** /v1/world/conventions | Conventions open to sellers now or soon: title, room, dates, and how to get a table (invite, free, fee or revenue share).
[**list_world_cosm_cosmetics**](WorldApi.md#list_world_cosm_cosmetics) | **GET** /v1/world/cosm/cosmetics | The active cosmetics catalogue, optionally by tier (premium | earned | stake_exclusive) or slot: sku, name, slot, tier, priceCents (premium only), priceBaseUnits (earned only, COSM base units string), supply, fusable, preview and what it unlocks. Read-only: cosmetics are bought on crossly.net. meta.available / meta.decimals.
[**list_world_cosm_designs**](WorldApi.md#list_world_cosm_designs) | **GET** /v1/world/cosm/designs | Your COSM designs in the world: popularity toward the free chain unlock a popular limited design earns, whether it is on chain, and your opt-out. Declare what a design does with chain cosm and the design id as the contract.
[**list_world_cosm_inventory**](WorldApi.md#list_world_cosm_inventory) | **GET** /v1/world/cosm/inventory | Cosmetics you hold, newest first: ownershipId, cosmeticId, sku, name, slot, tier, how it was acquired, bound, edition number, acquiredAt and what it unlocks. Pass a cosmeticId to POST /v1/world/avatar/equip to wear one. meta.available.
[**list_world_decor_catalogue**](WorldApi.md#list_world_decor_catalogue) | **GET** /v1/world/decor/catalogue | The Decor Shop: every active cosmetic grouped by surface (avatar | home | shop | stall | street) and slot -- name, tier, priceCents (premium: the dollar price, paid in CBX at spot) or priceBaseUnits (earned: COSM, base-unit string), edition supply and remaining, tint swatch, preview, what it unlocks, owned + your edition number, and buyWith. Read-only: buying and minting happen on crossly.net/shop/decor. meta.available, meta.mint {open, reason}, meta.cbx {open, reason} (CBX opens at mint), meta.balances (your COSM and CBX, base-unit strings), meta.surfaces. Never sold by card.
[**list_world_decor_designs**](WorldApi.md#list_world_decor_designs) | **GET** /v1/world/decor/designs | Your decor designs and their review state. available is false until COSM is provisioned.
[**list_world_decor_inventory**](WorldApi.md#list_world_decor_inventory) | **GET** /v1/world/decor/inventory | Your decor as an INVENTORY. Decor is a quantity: every copy is one unit -- a placed prop (home_prop, prop) uses one per placement, a finish (wallpaper, floor, theme, shopfront, facade, sign style, street lamp/banner/paving/arch, booth canopy/pattern/lights/counter) one per space it dresses; free values are unlimited. Per item: key, value, pair (key:value), name, sku, surface, placeable, tint, owned, placed, available, placements [{ space { kind: home | shop | stall | street, id, name }, count }], copies [{ ownershipId, editionNumber }]. meta.available, meta.spaces (every space you dress). Read-only: placing and retracting happen in the app or on crossly.net.
[**list_world_decor_market**](WorldApi.md#list_world_decor_market) | **GET** /v1/world/decor/market | Decor copies other people are reselling (asks in COSM); optional pair filter. Read only: listing and buying stay on crossly.net.
[**list_world_decor_mine**](WorldApi.md#list_world_decor_mine) | **GET** /v1/world/decor/mine | Decor you hold AND decor you have minted to your wallet, newest first: ownershipId, cosmeticId, sku, name, surface, slot, tier, acquiredVia, bound, editionNumber of maxSupply, mintState (held | minting | minted), mintAddress, mintable, and what it unlocks. meta.available, meta.mint.
[**list_world_decor_stalls**](WorldApi.md#list_world_decor_stalls) | **GET** /v1/world/decor/stalls | Your stall bookings, to dress their booths.
[**list_world_decor_streets**](WorldApi.md#list_world_decor_streets) | **GET** /v1/world/decor/streets | Streets you may dress.
[**list_world_dms**](WorldApi.md#list_world_dms) | **GET** /v1/world/dms | Your friends (mutual follows) for direct messages: each with userId, name, the last line between you (text, at, mine) or null, and how many of theirs you have not read; most recent first.
[**list_world_following_rooms**](WorldApi.md#list_world_following_rooms) | **GET** /v1/world/following/rooms | The public rooms of people you follow, to walk into.
[**list_world_follows**](WorldApi.md#list_world_follows) | **GET** /v1/world/follows | Everybody you follow in Crossly World, newest first, each with mutual: true when they follow you back (a friend).
[**list_world_friend_here**](WorldApi.md#list_world_friend_here) | **GET** /v1/world/friends/here | Your friends in Crossly World right now, and where (when they share it).
[**list_world_friends**](WorldApi.md#list_world_friends) | **GET** /v1/world/friends | Your friends (mutual follows), each with the PUBLIC room they are in right now when they chose to be seen by name there (else null), and their home address slug when they let friends visit. Invisible and anonymous friends are never located.
[**list_world_game_packages**](WorldApi.md#list_world_game_packages) | **GET** /v1/world/games/packages | Your game packages, each with its latest version and review state.
[**list_world_game_rewards**](WorldApi.md#list_world_game_rewards) | **GET** /v1/world/games/rewards | Rewards you won in games: granted, or recorded as unfunded for COSM and CBX (a game prints no value).
[**list_world_games**](WorldApi.md#list_world_games) | **GET** /v1/world/games | The Game Directory: every live, reviewed game package with its players, places, rewards, contests and the piece people place.
[**list_world_home_building**](WorldApi.md#list_world_home_building) | **GET** /v1/world/home/building | Homes whose owner made you a co-builder.
[**list_world_interactive_catalogue**](WorldApi.md#list_world_interactive_catalogue) | **GET** /v1/world/interactive/catalogue | Interactive objects: every piece that runs a shared script in a room (first-party and approved creator designs). Per item: model (the piece key a home layout or boutique prop stores), decorValue (the home_prop value its SKU claims), source (first_party | creator), name, blurb, size [x, z] metres, fallbackMesh, cosmeticId (the SKU on the Decor Shop, null when not on sale) and version (a creator design: the approved version every copy runs). Read-only: buying is on crossly.net/shop/decor.
[**list_world_interactive_designs**](WorldApi.md#list_world_interactive_designs) | **GET** /v1/world/interactive/designs | Your interactive-object designs (MML documents you wrote), newest first: id, handle, name, footprint (s | m | l | f), tint, priceBaseUnits (COSM, base-unit string), maxSupply, model, decorValue, cosmeticId, liveVersion (the approved version every copy runs; null = none), withdrawnReason, archivedAt (you archived it: off sale, owned copies keep running; null = not archived), latestVersion { version, state: pending | approved | rejected | withdrawn | cancelled, decisionNote }, inReview. `archived`: exclude (default), include or only archived designs. Read-only: writing, previewing, submitting and archiving happen on crossly.net/shop/decor?tab=interactive.
[**list_world_job_posted**](WorldApi.md#list_world_job_posted) | **GET** /v1/world/jobs/posted | Shifts you posted for your rooms, with who claimed each and how it went.
[**list_world_outfits**](WorldApi.md#list_world_outfits) | **GET** /v1/world/outfits | Your saved outfits (up to 6, oldest first): id, name, appearance (skin, hair, hairColour, eyes, build) and items -- each worn thing as the door it goes back on by (slot, door: cosmetic | receipt | stock, id, label). Put one on with POST /v1/world/outfits/:id/apply.
[**list_world_place_bans**](WorldApi.md#list_world_place_bans) | **GET** /v1/world/places/{slug}/bans | Who is banned from a place you own (your boutique or home).
[**list_world_place_bundles**](WorldApi.md#list_world_place_bundles) | **GET** /v1/world/places/{slug}/bundles | The owner's bundle deals mapped onto this room's items (itemIds null = any N from the shop). The discount is decided at checkout.
[**list_world_place_collections**](WorldApi.md#list_world_place_collections) | **GET** /v1/world/places/{slug}/collections | The owner's collection rooms: one per category, brand and model they collect.
[**list_world_place_offers**](WorldApi.md#list_world_place_offers) | **GET** /v1/world/places/{slug}/offers | What is for sale in a public room: price, condition and stock, with market references. 404 when no such room is shared.
[**list_world_place_radar**](WorldApi.md#list_world_place_radar) | **GET** /v1/world/places/{slug}/radar | What you have that someone here wants, and what is here that you want: matched on catalog ids.
[**list_world_place_trade_tables**](WorldApi.md#list_world_place_trade_tables) | **GET** /v1/world/places/{slug}/trade-tables | The open trade tables in a place: who is sitting and what is laid on the mat.
[**list_world_place_trades**](WorldApi.md#list_world_place_trades) | **GET** /v1/world/places/{slug}/trades | Items in this room open to trades, each with its trade post path.
[**list_world_provenance**](WorldApi.md#list_world_provenance) | **GET** /v1/world/provenance | Provenance trails: where in Crossly World your purchases were found, newest first (optionally ?listingIds=a,b -- platform listing ids). data: [{ platformListingId, slug (the listing public slug), kind (room | window | stall | space | drop | auction | worn | agent), place (its name, or null when gone), at, line -- e.g. \"Found at The Sneaker Vault, Oct 2026\" }]. Yours only.
[**list_world_quests**](WorldApi.md#list_world_quests) | **GET** /v1/world/quests | Every world quest with your progress: key, the pin it earns, goal, target, count, done, colour. meta.wearing is the pin on your nameplate (or null); meta.streak your current and best daily visit streak. Progress only comes from what the server saw -- there is no write of it.
[**list_world_space_mine**](WorldApi.md#list_world_space_mine) | **GET** /v1/world/spaces/mine | Spaces you belong to.
[**list_world_space_requests**](WorldApi.md#list_world_space_requests) | **GET** /v1/world/spaces/{slug}/requests | As host or co-host, who asked to join.
[**list_world_space_whats_on**](WorldApi.md#list_world_space_whats_on) | **GET** /v1/world/spaces/whats-on | Live and upcoming events across Spaces (streets=1 and stage=1 add street and Main Stage events).
[**list_world_spaces**](WorldApi.md#list_world_spaces) | **GET** /v1/world/spaces | The Spaces directory: seller-hosted clubs with a venue, members and events (optional venue filter).
[**list_world_streaks**](WorldApi.md#list_world_streaks) | **GET** /v1/world/streaks | The longest current daily visit streaks (2+ days), top ten: name and days. Only people still choosing to be seen by name.
[**list_world_trade_mine**](WorldApi.md#list_world_trade_mine) | **GET** /v1/world/trades/mine | Your trades and each item progress through the authentication gate (verified or failed).
[**list_world_trade_table_binder**](WorldApi.md#list_world_trade_table_binder) | **GET** /v1/world/trade-tables/binder | Your binder: unsold collection rows that are catalog items and may be laid on a trade mat -- key, variantId, condition, grade, portfolioId, name, image and valueCents (market value, cents).
[**list_world_trade_table_listings**](WorldApi.md#list_world_trade_table_listings) | **GET** /v1/world/trade-tables/listings | Your open Trade Block trade listings a trade table can be opened on in the world, each with its items' names.
[**update_spatial_scene_cosmetic**](WorldApi.md#update_spatial_scene_cosmetic) | **PATCH** /v1/spatial/scenes/{id}/cosmetics | Redecorate one of your rooms (wall, floor, lighting, trim...). Unknown keys are dropped; the response is what survived. `clear` lists keys to reset.
[**update_spatial_scene_node**](WorldApi.md#update_spatial_scene_node) | **PATCH** /v1/spatial/scenes/{id}/nodes/{nodeId} | Move a rack in one of your warehouse rooms (position in metres).
[**update_spatial_scene_setting**](WorldApi.md#update_spatial_scene_setting) | **PATCH** /v1/spatial/scenes/{id}/settings | How one of your rooms is laid out and lit: grouping and sort keys, theme, overlay and the arrival camera. Cosmetics and the shop layout have their own writers.
[**update_spatial_scene_shop_layout**](WorldApi.md#update_spatial_scene_shop_layout) | **PUT** /v1/spatial/scenes/{id}/shop/layout | Save your shop room's sections and displays (validated server-side; the response is the stored layout).
[**update_spatial_scene_window**](WorldApi.md#update_spatial_scene_window) | **PUT** /v1/spatial/scenes/{id}/window | Choose up to three of your live Crossly listings for the street window of your shop, in slot order. Picks that sell drop out and the newest fill in; an empty list goes back to newest first.
[**update_spatial_street_decor**](WorldApi.md#update_spatial_street_decor) | **PUT** /v1/spatial/streets/{slug}/decor | Dress a street you own with decor you hold (validated against your inventory).
[**update_spatial_street_owned**](WorldApi.md#update_spatial_street_owned) | **PATCH** /v1/spatial/streets/owned/{id} | Who may open a shop on a street you own (open, apply or invite).
[**update_world_auto_shop**](WorldApi.md#update_world_auto_shop) | **PATCH** /v1/world/auto-shop | Turn your automatic world shop off ({ enabled: false } -- an auto-created shop is made private, never deleted) or back on. Answers the new status.
[**update_world_avatar_identity**](WorldApi.md#update_world_avatar_identity) | **PUT** /v1/world/avatar/identity | Choose named or anonymous. Rooms you are already in keep the old answer until you rejoin.
[**update_world_avatar_look**](WorldApi.md#update_world_avatar_look) | **PUT** /v1/world/avatar/look | Save your world look to the account, so every device starts from it. look: null stays anonymous.
[**update_world_avatar_nft_badge**](WorldApi.md#update_world_avatar_nft_badge) | **PUT** /v1/world/avatar/nft-badge | Wear an NFT you hold on your nameplate (null to take it off). Checked again each time you join a room.
[**update_world_capability_wear**](WorldApi.md#update_world_capability_wear) | **PUT** /v1/world/capabilities/wear | Wear a held token on an avatar slot (refId from GET /v1/world/capabilities), or pass refId null to take it off. Its collection must declare a wear capability for that slot.
[**update_world_collection_capability**](WorldApi.md#update_world_collection_capability) | **PUT** /v1/world/collections/{chain}/{contract}/capabilities | Declare what the tokens of a collection can do (the whole declaration, spec crossly.nft-capabilities/1). As its creator (launched on Crossly or claimed) any kind; as a holder, cosmetic kinds only. Replaces your previous declaration; pending if it names an uploaded asset or an always-reviewed kind.
[**update_world_convention_status**](WorldApi.md#update_world_convention_status) | **PUT** /v1/world/conventions/{id}/status | Open, start, end or cancel a convention you host.
[**update_world_cosm_design_chain_auto_unlock**](WorldApi.md#update_world_cosm_design_chain_auto_unlock) | **PUT** /v1/world/cosm/designs/{id}/chain-auto-unlock | Opt one of your COSM designs in or out of the free chain unlock it earns when popular (on by default). Answers its popularity.
[**update_world_emote_wheel**](WorldApi.md#update_world_emote_wheel) | **PUT** /v1/world/emotes/wheel | Arrange your emote wheel: up to 8 slots, each an emote id, a moment slot or null. Returns the saved wheel (locked or unknown ids are dropped).
[**update_world_fit_value_share**](WorldApi.md#update_world_fit_value_share) | **PUT** /v1/world/fit-value/share | Whether people in a room can see what your outfit is worth.
[**update_world_home**](WorldApi.md#update_world_home) | **PATCH** /v1/world/home | Your home: privacy (open, friends, private), theme, and wall, floor and mood decor you hold.
[**update_world_home_builder**](WorldApi.md#update_world_home_builder) | **PUT** /v1/world/home/builders | Set the co-builders of your home: { userIds } -- friends (mutual follows) only, at most 4, replacing the list ([] clears it). They may rearrange your home in the world with your things and add free furniture, never your bought pieces. New co-builders get a world notice.
[**update_world_home_layout**](WorldApi.md#update_world_home_layout) | **PUT** /v1/world/home/layout | Arrange your home: the full layout (up to 200 pieces you own), validated server-side.
[**update_world_home_layout_by_slug**](WorldApi.md#update_world_home_layout_by_slug) | **PUT** /v1/world/homes/{slug}/layout | Rearrange a home you co-build (never adding the owner bought pieces).
[**update_world_interactive_design_draft**](WorldApi.md#update_world_interactive_design_draft) | **PUT** /v1/world/interactive/designs/{id}/draft | Save the draft MML source (up to 64 KB); the response carries its static check.
[**update_world_listing_holders_only**](WorldApi.md#update_world_listing_holders_only) | **PUT** /v1/world/listings/{id}/holders-only | Reserve one of your listings for holders of a collection you declared a holders_only perk for (or release it). Checkout refuses anybody else.
[**update_world_outfit**](WorldApi.md#update_world_outfit) | **PATCH** /v1/world/outfits/{id} | Rename a saved look: { name (1-32 chars) }. 404 when it is not yours.
[**update_world_pet_active**](WorldApi.md#update_world_pet_active) | **PUT** /v1/world/pets/active | Walk one of your pets with you in the world, or none (kind: null).
[**update_world_quest_pin**](WorldApi.md#update_world_quest_pin) | **PUT** /v1/world/quests/pin | Wear a pin you EARNED from a quest on your nameplate, or { pin: null } to wear none. 403 for a pin you have not earned. Shows from the next room you walk into.
[**update_world_room_holder_gate**](WorldApi.md#update_world_room_holder_gate) | **PUT** /v1/world/rooms/{id}/holder-gate | Make your room holders-only (its collection rooms follow). Send null to open it again. Returns `{ gate }`.
[**update_world_space**](WorldApi.md#update_world_space) | **PATCH** /v1/world/spaces/{slug} | As host, change a Space name, about or membership.
[**update_world_space_holder_gate**](WorldApi.md#update_world_space_holder_gate) | **PUT** /v1/world/spaces/{slug}/holder-gate | Make your Space holders-only: only holders of an NFT collection join it (others can look in). Send null to open it again. Returns `{ gate }`.
[**update_world_stall_booking_booth**](WorldApi.md#update_world_stall_booking_booth) | **PUT** /v1/world/stalls/bookings/{id}/booth | Dress the booth of a stall you booked (decor you hold).



## create_spatial_invite_accept

> crate::models::CreateSpatialInviteAcceptResponse create_spatial_invite_accept()
Redeem an invite code (dashes optional); returns the room it opens.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSpatialInviteAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_lease_withdraw

> crate::models::DeleteDaoProposalResponse create_spatial_property_lease_withdraw(id)
Withdraw a lease offer nobody has accepted yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteDaoProposalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene

> crate::models::SpatialSceneResponse2 create_spatial_scene(create_spatial_scene_request)
Start a 3D room: a collection (your items on shelves) or a warehouse (racks you arrange). Private until you make it public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_spatial_scene_request** | **CreateSpatialSceneRequest** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_grant

> crate::models::CreateSpatialSceneGrantResponse create_spatial_scene_grant(id)
Let a specific user into one of your rooms (visitor or staff), no code needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_invite

> crate::models::CreateSpatialSceneInviteResponse create_spatial_scene_invite(id)
Mint an invite code to one of your rooms (visitor or staff, optional expiry in hours). The code is in this response only: just its hash is stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialSceneInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_node

> crate::models::CreateSpatialSceneNodeResponse create_spatial_scene_node(id)
Add a unit (rack, shelf, bin) to one of your warehouse rooms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialSceneNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_place

> crate::models::SpatialSceneResponse create_spatial_scene_place(id)
Pin an inventory item to a slot in one of your rooms: by warehouse nodeId, or by a collection container key and index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_rename

> crate::models::SpatialSceneResponse2 create_spatial_scene_rename(id, create_spatial_scene_rename_request)
Rename one of your rooms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_spatial_scene_rename_request** | **CreateSpatialSceneRenameRequest** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_shop_place

> crate::models::SpatialSceneResponse create_spatial_scene_shop_place(id)
Place an inventory item in a section and display of your shop room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_unpin

> crate::models::DeleteSpatialSceneNodeResponse create_spatial_scene_unpin(id)
Clear every pin in one of your rooms, so it arranges itself again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_visibility

> crate::models::SpatialSceneResponse2 create_spatial_scene_visibility(id, create_spatial_scene_visibility_request)
Make one of your rooms private, unlisted (anyone with the link) or public (in the lobby and searchable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_spatial_scene_visibility_request** | **CreateSpatialSceneVisibilityRequest** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_scene_warehouse_stock

> crate::models::SpatialSceneResponse create_spatial_scene_warehouse_stock(id)
Put an inventory item (or unit) on a rack in one of your warehouse rooms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_shop_neighbourhood

> crate::models::CreateSpatialShopNeighbourhoodResponse create_spatial_shop_neighbourhood()
Move your shop to another neighbourhood (by slug).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSpatialShopNeighbourhoodResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_apply

> crate::models::CreateSpatialStreetApplyResponse create_spatial_street_apply(slug)
Ask to open your shop on a street whose owner reviews applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialStreetApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_invite

> crate::models::CreateSpatialStreetApplyResponse create_spatial_street_invite(slug)
Invite a shop (by its slug) to open on a street you own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialStreetApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_avatar_equip

> crate::models::CreateWorldAvatarEquipResponse create_world_avatar_equip(create_world_avatar_equip_request)
Wear something you already own -- exactly ONE of { cosmeticId } (a cosmetic you hold), { receiptId } (a Crossly purchase from your closet) or { inventoryItemId } (your own stock). The slot comes from the thing itself; slot is only a fallback for an item with no catalog shape. Replaces whatever was in that slot. Moves no money. Answers the slot and your loadout after the change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_avatar_equip_request** | **CreateWorldAvatarEquipRequest** |  | [required] |

### Return type

**crate::models::CreateWorldAvatarEquipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_avatar_unequip

> crate::models::CreateWorldAvatarUnequipResponse create_world_avatar_unequip(create_world_avatar_unequip_request)
Take off whatever is in one slot. Idempotent: an empty slot answers removed:false. Answers your loadout after the change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_avatar_unequip_request** | **CreateWorldAvatarUnequipRequest** |  | [required] |

### Return type

**crate::models::CreateWorldAvatarUnequipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_avatar_wearable

> crate::models::CreateWorldAvatarWearableResponse create_world_avatar_wearable()
Author a wearable for a body slot (a mesh profile and params, an asking price in COSM base units as a string). A submission: inactive until an operator approves it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldAvatarWearableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_contest

> crate::models::CreateWorldContestResponse create_world_contest()
Set up a skill contest on a game placed in your place: title, game (its instance id), entry { mode, feeCents? }, prizes by place, funder, window, attempts. Paid entry needs the operator to allow it where it runs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldContestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_contest_entry_settle_payment

> crate::models::CreateWorldContestEntrySettlePaymentResponse create_world_contest_entry_settle_payment(id)
Read the fee of a paid entry back from Stripe and settle it (safe to repeat).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldContestEntrySettlePaymentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_contest_join

> crate::models::CreateWorldContestJoinResponse create_world_contest_join(id)
Enter a contest. Free and sponsored contests take you in at once; a paid one answers a client secret to pay the entry fee.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldContestJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention

> crate::models::CreateWorldConventionResponse create_world_convention()
Host a convention in one of your rooms: title, dates, tables and how sellers get one (invite, free, a fee in cents, or a revenue share in basis points).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldConventionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention_accept

> crate::models::CreateWorldConventionAcceptResponse create_world_convention_accept(id)
Accept: your own invitation (no body), or, as host, an application (username).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldConventionAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention_apply

> crate::models::CreateWorldConventionApplyResponse create_world_convention_apply(id)
Ask for a table at a convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldConventionApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention_invite

> crate::models::CreateWorldSpaceInviteResponse create_world_convention_invite(id)
Invite a seller (by shop handle) to a convention you host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention_paid

> crate::models::CreateBreakCancelResponse create_world_convention_paid(id)
As host, record that a seller (username) paid their table fee off-platform. Moves no money.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_convention_withdraw

> crate::models::CreateBreakCancelResponse create_world_convention_withdraw(id)
Withdraw your table or decline your invitation; as host, decline someone (username).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_crew

> crate::models::CreateWorldCrewResponse create_world_crew(create_world_crew_request)
Start a crew you own: { name (2-32 chars), tag (2-5 letters or digits, shown on members' nameplates) }. Answers its id and join code. 409 when you are already in a crew.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_crew_request** | **CreateWorldCrewRequest** |  | [required] |

### Return type

**crate::models::CreateWorldCrewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_crew_chat

> crate::models::WorldResponse create_world_crew_chat(create_world_crew_chat_request)
Say a line to your crew (body: text, up to 400 characters). Returns data.messages, the crew chat ring (the last 50 lines, kept in memory only), and data.said when your line went out; a blocked or too-quick line is dropped silently. 404 when you are in no crew.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_crew_chat_request** | **CreateWorldCrewChatRequest** |  | [required] |

### Return type

**crate::models::WorldResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_crew_join

> crate::models::CreateWorldCrewJoinResponse create_world_crew_join(create_world_crew_join_request)
Join a crew by the code a member gave you. 404 for an unknown code, 409 when you are already in a crew or it is full (30).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_crew_join_request** | **CreateWorldCrewJoinRequest** |  | [required] |

### Return type

**crate::models::CreateWorldCrewJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_crew_leave

> crate::models::CreateWorldCrewLeaveResponse create_world_crew_leave()
Leave your crew. If you own it, the crew is disbanded. Idempotent: leaving when in none succeeds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldCrewLeaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_decor_design

> crate::models::CreateWorldDecorDesignResponse create_world_decor_design()
Author a decor design (what it unlocks, a mesh profile and params, an asking price in COSM base units as a string). Inactive until an operator approves it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldDecorDesignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_decor_design_submit

> crate::models::CreateWorldDecorDesignSubmitResponse create_world_decor_design_submit(id)
Send one of your decor designs to the review queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldDecorDesignSubmitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_decor_retract

> crate::models::CreateWorldDecorRetractResponse create_world_decor_retract()
Take a placed decor piece back into your inventory; answers the inventory after.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldDecorRetractResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_dm

> crate::models::CreateWorldDmResponse create_world_dm(user_id, create_world_crew_chat_request)
Send a direct message to a friend (a mutual follow; never across a block), up to 400 characters, through the world chat guard. 403 when they are not a friend, 429 when sent too quickly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**create_world_crew_chat_request** | **CreateWorldCrewChatRequest** |  | [required] |

### Return type

**crate::models::CreateWorldDmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_follow

> crate::models::CreateWorldFollowResponse create_world_follow(user_id)
Follow somebody. When they follow you back you are friends (friends-here, invites, messages). Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldFollowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_friend_invite

> crate::models::CreateWorldFriendInviteResponse create_world_friend_invite(user_id, create_world_friend_invite_request)
\"Join me\": send one friend (a mutual follow) a notification with a link to the PUBLIC room you name by slug. 404 when they are not a friend or the room is not public; 429 when you invited them in the last minute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**create_world_friend_invite_request** | **CreateWorldFriendInviteRequest** |  | [required] |

### Return type

**crate::models::CreateWorldFriendInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_game_package

> crate::models::CreateWorldGamePackageResponse create_world_game_package()
Submit a game package version for review: files as { path, base64 } (game.json, thumb.png, logic/, scene/, audio/, images/). Checked like the CLI, assets quarantined, the entry document dry-run, then reviewed by a person.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldGamePackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_game_package_check

> crate::models::CreateWorldGamePackageCheckResponse create_world_game_package_check()
Validate a game package exactly as a submission is checked (the shared intake report, your version still free) without storing, uploading or queuing anything: world-kit validate for an agent. Same files body as submit.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldGamePackageCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_game_rating

> crate::models::CreateWorldGameRatingResponse create_world_game_rating(id)
Rate a game you played, 1 to 5 stars (rating again replaces it). Only players the runner recorded in that game can rate it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldGameRatingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_game_report

> crate::models::CreateWorldGameReportResponse create_world_game_report(id)
Report a game: reason broken | unfair | offensive | scam | other, optional detail. Staff review it; the creator is told the reason, never who.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldGameReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_game_store_checkout

> crate::models::CreateWorldGameStoreCheckoutResponse create_world_game_store_checkout(id, item)
Buy an item or pass from a game store: answers a client secret for the payment; the item is yours once it succeeds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**item** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldGameStoreCheckoutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_grail_match

> crate::models::CreateWorldGrailMatchResponse create_world_grail_match()
Grail glints: send the ids on show somewhere (listingIds -- listings.id, inventoryIds, skuIds -- catalogue market SKUs; up to 400 each) and get back data.listingIds / data.inventoryIds / data.skuIds: the ones that are things you already want -- an open bid, a watched or wishlisted listing (or one selling the same SKU), a market watch, or a card your deck is missing. Read-only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldGrailMatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_hall_loan

> crate::models::CreateWorldHallLoanResponse create_world_hall_loan()
Lend a piece from your vault to this week's Hall of Grails exhibition: { vaultItemId, credit: 'name' | 'anonymous' }. It shows under your plaque until the exhibition ends; it stays in your vault (still listable and withdrawable), and stops showing if it leaves.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldHallLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_home_ask

> crate::models::CreateWorldHomeAskResponse create_world_home_ask(slug)
Ask a home owner whether they would sell a piece you saw there (an item or a decor piece).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldHomeAskResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_home_guestbook

> crate::models::CreateWorldHomeGuestbookResponse create_world_home_guestbook(slug)
Sign a home guestbook (one short line).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldHomeGuestbookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_home_guestbook_report

> crate::models::CreateWorldGameReportResponse create_world_home_guestbook_report(slug, id)
Report a guestbook entry to moderation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldGameReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_hunt_start

> crate::models::GetWorldHuntResponse create_world_hunt_start()
Start the Great Hunt this season by reading its first riddle (the Copper key's). Idempotent: starting again answers your progress unchanged. Answers the same shape as GET /v1/world/hunt. 409 when no season is running.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHuntResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_asset

> crate::models::CreateWorldInteractiveAssetResponse create_world_interactive_asset()
Upload one asset for your designs: kind glb (4 MB), wav (4 MB) or png (1 MB), the file as base64. Answers its sha256 and the path to reference from MML.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldInteractiveAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_design

> crate::models::GetWorldInteractiveDesignResponse create_world_interactive_design()
Create an interactive design: name, footprint (s, m, l or f), optional tint and description, an asking price in COSM base units (a string, six decimals) and optional max supply. Inactive until ops approve a submitted version.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldInteractiveDesignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_design_check

> crate::models::CreateWorldInteractiveDesignCheckResponse create_world_interactive_design_check(id)
Check the draft: static rules, that every referenced upload exists, and a sandboxed dry run. Budgeted per person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldInteractiveDesignCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_design_preview

> crate::models::CreateWorldInteractiveDesignPreviewResponse create_world_interactive_design_preview(id)
A private preview token for the draft (author only) and the socket path that runs it. Budgeted per person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldInteractiveDesignPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_design_submit

> crate::models::CreateWorldInteractiveDesignSubmitResponse create_world_interactive_design_submit(id)
Freeze the draft as a new version and send it to ops review. Up to 10 a day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldInteractiveDesignSubmitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_interactive_reset

> crate::models::CreateWorldInteractiveResetResponse create_world_interactive_reset()
Wipe the stored state of an interactive piece standing in a place you own (a home or your boutique).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldInteractiveResetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_job_appraisal_answer

> crate::models::CreateWorldJobAppraisalAnswerResponse create_world_job_appraisal_answer(id)
Answer a price check: your estimate of the item value in CENTS and an optional note. Moves no money; the asker may accept it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldJobAppraisalAnswerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_job_shift_claim

> crate::models::CreateWorldJobShiftClaimResponse create_world_job_shift_claim(id)
Claim an open shift. You work it by being in the place while it runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldJobShiftClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_job_shift_release

> crate::models::CreateWorldJobShiftReleaseResponse create_world_job_shift_release(id)
Give a shift you claimed back to the board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldJobShiftReleaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_outfit

> crate::models::CreateWorldOutfitResponse create_world_outfit(create_world_outfit_request)
Save what you are wearing now as a named look: { name (1-32 chars), appearance? (skin, hair, hairColour, eyes, build -- unknown keys fall back to the default) }. Each worn thing is recorded as the door it goes back on by; one that no longer traces to something you own is listed in skipped and not saved. 409 when you already keep 6.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_world_outfit_request** | **CreateWorldOutfitRequest** |  | [required] |

### Return type

**crate::models::CreateWorldOutfitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_outfit_apply

> crate::models::CreateWorldOutfitApplyResponse create_world_outfit_apply(id)
Put a saved look on. Every item goes through the same equip as POST /v1/world/avatar/equip, so ownership still holds: something you no longer own is listed in skipped with the reason and not worn. Slots the look left empty are emptied (cleared). Answers the appearance of the look (for your character -- appearance lives on your device), applied, skipped and cleared. Moves no money.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldOutfitApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party

> crate::models::CreateBreakResponse create_world_party()
Start a party with you as leader (up to 8 members).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBreakResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_chat

> crate::models::WorldResponse create_world_party_chat(id)
Say something in your party chat (up to 2000 characters).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_invite

> crate::models::CreateWorldPartyInviteResponse create_world_party_invite(id)
Invite a friend (userId) to your party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_invite_accept

> crate::models::CreateWorldPartyInviteAcceptResponse create_world_party_invite_accept(invite_id)
Accept a party invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyInviteAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_invite_decline

> crate::models::CreateWorldPartyInviteAcceptResponse create_world_party_invite_decline(invite_id)
Decline a party invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyInviteAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_kick

> crate::models::CreateWorldPartyKickResponse create_world_party_kick(id)
As leader, remove a member (userId).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyKickResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_leader

> crate::models::CreateWorldPartyLeaderResponse create_world_party_leader(id)
As leader, pass the lead to a member (userId).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyLeaderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_leave

> crate::models::CreateWorldPartyLeaveResponse create_world_party_leave(id)
Leave your party (the lead passes on if you led it).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyLeaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_wishlist

> crate::models::CreateWorldPartyWishlistResponse create_world_party_wishlist(id)
Add a listing (listingId, or its shop slug) to the party wishlist; answers the wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyWishlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_party_wishlist_vote

> crate::models::CreateWorldPartyWishlistResponse create_world_party_wishlist_vote(id, listing_id)
Thumb a wishlist listing up or down (null clears your vote); answers the wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**listing_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyWishlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_place_item_want

> crate::models::CreateWorldPlaceItemWantResponse create_world_place_item_want(slug, item_id)
Tell a room owner you would buy one of their items (once per person per item; they see the count). Moves no money.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**item_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPlaceItemWantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space

> crate::models::CreateWorldSpaceResponse create_world_space()
Open a Space (sellers only, at most two): slug, name, venue, optional membership and about.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldSpaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_event

> crate::models::CreateWorldSpaceEventResponse create_world_space_event(slug)
As host or co-host, put an event on: title, kind, optional about, ISO start and end with offset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_event_rsvp

> crate::models::CreateWorldSpaceEventRsvpResponse create_world_space_event_rsvp(slug, id)
Say whether you are going to an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceEventRsvpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_invite

> crate::models::CreateWorldSpaceInviteResponse create_world_space_invite(slug)
Invite somebody (userId) to a Space.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_join

> crate::models::CreateWorldSpaceJoinResponse create_world_space_join(slug)
Join a Space, or ask to when it is request-only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_leave

> crate::models::CreateWorldCrewLeaveResponse create_world_space_leave(slug)
Leave a Space.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldCrewLeaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_request_by_user_id

> crate::models::CreateWorldSpaceRequestByUserIdResponse create_world_space_request_by_user_id(slug, user_id)
As host or co-host, accept or turn down a request to join.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceRequestByUserIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_space_role

> crate::models::CreateWorldSpaceRoleResponse create_world_space_role(slug)
As host, make a member a co-host or back to a member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceRoleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_spatial_scene_grant

> crate::models::DeleteSpatialSceneGrantResponse delete_spatial_scene_grant(id, grant_id)
Take somebody's access to your room away, or kill an invite nobody has used yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**grant_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_spatial_scene_node

> crate::models::DeleteSpatialSceneNodeResponse delete_spatial_scene_node(id)
Remove units (by id, up to 500) from one of your warehouse rooms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_spatial_scene_warehouse_stock

> crate::models::DeleteSpatialSceneGrantResponse delete_spatial_scene_warehouse_stock(id)
Take an inventory item (or unit) off its rack.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_spatial_street_request

> crate::models::DeleteDaoProposalResponse delete_spatial_street_request(id)
Withdraw a street request you made that has not been founded yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteDaoProposalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_block

> crate::models::DeleteWorldBlockResponse delete_world_block(id)
Unblock somebody (by the block id from GET /v1/world/blocks).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldBlockResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_collection_capability

> delete_world_collection_capability(chain, contract)
Withdraw your declaration for a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_follow

> crate::models::CreateWorldFollowResponse delete_world_follow(user_id)
Unfollow somebody. Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldFollowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_hall_loan

> crate::models::CreateBreakResponse delete_world_hall_loan(id)
Take one of your loans off show in the Hall of Grails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBreakResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_home_guestbook

> crate::models::DeleteWorldHomeGuestbookResponse delete_world_home_guestbook(slug, id)
As the home owner, hide a guestbook entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldHomeGuestbookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_interactive_design

> crate::models::DeleteWorldInteractiveDesignResponse delete_world_interactive_design(id)
Delete a draft design, or archive a submitted one (off sale; owned copies keep working).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldInteractiveDesignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_job_shift

> crate::models::DeleteWorldJobShiftResponse delete_world_job_shift(id)
Cancel a shift you posted that nobody has worked yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldJobShiftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_outfit

> crate::models::DeleteWorldOutfitResponse delete_world_outfit(id)
Delete a saved look. 404 when it is not yours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldOutfitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_party_wishlist

> crate::models::CreateWorldPartyWishlistResponse delete_world_party_wishlist(id, listing_id)
Take a listing you added off the party wishlist; answers the wishlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**listing_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyWishlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_place_ban

> crate::models::DeleteWorldPlaceBanResponse delete_world_place_ban(slug, user_id)
Lift a ban from a place you own; they can walk in again right away.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldPlaceBanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_space

> crate::models::DeleteWorldSpaceResponse delete_world_space(slug)
As host, close a Space.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldSpaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_space_event

> crate::models::DeleteWorldJobShiftResponse delete_world_space_event(slug, id)
As host or co-host, cancel an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldJobShiftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_scene_attention

> crate::models::GetSpatialSceneAttentionResponse get_spatial_scene_attention(scene_id)
What visitors looked at in one of your rooms: viewers, looks, inspects and dwell per item over the last days (1-35, default 30).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialSceneAttentionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_scene_window

> crate::models::GetSpatialSceneWindowResponse get_spatial_scene_window(id)
The listings you chose for the street window of your shop, in slot order (left, middle, right; empty = newest first), and `choices`: your listings that can go in it, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialSceneWindowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_shop_neighbourhood

> crate::models::GetSpatialShopNeighbourhoodResponse get_spatial_shop_neighbourhood()
Your shop's neighbourhood, and whether and where it may move.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialShopNeighbourhoodResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_space

> crate::models::GetSpatialSpaceResponse get_spatial_space()
The room catalogue: every space profile (what it holds, how it is sorted, its containers) and the warehouse units.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialSpaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_street_admission

> crate::models::GetSpatialStreetAdmissionResponse get_spatial_street_admission()
Street applications and invitations: yours, and those waiting on streets you own.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialStreetAdmissionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_street_decor

> crate::models::GetSpatialStreetDecorResponse get_spatial_street_decor(slug)
The decor on a street you may dress, and what you can place.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialStreetDecorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_arcade

> crate::models::GetWorldArcadeResponse get_world_arcade()
The Arcade in Crossly World: data.cabinets (slot, where it stands, host -- piece, price_guess or empty --, game, title, blurb, the piece it hosts and where, record, packageId and thumbnail for a creator game from the Game Directory, players) and data.scores (the High Score wall: per game a title, kind -- leaderboard, record or top -- and up to five ASCII lines; a name shows only for somebody who chose to be seen by name, everyone else is \"A player\"), plus creatorGames (creator games with a cabinet now). Games are played in the world.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldArcadeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_aurora

> crate::models::GetWorldAuroraResponse get_world_aurora()
The Sales Aurora over the Commons in Crossly World: every real sale on Crossly in the last data.windowDays (30) days. data.total (sales in the window), data.families (key, label, colour, count per colour family: cards, coins, jewellery, electronics, media, fashion, toys, other) and data.sales (newest first, at most 48: family, ageHours, fresh -- sold in the last ten minutes, which shimmers). Aggregate only: no item, price, seller or platform. data.updatedAt is when it was read.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAuroraResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_auto_shop

> crate::models::GetWorldAutoShopResponse get_world_auto_shop()
Whether Crossly keeps a world shop for you automatically: enabled, and the shop (id, name, visibility, autoCreated, address, label) or null. Change it with PATCH /v1/world/auto-shop.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAutoShopResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_avatar_identity

> crate::models::GetWorldAvatarIdentityResponse get_world_avatar_identity()
Whether others see your name in the world (named), or not (anonymous); null if you have never chosen.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAvatarIdentityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_avatar_loadout

> crate::models::GetWorldAvatarLoadoutResponse get_world_avatar_loadout()
What you are wearing: the rig, its slots (avatar_head, avatar_body, avatar_outer -- a jacket over the top, avatar_legs, avatar_feet, avatar_back) and each worn entry -- a cosmetic or a real item -- with its verified-purchase mark. Carries no price, receipt id, seller or order.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAvatarLoadoutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_avatar_loadout_by_user_id

> crate::models::GetWorldAvatarLoadoutResponse get_world_avatar_loadout_by_user_id(user_id)
What somebody else is wearing, with the verified-purchase mark on each real item -- public on purpose, since a check only its owner can see is not a check. No price, receipt id, seller or order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldAvatarLoadoutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_avatar_look

> crate::models::GetWorldAvatarLookResponse get_world_avatar_look()
Your world look on the account: the handle you chose (null if anonymous) and your appearance; null if you never made one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAvatarLookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_avatar_nft_badge

> crate::models::GetWorldAvatarNftBadgeResponse get_world_avatar_nft_badge()
The NFT you wear on your nameplate in the world, or null.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAvatarNftBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_capability

> crate::models::GetWorldCapabilityResponse get_world_capability()
What your held NFTs unlock in Crossly World: each token with the capabilities its collection declared (wear, display, emote, gate, ...). Holding is your verified wallets (their synced tokens); the capabilities of a transferred token go within five minutes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCapabilityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_capability_catalogue

> crate::models::GetWorldCapabilityCatalogueResponse get_world_capability_catalogue()
The capability kinds of spec crossly.nft-capabilities/1: what each does, whether only the creator of the collection may declare it, whether it needs review, and which clients honour it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCapabilityCatalogueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_capability_wear

> crate::models::GetWorldCapabilityWearResponse get_world_capability_wear()
What you wear from your NFTs and COSM copies: your pick per avatar slot (head, body, legs, feet, back) and what the room sees.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCapabilityWearResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_chest

> crate::models::GetWorldChestResponse get_world_chest()
The Commons chest: data.window (this 20-minute window, UTC), spot ([x, z] on the plaza), endsAt (when the next chest comes) and mine (openedThis, and opened -- how many windows you have opened one in; the Chest opener pin comes at 10). A chest is opened by walking up to it in Crossly World, never over the API.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldChestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_collection_capability

> crate::models::GetWorldCollectionCapabilityResponse get_world_collection_capability(chain, contract)
The live capability declaration of a collection (404 when it has none).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWorldCollectionCapabilityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_contest_mine

> crate::models::GetWorldContestMineResponse get_world_contest_mine()
Your contest entries (best score, attempts used, payment state) and prizes (granted, or owed until their gate opens).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldContestMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_convention

> crate::models::GetWorldConventionResponse get_world_convention(id)
One convention. 404 for an unknown or malformed id alike.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldConventionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_cosm_balance

> crate::models::GetWorldCosmBalanceResponse get_world_cosm_balance()
Your COSM balance: baseUnits (a STRING -- COSM has 6 decimals and overflows a JSON number), decimals, available (false = COSM is not provisioned on this deployment, not a zero balance).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCosmBalanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_crew

> crate::models::GetWorldCrewResponse get_world_crew()
Your crew: data.crew is null when you are in none, else its name, tag, join code and whether you own it; data.members lists up to 30 members, each with the public room they are in right now when they chose to be seen by name (else null).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCrewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_crew_chat

> crate::models::WorldResponse get_world_crew_chat()
Your crew chat: data.messages is the last 50 lines (from, name, text, at), kept in memory only, so it starts empty after a restart. 404 when you are in no crew.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_crew_leaderboard

> crate::models::GetGameLeaderboardResponse get_world_crew_leaderboard()
The game leaderboards (Price Guess, hunts, speed trades; this week and all time) ranked among your crew only, each member by the name the crew sees. 404 when you are in no crew.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetGameLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_dm

> crate::models::GetWorldDmResponse get_world_dm(user_id)
Your direct-message thread with one friend, oldest first (the last 100 lines: id, from, text, at, mine). Reading it marks their lines read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldDmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_emote_showable

> crate::models::GetWorldEmoteShowableResponse get_world_emote_showable()
The things you own that a show-off emote can hold up.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldEmoteShowableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_firefly

> crate::models::GetWorldFireflyResponse get_world_firefly()
Fireflies on the Commons lawn: data.day (today, UTC), today (fireflies everyone has caught today) and mine (today -- your catches today, up to perDay -- total, all your catches; the Firefly Catcher pin comes at 30 -- and perDay). Fireflies are caught by walking up to a lit one in Crossly World after dark, never over the API, and nothing is spent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldFireflyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_fit_stat

> crate::models::GetWorldFitStatResponse get_world_fit_stat()
Your walking storefront, last 7 days: distinct people who tapped \"Shop this\" or \"Offer\" on what you wore, and how many of each. Counts only; nobody is identified.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldFitStatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_fit_value

> crate::models::GetWorldFitValueResponse get_world_fit_value()
What your outfit is worth on the market: cents (sum of each catalogued piece's order-book price -- last sale, else lowest ask), pieces worn, how many were priced, and share (whether you show it on your card).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldFitValueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game

> crate::models::GetWorldGameResponse get_world_game(id)
One live game: its manifest, its store items and which you own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_leaderboard

> crate::models::GetWorldGameLeaderboardResponse get_world_game_leaderboard(id)
A live game leaderboard, this week and all time, by its manifest scoring (wins, best points, or best time). Only people who chose to be named show by name. scoring is null for a game that keeps no board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_package

> crate::models::GetWorldGamePackageResponse get_world_game_package(id)
One of your game packages with every version, its review state and decision note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGamePackageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_template

> crate::models::GetWorldGameTemplateResponse get_world_game_template(kind)
A fresh game package from a template (table, arena, object; collection is an NFT capability declaration), as { path, base64 } files: the world-kit init, for an agent with no local kit. ?id=my-game&name=My Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameTemplateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_hall

> crate::models::GetWorldHallResponse get_world_hall()
The Hall of Grails in Crossly World: data.exhibition (key, title, blurb, endsAt -- a new one every Monday UTC -- and next, the coming one's title) and data.exhibits, up to twelve real catalogue items in slot order (skuId, name, brand, imageUrl, thumbUrl, retailCents, lowestAskCents, lastSaleCents: the live order book, null when the market has none). The first stands on the centre plinth. Buy, bid or watch an exhibit through the market endpoints with its skuId.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHallResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_hall_loan

> crate::models::GetWorldHallLoanResponse get_world_hall_loan()
Lend to the Hall: this week's exhibition (key, title, categorySlug, endsAt), your loans on show (id, vaultItemId, skuId, name, credit, since) and the vault pieces you can lend (eligible: vaultItemId, skuId, name, imageUrl, grade) -- held in your Crossly vault, not pledged, in the exhibition's category. perMember is how many you may lend each week.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHallLoanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_home_placeable

> crate::models::GetWorldHomePlaceableResponse get_world_home_placeable()
What you can place in your home: your items and your furniture.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHomePlaceableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_hunt

> crate::models::GetWorldHuntResponse get_world_hunt()
Your Great Hunt: data.season (id, name, startsAt, endsAt, the title and how many earn it, live), started (you read the first clue), held (keys held, 0-3, always in the order copper, jade, crystal), keys (each with its name, colour, its riddle once it is yours to read, and when you picked it up), and cleared (your rank through the Gate this season, when, and whether you earned the title) or null. Keys are picked up by walking to them in Crossly World; nothing here takes one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHuntResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_hunt_scoreboard

> crate::models::GetWorldHuntScoreboardResponse get_world_hunt_scoreboard()
The Great Hunt Scoreboard, as the monolith in the Commons shows it: data.season (the running season, or the last one between seasons), finishers (the first ten through the Gate in order: rank, name, short, named, at, titled -- a name shows only for somebody who chose to be seen by name in the world, everyone else is \"A player\") and total (everybody through the Gate this season).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldHuntScoreboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_interactive_design

> crate::models::GetWorldInteractiveDesignResponse get_world_interactive_design(id)
One of your interactive designs: its draft source, the last static check, and every submitted version with its review state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldInteractiveDesignResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_job

> crate::models::GetWorldJobResponse get_world_job()
The jobs board: open staff shifts you could claim, your claimed shifts and open price checks (appraisals), as the world shows them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldJobResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_kite

> crate::models::GetWorldKiteResponse get_world_kite()
Kites flown on the Commons lawn: data.day (today, UTC), today (kites everyone has flown today) and mine (today -- yours today, up to perDay -- perDay, and days -- the days you have flown one on; the Kite flyer pin comes at 5). Kites are flown by standing out on the lawn in Crossly World by day, never over the API, and nothing is spent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldKiteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_lantern

> crate::models::GetWorldKiteResponse get_world_lantern()
Sky lanterns in the Commons: data.day (today, UTC), today (lanterns everyone has released today) and mine (today -- yours today, up to perDay -- perDay, and days -- the days you have released one on; the Lantern lighter pin comes at 5). Lanterns are released at the lantern stand in Crossly World, never over the API, and nothing is spent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldKiteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_party_chat

> crate::models::WorldResponse get_world_party_chat(id)
Your party chat: the recent lines. 404 when this is not your party.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_board

> crate::models::GetWorldPlaceBoardResponse get_world_place_board(slug)
The Trade Floor's wall of wants, its authentication desk and its Vault Markets ticker (vaultMovers: bookKey, name, label, units, markCents, change24hBps; only the trade-floor place has them).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceBoardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_collection

> crate::models::GetWorldPlaceCollectionResponse get_world_place_collection(slug, category, brand, model)
One collection room: every piece of the set, owned or missing. 404 for an unknown room or set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**category** | **String** |  | [required] |
**brand** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceCollectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_drop

> crate::models::GetWorldPlaceDropResponse get_world_place_drop(slug)
The owner's next drop (name, cover, time, live) and how many people are waiting in the room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_item_interest

> crate::models::GetWorldPlaceItemInterestResponse get_world_place_item_interest(slug, item_id)
How many people looked at one item in this room today.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**item_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceItemInterestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_live

> crate::models::GetWorldPlaceLiveResponse get_world_place_live(slug)
The owner's live sale: their stream and the lot on the block, or null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceLiveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_seller

> crate::models::GetWorldPlaceSellerResponse get_world_place_seller(slug)
The shop whose public room this is (username and name), or null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceSellerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_place_shop_pet

> crate::models::GetWorldPlaceShopPetResponse get_world_place_shop_pet(slug)
The shop's pet kind, or null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlaceShopPetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_space

> crate::models::GetWorldSpaceResponse get_world_space(slug)
One Space: its layout, host, events and your place in it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldSpaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_space_can_host

> crate::models::GetWorldSpaceCanHostResponse get_world_space_can_host()
Whether you may open a Space: sellers only, at most two active.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldSpaceCanHostResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_trial_leaderboard

> crate::models::GetWorldTrialLeaderboardResponse get_world_trial_leaderboard(course)
A Time Trials course's boards, as the board beside it in the Commons shows them: data.course, week, weekly (this week's ten fastest: rank, name, short, named, ms, at) and allTime (the ten fastest ever, each person once). A name shows only for somebody who chose to be seen by name in the world; everyone else is \"A player\". 404 for an unknown course; course 1 is commons-loop.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course** | **String** |  | [required] |

### Return type

**crate::models::GetWorldTrialLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_trial_mine

> crate::models::GetWorldTrialMineResponse get_world_trial_mine()
Your Hoverboard Time Trials bests: data.week (its Monday, YYYY-MM-DD, UTC) and courses, each with course (id, name, parMs, rings, lengthM), week (your best lap this week in ms, your place on the weekly board, laps finished, and the split at each ring) or null, and allTime (your best ever in ms, your place on the all-time board, when) or null. Laps are timed by Crossly from where it saw you ride in the world; nothing here writes one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldTrialMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_wallet

> crate::models::GetWorldWalletResponse get_world_wallet()
Every balance you hold in Crossly World, read-only: bucksCents (Crossly Bucks, cents); cbx (available, decimals, and the three CBX balances kept apart as base-unit STRINGS -- granted (spend here only), earned (withdrawable once matured), connected (a delegation ceiling on your own wallet, not cash on hand) -- plus pendingCents of cashback not yet matured); cosm (available, decimals, baseUnits string). available:false means that currency is not provisioned on this deployment, not that you hold none.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldWalletResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_wish

> crate::models::GetWorldWishResponse get_world_wish()
The Wishing Fountain in the Commons: data.day (today, UTC), today (wishes everyone has made today) and mine (madeToday, days -- how many days you have wished on, the Wishmaker pin comes at 7 -- and nextAt, when tomorrow's wish opens). One wish a day; a wish is made by tossing a coin at the fountain in Crossly World, never over the API, and nothing is spent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldWishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_scene_grants

> crate::models::ListSpatialSceneGrantsResponse list_spatial_scene_grants(id)
Everyone who can currently walk into one of your rooms, and unused invites. Never returns codes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialSceneGrantsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_scene_warehouse_unplaced

> crate::models::ListSpatialSceneWarehouseUnplacedResponse list_spatial_scene_warehouse_unplaced(id)
Stock not yet on a shelf in a warehouse room you can arrange (q filters by title).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialSceneWarehouseUnplacedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_shared_with_me

> crate::models::ListSpatialSharedWithMeResponse list_spatial_shared_with_me()
Rooms other people let you into.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialSharedWithMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_street_leases

> crate::models::ListSpatialStreetLeasesResponse list_spatial_street_leases(slug)
Plots on a street offered for lease: weekly rent in CENTS and terms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialStreetLeasesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_avatar_closet

> crate::models::ListWorldAvatarClosetResponse list_world_avatar_closet(limit, offset)
Everything you can verifiably wear (your Crossly purchase receipts), newest first: receiptId, item title, brand, size, image, catalog sku/variant, quantity, purchasedAt. Pass a receiptId to POST /v1/world/avatar/equip to put one on. limit 1-200, offset for paging.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

**crate::models::ListWorldAvatarClosetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_avatar_stock

> crate::models::ListWorldAvatarStockResponse list_world_avatar_stock()
Items of your OWN inventory you could wear into a room as a walking storefront: inventoryItemId, title, image and the slot it would go on. Pass an inventoryItemId to POST /v1/world/avatar/equip.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldAvatarStockResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_avatar_wearables

> crate::models::ListWorldAvatarWearablesResponse list_world_avatar_wearables()
Wearables you have authored as a designer: id, sku, name, slot, priceBaseUnits (COSM base units, a string), maxSupply, mintedCount, active (false until reviewed). meta.available is false when COSM is not provisioned here.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldAvatarWearablesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_best_dressed

> crate::models::ListWorldBestDressedResponse list_world_best_dressed()
This week's best-dressed board (Monday-based), top ten: name, profile username and loves. Only people still choosing to be seen by name.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldBestDressedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_capability_declarations

> crate::models::ListWorldCapabilityDeclarationsResponse list_world_capability_declarations()
Your capability declarations, in every review state (pending, live, rejected with the note from the reviewer).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldCapabilityDeclarationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_contest_standings

> crate::models::ListWorldContestStandingsResponse list_world_contest_standings(id)
The standings of a contest now: best score first (or lowest, for a time game), ties to whoever got there first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldContestStandingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_contests

> crate::models::ListWorldContestsResponse list_world_contests()
Open skill contests on Crossly World games, soonest ending first: entry mode (free, sponsored, paid) and prizes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldContestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_convention_exhibitors

> crate::models::ListWorldConventionExhibitorsResponse list_world_convention_exhibitors(id)
The host roster: who applied, who was invited, who is coming (by shop handle). Host only; 404 otherwise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldConventionExhibitorsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_convention_mine

> crate::models::ListWorldConventionMineResponse list_world_convention_mine()
Conventions you host and conventions you are exhibiting at.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldConventionMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_conventions

> crate::models::ListWorldConventionsResponse list_world_conventions()
Conventions open to sellers now or soon: title, room, dates, and how to get a table (invite, free, fee or revenue share).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldConventionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_cosm_cosmetics

> crate::models::ListCosmCosmeticsResponse list_world_cosm_cosmetics(tier, slot)
The active cosmetics catalogue, optionally by tier (premium | earned | stake_exclusive) or slot: sku, name, slot, tier, priceCents (premium only), priceBaseUnits (earned only, COSM base units string), supply, fusable, preview and what it unlocks. Read-only: cosmetics are bought on crossly.net. meta.available / meta.decimals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tier** | Option<**String**> |  |  |
**slot** | Option<**String**> |  |  |

### Return type

**crate::models::ListCosmCosmeticsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_cosm_designs

> crate::models::ListWorldCosmDesignsResponse list_world_cosm_designs()
Your COSM designs in the world: popularity toward the free chain unlock a popular limited design earns, whether it is on chain, and your opt-out. Declare what a design does with chain cosm and the design id as the contract.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldCosmDesignsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_cosm_inventory

> crate::models::ListWorldCosmInventoryResponse list_world_cosm_inventory()
Cosmetics you hold, newest first: ownershipId, cosmeticId, sku, name, slot, tier, how it was acquired, bound, edition number, acquiredAt and what it unlocks. Pass a cosmeticId to POST /v1/world/avatar/equip to wear one. meta.available.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldCosmInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_catalogue

> crate::models::ListWorldDecorCatalogueResponse list_world_decor_catalogue(surface)
The Decor Shop: every active cosmetic grouped by surface (avatar | home | shop | stall | street) and slot -- name, tier, priceCents (premium: the dollar price, paid in CBX at spot) or priceBaseUnits (earned: COSM, base-unit string), edition supply and remaining, tint swatch, preview, what it unlocks, owned + your edition number, and buyWith. Read-only: buying and minting happen on crossly.net/shop/decor. meta.available, meta.mint {open, reason}, meta.cbx {open, reason} (CBX opens at mint), meta.balances (your COSM and CBX, base-unit strings), meta.surfaces. Never sold by card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**surface** | Option<**String**> |  |  |

### Return type

**crate::models::ListWorldDecorCatalogueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_designs

> crate::models::ListWorldDecorDesignsResponse list_world_decor_designs()
Your decor designs and their review state. available is false until COSM is provisioned.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDecorDesignsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_inventory

> crate::models::ListWorldDecorInventoryResponse list_world_decor_inventory()
Your decor as an INVENTORY. Decor is a quantity: every copy is one unit -- a placed prop (home_prop, prop) uses one per placement, a finish (wallpaper, floor, theme, shopfront, facade, sign style, street lamp/banner/paving/arch, booth canopy/pattern/lights/counter) one per space it dresses; free values are unlimited. Per item: key, value, pair (key:value), name, sku, surface, placeable, tint, owned, placed, available, placements [{ space { kind: home | shop | stall | street, id, name }, count }], copies [{ ownershipId, editionNumber }]. meta.available, meta.spaces (every space you dress). Read-only: placing and retracting happen in the app or on crossly.net.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDecorInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_market

> crate::models::ListWorldDecorMarketResponse list_world_decor_market()
Decor copies other people are reselling (asks in COSM); optional pair filter. Read only: listing and buying stay on crossly.net.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDecorMarketResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_mine

> crate::models::ListWorldDecorMineResponse list_world_decor_mine()
Decor you hold AND decor you have minted to your wallet, newest first: ownershipId, cosmeticId, sku, name, surface, slot, tier, acquiredVia, bound, editionNumber of maxSupply, mintState (held | minting | minted), mintAddress, mintable, and what it unlocks. meta.available, meta.mint.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDecorMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_stalls

> crate::models::ListStallMineResponse list_world_decor_stalls()
Your stall bookings, to dress their booths.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStallMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_decor_streets

> crate::models::ListWorldDecorStreetsResponse list_world_decor_streets()
Streets you may dress.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDecorStreetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_dms

> crate::models::ListWorldDmsResponse list_world_dms()
Your friends (mutual follows) for direct messages: each with userId, name, the last line between you (text, at, mine) or null, and how many of theirs you have not read; most recent first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldDmsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_following_rooms

> crate::models::ListWorldFollowingRoomsResponse list_world_following_rooms()
The public rooms of people you follow, to walk into.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldFollowingRoomsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_follows

> crate::models::ListWorldFollowsResponse list_world_follows()
Everybody you follow in Crossly World, newest first, each with mutual: true when they follow you back (a friend).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldFollowsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_friend_here

> crate::models::ListWorldFriendsResponse list_world_friend_here()
Your friends in Crossly World right now, and where (when they share it).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldFriendsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_friends

> crate::models::ListWorldFriendsResponse list_world_friends()
Your friends (mutual follows), each with the PUBLIC room they are in right now when they chose to be seen by name there (else null), and their home address slug when they let friends visit. Invisible and anonymous friends are never located.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldFriendsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_game_packages

> crate::models::ListWorldGamePackagesResponse list_world_game_packages()
Your game packages, each with its latest version and review state.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldGamePackagesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_game_rewards

> crate::models::ListWorldGameRewardsResponse list_world_game_rewards()
Rewards you won in games: granted, or recorded as unfunded for COSM and CBX (a game prints no value).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldGameRewardsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_games

> crate::models::ListWorldGamesResponse list_world_games()
The Game Directory: every live, reviewed game package with its players, places, rewards, contests and the piece people place.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldGamesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_home_building

> crate::models::ListWorldHomeBuildingResponse list_world_home_building()
Homes whose owner made you a co-builder.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldHomeBuildingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_interactive_catalogue

> crate::models::ListWorldInteractiveCatalogueResponse list_world_interactive_catalogue()
Interactive objects: every piece that runs a shared script in a room (first-party and approved creator designs). Per item: model (the piece key a home layout or boutique prop stores), decorValue (the home_prop value its SKU claims), source (first_party | creator), name, blurb, size [x, z] metres, fallbackMesh, cosmeticId (the SKU on the Decor Shop, null when not on sale) and version (a creator design: the approved version every copy runs). Read-only: buying is on crossly.net/shop/decor.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldInteractiveCatalogueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_interactive_designs

> crate::models::ListWorldInteractiveDesignsResponse list_world_interactive_designs(archived)
Your interactive-object designs (MML documents you wrote), newest first: id, handle, name, footprint (s | m | l | f), tint, priceBaseUnits (COSM, base-unit string), maxSupply, model, decorValue, cosmeticId, liveVersion (the approved version every copy runs; null = none), withdrawnReason, archivedAt (you archived it: off sale, owned copies keep running; null = not archived), latestVersion { version, state: pending | approved | rejected | withdrawn | cancelled, decisionNote }, inReview. `archived`: exclude (default), include or only archived designs. Read-only: writing, previewing, submitting and archiving happen on crossly.net/shop/decor?tab=interactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archived** | Option<**String**> |  |  |

### Return type

**crate::models::ListWorldInteractiveDesignsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_job_posted

> crate::models::ListJobShiftsResponse list_world_job_posted()
Shifts you posted for your rooms, with who claimed each and how it went.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListJobShiftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_outfits

> crate::models::ListWorldOutfitsResponse list_world_outfits()
Your saved outfits (up to 6, oldest first): id, name, appearance (skin, hair, hairColour, eyes, build) and items -- each worn thing as the door it goes back on by (slot, door: cosmetic | receipt | stock, id, label). Put one on with POST /v1/world/outfits/:id/apply.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldOutfitsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_bans

> crate::models::ListWorldPlaceBansResponse list_world_place_bans(slug)
Who is banned from a place you own (your boutique or home).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceBansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_bundles

> crate::models::ListWorldPlaceBundlesResponse list_world_place_bundles(slug)
The owner's bundle deals mapped onto this room's items (itemIds null = any N from the shop). The discount is decided at checkout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceBundlesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_collections

> crate::models::ListWorldPlaceCollectionsResponse list_world_place_collections(slug)
The owner's collection rooms: one per category, brand and model they collect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceCollectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_offers

> crate::models::ListWorldPlaceOffersResponse list_world_place_offers(slug)
What is for sale in a public room: price, condition and stock, with market references. 404 when no such room is shared.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_radar

> crate::models::ListWorldPlaceRadarResponse list_world_place_radar(slug)
What you have that someone here wants, and what is here that you want: matched on catalog ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceRadarResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_trade_tables

> crate::models::ListWorldPlaceTradeTablesResponse list_world_place_trade_tables(slug)
The open trade tables in a place: who is sitting and what is laid on the mat.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceTradeTablesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_place_trades

> crate::models::ListWorldPlaceTradesResponse list_world_place_trades(slug)
Items in this room open to trades, each with its trade post path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldPlaceTradesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_provenance

> crate::models::ListWorldProvenanceResponse list_world_provenance()
Provenance trails: where in Crossly World your purchases were found, newest first (optionally ?listingIds=a,b -- platform listing ids). data: [{ platformListingId, slug (the listing public slug), kind (room | window | stall | space | drop | auction | worn | agent), place (its name, or null when gone), at, line -- e.g. \"Found at The Sneaker Vault, Oct 2026\" }]. Yours only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldProvenanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_quests

> crate::models::ListWorldQuestsResponse list_world_quests()
Every world quest with your progress: key, the pin it earns, goal, target, count, done, colour. meta.wearing is the pin on your nameplate (or null); meta.streak your current and best daily visit streak. Progress only comes from what the server saw -- there is no write of it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldQuestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_space_mine

> crate::models::ListWorldSpacesResponse list_world_space_mine()
Spaces you belong to.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldSpacesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_space_requests

> crate::models::ListWorldSpaceRequestsResponse list_world_space_requests(slug)
As host or co-host, who asked to join.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListWorldSpaceRequestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_space_whats_on

> crate::models::ListWorldSpaceWhatsOnResponse list_world_space_whats_on()
Live and upcoming events across Spaces (streets=1 and stage=1 add street and Main Stage events).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldSpaceWhatsOnResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_spaces

> crate::models::ListWorldSpacesResponse list_world_spaces()
The Spaces directory: seller-hosted clubs with a venue, members and events (optional venue filter).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldSpacesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_streaks

> crate::models::ListWorldStreaksResponse list_world_streaks()
The longest current daily visit streaks (2+ days), top ten: name and days. Only people still choosing to be seen by name.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldStreaksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_trade_mine

> crate::models::ListWorldTradeMineResponse list_world_trade_mine()
Your trades and each item progress through the authentication gate (verified or failed).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldTradeMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_trade_table_binder

> crate::models::ListWorldTradeTableBinderResponse list_world_trade_table_binder()
Your binder: unsold collection rows that are catalog items and may be laid on a trade mat -- key, variantId, condition, grade, portfolioId, name, image and valueCents (market value, cents).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldTradeTableBinderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_trade_table_listings

> crate::models::ListWorldTradeTableListingsResponse list_world_trade_table_listings()
Your open Trade Block trade listings a trade table can be opened on in the world, each with its items' names.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldTradeTableListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_scene_cosmetic

> crate::models::UpdateSpatialSceneCosmeticResponse update_spatial_scene_cosmetic(id)
Redecorate one of your rooms (wall, floor, lighting, trim...). Unknown keys are dropped; the response is what survived. `clear` lists keys to reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateSpatialSceneCosmeticResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_scene_node

> crate::models::UpdateSpatialSceneNodeResponse update_spatial_scene_node(id, node_id)
Move a rack in one of your warehouse rooms (position in metres).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateSpatialSceneNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_scene_setting

> crate::models::SpatialSceneResponse2 update_spatial_scene_setting(id)
How one of your rooms is laid out and lit: grouping and sort keys, theme, overlay and the arrival camera. Cosmetics and the shop layout have their own writers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialSceneResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_scene_shop_layout

> crate::models::UpdateSpatialSceneShopLayoutResponse update_spatial_scene_shop_layout(id)
Save your shop room's sections and displays (validated server-side; the response is the stored layout).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateSpatialSceneShopLayoutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_scene_window

> crate::models::UpdateSpatialSceneWindowResponse update_spatial_scene_window(id)
Choose up to three of your live Crossly listings for the street window of your shop, in slot order. Picks that sell drop out and the newest fill in; an empty list goes back to newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateSpatialSceneWindowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_street_decor

> crate::models::GetSpatialStreetDecorResponse update_spatial_street_decor(slug)
Dress a street you own with decor you hold (validated against your inventory).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialStreetDecorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_street_owned

> crate::models::SpatialStreetResponse update_spatial_street_owned(id)
Who may open a shop on a street you own (open, apply or invite).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialStreetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_auto_shop

> crate::models::GetWorldAutoShopResponse update_world_auto_shop(update_world_auto_shop_request)
Turn your automatic world shop off ({ enabled: false } -- an auto-created shop is made private, never deleted) or back on. Answers the new status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_world_auto_shop_request** | **UpdateWorldAutoShopRequest** |  | [required] |

### Return type

**crate::models::GetWorldAutoShopResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_avatar_identity

> crate::models::UpdateWorldAvatarIdentityResponse update_world_avatar_identity()
Choose named or anonymous. Rooms you are already in keep the old answer until you rejoin.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateWorldAvatarIdentityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_avatar_look

> crate::models::UpdateWorldAvatarLookResponse update_world_avatar_look()
Save your world look to the account, so every device starts from it. look: null stays anonymous.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateWorldAvatarLookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_avatar_nft_badge

> crate::models::GetWorldAvatarNftBadgeResponse update_world_avatar_nft_badge()
Wear an NFT you hold on your nameplate (null to take it off). Checked again each time you join a room.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAvatarNftBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_capability_wear

> crate::models::GetWorldCapabilityWearResponse update_world_capability_wear()
Wear a held token on an avatar slot (refId from GET /v1/world/capabilities), or pass refId null to take it off. Its collection must declare a wear capability for that slot.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldCapabilityWearResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_collection_capability

> crate::models::GetWorldCollectionCapabilityResponse update_world_collection_capability(chain, contract)
Declare what the tokens of a collection can do (the whole declaration, spec crossly.nft-capabilities/1). As its creator (launched on Crossly or claimed) any kind; as a holder, cosmetic kinds only. Replaces your previous declaration; pending if it names an uploaded asset or an always-reviewed kind.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**contract** | **String** |  | [required] |

### Return type

**crate::models::GetWorldCollectionCapabilityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_convention_status

> crate::models::UpdateWorldConventionStatusResponse update_world_convention_status(id)
Open, start, end or cancel a convention you host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldConventionStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_cosm_design_chain_auto_unlock

> crate::models::CreateCosmDesignerChainAutoUnlockResponse update_world_cosm_design_chain_auto_unlock(id)
Opt one of your COSM designs in or out of the free chain unlock it earns when popular (on by default). Answers its popularity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCosmDesignerChainAutoUnlockResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_emote_wheel

> crate::models::UpdateWorldEmoteWheelResponse update_world_emote_wheel(update_world_emote_wheel_request)
Arrange your emote wheel: up to 8 slots, each an emote id, a moment slot or null. Returns the saved wheel (locked or unknown ids are dropped).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_world_emote_wheel_request** | **UpdateWorldEmoteWheelRequest** |  | [required] |

### Return type

**crate::models::UpdateWorldEmoteWheelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_fit_value_share

> crate::models::UpdateWorldFitValueShareResponse update_world_fit_value_share()
Whether people in a room can see what your outfit is worth.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateWorldFitValueShareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_home

> crate::models::WorldHomeResponse update_world_home()
Your home: privacy (open, friends, private), theme, and wall, floor and mood decor you hold.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldHomeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_home_builder

> crate::models::UpdateWorldHomeBuilderResponse update_world_home_builder(update_world_home_builder_request)
Set the co-builders of your home: { userIds } -- friends (mutual follows) only, at most 4, replacing the list ([] clears it). They may rearrange your home in the world with your things and add free furniture, never your bought pieces. New co-builders get a world notice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_world_home_builder_request** | **UpdateWorldHomeBuilderRequest** |  | [required] |

### Return type

**crate::models::UpdateWorldHomeBuilderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_home_layout

> crate::models::WorldHomeResponse update_world_home_layout()
Arrange your home: the full layout (up to 200 pieces you own), validated server-side.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldHomeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_home_layout_by_slug

> crate::models::WorldHomeResponse update_world_home_layout_by_slug(slug)
Rearrange a home you co-build (never adding the owner bought pieces).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::WorldHomeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_interactive_design_draft

> crate::models::UpdateWorldInteractiveDesignDraftResponse update_world_interactive_design_draft(id)
Save the draft MML source (up to 64 KB); the response carries its static check.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldInteractiveDesignDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_listing_holders_only

> crate::models::UpdateWorldListingHoldersOnlyResponse update_world_listing_holders_only(id)
Reserve one of your listings for holders of a collection you declared a holders_only perk for (or release it). Checkout refuses anybody else.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldListingHoldersOnlyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_outfit

> crate::models::UpdateWorldOutfitResponse update_world_outfit(id, update_world_outfit_request)
Rename a saved look: { name (1-32 chars) }. 404 when it is not yours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_world_outfit_request** | **UpdateWorldOutfitRequest** |  | [required] |

### Return type

**crate::models::UpdateWorldOutfitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_pet_active

> crate::models::UpdateWorldPetActiveResponse update_world_pet_active(update_world_pet_active_request)
Walk one of your pets with you in the world, or none (kind: null).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_world_pet_active_request** | **UpdateWorldPetActiveRequest** |  | [required] |

### Return type

**crate::models::UpdateWorldPetActiveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_quest_pin

> crate::models::UpdateWorldQuestPinResponse update_world_quest_pin(update_world_quest_pin_request)
Wear a pin you EARNED from a quest on your nameplate, or { pin: null } to wear none. 403 for a pin you have not earned. Shows from the next room you walk into.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_world_quest_pin_request** | **UpdateWorldQuestPinRequest** |  | [required] |

### Return type

**crate::models::UpdateWorldQuestPinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_room_holder_gate

> crate::models::UpdateWorldRoomHolderGateResponse update_world_room_holder_gate(id)
Make your room holders-only (its collection rooms follow). Send null to open it again. Returns `{ gate }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldRoomHolderGateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_space

> crate::models::UpdateWorldSpaceResponse update_world_space(slug)
As host, change a Space name, about or membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldSpaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_space_holder_gate

> crate::models::UpdateWorldRoomHolderGateResponse update_world_space_holder_gate(slug)
Make your Space holders-only: only holders of an NFT collection join it (others can look in). Send null to open it again. Returns `{ gate }`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldRoomHolderGateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_stall_booking_booth

> crate::models::UpdateWorldStallBookingBoothResponse update_world_stall_booking_booth(id)
Dress the booth of a stall you booked (decor you hold).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldStallBookingBoothResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

