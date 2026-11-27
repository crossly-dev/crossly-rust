# \WorldsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience**](WorldsApi.md#create_world_experience) | **POST** /v1/worlds/experiences | Create an experience with a start place (no version yet). Counts against the plan's experience limit.
[**create_world_experience_access**](WorldsApi.md#create_world_experience_access) | **POST** /v1/worlds/experiences/{experienceId}/access | Who can play: private, friends, unlisted or public (public/unlisted need a reviewed publish).
[**create_world_experience_alert_ack**](WorldsApi.md#create_world_experience_alert_ack) | **POST** /v1/worlds/experiences/{experienceId}/alerts/{alertId}/ack | Acknowledge an alert.
[**create_world_experience_analytic_query**](WorldsApi.md#create_world_experience_analytic_query) | **POST** /v1/worlds/experiences/{experienceId}/analytics/query | Query one custom event over ≤ 31 days: count, sum, avg or players, by day or hour, optionally grouped by a field. Groups under 50 players are suppressed.
[**create_world_experience_archive**](WorldsApi.md#create_world_experience_archive) | **POST** /v1/worlds/experiences/{experienceId}/archive | Archive: closes it, drains its servers, frees its plan slot; data is kept.
[**create_world_experience_ban**](WorldsApi.md#create_world_experience_ban) | **POST** /v1/worlds/experiences/{experienceId}/bans | Ban a player from your experience: { sub, duration (seconds, null = permanent), reason, displayReason?, applyToAlts? }. Enforced at join and on every running server.
[**create_world_experience_close**](WorldsApi.md#create_world_experience_close) | **POST** /v1/worlds/experiences/{experienceId}/close | Make the experience private again (owner-only joins).
[**create_world_experience_config_rollback**](WorldsApi.md#create_world_experience_config_rollback) | **POST** /v1/worlds/experiences/{experienceId}/configs/rollback | Instant rollback: an earlier version becomes the newest one.
[**create_world_experience_event**](WorldsApi.md#create_world_experience_event) | **POST** /v1/worlds/experiences/{experienceId}/events | Schedule a live event: start and end, config values while live, an optional Messaging topic the platform publishes to at start and end.
[**create_world_experience_event_cancel**](WorldsApi.md#create_world_experience_event_cancel) | **POST** /v1/worlds/experiences/{experienceId}/events/{eventId}/cancel | Cancel a scheduled event, or end a live one now.
[**create_world_experience_experiment**](WorldsApi.md#create_world_experience_experiment) | **POST** /v1/worlds/experiences/{experienceId}/experiments | Create a draft experiment: 2–5 variants of config values with traffic weights (+ holdout = 100), targeting, a primary metric and guardrails, minimum 7 days.
[**create_world_experience_experiment_ship**](WorldsApi.md#create_world_experience_experiment_ship) | **POST** /v1/worlds/experiences/{experienceId}/experiments/{experimentId}/ship | Ship a variant: its values become the base config for everyone and the experiment ends.
[**create_world_experience_experiment_start**](WorldsApi.md#create_world_experience_experiment_start) | **POST** /v1/worlds/experiences/{experienceId}/experiments/{experimentId}/start | Start: running servers begin assigning players by their per-experience id.
[**create_world_experience_experiment_stop**](WorldsApi.md#create_world_experience_experiment_stop) | **POST** /v1/worlds/experiences/{experienceId}/experiments/{experimentId}/stop | Stop: every player sees the base config again.
[**create_world_experience_join**](WorldsApi.md#create_world_experience_join) | **POST** /v1/worlds/experiences/{experienceId}/join | Join as the token's owner: a server and a 60-second single-use ticket for wss …/v1/realtime/worlds/{serverId}. 202 while a new server boots.
[**create_world_experience_moderation_appeal**](WorldsApi.md#create_world_experience_moderation_appeal) | **POST** /v1/worlds/experiences/{experienceId}/moderation/appeals | Appeal a moderation decision on your game within 6 months (183 days): { kind: action | game | version | asset, targetId (from GET …/moderation/appeals targets), reason (20-4000 characters), evidence?: up to 5 https links }. The older shape { actionId, statement } still appeals an action. One open appeal per decision; a different reviewer answers and you are notified.
[**create_world_experience_open**](WorldsApi.md#create_world_experience_open) | **POST** /v1/worlds/experiences/{experienceId}/open | Open the experience to players (needs a published start place).
[**create_world_experience_place**](WorldsApi.md#create_world_experience_place) | **POST** /v1/worlds/experiences/{experienceId}/places | Add a place (at most 10 per experience in P1).
[**create_world_experience_place_check**](WorldsApi.md#create_world_experience_place_check) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/checks | Run the publish checks (script lint, budgets, shop-object listings) on a project. Writes nothing.
[**create_world_experience_place_publish**](WorldsApi.md#create_world_experience_place_publish) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/publishes | Step 1: run checks and freeze a version (the saved draft, or an uploaded versionId). Blocking problems answer 400 with the list.
[**create_world_experience_place_start**](WorldsApi.md#create_world_experience_place_start) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/start | Make this the start place (where a plain join lands).
[**create_world_experience_place_test**](WorldsApi.md#create_world_experience_place_test) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/test | Cloud playtest: a private test version from the saved draft, joined on a fleet server (owner only, never live). 202 while it boots.
[**create_world_experience_place_version**](WorldsApi.md#create_world_experience_place_version) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/versions | Upload a version bundle (crossly-worlds-project/1). Not live until published.
[**create_world_experience_place_version_publish**](WorldsApi.md#create_world_experience_place_version_publish) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/versions/{versionId}/publish | Make a version current (publish or roll back). New joins land on it; older servers drain as they empty.
[**create_world_experience_place_version_rollback**](WorldsApi.md#create_world_experience_place_version_rollback) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/versions/{versionId}/rollback | Make an earlier reviewed version current again. Unreviewed versions go through …/publishes.
[**create_world_experience_playtester**](WorldsApi.md#create_world_experience_playtester) | **POST** /v1/worlds/experiences/{experienceId}/playtesters | Add a playtester by Crossly handle (max 200 per game).
[**create_world_experience_publish_cancel**](WorldsApi.md#create_world_experience_publish_cancel) | **POST** /v1/worlds/experiences/{experienceId}/publishes/{requestId}/cancel | Give up a publish that is not live.
[**create_world_experience_publish_halt**](WorldsApi.md#create_world_experience_publish_halt) | **POST** /v1/worlds/experiences/{experienceId}/publishes/{requestId}/halt | Stop a rollout: new joins go back to the current version.
[**create_world_experience_publish_rollout**](WorldsApi.md#create_world_experience_publish_rollout) | **POST** /v1/worlds/experiences/{experienceId}/publishes/{requestId}/rollout | Step 7: serve the approved version to percent (1–100) of new joins; 100 makes it current. Crash-heavy rollouts halt automatically.
[**create_world_experience_publish_submit**](WorldsApi.md#create_world_experience_publish_submit) | **POST** /v1/worlds/experiences/{experienceId}/publishes/{requestId}/submit | Step 6: moderation. First publishes and script changes go to a person (24 h SLA); layout-only and private publishes are approved automatically.
[**create_world_experience_reserved_server**](WorldsApi.md#create_world_experience_reserved_server) | **POST** /v1/worlds/experiences/{experienceId}/reserved-servers | An access code for a reserved server of a place (teleports, matches, tournaments). The code is shown once.
[**create_world_experience_restore**](WorldsApi.md#create_world_experience_restore) | **POST** /v1/worlds/experiences/{experienceId}/restore | Restore an archived game, or a deleted one within 30 days; it comes back private.
[**create_world_experience_server_shutdown**](WorldsApi.md#create_world_experience_server_shutdown) | **POST** /v1/worlds/experiences/{experienceId}/servers/shutdown | Shut down servers: mode soft (default) teleports players to fresh servers of the current version first; hard stops now. scope all | outdated (only servers not on a served version).
[**create_world_experience_transfer_request**](WorldsApi.md#create_world_experience_transfer_request) | **POST** /v1/worlds/experiences/{experienceId}/transfer-requests | Ask another Crossly account (handle or email) to take over a 3D game you own. One waiting request per game; it expires after 14 days.
[**create_world_mature_verify**](WorldsApi.md#create_world_mature_verify) | **POST** /v1/worlds/mature/verify | Start the ID check Mature needs (Stripe Identity: photo ID + selfie; the date of birth is read off the ID). Returns a hosted URL and client secret, or alreadyVerified.
[**create_world_parental_accept**](WorldsApi.md#create_world_parental_accept) | **POST** /v1/worlds/parental/accept | Accept a child's invite as the parent: { code, pin }. The parent must be an ID-verified adult (18+ by the date of birth on the ID). The PIN protects every later change.
[**create_world_parental_children_pin**](WorldsApi.md#create_world_parental_children_pin) | **POST** /v1/worlds/parental/children/{childId}/pin | Change the parent PIN: { pin, newPin } (4-8 digits). Five wrong PINs lock changes for 15 minutes.
[**create_world_parental_children_unlink**](WorldsApi.md#create_world_parental_children_unlink) | **POST** /v1/worlds/parental/children/{childId}/unlink | Remove the parent link (and its limits) with your PIN: { pin }.
[**create_world_parental_invite**](WorldsApi.md#create_world_parental_invite) | **POST** /v1/worlds/parental/invite | An under-18 account asks for a parent: a one-time code (72 hours) the parent enters while signed in to their own account. Replaces any pending code.
[**create_world_rating_compute**](WorldsApi.md#create_world_rating_compute) | **POST** /v1/worlds/rating/compute | Rate a set of questionnaire answers exactly as publishing does: { answers } → rating (null when refused), reasons, content descriptors, interactive notices and refusals.
[**create_world_report**](WorldsApi.md#create_world_report) | **POST** /v1/worlds/reports | Report a player (targetSub), the experience, an object (targetInstanceId) or a chat line (chatLineId). Crossly attaches the server, place version and its own chat log; you send ids only.
[**create_world_transfer_request_accept**](WorldsApi.md#create_world_transfer_request_accept) | **POST** /v1/worlds/transfer-requests/{requestId}/accept | Accept a transfer asked of you: the game becomes yours now. Earnings already credited stay with the previous owner; new sales pay you.
[**create_world_transfer_request_cancel**](WorldsApi.md#create_world_transfer_request_cancel) | **POST** /v1/worlds/transfer-requests/{requestId}/cancel | Cancel a transfer request you sent while it waits.
[**create_world_transfer_request_decline**](WorldsApi.md#create_world_transfer_request_decline) | **POST** /v1/worlds/transfer-requests/{requestId}/decline | Decline a transfer asked of you; nothing changes.
[**delete_world_experience**](WorldsApi.md#delete_world_experience) | **DELETE** /v1/worlds/experiences/{experienceId} | Delete a game (a tombstone, restorable for 30 days). Body { confirmName } = the game's name.
[**delete_world_experience_ban**](WorldsApi.md#delete_world_experience_ban) | **DELETE** /v1/worlds/experiences/{experienceId}/bans/{sub} | Lift your experience's ban on a player. Bans placed by Crossly safety are lifted only by appeal.
[**delete_world_experience_playtester**](WorldsApi.md#delete_world_experience_playtester) | **DELETE** /v1/worlds/experiences/{experienceId}/playtesters/{userId} | Remove a playtester.
[**delete_world_parental_invite**](WorldsApi.md#delete_world_parental_invite) | **DELETE** /v1/worlds/parental/invite | Withdraw a pending parent invite.
[**get_world_chart**](WorldsApi.md#get_world_chart) | **GET** /v1/worlds/charts/{chart} | A chart: top-playing (players now), trending (this week vs last, retention-weighted), top-rated (Wilson lower bound of thumbs up, 10+ votes), new-rising, friends, most-favorited; ?genre= cuts it to one genre. Organic only: nothing paid.
[**get_world_discover_experience**](WorldsApi.md#get_world_discover_experience) | **GET** /v1/worlds/discover/experiences/{key} | An experience's thumbs up/down, favourites, your vote and favourite, and whether you have played enough to vote (10 minutes). key: worlds:<id>, classic:<id>, tcg:<game>, cards:<game>.
[**get_world_discover_row**](WorldsApi.md#get_world_discover_row) | **GET** /v1/worlds/discover/rows | The Play hub's rows for you: Continue playing, Friends are playing, Recommended for you, Featured (editorial, first-party capped), Top Playing, Trending, New & Rising, Top Rated, Made in Carnival Studio, Shop the world, card games, Classic games and genre rows. At most one tile is paid, always labelled \"Sponsored\", never in the first two rows or a row's first two places.
[**get_world_experience**](WorldsApi.md#get_world_experience) | **GET** /v1/worlds/experiences/{experienceId} | One of your experiences with its places, live CCU and server count.
[**get_world_experience_access_limit**](WorldsApi.md#get_world_experience_access_limit) | **GET** /v1/worlds/experiences/{experienceId}/access-limits | The game's crew-only and country limits, and the owner's crew.
[**get_world_experience_alert_setting**](WorldsApi.md#get_world_experience_alert_setting) | **GET** /v1/worlds/experiences/{experienceId}/alerts/settings | Alert rules (threshold, on/off, inbox, webhook) and quiet hours.
[**get_world_experience_analytic**](WorldsApi.md#get_world_experience_analytic) | **GET** /v1/worlds/experiences/{experienceId}/analytics/{report} | An analytics report: overview, engagement, retention, monetization, funnels, economy, performance, custom or costs (fleet CPU-minutes, egress, estimated cost). ?from=&to= (YYYY-MM-DD), retention ?segment=, funnels ?funnel=, economy ?currency=.
[**get_world_experience_ban**](WorldsApi.md#get_world_experience_ban) | **GET** /v1/worlds/experiences/{experienceId}/bans/{sub} | The live ban on one player, or 404.
[**get_world_experience_chat_setting**](WorldsApi.md#get_world_experience_chat_setting) | **GET** /v1/worlds/experiences/{experienceId}/chat-settings | The game's text chat setting and filter level.
[**get_world_experience_config**](WorldsApi.md#get_world_experience_config) | **GET** /v1/worlds/experiences/{experienceId}/configs | The remote config document ({ values, rules }) and its version.
[**get_world_experience_experiment**](WorldsApi.md#get_world_experience_experiment) | **GET** /v1/worlds/experiences/{experienceId}/experiments/{experimentId} | Results per variant: n, mean, lift, fixed and always-valid 95% intervals, the verdict and a sample-size estimate.
[**get_world_experience_maturity**](WorldsApi.md#get_world_experience_maturity) | **GET** /v1/worlds/experiences/{experienceId}/maturity | The content maturity questionnaire: questions, answers, the published rating and any rating waiting for the next publish.
[**get_world_experience_moderation**](WorldsApi.md#get_world_experience_moderation) | **GET** /v1/worlds/experiences/{experienceId}/moderation | Moderation status of your experience: holds (unlisted / taken down), the latest review, the automated scan, strikes, trusted-lane status, open reports and live bans.
[**get_world_experience_moderation_appeal**](WorldsApi.md#get_world_experience_moderation_appeal) | **GET** /v1/worlds/experiences/{experienceId}/moderation/appeals | What you can appeal on your game and every appeal of it: data.targets (Crossly's actions, a moderation hold, rejected versions, blocked assets; each with appealableUntil and its latest appeal) and data.appeals (newest first: kind, status open | upheld | overturned, reason, evidence, the reviewer's note).
[**get_world_experience_place_draft**](WorldsApi.md#get_world_experience_place_draft) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/draft | The Studio draft of a place ({ world, place }, revision). Falls back to the current version, or an empty place.
[**get_world_experience_publish**](WorldsApi.md#get_world_experience_publish) | **GET** /v1/worlds/experiences/{experienceId}/publishes/{requestId} | One publish with its review state and rollout.
[**get_world_experience_shop**](WorldsApi.md#get_world_experience_shop) | **GET** /v1/worlds/experiences/{experienceId}/shop | An open experience's way back to commerce: its creator's shop and how many live listings it has.
[**get_world_experience_showcase**](WorldsApi.md#get_world_experience_showcase) | **GET** /v1/worlds/experiences/{experienceId}/showcase | The listings the game's page showcases, and the owner's live Crossly listings that can be picked.
[**get_world_friend_playing**](WorldsApi.md#get_world_friend_playing) | **GET** /v1/worlds/friends/playing | Your friends (mutual follows) and where each is playing right now, with the link that joins them.
[**get_world_game_by_game**](WorldsApi.md#get_world_game_by_game) | **GET** /v1/worlds/games/{game} | A game's page, as the /games launcher shows it: the tile (players now, thumbs-up %, friends inside), description, tags, creator (Crossly for its own), rating with its content descriptors, visits, created and updated, release notes, gallery, what it sells (bought through Crossly checkout) and the Crossly World app link. :game is a Worlds experience id or any experience key.
[**get_world_game_mine**](WorldsApi.md#get_world_game_mine) | **GET** /v1/worlds/games/mine/{gameId} | A game's summary: status, timeline, the sub-pages you may open, health and next steps.
[**get_world_game_mine_assets_used**](WorldsApi.md#get_world_game_mine_assets_used) | **GET** /v1/worlds/games/mine/{gameId}/assets-used | Everything a game's published build ships (Assets used): models, sounds, images, scripts, Toolbox assets and other files, each with the places that ship it. A 3D world reads each place's current version; a Classic game its latest reviewed package version.
[**get_world_game_server**](WorldsApi.md#get_world_game_server) | **GET** /v1/worlds/games/{game}/servers | A Crossly Carnival game's public servers that are up: players, max, whether a stranger still fits, your friends on each (friends' servers first, then the fullest with room); and the private servers you pay for. Join one with POST /v1/worlds/experiences/:id/join { serverId }.
[**get_world_link_card_game**](WorldsApi.md#get_world_link_card_game) | **GET** /v1/worlds/links/card-game/{game} | Where to play a card game: Crossly's own game, its Commons tables, and creator worlds that tag it.
[**get_world_link_listing**](WorldsApi.md#get_world_link_listing) | **GET** /v1/worlds/links/listing/{slug} | \"Try it in Crossly Carnival\": open games whose live version shows this Crossly listing (its shop slug), then other games by the same seller. Links only: a sale is attributed to a game only through in-world touches its server reports.
[**get_world_link_seller**](WorldsApi.md#get_world_link_seller) | **GET** /v1/worlds/links/seller/{username} | \"Games by this seller\": the seller's open games, best first.
[**get_world_mature**](WorldsApi.md#get_world_mature) | **GET** /v1/worlds/mature | Mature (17+) for this account: eligible or not, the steps still to take in order (birth month, ID check, opt-in; region, parental controls and safety holds are final), and the ID age check status.
[**get_world_me_pref**](WorldsApi.md#get_world_me_pref) | **GET** /v1/worlds/me/prefs | Your Crossly Carnival choices (whether games may see your Crossly wallet address).
[**get_world_parental_children**](WorldsApi.md#get_world_parental_children) | **GET** /v1/worlds/parental/children/{childId} | One linked account with ?days= (1-90) of playtime per experience.
[**get_world_parental_me**](WorldsApi.md#get_world_parental_me) | **GET** /v1/worlds/parental/me | This account's parent link: none, pending (an invite is out) or active (the parent's name and the limits they set).
[**get_world_plaza**](WorldsApi.md#get_world_plaza) | **GET** /v1/worlds/plaza | The Carnival Plaza district: the twelve-portal ring (a sponsored portal can only take the last slot, labelled), the new & rising row, the featured stage's editorial pick and four chart boards, with players now and friends inside.
[**get_world_policy_me**](WorldsApi.md#get_world_policy_me) | **GET** /v1/worlds/policy/me | What this account may do in experiences, by age and region: chat level, purchases, paid random items, trading, links, ads, contests, rating ceiling — with the reason for each \"no\".
[**get_world_rating_questionnaire**](WorldsApi.md#get_world_rating_questionnaire) | **GET** /v1/worlds/rating/questionnaire | The content-rating questionnaire (IARC-aligned): every question and choice, the three ratings (Everyone, Teen 13+, Mature 17+), what is never allowed, and the regions where Mature is restricted.
[**get_world_search**](WorldsApi.md#get_world_search) | **GET** /v1/worlds/search | Search experiences by title, creator, tags and genre (typo-tolerant, synonyms such as obby / obstacle). Filters: genre, rating (everyone keeps to Everyone), device, players (one server holds at least N), minThumbs (percent), live, friends. No results suggests a chart.
[**get_world_transfer_request**](WorldsApi.md#get_world_transfer_request) | **GET** /v1/worlds/transfer-requests/{requestId} | One ownership transfer request (only its two people can read it), with the actions you may take.
[**get_world_transparency**](WorldsApi.md#get_world_transparency) | **GET** /v1/worlds/transparency/{month} | One month's transparency report (YYYY-MM): reports by reason and source, decisions and median time to decide, actions by kind and issuer, appeals and overturn rate, publish reviews, second reviews of creator bans.
[**list_world_appeals**](WorldsApi.md#list_world_appeals) | **GET** /v1/worlds/appeals | Your appeals and their decisions.
[**list_world_experience_alerts**](WorldsApi.md#list_world_experience_alerts) | **GET** /v1/worlds/experiences/{experienceId}/alerts | Alerts that fired, newest first.
[**list_world_experience_audit**](WorldsApi.md#list_world_experience_audit) | **GET** /v1/worlds/experiences/{experienceId}/audit | The live-ops audit log: every config, experiment, event and alert change, who made it, when.
[**list_world_experience_bans**](WorldsApi.md#list_world_experience_bans) | **GET** /v1/worlds/experiences/{experienceId}/bans | Players banned from your experience (by pairwise sub), oldest first (cursor).
[**list_world_experience_config_versions**](WorldsApi.md#list_world_experience_config_versions) | **GET** /v1/worlds/experiences/{experienceId}/configs/versions | Config history, newest first: who changed which keys, when.
[**list_world_experience_events**](WorldsApi.md#list_world_experience_events) | **GET** /v1/worlds/experiences/{experienceId}/events | Scheduled and live events (?includeEnded=true for history).
[**list_world_experience_experiments**](WorldsApi.md#list_world_experience_experiments) | **GET** /v1/worlds/experiences/{experienceId}/experiments | A/B experiments, newest first, with their live verdict.
[**list_world_experience_moderation_reports**](WorldsApi.md#list_world_experience_moderation_reports) | **GET** /v1/worlds/experiences/{experienceId}/moderation/reports | Reports about your experience, newest first (cursor). Reporter identity, account ids and other players' words are removed.
[**list_world_experience_place_publishes**](WorldsApi.md#list_world_experience_place_publishes) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/publishes | Publishes of a place, newest first: state, checks, rating, access, review, rollout.
[**list_world_experience_place_versions**](WorldsApi.md#list_world_experience_place_versions) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/versions | Version history of a place, newest first; `current` marks the live one.
[**list_world_experience_places**](WorldsApi.md#list_world_experience_places) | **GET** /v1/worlds/experiences/{experienceId}/places | The places of an experience, start place first.
[**list_world_experience_playtesters**](WorldsApi.md#list_world_experience_playtesters) | **GET** /v1/worlds/experiences/{experienceId}/playtesters | People who may join while the game is private.
[**list_world_experience_servers**](WorldsApi.md#list_world_experience_servers) | **GET** /v1/worlds/experiences/{experienceId}/servers | Live servers: place, version, region, players, health.
[**list_world_experiences**](WorldsApi.md#list_world_experiences) | **GET** /v1/worlds/experiences | Your experiences, newest first; cursor-paged (?cursor=&limit=).
[**list_world_favorites**](WorldsApi.md#list_world_favorites) | **GET** /v1/worlds/favorites | Your favourite experiences that are still open, newest first.
[**list_world_game_mine**](WorldsApi.md#list_world_game_mine) | **GET** /v1/worlds/games/mine | Every game you own or collaborate on, 3D worlds and Classic, filtered (?q&status&engine&genre&template&owner&flag&sort; sort=revenue = highest earning first). Each row carries revenueCents: your creator-ledger earnings from it in USD cents (null for Classic).
[**list_world_parental_children**](WorldsApi.md#list_world_parental_children) | **GET** /v1/worlds/parental/children | Accounts you are the linked parent of: limits (max rating, chat, monthly spend), spend this month, and the last 7 days of playtime per experience.
[**list_world_recent**](WorldsApi.md#list_world_recent) | **GET** /v1/worlds/recent | Continue playing: what you played in the last 90 days that is still open, newest first (from what the servers recorded).
[**list_world_regions**](WorldsApi.md#list_world_regions) | **GET** /v1/worlds/regions | Live server regions with a probe URL each: GET it a few times, send the round trips as `pings` with the join to land in the nearest region.
[**list_world_shop_listings**](WorldsApi.md#list_world_shop_listings) | **GET** /v1/worlds/shop-listings | Your live Crossly listings a game can show and sell (shop objects bind to their id).
[**list_world_transfer_requests**](WorldsApi.md#list_world_transfer_requests) | **GET** /v1/worlds/transfer-requests | Ownership transfer requests you were asked (incoming) or sent (outgoing), newest first (?direction&status&experienceId).
[**list_world_transparency**](WorldsApi.md#list_world_transparency) | **GET** /v1/worlds/transparency | Published monthly transparency reports, newest first: reports received and actions taken, by type (counts only).
[**update_world_experience**](WorldsApi.md#update_world_experience) | **PATCH** /v1/worlds/experiences/{experienceId} | Edit name, description, genre, rating or icon.
[**update_world_experience_access_limit**](WorldsApi.md#update_world_experience_access_limit) | **PUT** /v1/worlds/experiences/{experienceId}/access-limits | Limit who can play: { crewOnly } (only members of the owner's crew), { countryMode: all|allow|block, countries: ['US', ...] }. The owner and team are never limited.
[**update_world_experience_alert_setting**](WorldsApi.md#update_world_experience_alert_setting) | **PUT** /v1/worlds/experiences/{experienceId}/alerts/settings | Change alert rules and quiet hours.
[**update_world_experience_chat_setting**](WorldsApi.md#update_world_experience_chat_setting) | **PUT** /v1/worlds/experiences/{experienceId}/chat-settings | Turn text chat off or use the strict filter for everyone (never looser than the platform).
[**update_world_experience_config**](WorldsApi.md#update_world_experience_config) | **PUT** /v1/worlds/experiences/{experienceId}/configs | Save the whole config document. Running servers get it within seconds (ConfigService.Changed). baseVersion guards against overwriting a newer save (409).
[**update_world_experience_config_value**](WorldsApi.md#update_world_experience_config_value) | **PATCH** /v1/worlds/experiences/{experienceId}/configs/values | Set and/or remove individual config keys.
[**update_world_experience_event**](WorldsApi.md#update_world_experience_event) | **PATCH** /v1/worlds/experiences/{experienceId}/events/{eventId} | Reschedule or change a scheduled event; a live event may only move its end.
[**update_world_experience_experiment**](WorldsApi.md#update_world_experience_experiment) | **PATCH** /v1/worlds/experiences/{experienceId}/experiments/{experimentId} | Edit a draft (a running experiment may only rename or reword).
[**update_world_experience_maturity**](WorldsApi.md#update_world_experience_maturity) | **PUT** /v1/worlds/experiences/{experienceId}/maturity | Answer the questionnaire: { answers }. A stricter rating applies now; a looser or first Mature rating waits for the next reviewed publish (takesEffect).
[**update_world_experience_place**](WorldsApi.md#update_world_experience_place) | **PATCH** /v1/worlds/experiences/{experienceId}/places/{placeId} | Place settings: name, max players (1-50), fill target, social slots, empty shutdown.
[**update_world_experience_place_draft**](WorldsApi.md#update_world_experience_place_draft) | **PUT** /v1/worlds/experiences/{experienceId}/places/{placeId}/draft | Save the draft. baseRevision is the revision you loaded (0 for none); a newer save elsewhere answers 409 draft_conflict.
[**update_world_experience_place_version_note**](WorldsApi.md#update_world_experience_place_version_note) | **PUT** /v1/worlds/experiences/{experienceId}/places/{placeId}/versions/{versionId}/notes | Edit a release version's changelog (the game page's What's new): { notes } up to 2000 characters.
[**update_world_experience_publish**](WorldsApi.md#update_world_experience_publish) | **PATCH** /v1/worlds/experiences/{experienceId}/publishes/{requestId} | Steps 2–5: details, rating questionnaire answers, access (private | friends | unlisted | public), money review acknowledgement.
[**update_world_experience_showcase**](WorldsApi.md#update_world_experience_showcase) | **PUT** /v1/worlds/experiences/{experienceId}/showcase | Set the showcased listings: { listingIds } (Crossly shop slugs of the owner's live listings, max 12, in order).
[**update_world_favorite**](WorldsApi.md#update_world_favorite) | **PUT** /v1/worlds/favorites | Favourite or un-favourite an experience (idempotent).
[**update_world_mature**](WorldsApi.md#update_world_mature) | **PUT** /v1/worlds/mature | Switch \"Show Mature experiences\" on or off: { showMature }. On is refused while a final step blocks (too young, region, parental controls, safety hold); it never skips the ID check.
[**update_world_me_pref**](WorldsApi.md#update_world_me_pref) | **PUT** /v1/worlds/me/prefs | Set your Crossly Carnival choices.
[**update_world_parental_children**](WorldsApi.md#update_world_parental_children) | **PATCH** /v1/worlds/parental/children/{childId} | Change a linked account's limits with your PIN: { pin, settings: { maxRating: everyone|teen, chat: on|friends|off, monthlySpendLimitCents (0 = no purchases, null = no limit) } }. Enforced server-side at join, chat, the purchase prompt and checkout.
[**update_world_vote**](WorldsApi.md#update_world_vote) | **PUT** /v1/worlds/votes | Thumbs up, down, or null to withdraw: one vote per player per experience, counted only after 10 minutes of play (403 vote_needs_playtime before). A light ranking signal.



## create_world_experience

> crate::models::CreateWorldExperienceResponse create_world_experience()
Create an experience with a start place (no version yet). Counts against the plan's experience limit.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_access

> crate::models::WorldExperienceResponse create_world_experience_access(experience_id)
Who can play: private, friends, unlisted or public (public/unlisted need a reviewed publish).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_alert_ack

> crate::models::CreateWorldExperienceAlertAckResponse create_world_experience_alert_ack(experience_id, alert_id)
Acknowledge an alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**alert_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceAlertAckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_analytic_query

> crate::models::CreateWorldExperienceAnalyticQueryResponse create_world_experience_analytic_query(experience_id)
Query one custom event over ≤ 31 days: count, sum, avg or players, by day or hour, optionally grouped by a field. Groups under 50 players are suppressed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceAnalyticQueryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_archive

> crate::models::WorldExperienceResponse create_world_experience_archive(experience_id)
Archive: closes it, drains its servers, frees its plan slot; data is kept.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_ban

> crate::models::GetWorldExperienceBanResponse create_world_experience_ban(experience_id)
Ban a player from your experience: { sub, duration (seconds, null = permanent), reason, displayReason?, applyToAlts? }. Enforced at join and on every running server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceBanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_close

> crate::models::WorldExperienceResponse create_world_experience_close(experience_id)
Make the experience private again (owner-only joins).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_config_rollback

> crate::models::WorldExperienceConfigResponse create_world_experience_config_rollback(experience_id)
Instant rollback: an earlier version becomes the newest one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_event

> crate::models::WorldExperienceEventResponse create_world_experience_event(experience_id)
Schedule a live event: start and end, config values while live, an optional Messaging topic the platform publishes to at start and end.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_event_cancel

> crate::models::WorldExperienceEventResponse create_world_experience_event_cancel(experience_id, event_id)
Cancel a scheduled event, or end a live one now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**event_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experiment

> crate::models::WorldExperienceExperimentResponse create_world_experience_experiment(experience_id)
Create a draft experiment: 2–5 variants of config values with traffic weights (+ holdout = 100), targeting, a primary metric and guardrails, minimum 7 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experiment_ship

> crate::models::WorldExperienceExperimentResponse create_world_experience_experiment_ship(experience_id, experiment_id)
Ship a variant: its values become the base config for everyone and the experiment ends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**experiment_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experiment_start

> crate::models::WorldExperienceExperimentResponse create_world_experience_experiment_start(experience_id, experiment_id)
Start: running servers begin assigning players by their per-experience id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**experiment_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experiment_stop

> crate::models::WorldExperienceExperimentResponse create_world_experience_experiment_stop(experience_id, experiment_id)
Stop: every player sees the base config again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**experiment_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_join

> crate::models::CreateWorldExperienceJoinResponse create_world_experience_join(experience_id)
Join as the token's owner: a server and a 60-second single-use ticket for wss …/v1/realtime/worlds/{serverId}. 202 while a new server boots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_moderation_appeal

> crate::models::CreateWorldExperienceModerationAppealResponse create_world_experience_moderation_appeal(experience_id)
Appeal a moderation decision on your game within 6 months (183 days): { kind: action | game | version | asset, targetId (from GET …/moderation/appeals targets), reason (20-4000 characters), evidence?: up to 5 https links }. The older shape { actionId, statement } still appeals an action. One open appeal per decision; a different reviewer answers and you are notified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceModerationAppealResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_open

> crate::models::WorldExperienceResponse create_world_experience_open(experience_id)
Open the experience to players (needs a published start place).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place

> crate::models::WorldExperiencePlaceResponse create_world_experience_place(experience_id)
Add a place (at most 10 per experience in P1).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_check

> crate::models::CreateWorldExperiencePlaceCheckResponse create_world_experience_place_check(experience_id, place_id)
Run the publish checks (script lint, budgets, shop-object listings) on a project. Writes nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_publish

> crate::models::WorldExperienceResponse2 create_world_experience_place_publish(experience_id, place_id)
Step 1: run checks and freeze a version (the saved draft, or an uploaded versionId). Blocking problems answer 400 with the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_start

> crate::models::WorldExperiencePlaceResponse create_world_experience_place_start(experience_id, place_id)
Make this the start place (where a plain join lands).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_test

> crate::models::CreateWorldExperiencePlaceTestResponse create_world_experience_place_test(experience_id, place_id)
Cloud playtest: a private test version from the saved draft, joined on a fleet server (owner only, never live). 202 while it boots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceTestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_version

> crate::models::WorldExperiencePlaceVersionResponse create_world_experience_place_version(experience_id, place_id)
Upload a version bundle (crossly-worlds-project/1). Not live until published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_version_publish

> crate::models::WorldExperiencePlaceVersionResponse create_world_experience_place_version_publish(experience_id, place_id, version_id)
Make a version current (publish or roll back). New joins land on it; older servers drain as they empty.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_version_rollback

> crate::models::WorldExperiencePlaceVersionResponse create_world_experience_place_version_rollback(experience_id, place_id, version_id)
Make an earlier reviewed version current again. Unreviewed versions go through …/publishes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceVersionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_playtester

> crate::models::CreateWorldExperiencePlaytesterResponse create_world_experience_playtester(experience_id)
Add a playtester by Crossly handle (max 200 per game).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaytesterResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_publish_cancel

> crate::models::WorldExperienceResponse2 create_world_experience_publish_cancel(experience_id, request_id)
Give up a publish that is not live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_publish_halt

> crate::models::WorldExperienceResponse2 create_world_experience_publish_halt(experience_id, request_id)
Stop a rollout: new joins go back to the current version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_publish_rollout

> crate::models::WorldExperienceResponse2 create_world_experience_publish_rollout(experience_id, request_id)
Step 7: serve the approved version to percent (1–100) of new joins; 100 makes it current. Crash-heavy rollouts halt automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_publish_submit

> crate::models::WorldExperienceResponse2 create_world_experience_publish_submit(experience_id, request_id)
Step 6: moderation. First publishes and script changes go to a person (24 h SLA); layout-only and private publishes are approved automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_reserved_server

> crate::models::CreateWorldExperienceReservedServerResponse create_world_experience_reserved_server(experience_id)
An access code for a reserved server of a place (teleports, matches, tournaments). The code is shown once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceReservedServerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_restore

> crate::models::WorldExperienceResponse create_world_experience_restore(experience_id)
Restore an archived game, or a deleted one within 30 days; it comes back private.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_server_shutdown

> crate::models::CreateWorldExperienceServerShutdownResponse create_world_experience_server_shutdown(experience_id)
Shut down servers: mode soft (default) teleports players to fresh servers of the current version first; hard stops now. scope all | outdated (only servers not on a served version).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceServerShutdownResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_transfer_request

> crate::models::WorldResponse2 create_world_experience_transfer_request(experience_id)
Ask another Crossly account (handle or email) to take over a 3D game you own. One waiting request per game; it expires after 14 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_mature_verify

> crate::models::CreateWorldMatureVerifyResponse create_world_mature_verify()
Start the ID check Mature needs (Stripe Identity: photo ID + selfie; the date of birth is read off the ID). Returns a hosted URL and client secret, or alreadyVerified.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldMatureVerifyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_parental_accept

> crate::models::WorldParentalResponse create_world_parental_accept()
Accept a child's invite as the parent: { code, pin }. The parent must be an ID-verified adult (18+ by the date of birth on the ID). The PIN protects every later change.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldParentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_parental_children_pin

> crate::models::CreateWorldParentalChildrenPinResponse create_world_parental_children_pin(child_id)
Change the parent PIN: { pin, newPin } (4-8 digits). Five wrong PINs lock changes for 15 minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldParentalChildrenPinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_parental_children_unlink

> crate::models::CreateWorldParentalChildrenUnlinkResponse create_world_parental_children_unlink(child_id)
Remove the parent link (and its limits) with your PIN: { pin }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldParentalChildrenUnlinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_parental_invite

> crate::models::CreateWorldParentalInviteResponse create_world_parental_invite()
An under-18 account asks for a parent: a one-time code (72 hours) the parent enters while signed in to their own account. Replaces any pending code.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldParentalInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_rating_compute

> crate::models::CreateWorldRatingComputeResponse create_world_rating_compute()
Rate a set of questionnaire answers exactly as publishing does: { answers } → rating (null when refused), reasons, content descriptors, interactive notices and refusals.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldRatingComputeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_report

> crate::models::CreateWorldReportResponse create_world_report()
Report a player (targetSub), the experience, an object (targetInstanceId) or a chat line (chatLineId). Crossly attaches the server, place version and its own chat log; you send ids only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_transfer_request_accept

> crate::models::WorldResponse2 create_world_transfer_request_accept(request_id)
Accept a transfer asked of you: the game becomes yours now. Earnings already credited stay with the previous owner; new sales pay you.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_transfer_request_cancel

> crate::models::WorldResponse2 create_world_transfer_request_cancel(request_id)
Cancel a transfer request you sent while it waits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_transfer_request_decline

> crate::models::WorldResponse2 create_world_transfer_request_decline(request_id)
Decline a transfer asked of you; nothing changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience

> crate::models::DeleteWorldExperienceResponse delete_world_experience(experience_id)
Delete a game (a tombstone, restorable for 30 days). Body { confirmName } = the game's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_ban

> crate::models::DeleteWorldExperienceBanResponse delete_world_experience_ban(experience_id, sub)
Lift your experience's ban on a player. Bans placed by Crossly safety are lifted only by appeal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**sub** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperienceBanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_playtester

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_playtester(experience_id, user_id)
Remove a playtester.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_parental_invite

> crate::models::DeleteDaoProposalResponse delete_world_parental_invite()
Withdraw a pending parent invite.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteDaoProposalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_chart

> crate::models::GetWorldChartResponse get_world_chart(chart)
A chart: top-playing (players now), trending (this week vs last, retention-weighted), top-rated (Wilson lower bound of thumbs up, 10+ votes), new-rising, friends, most-favorited; ?genre= cuts it to one genre. Organic only: nothing paid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chart** | **String** |  | [required] |

### Return type

**crate::models::GetWorldChartResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_discover_experience

> crate::models::WorldResponse3 get_world_discover_experience(key)
An experience's thumbs up/down, favourites, your vote and favourite, and whether you have played enough to vote (10 minutes). key: worlds:<id>, classic:<id>, tcg:<game>, cards:<game>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_discover_row

> crate::models::GetWorldDiscoverRowResponse get_world_discover_row()
The Play hub's rows for you: Continue playing, Friends are playing, Recommended for you, Featured (editorial, first-party capped), Top Playing, Trending, New & Rising, Top Rated, Made in Carnival Studio, Shop the world, card games, Classic games and genre rows. At most one tile is paid, always labelled \"Sponsored\", never in the first two rows or a row's first two places.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldDiscoverRowResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience

> crate::models::GetWorldExperienceResponse get_world_experience(experience_id)
One of your experiences with its places, live CCU and server count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_access_limit

> crate::models::GetWorldExperienceAccessLimitResponse get_world_experience_access_limit(experience_id)
The game's crew-only and country limits, and the owner's crew.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAccessLimitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_alert_setting

> crate::models::GetWorldExperienceAlertSettingResponse get_world_experience_alert_setting(experience_id)
Alert rules (threshold, on/off, inbox, webhook) and quiet hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAlertSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_analytic

> crate::models::GetWorldExperienceAnalyticResponse get_world_experience_analytic(experience_id, report)
An analytics report: overview, engagement, retention, monetization, funnels, economy, performance, custom or costs (fleet CPU-minutes, egress, estimated cost). ?from=&to= (YYYY-MM-DD), retention ?segment=, funnels ?funnel=, economy ?currency=.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**report** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAnalyticResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_ban

> crate::models::GetWorldExperienceBanResponse get_world_experience_ban(experience_id, sub)
The live ban on one player, or 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**sub** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceBanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_chat_setting

> crate::models::GetWorldExperienceChatSettingResponse get_world_experience_chat_setting(experience_id)
The game's text chat setting and filter level.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceChatSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_config

> crate::models::WorldExperienceConfigResponse get_world_experience_config(experience_id)
The remote config document ({ values, rules }) and its version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_experiment

> crate::models::GetWorldExperienceExperimentResponse get_world_experience_experiment(experience_id, experiment_id)
Results per variant: n, mean, lift, fixed and always-valid 95% intervals, the verdict and a sample-size estimate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**experiment_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_maturity

> crate::models::GetWorldExperienceMaturityResponse get_world_experience_maturity(experience_id)
The content maturity questionnaire: questions, answers, the published rating and any rating waiting for the next publish.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMaturityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_moderation

> crate::models::GetWorldExperienceModerationResponse get_world_experience_moderation(experience_id)
Moderation status of your experience: holds (unlisted / taken down), the latest review, the automated scan, strikes, trusted-lane status, open reports and live bans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceModerationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_moderation_appeal

> crate::models::GetWorldExperienceModerationAppealResponse get_world_experience_moderation_appeal(experience_id)
What you can appeal on your game and every appeal of it: data.targets (Crossly's actions, a moderation hold, rejected versions, blocked assets; each with appealableUntil and its latest appeal) and data.appeals (newest first: kind, status open | upheld | overturned, reason, evidence, the reviewer's note).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceModerationAppealResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_draft

> crate::models::GetWorldExperiencePlaceDraftResponse get_world_experience_place_draft(experience_id, place_id)
The Studio draft of a place ({ world, place }, revision). Falls back to the current version, or an empty place.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_publish

> crate::models::WorldExperienceResponse2 get_world_experience_publish(experience_id, request_id)
One publish with its review state and rollout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_shop

> crate::models::GetWorldExperienceShopResponse get_world_experience_shop(experience_id)
An open experience's way back to commerce: its creator's shop and how many live listings it has.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceShopResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_showcase

> crate::models::GetWorldExperienceShowcaseResponse get_world_experience_showcase(experience_id)
The listings the game's page showcases, and the owner's live Crossly listings that can be picked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceShowcaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_friend_playing

> crate::models::GetWorldFriendPlayingResponse get_world_friend_playing()
Your friends (mutual follows) and where each is playing right now, with the link that joins them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldFriendPlayingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_by_game

> crate::models::GetWorldGameByGameResponse get_world_game_by_game(game)
A game's page, as the /games launcher shows it: the tile (players now, thumbs-up %, friends inside), description, tags, creator (Crossly for its own), rating with its content descriptors, visits, created and updated, release notes, gallery, what it sells (bought through Crossly checkout) and the Crossly World app link. :game is a Worlds experience id or any experience key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameByGameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_mine

> crate::models::GetWorldGameMineResponse get_world_game_mine(game_id)
A game's summary: status, timeline, the sub-pages you may open, health and next steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_mine_assets_used

> crate::models::GetWorldGameMineAssetsUsedResponse get_world_game_mine_assets_used(game_id)
Everything a game's published build ships (Assets used): models, sounds, images, scripts, Toolbox assets and other files, each with the places that ship it. A 3D world reads each place's current version; a Classic game its latest reviewed package version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameMineAssetsUsedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_server

> crate::models::GetWorldGameServerResponse get_world_game_server(game)
A Crossly Carnival game's public servers that are up: players, max, whether a stranger still fits, your friends on each (friends' servers first, then the fullest with room); and the private servers you pay for. Join one with POST /v1/worlds/experiences/:id/join { serverId }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameServerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_link_card_game

> crate::models::WorldLinkResponse get_world_link_card_game(game)
Where to play a card game: Crossly's own game, its Commons tables, and creator worlds that tag it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::WorldLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_link_listing

> crate::models::WorldLinkResponse get_world_link_listing(slug)
\"Try it in Crossly Carnival\": open games whose live version shows this Crossly listing (its shop slug), then other games by the same seller. Links only: a sale is attributed to a game only through in-world touches its server reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::WorldLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_link_seller

> crate::models::WorldLinkResponse get_world_link_seller(username)
\"Games by this seller\": the seller's open games, best first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

**crate::models::WorldLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_mature

> crate::models::GetWorldMatureResponse get_world_mature()
Mature (17+) for this account: eligible or not, the steps still to take in order (birth month, ID check, opt-in; region, parental controls and safety holds are final), and the ID age check status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldMatureResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_me_pref

> crate::models::GetWorldMePrefResponse get_world_me_pref()
Your Crossly Carnival choices (whether games may see your Crossly wallet address).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldMePrefResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_parental_children

> crate::models::WorldParentalResponse get_world_parental_children(child_id)
One linked account with ?days= (1-90) of playtime per experience.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_id** | **String** |  | [required] |

### Return type

**crate::models::WorldParentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_parental_me

> crate::models::GetWorldParentalMeResponse get_world_parental_me()
This account's parent link: none, pending (an invite is out) or active (the parent's name and the limits they set).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldParentalMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_plaza

> crate::models::GetWorldPlazaResponse get_world_plaza()
The Carnival Plaza district: the twelve-portal ring (a sponsored portal can only take the last slot, labelled), the new & rising row, the featured stage's editorial pick and four chart boards, with players now and friends inside.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldPlazaResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_policy_me

> crate::models::GetWorldPolicyMeResponse get_world_policy_me()
What this account may do in experiences, by age and region: chat level, purchases, paid random items, trading, links, ads, contests, rating ceiling — with the reason for each \"no\".

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldPolicyMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_rating_questionnaire

> crate::models::GetWorldRatingQuestionnaireResponse get_world_rating_questionnaire()
The content-rating questionnaire (IARC-aligned): every question and choice, the three ratings (Everyone, Teen 13+, Mature 17+), what is never allowed, and the regions where Mature is restricted.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldRatingQuestionnaireResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_search

> crate::models::GetWorldSearchResponse get_world_search()
Search experiences by title, creator, tags and genre (typo-tolerant, synonyms such as obby / obstacle). Filters: genre, rating (everyone keeps to Everyone), device, players (one server holds at least N), minThumbs (percent), live, friends. No results suggests a chart.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldSearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_transfer_request

> crate::models::WorldResponse2 get_world_transfer_request(request_id)
One ownership transfer request (only its two people can read it), with the actions you may take.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_transparency

> crate::models::GetWorldTransparencyResponse get_world_transparency(month)
One month's transparency report (YYYY-MM): reports by reason and source, decisions and median time to decide, actions by kind and issuer, appeals and overturn rate, publish reviews, second reviews of creator bans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |

### Return type

**crate::models::GetWorldTransparencyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_appeals

> crate::models::ListWorldAppealsResponse list_world_appeals()
Your appeals and their decisions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldAppealsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_alerts

> crate::models::ListWorldExperienceAlertsResponse list_world_experience_alerts(experience_id)
Alerts that fired, newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceAlertsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_audit

> crate::models::ListWorldExperienceAuditResponse list_world_experience_audit(experience_id)
The live-ops audit log: every config, experiment, event and alert change, who made it, when.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceAuditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_bans

> crate::models::ListWorldExperienceBansResponse list_world_experience_bans(experience_id)
Players banned from your experience (by pairwise sub), oldest first (cursor).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceBansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_config_versions

> crate::models::ListWorldExperienceConfigVersionsResponse list_world_experience_config_versions(experience_id)
Config history, newest first: who changed which keys, when.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceConfigVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_events

> crate::models::ListWorldExperienceEventsResponse list_world_experience_events(experience_id)
Scheduled and live events (?includeEnded=true for history).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_experiments

> crate::models::ListWorldExperienceExperimentsResponse list_world_experience_experiments(experience_id)
A/B experiments, newest first, with their live verdict.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceExperimentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_moderation_reports

> crate::models::ListWorldExperienceModerationReportsResponse list_world_experience_moderation_reports(experience_id)
Reports about your experience, newest first (cursor). Reporter identity, account ids and other players' words are removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceModerationReportsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_place_publishes

> crate::models::ListWorldExperiencePlacePublishesResponse list_world_experience_place_publishes(experience_id, place_id)
Publishes of a place, newest first: state, checks, rating, access, review, rollout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlacePublishesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_place_versions

> crate::models::ListWorldExperiencePlaceVersionsResponse list_world_experience_place_versions(experience_id, place_id)
Version history of a place, newest first; `current` marks the live one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlaceVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_places

> crate::models::ListWorldExperiencePlacesResponse list_world_experience_places(experience_id)
The places of an experience, start place first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlacesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_playtesters

> crate::models::ListWorldExperiencePlaytestersResponse list_world_experience_playtesters(experience_id)
People who may join while the game is private.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlaytestersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_servers

> crate::models::ListWorldExperienceServersResponse list_world_experience_servers(experience_id)
Live servers: place, version, region, players, health.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceServersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experiences

> crate::models::ListWorldExperiencesResponse list_world_experiences()
Your experiences, newest first; cursor-paged (?cursor=&limit=).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldExperiencesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_favorites

> crate::models::ListWorldRecentResponse list_world_favorites()
Your favourite experiences that are still open, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldRecentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_game_mine

> crate::models::ListWorldGameMineResponse list_world_game_mine()
Every game you own or collaborate on, 3D worlds and Classic, filtered (?q&status&engine&genre&template&owner&flag&sort; sort=revenue = highest earning first). Each row carries revenueCents: your creator-ledger earnings from it in USD cents (null for Classic).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldGameMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_parental_children

> crate::models::ListWorldParentalChildrenResponse list_world_parental_children()
Accounts you are the linked parent of: limits (max rating, chat, monthly spend), spend this month, and the last 7 days of playtime per experience.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldParentalChildrenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_recent

> crate::models::ListWorldRecentResponse list_world_recent()
Continue playing: what you played in the last 90 days that is still open, newest first (from what the servers recorded).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldRecentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_regions

> crate::models::ListWorldRegionsResponse list_world_regions()
Live server regions with a probe URL each: GET it a few times, send the round trips as `pings` with the join to land in the nearest region.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldRegionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_shop_listings

> crate::models::ListWorldShopListingsResponse list_world_shop_listings()
Your live Crossly listings a game can show and sell (shop objects bind to their id).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldShopListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_transfer_requests

> crate::models::ListWorldTransferRequestsResponse list_world_transfer_requests()
Ownership transfer requests you were asked (incoming) or sent (outgoing), newest first (?direction&status&experienceId).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldTransferRequestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_transparency

> crate::models::ListWorldTransparencyResponse list_world_transparency()
Published monthly transparency reports, newest first: reports received and actions taken, by type (counts only).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldTransparencyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience

> crate::models::WorldExperienceResponse update_world_experience(experience_id)
Edit name, description, genre, rating or icon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_access_limit

> crate::models::GetWorldExperienceAccessLimitResponse update_world_experience_access_limit(experience_id)
Limit who can play: { crewOnly } (only members of the owner's crew), { countryMode: all|allow|block, countries: ['US', ...] }. The owner and team are never limited.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAccessLimitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_alert_setting

> crate::models::GetWorldExperienceAlertSettingResponse update_world_experience_alert_setting(experience_id)
Change alert rules and quiet hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAlertSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_chat_setting

> crate::models::GetWorldExperienceChatSettingResponse update_world_experience_chat_setting(experience_id)
Turn text chat off or use the strict filter for everyone (never looser than the platform).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceChatSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_config

> crate::models::WorldExperienceConfigResponse update_world_experience_config(experience_id)
Save the whole config document. Running servers get it within seconds (ConfigService.Changed). baseVersion guards against overwriting a newer save (409).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_config_value

> crate::models::WorldExperienceConfigResponse update_world_experience_config_value(experience_id)
Set and/or remove individual config keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_event

> crate::models::WorldExperienceEventResponse update_world_experience_event(experience_id, event_id)
Reschedule or change a scheduled event; a live event may only move its end.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**event_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_experiment

> crate::models::WorldExperienceExperimentResponse update_world_experience_experiment(experience_id, experiment_id)
Edit a draft (a running experiment may only rename or reword).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**experiment_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperimentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_maturity

> crate::models::UpdateWorldExperienceMaturityResponse update_world_experience_maturity(experience_id)
Answer the questionnaire: { answers }. A stricter rating applies now; a looser or first Mature rating waits for the next reviewed publish (takesEffect).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceMaturityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place

> crate::models::WorldExperiencePlaceResponse update_world_experience_place(experience_id, place_id)
Place settings: name, max players (1-50), fill target, social slots, empty shutdown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperiencePlaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place_draft

> crate::models::GetWorldExperiencePlaceDraftResponse update_world_experience_place_draft(experience_id, place_id)
Save the draft. baseRevision is the revision you loaded (0 for none); a newer save elsewhere answers 409 draft_conflict.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place_version_note

> crate::models::UpdateWorldExperiencePlaceVersionNoteResponse update_world_experience_place_version_note(experience_id, place_id, version_id)
Edit a release version's changelog (the game page's What's new): { notes } up to 2000 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperiencePlaceVersionNoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_publish

> crate::models::WorldExperienceResponse2 update_world_experience_publish(experience_id, request_id)
Steps 2–5: details, rating questionnaire answers, access (private | friends | unlisted | public), money review acknowledgement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**request_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceResponse2**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_showcase

> crate::models::UpdateWorldExperienceShowcaseResponse update_world_experience_showcase(experience_id)
Set the showcased listings: { listingIds } (Crossly shop slugs of the owner's live listings, max 12, in order).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceShowcaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_favorite

> crate::models::WorldResponse3 update_world_favorite()
Favourite or un-favourite an experience (idempotent).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_mature

> crate::models::GetWorldMatureResponse update_world_mature()
Switch \"Show Mature experiences\" on or off: { showMature }. On is refused while a final step blocks (too young, region, parental controls, safety hold); it never skips the ID check.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldMatureResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_me_pref

> crate::models::GetWorldMePrefResponse update_world_me_pref()
Set your Crossly Carnival choices.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldMePrefResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_parental_children

> crate::models::WorldParentalResponse update_world_parental_children(child_id)
Change a linked account's limits with your PIN: { pin, settings: { maxRating: everyone|teen, chat: on|friends|off, monthlySpendLimitCents (0 = no purchases, null = no limit) } }. Enforced server-side at join, chat, the purchase prompt and checkout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_id** | **String** |  | [required] |

### Return type

**crate::models::WorldParentalResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_vote

> crate::models::WorldResponse3 update_world_vote()
Thumbs up, down, or null to withdraw: one vote per player per experience, counted only after 10 minutes of play (403 vote_needs_playtime before). A light ranking signal.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::WorldResponse3**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

