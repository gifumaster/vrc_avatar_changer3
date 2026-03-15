<template>
  <div class="toolbar">
    <div class="toolbar-top-row">
      <input
        :value="query"
        class="search-input"
        type="search"
        placeholder="Search by name, description, or tags"
        @input="handleInput"
      />
      <div class="toolbar-spacer" aria-hidden="true"></div>
      <div class="saved-search-form">
        <div class="saved-search-row">
          <input
            v-model.trim="newSearchTerm"
            class="text-input saved-search-input"
            type="text"
            placeholder="よく使う単語を登録"
            @keydown.enter.prevent="handleAddSearchTerm"
          />
          <button class="ghost-button" type="button" @click="handleAddSearchTerm">Add</button>
        </div>
      </div>
    </div>

    <div class="saved-search-chip-row">
      <div v-if="savedSearchTerms.length > 0" class="chip-row">
        <div
          v-for="term in savedSearchTerms"
          :key="term"
          class="saved-search-chip"
          :class="{ 'saved-search-chip-active': activeSearchTerms.includes(term) }"
        >
          <button class="chip saved-search-toggle" type="button" @click="emit('toggle-search-term', term)">
            {{ term }}
          </button>
          <button class="saved-search-remove" type="button" :aria-label="`Remove ${term}`" @click="emit('remove-search-term', term)">
            ×
          </button>
        </div>
      </div>
      <p v-else class="muted">No saved search words yet</p>
    </div>

    <div v-if="showTags" class="filter-stack">
      <div class="filter-group">
        <div class="filter-group-header">
          <span class="filter-label">Tags</span>
          <div class="mode-toggle">
            <button
              class="chip chip-small"
              :class="{ 'chip-active': multiTagMode === 'all' }"
              type="button"
              @click="emit('set-multi-tag-mode', 'all')"
            >
              All
            </button>
            <button
              class="chip chip-small"
              :class="{ 'chip-active': multiTagMode === 'any' }"
              type="button"
              @click="emit('set-multi-tag-mode', 'any')"
            >
              Any
            </button>
          </div>
        </div>
        <div v-if="multiTags.length > 0" class="chip-row">
          <button
            v-for="tag in multiTags"
            :key="tag"
            class="chip"
            :class="{ 'chip-active': activeMultiTags.includes(tag) }"
            type="button"
            @click="emit('toggle-multi-tag', tag)"
          >
            {{ tag }}
          </button>
        </div>
        <p v-else class="muted">No multi tags available</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

import type { MultiTagMode } from "@/lib/tags";

defineProps<{
  query: string;
  savedSearchTerms: string[];
  activeSearchTerms: string[];
  showTags: boolean;
  multiTags: string[];
  activeMultiTags: string[];
  multiTagMode: MultiTagMode;
}>();

const emit = defineEmits<{
  "update:query": [value: string];
  "add-search-term": [term: string];
  "toggle-search-term": [term: string];
  "remove-search-term": [term: string];
  "toggle-multi-tag": [tag: string];
  "set-multi-tag-mode": [mode: MultiTagMode];
}>();

const newSearchTerm = ref("");

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:query", target.value);
}

function handleAddSearchTerm() {
  if (newSearchTerm.value === "") {
    return;
  }

  emit("add-search-term", newSearchTerm.value);
  newSearchTerm.value = "";
}
</script>
