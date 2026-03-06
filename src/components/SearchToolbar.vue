<template>
  <div class="toolbar">
    <input
      :value="query"
      class="search-input"
      type="search"
      placeholder="Search by name, description, or tags"
      @input="handleInput"
    />

    <div class="filter-stack">
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
import type { MultiTagMode } from "@/lib/tags";

defineProps<{
  query: string;
  multiTags: string[];
  activeMultiTags: string[];
  multiTagMode: MultiTagMode;
}>();

const emit = defineEmits<{
  "update:query": [value: string];
  "toggle-multi-tag": [tag: string];
  "set-multi-tag-mode": [mode: MultiTagMode];
}>();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:query", target.value);
}
</script>
