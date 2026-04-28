<template>
  <Teleport to="body">
    <div v-if="open && props.avatar" class="dialog-backdrop" @click.self="emit('close')">
      <div class="dialog-frame">
        <button class="dialog-close-button" type="button" aria-label="Close dialog" @click="emit('close')">
          ✕
        </button>
        <section class="dialog-card">
          <div class="dialog-body-grid">
            <div class="dialog-side">
              <div class="dialog-image-wrap">
                <button
                  class="favorite-button favorite-button-detail"
                  :class="{ 'favorite-button-active': isFavorite }"
                  type="button"
                  :aria-label="isFavorite ? 'Remove from favorites' : 'Add to favorites'"
                  @click="emit('toggle-favorite', props.avatar.id)"
                >
                  ★
                </button>
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
            <div class="dialog-actions-group">
              <button class="ghost-button dialog-link" type="button" @click="handleOpenVrchat">
                VRChat
              </button>
              <button class="ghost-button" type="button" :disabled="uploadBusy || busy" @click="openImagePicker">
                {{ uploadBusy ? "アップロード中..." : "サムネイル画像を変更" }}
              </button>
              <button class="ghost-button" type="button" :disabled="busy" @click="handleRefreshAvatar">
                {{ busy ? "更新中..." : "サムネイル更新" }}
              </button>
            </div>
            <button class="primary-button" type="button" @click="emit('switch-avatar', props.avatar.id)">
              Switch
            </button>
          </div>

          <p v-if="errorMessage" class="error-text dialog-error">{{ errorMessage }}</p>
          <input
            ref="imageInputRef"
            type="file"
            accept="image/*"
            hidden
            @change="handleImageSelected"
          />
        </section>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";

import { openExternalUrl, refreshAvatarDetail, uploadAvatarImage } from "@/lib/commands";
import { extractBase64Payload, processAvatarImageFile } from "@/lib/avatarImage";
import { formatAvatarUploadError } from "@/lib/avatarUploadErrors";
import { addMultiTag, removeTag, splitTags } from "@/lib/tags";
import type { AvatarCachePayload, AvatarSummary } from "@/lib/commands";

const props = defineProps<{
  open: boolean;
  avatar: AvatarSummary | null;
  tagsEnabled: boolean;
  tagSuggestions: string[];
  isFavorite: boolean;
}>();

const emit = defineEmits<{
  close: [];
  "switch-avatar": [avatarId: string];
  "save-tags": [payload: { avatarId: string; tags: string[] }];
  "refresh-avatar": [payload: AvatarCachePayload];
  "toggle-favorite": [avatarId: string];
  notify: [payload: { message: string; tone: "success" | "error" | "info" }];
}>();

const draftTag = ref("");
const editableTags = ref<string[]>([]);
const busy = ref(false);
const uploadBusy = ref(false);
const errorMessage = ref("");
const imageInputRef = ref<HTMLInputElement | null>(null);
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
  () => {
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

function openImagePicker() {
  if (!props.avatar || uploadBusy.value) {
    return;
  }

  imageInputRef.value?.click();
}

async function handleImageSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  const [file] = Array.from(target.files ?? []);
  target.value = "";

  if (!props.avatar || !file) {
    return;
  }

  uploadBusy.value = true;
  errorMessage.value = "";

  try {
    const processedImage = await processAvatarImageFile(file);
    const payload = await uploadAvatarImage({
      avatarId: props.avatar.id,
      imageBase64: extractBase64Payload(processedImage),
    });
    emit("refresh-avatar", payload);
    emit("notify", {
      message: "サムネイル画像を更新しました。反映まで少し時間がかかる場合があります。",
      tone: "success",
    });
  } catch (error) {
    errorMessage.value = formatAvatarUploadError(error);
    emit("notify", {
      message: errorMessage.value,
      tone: "error",
    });
  } finally {
    uploadBusy.value = false;
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
  if (imageInputRef.value) {
    imageInputRef.value.value = "";
  }
}
</script>
