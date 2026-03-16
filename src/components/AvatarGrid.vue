<template>
  <div class="grid">
    <article
      v-for="avatar in avatars"
      :key="avatar.id"
      :ref="(element) => registerAvatarCard(avatar.id, element)"
      class="card avatar-card"
    >
      <div class="avatar-thumb avatar-clickable" @click="emit('select-avatar', avatar.id)">
        <img
          v-if="thumbnailSrc(avatar)"
          :src="thumbnailSrc(avatar) ?? undefined"
          :alt="avatar.name"
          class="avatar-image"
          referrerpolicy="no-referrer"
        />
        <span v-else>No thumbnail yet</span>
        <button
          v-if="showSwitchButton"
          class="primary-button avatar-thumb-action"
          type="button"
          @click.stop="emit('switch-avatar', avatar.id)"
        >
          Switch
        </button>
      </div>
      <div class="avatar-copy">
        <h3>{{ avatar.name }}</h3>
        <p v-if="showTags && avatar.tags.length > 0" class="meta">{{ avatar.tags.join(" / ") }}</p>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";
import { onBeforeUnmount, onMounted, watch } from "vue";

import type { AvatarSummary } from "@/lib/commands";

const props = defineProps<{
  avatars: AvatarSummary[];
  showTags: boolean;
  showSwitchButton: boolean;
}>();

const emit = defineEmits<{
  "select-avatar": [avatarId: string];
  "switch-avatar": [avatarId: string];
  "visible-avatar-ids": [avatarIds: string[]];
}>();

const avatarCardElements = new Map<string, Element>();
const visibleAvatarIds = new Set<string>();
let observer: IntersectionObserver | null = null;

function thumbnailSrc(avatar: AvatarSummary): string | null {
  if (avatar.thumbnailPath) {
    const version = avatar.thumbnailVersion ?? 0;
    return `${convertFileSrc(avatar.thumbnailPath)}?v=${version}`;
  }

  return avatar.thumbnailUrl;
}

function emitVisibleAvatarIds() {
  const orderedIds = props.avatars
    .map((avatar) => avatar.id)
    .filter((avatarId) => visibleAvatarIds.has(avatarId));
  emit("visible-avatar-ids", orderedIds);
}

function registerAvatarCard(avatarId: string, element: Element | null) {
  const previousElement = avatarCardElements.get(avatarId);
  if (previousElement && observer) {
    observer.unobserve(previousElement);
  }

  if (!element) {
    avatarCardElements.delete(avatarId);
    visibleAvatarIds.delete(avatarId);
    emitVisibleAvatarIds();
    return;
  }

  avatarCardElements.set(avatarId, element);
  observer?.observe(element);
}

function rebuildObserver() {
  observer?.disconnect();
  visibleAvatarIds.clear();

  observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        const avatarId = (entry.target as HTMLElement).dataset.avatarId;
        if (!avatarId) {
          continue;
        }

        if (entry.isIntersecting) {
          visibleAvatarIds.add(avatarId);
        } else {
          visibleAvatarIds.delete(avatarId);
        }
      }

      emitVisibleAvatarIds();
    },
    {
      root: null,
      rootMargin: "240px 0px",
      threshold: 0.01,
    },
  );

  for (const [avatarId, element] of avatarCardElements) {
    (element as HTMLElement).dataset.avatarId = avatarId;
    observer.observe(element);
  }
}

onMounted(() => {
  rebuildObserver();
});

watch(
  () => props.avatars.map((avatar) => avatar.id),
  () => {
    rebuildObserver();
    emitVisibleAvatarIds();
  },
);

onBeforeUnmount(() => {
  observer?.disconnect();
  observer = null;
});
</script>
