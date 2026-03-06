<template>
  <div class="app-shell">
    <main class="layout">
      <section class="panel-wide main-shell">
        <section class="section-header">
          <div class="section-title">
            <p class="section-kicker">Avatar Changer</p>
            <div class="title-row">
              <h2>{{ cacheState.itemCount }} avatars</h2>
              <button
                class="ghost-button title-action-button"
                type="button"
                :disabled="filteredAvatars.length === 0"
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
            <button class="ghost-button" type="button" :disabled="isFetching" @click="handleQuickRefresh">
              {{ isFetching && fetchMode === "latest" ? "Fetching Latest..." : "Fetch Latest 20" }}
            </button>
            <button class="ghost-button" type="button" :disabled="isFetching" @click="handleRefresh">
              {{ isFetching && fetchMode === "full" ? "Fetching All..." : "Fetch All Avatars" }}
            </button>
          </div>
        </section>

        <SearchToolbar
          v-model:query="searchQuery"
          :multi-tags="availableMultiTags"
          :active-multi-tags="activeMultiTags"
          :multi-tag-mode="multiTagMode"
          @toggle-multi-tag="handleToggleMultiTag"
          @set-multi-tag-mode="handleSetMultiTagMode"
        />
        <div v-if="isFetching" class="fetch-status">
          <span class="fetch-spinner" aria-hidden="true"></span>
          <span>{{ fetchMode === "latest" ? "Fetching the latest 20 avatars..." : "Fetching all avatars..." }}</span>
        </div>
        <p v-if="oscMessage" class="muted">{{ oscMessage }}</p>
        <AvatarGrid
          :avatars="filteredAvatars"
          @select-avatar="selectedAvatarId = $event"
          @switch-avatar="handleSwitchAvatar"
        />
        <AvatarDialog
          :open="selectedAvatar !== null"
          :avatar="selectedAvatar"
          @close="selectedAvatarId = null"
          @save-tags="handleSaveTags"
          @switch-avatar="handleDialogSwitchAvatar"
        />
      </section>

      <aside class="sidebar-panel" :class="{ 'sidebar-panel-open': sidebarOpen }">
        <section class="sidebar">
          <div class="sidebar-header">
            <button class="ghost-button sidebar-close-button" type="button" @click="sidebarOpen = false">
              Close Panel
            </button>
          </div>
          <LoginCard
            :session="sessionState"
            @signed-in="refreshSession"
            @pending-two-factor="markPendingTwoFactor"
            @cleared="refreshSession"
          />
          <SettingsCard :osc="oscState" :message="settingsMessage" @save="handleSaveOscSettings" />
        </section>
      </aside>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

import AvatarGrid from "@/components/AvatarGrid.vue";
import AvatarDialog from "@/components/AvatarDialog.vue";
import LoginCard from "@/components/LoginCard.vue";
import SearchToolbar from "@/components/SearchToolbar.vue";
import SettingsCard from "@/components/SettingsCard.vue";
import {
  loadCachedAvatarList,
  loadOscSettings,
  refreshLatestAvatarPage,
  refreshAvatarList,
  saveAvatarTags,
  saveOscSettings,
  switchAvatarViaOsc,
  verifySession,
} from "@/lib/commands";
import type { AvatarCachePayload, AvatarSummary } from "@/lib/commands";
import { type MultiTagMode, splitTags } from "@/lib/tags";
import type { CacheState, OscState, SessionState } from "@/types";

const avatars = ref<AvatarSummary[]>([]);
const searchQuery = ref("");
const activeMultiTags = ref<string[]>([]);
const multiTagMode = ref<MultiTagMode>("all");
const sidebarOpen = ref(false);
const selectedAvatarId = ref<string | null>(null);

const sessionState = ref<SessionState>({
  status: "signed_out",
  username: null,
  twoFactorRequired: false,
});

const cacheState = ref<CacheState>({
  itemCount: 0,
  lastSyncedAt: null,
});

const oscState = ref<OscState>({
  enabled: true,
  host: "127.0.0.1",
  port: 9000,
});
const oscMessage = ref("");
const settingsMessage = ref("");
const isFetching = ref(false);
const fetchMode = ref<"latest" | "full" | null>(null);

const filteredAvatars = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  return avatars.value.filter((avatar) => {
    const text = `${avatar.name} ${avatar.description} ${avatar.tags.join(" ")}`.toLowerCase();
    const matchesQuery = query === "" || text.includes(query);
    const { multiTags } = splitTags(avatar.tags);
    const matchesMulti =
      activeMultiTags.value.length === 0 ||
      (multiTagMode.value === "all"
        ? activeMultiTags.value.every((tag) => multiTags.includes(tag))
        : activeMultiTags.value.some((tag) => multiTags.includes(tag)));
    return matchesQuery && matchesMulti;
  });
});

const availableMultiTags = computed(() => {
  return [...new Set([...avatars.value.flatMap((avatar) => splitTags(avatar.tags).multiTags), ...activeMultiTags.value])].sort(
    (left, right) => left.localeCompare(right),
  );
});

const selectedAvatar = computed(() => {
  if (!selectedAvatarId.value) {
    return null;
  }

  return avatars.value.find((avatar) => avatar.id === selectedAvatarId.value) ?? null;
});

async function loadAvatars() {
  if (sessionState.value.status !== "authenticated") {
    return;
  }

  try {
    applyAvatarPayload(await refreshAvatarList());
  } catch {
  }
}

function applyAvatarPayload(payload: AvatarCachePayload) {
  avatars.value = payload.avatars;
  cacheState.value = {
    itemCount: payload.avatars.length,
    lastSyncedAt: payload.lastSyncedAt,
  };
}

async function refreshSession() {
  try {
    sessionState.value = await verifySession();
  } catch {
    sessionState.value = {
      status: "signed_out",
      username: null,
      twoFactorRequired: false,
    };
  }
}

function markPendingTwoFactor() {
  sessionState.value = {
    status: "pending_2fa",
    username: sessionState.value.username,
    twoFactorRequired: true,
  };
}

onMounted(() => {
  void loadOscSettings()
    .then((settings) => {
      oscState.value = settings;
    })
    .catch(() => {});
  void loadCachedAvatarList()
    .then((payload) => {
      applyAvatarPayload(payload);
    })
    .catch(() => {
      avatars.value = [];
      cacheState.value = {
        itemCount: 0,
        lastSyncedAt: null,
      };
    });
  void refreshSession();
});

async function handleRefresh() {
  if (isFetching.value) {
    return;
  }

  isFetching.value = true;
  fetchMode.value = "full";
  oscMessage.value = "";

  try {
    await loadAvatars();
  } finally {
    isFetching.value = false;
    fetchMode.value = null;
  }
}

async function handleQuickRefresh() {
  if (sessionState.value.status !== "authenticated" || isFetching.value) {
    return;
  }

  isFetching.value = true;
  fetchMode.value = "latest";
  oscMessage.value = "";

  try {
    applyAvatarPayload(await refreshLatestAvatarPage());
  } catch {
  } finally {
    isFetching.value = false;
    fetchMode.value = null;
  }
}

function handleRandomAvatar() {
  if (filteredAvatars.value.length === 0) {
    oscMessage.value = "No avatars match the current search or tag filters.";
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

function handleToggleMultiTag(tag: string) {
  activeMultiTags.value = activeMultiTags.value.includes(tag)
    ? activeMultiTags.value.filter((item) => item !== tag)
    : [...activeMultiTags.value, tag];
}

function handleSetMultiTagMode(mode: MultiTagMode) {
  multiTagMode.value = mode;
}

async function handleSwitchAvatar(avatarId: string) {
  try {
    await switchAvatarViaOsc(avatarId);
    oscMessage.value = `Sent /avatar/change for ${avatarId}`;
  } catch (error) {
    oscMessage.value = error instanceof Error ? error.message : "Failed to send OSC message.";
  }
}

async function handleDialogSwitchAvatar(avatarId: string) {
  await handleSwitchAvatar(avatarId);
  selectedAvatarId.value = null;
}

async function handleSaveOscSettings(nextOsc: OscState) {
  try {
    oscState.value = await saveOscSettings(nextOsc);
    settingsMessage.value = `OSC target updated to ${oscState.value.host}:${oscState.value.port}`;
  } catch (error) {
    settingsMessage.value = error instanceof Error ? error.message : "Failed to save OSC settings.";
  }
}
</script>
