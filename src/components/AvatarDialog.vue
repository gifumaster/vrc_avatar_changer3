<template>
  <Teleport to="body">
    <div v-if="open && props.avatar" class="dialog-backdrop" @click.self="emit('close')">
      <section class="dialog-card">
        <div class="dialog-image-wrap">
          <img
            v-if="thumbnailSrc"
            :src="thumbnailSrc"
            :alt="props.avatar.name"
            class="dialog-image"
            referrerpolicy="no-referrer"
          />
          <div v-else class="dialog-image dialog-image-empty">No thumbnail</div>
        </div>

        <div class="dialog-body">
          <h3>{{ props.avatar.name }}</h3>
          <p class="dialog-description">{{ props.avatar.description || "No description" }}</p>
          <section class="tag-editor">
            <div class="tag-editor-row">
              <input
                v-model="draftTag"
                class="text-input"
                type="text"
                placeholder="Add tag"
                @keydown.enter.prevent="handleAddTag"
              />
              <button class="ghost-button" type="button" @click="handleAddTag">Add Tag</button>
            </div>

            <div v-if="multiTags.length > 0" class="tag-pill-row">
              <button
                v-for="tag in multiTags"
                :key="tag"
                class="tag-pill"
                type="button"
                @click="handleRemoveTag(tag)"
              >
                {{ tag }} x
              </button>
            </div>
            <p v-else class="muted">No tags yet</p>
          </section>
        </div>

        <div class="dialog-actions">
          <button class="ghost-button dialog-link" type="button" @click="handleOpenVrchat">
            VRChat
          </button>
          <button class="primary-button" type="button" @click="emit('switch-avatar', props.avatar.id)">
            Switch
          </button>
          <button class="ghost-button" type="button" @click="emit('close')">Close</button>
        </div>
      </section>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";

import { openExternalUrl } from "@/lib/commands";
import { addMultiTag, removeTag, splitTags } from "@/lib/tags";
import type { AvatarSummary } from "@/lib/commands";

const props = defineProps<{
  open: boolean;
  avatar: AvatarSummary | null;
}>();

const emit = defineEmits<{
  close: [];
  "switch-avatar": [avatarId: string];
  "save-tags": [payload: { avatarId: string; tags: string[] }];
}>();

const draftTag = ref("");
const editableTags = ref<string[]>([]);

const thumbnailSrc = computed(() => {
  if (!props.avatar) {
    return null;
  }

  if (props.avatar.thumbnailPath) {
    return convertFileSrc(props.avatar.thumbnailPath);
  }

  return props.avatar.thumbnailUrl;
});

const vrchatUrl = computed(() => {
  if (!props.avatar) {
    return "#";
  }

  return `https://vrchat.com/home/avatar/${props.avatar.id}`;
});

const multiTags = computed(() => splitTags(editableTags.value).multiTags);

watch(
  () => [props.open, props.avatar?.id, props.avatar?.tags] as const,
  () => {
    editableTags.value = props.avatar?.tags ? [...props.avatar.tags] : [];
    draftTag.value = "";
  },
  { immediate: true },
);

async function handleOpenVrchat() {
  if (!props.avatar) {
    return;
  }

  await openExternalUrl(vrchatUrl.value);
}

function handleAddTag() {
  const nextTags = addMultiTag(editableTags.value, draftTag.value);
  if (nextTags === editableTags.value) {
    return;
  }

  editableTags.value = nextTags;
  draftTag.value = "";
  emitSaveTags();
}

function handleRemoveTag(tag: string) {
  editableTags.value = removeTag(editableTags.value, tag);
  emitSaveTags();
}

function emitSaveTags() {
  if (!props.avatar) {
    return;
  }

  emit("save-tags", {
    avatarId: props.avatar.id,
    tags: editableTags.value,
  });
}
</script>
