<template>
  <Teleport to="body">
    <div v-if="open && props.avatar" class="dialog-backdrop" @click.self="emit('close')">
      <section class="dialog-card">
        <div class="dialog-body-grid">
          <div class="dialog-side">
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

            <div class="dialog-summary">
              <h3>{{ props.avatar.name }}</h3>
              <p class="dialog-description">{{ props.avatar.description || "No description" }}</p>
              <div class="tag-section">
                <p class="tag-section-title">Current tags</p>
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
              </div>
            </div>
          </div>

          <div class="dialog-body">
            <section v-if="tagsEnabled" class="tag-editor">
              <div class="tag-editor-row">
                <input
                  v-model="draftTag"
                  class="text-input"
                  type="text"
                  placeholder="Add tag"
                  @keydown.enter.prevent="handleAddTag"
                />
                <button class="ghost-button" type="button" @click="handleAddTag()">Add Tag</button>
              </div>

              <section class="tag-candidate-panel">
                <div class="tag-candidate-header">
                  <p class="tag-section-title">Existing tags</p>
                  <span class="muted">{{ filteredTagSuggestions.length }} shown</span>
                </div>
                <div v-if="filteredTagSuggestions.length > 0" class="tag-cloud">
                  <button
                    v-for="tag in filteredTagSuggestions"
                    :key="tag"
                    class="tag-pill tag-pill-candidate"
                    type="button"
                    @click="handleAddTag(tag)"
                  >
                    {{ tag }}
                  </button>
                </div>
                <p v-else class="muted">No matching existing tags.</p>
              </section>
            </section>
          </div>
        </div>

        <div class="dialog-actions">
          <button class="ghost-button dialog-link" type="button" @click="handleOpenVrchat">
            VRChat
          </button>
          <button class="ghost-button" type="button" :disabled="busy" @click="handleRefreshAvatar">
            {{ busy ? "Refreshing..." : "Refresh" }}
          </button>
          <button class="primary-button" type="button" @click="emit('switch-avatar', props.avatar.id)">
            Switch
          </button>
          <button class="ghost-button" type="button" @click="emit('close')">Close</button>
        </div>

        <p v-if="errorMessage" class="error-text dialog-error">{{ errorMessage }}</p>
      </section>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";

import { openExternalUrl, refreshAvatarDetail } from "@/lib/commands";
import { addMultiTag, removeTag, splitTags } from "@/lib/tags";
import type { AvatarCachePayload, AvatarSummary } from "@/lib/commands";

const props = defineProps<{
  open: boolean;
  avatar: AvatarSummary | null;
  tagsEnabled: boolean;
  tagSuggestions: string[];
}>();

const emit = defineEmits<{
  close: [];
  "switch-avatar": [avatarId: string];
  "save-tags": [payload: { avatarId: string; tags: string[] }];
  "refresh-avatar": [payload: AvatarCachePayload];
}>();

const draftTag = ref("");
const editableTags = ref<string[]>([]);
const busy = ref(false);
const errorMessage = ref("");
const avatarTags = computed(() => props.avatar?.tags ?? []);

const thumbnailSrc = computed(() => {
  if (!props.avatar) {
    return null;
  }

  if (props.avatar.thumbnailPath) {
    const version = props.avatar.thumbnailVersion ?? 0;
    return `${convertFileSrc(props.avatar.thumbnailPath)}?v=${version}`;
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
const filteredTagSuggestions = computed(() => props.tagSuggestions.filter((tag) => !editableTags.value.includes(tag)).slice(0, 30));

watch(
  () => [props.open, props.avatar?.id, props.avatar?.tags] as const,
  (current, previous) => {
    const [open, avatarId] = current;
    const previousOpen = previous?.[0] ?? false;
    const previousAvatarId = previous?.[1] ?? null;

    resetEditorState();
  },
  { immediate: true },
);

async function handleOpenVrchat() {
  if (!props.avatar) {
    return;
  }

  await openExternalUrl(vrchatUrl.value);
}

async function handleRefreshAvatar() {
  if (!props.avatar || busy.value) {
    return;
  }

  busy.value = true;
  errorMessage.value = "";

  try {
    emit("refresh-avatar", await refreshAvatarDetail(props.avatar.id));
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : "Failed to refresh avatar.";
  } finally {
    busy.value = false;
  }
}

function handleAddTag(tag = draftTag.value) {
  const nextTags = addMultiTag(editableTags.value, tag);
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

function resetEditorState() {
  editableTags.value = [...avatarTags.value];
  draftTag.value = "";
  errorMessage.value = "";
}
</script>
