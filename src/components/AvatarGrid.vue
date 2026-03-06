<template>
  <div class="grid">
    <article v-for="avatar in avatars" :key="avatar.id" class="card avatar-card">
      <div class="avatar-thumb avatar-clickable" @click="emit('select-avatar', avatar.id)">
        <img
          v-if="thumbnailSrc(avatar)"
          :src="thumbnailSrc(avatar) ?? undefined"
          :alt="avatar.name"
          class="avatar-image"
          referrerpolicy="no-referrer"
        />
        <span v-else>No thumbnail yet</span>
      </div>
      <div class="avatar-copy">
        <h3>{{ avatar.name }}</h3>
        <p>{{ avatar.description }}</p>
        <p class="meta">{{ avatar.tags.length > 0 ? avatar.tags.join(" / ") : "No tags yet" }}</p>
        <button
          class="primary-button avatar-action"
          type="button"
          @click.stop="emit('switch-avatar', avatar.id)"
        >
          Switch
        </button>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/core";

import type { AvatarSummary } from "@/lib/commands";

defineProps<{
  avatars: AvatarSummary[];
}>();

const emit = defineEmits<{
  "select-avatar": [avatarId: string];
  "switch-avatar": [avatarId: string];
}>();

function thumbnailSrc(avatar: AvatarSummary): string | null {
  if (avatar.thumbnailPath) {
    return convertFileSrc(avatar.thumbnailPath);
  }

  return avatar.thumbnailUrl;
}
</script>
