<template>
  <div class="app-shell">
    <div v-if="thumbnailStatusText || toastMessage" class="floating-status-stack" aria-live="polite" aria-atomic="true">
      <div v-if="fetchStatusText" class="floating-status">
        <span class="fetch-spinner" aria-hidden="true"></span>
        <span>{{ fetchStatusText }}</span>
      </div>
      <div v-if="thumbnailStatusText" class="floating-status">
        <span class="fetch-spinner" aria-hidden="true"></span>
        <span>{{ thumbnailStatusText }}</span>
      </div>
      <div v-if="toastMessage" class="floating-status floating-status-toast" :data-tone="toastTone">
        <span class="toast-indicator" aria-hidden="true"></span>
        <span>{{ toastMessage }}</span>
      </div>
    </div>
    <main class="layout">
      <section class="panel-wide main-shell">
        <section class="section-header">
          <div class="section-title">
            <p class="section-kicker">Avatar Changer</p>
            <div class="title-row">
              <h2>{{ visibleAvatarCount }} avatars</h2>
              <button
                class="ghost-button title-action-button"
                type="button"
                :disabled="!showAvatarBrowser || filteredAvatars.length === 0"
                @click="handleRandomAvatar"
              >
                Random
              </button>
              <button
                class="icon-button"
                type="button"
                :aria-label="sidebarOpen ? 'Close settings panel' : 'Open settings panel'"
                @click="sidebarOpen = !sidebarOpen"
              >
                <svg viewBox="0 0 24 24" class="icon-svg" aria-hidden="true">
                  <path
                    d="M10.9 2.4h2.2l.5 2.1c.5.1.9.3 1.4.6l1.9-1.1 1.6 1.6-1.1 1.9c.3.4.5.9.6 1.4l2.1.5v2.2l-2.1.5c-.1.5-.3 1-.6 1.4l1.1 1.9-1.6 1.6-1.9-1.1c-.4.3-.9.5-1.4.6l-.5 2.1h-2.2l-.5-2.1c-.5-.1-1-.3-1.4-.6l-1.9 1.1-1.6-1.6 1.1-1.9c-.3-.4-.5-.9-.6-1.4l-2.1-.5V9.8l2.1-.5c.1-.5.3-1 .6-1.4L5.7 6l1.6-1.6 1.9 1.1c.4-.3.9-.5 1.4-.6zM12 8.4A3.6 3.6 0 1 0 12 15.6 3.6 3.6 0 0 0 12 8.4z"
                  />
                </svg>
              </button>
            </div>
          </div>
          <div class="row">
            <button class="ghost-button" type="button" :disabled="!showAvatarBrowser || isFetching" @click="handleQuickRefresh">
              {{
                isFetching && fetchMode === "latest"
                  ? `Fetching Latest ${uiSettings.latestFetchCount}...`
                  : `Fetch Latest ${uiSettings.latestFetchCount}`
              }}
            </button>
            <button class="ghost-button" type="button" :disabled="!showAvatarBrowser || isFetching" @click="handleRefresh">
              {{ isFetching && fetchMode === "full" ? "Fetching All..." : "Fetch All Avatars" }}
            </button>
          </div>
        </section>

        <SearchToolbar
          v-if="showAvatarBrowser"
          v-model:query="searchQuery"
          :saved-search-terms="savedSearchTerms"
          :active-search-terms="activeSearchTerms"
          :show-tags="uiSettings.tagsEnabled"
          :multi-tags="availableMultiTags"
          :active-multi-tags="activeMultiTags"
          :multi-tag-mode="multiTagMode"
          @add-search-term="handleAddSearchTerm"
          @toggle-search-term="handleToggleSearchTerm"
          @remove-search-term="handleRemoveSearchTerm"
          @toggle-multi-tag="handleToggleMultiTag"
          @set-multi-tag-mode="handleSetMultiTagMode"
        />
        <p v-if="!showAvatarBrowser" class="muted">Sign in to load and browse your VRChat avatars.</p>
        <AvatarGrid
          v-if="showAvatarBrowser"
          :avatars="filteredAvatars"
          :show-tags="uiSettings.tagsEnabled"
          :show-switch-button="uiSettings.switchButtonsEnabled"
          :favorite-avatar-ids="favoriteAvatarIds"
          @select-avatar="selectedAvatarId = $event"
          @switch-avatar="handleSwitchAvatar"
          @toggle-favorite="toggleFavoriteAvatar"
          @visible-avatar-ids="visibleAvatarIds = $event"
        />
        <AvatarDialog
          :open="selectedAvatar !== null"
          :avatar="selectedAvatar"
          :tags-enabled="uiSettings.tagsEnabled"
          :tag-suggestions="availableTagSuggestions"
          :is-favorite="selectedAvatar ? favoriteAvatarIdSet.has(selectedAvatar.id) : false"
          @close="selectedAvatarId = null"
          @refresh-avatar="applyAvatarPayload"
          @save-tags="handleSaveTags"
          @switch-avatar="handleDialogSwitchAvatar"
          @toggle-favorite="toggleFavoriteAvatar"
        />
      </section>

      <aside class="sidebar-panel" :class="{ 'sidebar-panel-open': sidebarOpen }">
        <section class="sidebar">
            <div class="sidebar-header">
              <button class="ghost-button sidebar-close-button" type="button" @click="sidebarOpen = false">
                <span class="sidebar-close-icon" aria-hidden="true">
                  <svg viewBox="0 0 20 20" fill="none">
                    <path
                      d="M11.75 5.5 7.25 10l4.5 4.5"
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="1.8"
                    />
                  </svg>
                </span>
                <span>Close Panel</span>
              </button>
            </div>
          <LoginCard
            :session="sessionState"
            @signed-in="handleSignedIn"
            @pending-two-factor="markPendingTwoFactor"
            @cleared="refreshSession"
          />
          <section class="card stack">
            <div>
              <p class="section-kicker">Favorites</p>
              <h3>{{ favoriteAvatarCount }} / {{ MAX_FAVORITE_AVATAR_COUNT }}</h3>
            </div>
          </section>
          <SettingsCard :switch-settings="switchSettings" :ui="uiSettings" :message="settingsMessage" @save="handleSaveSettings" />
        </section>
      </aside>
    </main>
  </div>
</template>

<script setup lang="ts">
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";

import AvatarGrid from "@/components/AvatarGrid.vue";
import AvatarDialog from "@/components/AvatarDialog.vue";
import LoginCard from "@/components/LoginCard.vue";
import SearchToolbar from "@/components/SearchToolbar.vue";
import SettingsCard from "@/components/SettingsCard.vue";
import {
  cacheAvatarThumbnails,
  clearSavedSession,
  loadCachedAvatarList,
  loadSwitchSettings,
  refreshLatestAvatarPage,
  refreshAvatarList,
  saveAvatarTags,
  saveSwitchSettings,
  switchAvatar,
  verifySession,
} from "@/lib/commands";
import type { AvatarCachePayload, AvatarSummary } from "@/lib/commands";
import { type MultiTagMode, splitTags } from "@/lib/tags";
import type { AvatarSwitchState, CacheState, SessionState, UiSettings } from "@/types";

const avatars = ref<AvatarSummary[]>([]);
const searchQuery = ref("");
const savedSearchTerms = ref<string[]>([]);
const activeSearchTerms = ref<string[]>([]);
const activeMultiTags = ref<string[]>([]);
const multiTagMode = ref<MultiTagMode>("all");
const sidebarOpen = ref(false);
const selectedAvatarId = ref<string | null>(null);
const SEARCH_TERMS_STORAGE_KEY = "avatar-changer.search-terms";
const UI_SETTINGS_STORAGE_KEY = "avatar-changer.ui-settings";
const FAVORITE_AVATAR_IDS_STORAGE_KEY = "avatar-changer.favorite-avatar-ids";
const MAX_FAVORITE_AVATAR_COUNT = 100;

const sessionState = ref<SessionState>({
  status: "signed_out",
  username: null,
  twoFactorRequired: false,
});

const cacheState = ref<CacheState>({
  itemCount: 0,
  lastSyncedAt: null,
});

const switchSettings = ref<AvatarSwitchState>({
  method: "osc",
  osc: {
    enabled: true,
    host: "127.0.0.1",
    port: 9000,
  },
});
const uiSettings = ref<UiSettings>({
  tagsEnabled: false,
  switchButtonsEnabled: false,
  latestFetchCount: 20,
});
const toastMessage = ref("");
const toastTone = ref<"info" | "success" | "error">("info");
const settingsMessage = ref("");
const isFetching = ref(false);
const fetchMode = ref<"latest" | "full" | null>(null);
const fetchProgress = ref<{ phase: "avatars" | "thumbnails"; fetched: number; total: number | null } | null>(null);
let unlistenFetchProgress: UnlistenFn | null = null;
let toastTimeout: number | null = null;
const thumbnailCacheInFlight = ref(false);
const pendingThumbnailAvatarIds = ref<string[]>([]);
const visibleAvatarIds = ref<string[]>([]);
const favoriteAvatarIds = ref<string[]>([]);

function createSignedOutSession(): SessionState {
  return {
    status: "signed_out",
    username: null,
    twoFactorRequired: false,
  };
}

function resetAvatarState() {
  avatars.value = [];
  cacheState.value = {
    itemCount: 0,
    lastSyncedAt: null,
  };
  selectedAvatarId.value = null;
  visibleAvatarIds.value = [];
  pendingThumbnailAvatarIds.value = [];
  thumbnailCacheInFlight.value = false;
}

function clearToast() {
  if (toastTimeout !== null) {
    window.clearTimeout(toastTimeout);
    toastTimeout = null;
  }

  toastMessage.value = "";
}

function showToast(message: string, tone: "info" | "success" | "error" = "info", duration = 3600) {
  clearToast();
  toastTone.value = tone;
  toastMessage.value = message;
  if (duration <= 0) {
    return;
  }

  toastTimeout = window.setTimeout(() => {
    toastMessage.value = "";
    toastTimeout = null;
  }, duration);
}

function isUnauthorizedError(error: unknown) {
  if (typeof error === "string") {
    return error.includes("unauthorized") || error.includes("401");
  }

  if (error instanceof Error) {
    return error.message.includes("unauthorized") || error.message.includes("401");
  }

  return false;
}

async function signOutExpiredSession() {
  try {
    await clearSavedSession();
  } catch {
  } finally {
    sessionState.value = createSignedOutSession();
  }
}

const filteredAvatars = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const requiredTerms = [query, ...activeSearchTerms.value.map((term) => term.trim().toLowerCase())].filter(Boolean);
  return avatars.value
    .filter((avatar) => {
      const text = `${avatar.name} ${avatar.description} ${avatar.tags.join(" ")}`.toLowerCase();
      const matchesQuery = requiredTerms.length === 0 || requiredTerms.every((term) => text.includes(term));
      const { multiTags } = splitTags(avatar.tags);
      const matchesMulti =
        activeMultiTags.value.length === 0 ||
        (multiTagMode.value === "all"
          ? activeMultiTags.value.every((tag) => multiTags.includes(tag))
          : activeMultiTags.value.some((tag) => multiTags.includes(tag)));
      return matchesQuery && matchesMulti;
    })
    .sort((left, right) => {
      const leftFavorite = favoriteAvatarIdSet.value.has(left.id) ? 1 : 0;
      const rightFavorite = favoriteAvatarIdSet.value.has(right.id) ? 1 : 0;
      return rightFavorite - leftFavorite;
    });
});

const favoriteAvatarIdSet = computed(() => new Set(favoriteAvatarIds.value));
const favoriteAvatarCount = computed(() => favoriteAvatarIds.value.length);

const availableMultiTags = computed(() => {
  return [...new Set([...avatars.value.flatMap((avatar) => splitTags(avatar.tags).multiTags), ...activeMultiTags.value])].sort(
    (left, right) => left.localeCompare(right),
  );
});

const availableTagSuggestions = computed(() => {
  return [...new Set(avatars.value.flatMap((avatar) => splitTags(avatar.tags).multiTags))].sort((left, right) =>
    left.localeCompare(right),
  );
});

const selectedAvatar = computed(() => {
  if (!selectedAvatarId.value) {
    return null;
  }

  return avatars.value.find((avatar) => avatar.id === selectedAvatarId.value) ?? null;
});

const fetchStatusText = computed(() => {
  if (fetchMode.value === "latest") {
    return `Fetching the latest ${uiSettings.value.latestFetchCount} avatars...`;
  }

  if (fetchMode.value !== "full") {
    return "";
  }

  if (fetchProgress.value?.total != null) {
    return `Fetching avatars... ${fetchProgress.value.fetched} / ${fetchProgress.value.total}`;
  }

  if ((fetchProgress.value?.fetched ?? 0) > 0) {
    return `Fetching avatars... ${fetchProgress.value?.fetched} fetched`;
  }

  return "Fetching all avatars...";
});

const thumbnailStatusText = computed(() => {
  if (fetchProgress.value?.phase !== "thumbnails") {
    return "";
  }

  if (fetchProgress.value.total != null) {
    return isFetching.value
      ? `Refreshing latest thumbnails... ${fetchProgress.value.fetched} / ${fetchProgress.value.total}`
      : `Caching visible thumbnails... ${fetchProgress.value.fetched} / ${fetchProgress.value.total}`;
  }

  return isFetching.value ? "Refreshing latest thumbnails..." : "Caching visible thumbnails...";
});

const showAvatarBrowser = computed(() => sessionState.value.status === "authenticated");

const visibleAvatarCount = computed(() => (showAvatarBrowser.value ? cacheState.value.itemCount : 0));

const prioritizedThumbnailAvatarIds = computed(() => {
  const ids = visibleAvatarIds.value.filter((avatarId) => {
    const avatar = filteredAvatars.value.find((item) => item.id === avatarId);
    return avatar && !avatar.thumbnailPath;
  });

  if (selectedAvatar.value && !selectedAvatar.value.thumbnailPath && !ids.includes(selectedAvatar.value.id)) {
    ids.unshift(selectedAvatar.value.id);
  }

  return ids;
});

async function loadAvatars() {
  if (sessionState.value.status !== "authenticated") {
    return;
  }

  try {
    applyAvatarPayload(await refreshAvatarList());
  } catch (error) {
    if (isUnauthorizedError(error)) {
      await signOutExpiredSession();
    }
  }
}

function applyAvatarPayload(payload: AvatarCachePayload) {
  avatars.value = payload.avatars;
  cacheState.value = {
    itemCount: payload.avatars.length,
    lastSyncedAt: payload.lastSyncedAt,
  };
}

async function processThumbnailCacheQueue() {
  if (thumbnailCacheInFlight.value || pendingThumbnailAvatarIds.value.length === 0) {
    return;
  }

  thumbnailCacheInFlight.value = true;

  try {
    while (pendingThumbnailAvatarIds.value.length > 0) {
      const avatarIds = [...pendingThumbnailAvatarIds.value];
      pendingThumbnailAvatarIds.value = [];
      applyAvatarPayload(await cacheAvatarThumbnails(avatarIds));
    }
  } catch {
  } finally {
    thumbnailCacheInFlight.value = false;
    if (fetchProgress.value?.phase === "thumbnails") {
      fetchProgress.value = null;
    }
  }
}

function queueThumbnailCaching(avatarIds: string[]) {
  if (sessionState.value.status !== "authenticated" || avatarIds.length === 0) {
    return;
  }

  const merged = new Set([...pendingThumbnailAvatarIds.value, ...avatarIds]);
  pendingThumbnailAvatarIds.value = [...merged];
  void processThumbnailCacheQueue();
}

async function loadCachedAvatarsForSession() {
  if (!showAvatarBrowser.value) {
    resetAvatarState();
    return;
  }

  try {
    const payload = await loadCachedAvatarList();
    if (payload.ownerUsername && payload.ownerUsername !== sessionState.value.username) {
      resetAvatarState();
      return;
    }

    applyAvatarPayload(payload);
  } catch {
    resetAvatarState();
  }
}

async function refreshSession() {
  try {
    sessionState.value = await verifySession();
  } catch {
    sessionState.value = createSignedOutSession();
  }

  if (!showAvatarBrowser.value) {
    resetAvatarState();
    return;
  }

  await loadCachedAvatarsForSession();
}

function markPendingTwoFactor() {
  sessionState.value = {
    status: "pending_2fa",
    username: sessionState.value.username,
    twoFactorRequired: true,
  };
}

onMounted(() => {
  void listen<{ phase: "avatars" | "thumbnails"; fetched: number; total: number | null }>("avatar-fetch-progress", (event) => {
    fetchProgress.value = event.payload;
    if (
      event.payload.phase === "thumbnails" &&
      event.payload.total != null &&
      event.payload.fetched >= event.payload.total
    ) {
      window.setTimeout(() => {
        if (
          fetchProgress.value?.phase === "thumbnails" &&
          fetchProgress.value.total != null &&
          fetchProgress.value.fetched >= fetchProgress.value.total
        ) {
          fetchProgress.value = null;
        }
      }, 600);
    }
  }).then((unlisten) => {
    unlistenFetchProgress = unlisten;
  });
  try {
    const storedTerms = window.localStorage.getItem(SEARCH_TERMS_STORAGE_KEY);
    savedSearchTerms.value = storedTerms ? JSON.parse(storedTerms) : [];
  } catch {
    savedSearchTerms.value = [];
  }
  try {
    const storedUiSettings = window.localStorage.getItem(UI_SETTINGS_STORAGE_KEY);
    uiSettings.value = storedUiSettings ? { ...uiSettings.value, ...JSON.parse(storedUiSettings) } : uiSettings.value;
  } catch {
    uiSettings.value = {
      tagsEnabled: false,
      switchButtonsEnabled: false,
      latestFetchCount: 20,
    };
  }
  try {
    const storedFavoriteAvatarIds = window.localStorage.getItem(FAVORITE_AVATAR_IDS_STORAGE_KEY);
    favoriteAvatarIds.value = storedFavoriteAvatarIds ? JSON.parse(storedFavoriteAvatarIds).slice(0, MAX_FAVORITE_AVATAR_COUNT) : [];
  } catch {
    favoriteAvatarIds.value = [];
  }
  void loadSwitchSettings()
    .then((settings) => {
      switchSettings.value = settings;
    })
    .catch(() => {});
  void refreshSession();
});

onBeforeUnmount(() => {
  clearToast();
  if (!unlistenFetchProgress) {
    return;
  }

  void unlistenFetchProgress();
  unlistenFetchProgress = null;
});

watch(
  savedSearchTerms,
  (terms) => {
    window.localStorage.setItem(SEARCH_TERMS_STORAGE_KEY, JSON.stringify(terms));
    activeSearchTerms.value = activeSearchTerms.value.filter((term) => terms.includes(term));
  },
  { deep: true },
);

watch(
  uiSettings,
  (nextUiSettings) => {
    window.localStorage.setItem(UI_SETTINGS_STORAGE_KEY, JSON.stringify(nextUiSettings));
    if (!nextUiSettings.tagsEnabled) {
      activeMultiTags.value = [];
    }
  },
  { deep: true },
);

watch(
  favoriteAvatarIds,
  (nextFavoriteAvatarIds) => {
    window.localStorage.setItem(FAVORITE_AVATAR_IDS_STORAGE_KEY, JSON.stringify(nextFavoriteAvatarIds));
  },
  { deep: true },
);

watch(
  prioritizedThumbnailAvatarIds,
  (avatarIds) => {
    queueThumbnailCaching(avatarIds);
  },
  { immediate: true },
);

async function handleRefresh() {
  if (isFetching.value) {
    return;
  }

  isFetching.value = true;
  fetchMode.value = "full";
  fetchProgress.value = {
    phase: "avatars",
    fetched: 0,
    total: null,
  };
  clearToast();

  try {
    await loadAvatars();
  } finally {
    isFetching.value = false;
    fetchMode.value = null;
    fetchProgress.value = null;
  }
}

async function handleQuickRefresh() {
  if (sessionState.value.status !== "authenticated" || isFetching.value) {
    return;
  }

  isFetching.value = true;
  fetchMode.value = "latest";
  fetchProgress.value = null;
  clearToast();

  try {
    applyAvatarPayload(await refreshLatestAvatarPage(uiSettings.value.latestFetchCount));
  } catch (error) {
    if (isUnauthorizedError(error)) {
      await signOutExpiredSession();
    }
  } finally {
    isFetching.value = false;
    fetchMode.value = null;
  }
}

async function handleSignedIn(cacheReset: boolean) {
  await refreshSession();
  if (cacheReset) {
    showToast("Signed in with a different account. Avatar list cache was reset.", "info");
  }
}

function handleRandomAvatar() {
  if (filteredAvatars.value.length === 0) {
    showToast("No avatars match the current search or tag filters.", "info");
    return;
  }

  const randomIndex = Math.floor(Math.random() * filteredAvatars.value.length);
  selectedAvatarId.value = filteredAvatars.value[randomIndex]?.id ?? null;
}

async function handleSaveTags(payload: { avatarId: string; tags: string[] }) {
  try {
    applyAvatarPayload(await saveAvatarTags(payload.avatarId, payload.tags));
  } catch {
  }
}

function toggleFavoriteAvatar(avatarId: string) {
  if (favoriteAvatarIdSet.value.has(avatarId)) {
    favoriteAvatarIds.value = favoriteAvatarIds.value.filter((id) => id !== avatarId);
    return;
  }

  if (favoriteAvatarIds.value.length >= MAX_FAVORITE_AVATAR_COUNT) {
    showToast(`Favorites are limited to ${MAX_FAVORITE_AVATAR_COUNT} avatars.`, "info");
    return;
  }

  favoriteAvatarIds.value = [...favoriteAvatarIds.value, avatarId];
}

function handleToggleMultiTag(tag: string) {
  activeMultiTags.value = activeMultiTags.value.includes(tag)
    ? activeMultiTags.value.filter((item) => item !== tag)
    : [...activeMultiTags.value, tag];
}

function handleSetMultiTagMode(mode: MultiTagMode) {
  multiTagMode.value = mode;
}

function handleAddSearchTerm(term: string) {
  const normalized = term.trim();
  if (normalized === "" || savedSearchTerms.value.includes(normalized)) {
    return;
  }

  savedSearchTerms.value = [...savedSearchTerms.value, normalized].sort((left, right) => left.localeCompare(right));
}

function handleToggleSearchTerm(term: string) {
  activeSearchTerms.value = activeSearchTerms.value.includes(term)
    ? activeSearchTerms.value.filter((item) => item !== term)
    : [...activeSearchTerms.value, term];
}

function handleRemoveSearchTerm(term: string) {
  savedSearchTerms.value = savedSearchTerms.value.filter((item) => item !== term);
}

async function handleSwitchAvatar(avatarId: string) {
  try {
    await switchAvatar(avatarId);
    showToast(
      switchSettings.value.method === "api"
        ? `Switched avatar via VRChat API: ${avatarId}`
        : `Sent /avatar/change for ${avatarId}`,
      "success",
    );
  } catch (error) {
    showToast(
      error instanceof Error
        ? error.message
        : switchSettings.value.method === "api"
          ? "Failed to switch avatar via VRChat API."
          : "Failed to send OSC message.",
      "error",
      4800,
    );
  }
}

async function handleDialogSwitchAvatar(avatarId: string) {
  await handleSwitchAvatar(avatarId);
  selectedAvatarId.value = null;
}

async function handleSaveSettings(payload: { switchSettings: AvatarSwitchState; ui: UiSettings }) {
  try {
    switchSettings.value = await saveSwitchSettings(payload.switchSettings);
    uiSettings.value = payload.ui;
    settingsMessage.value =
      switchSettings.value.method === "api"
        ? "Settings updated. Avatar switching will use the VRChat API."
        : `Settings updated. OSC target: ${switchSettings.value.osc.host}:${switchSettings.value.osc.port}`;
    sidebarOpen.value = false;
  } catch (error) {
    settingsMessage.value = error instanceof Error ? error.message : "Failed to save switch settings.";
  }
}
</script>
