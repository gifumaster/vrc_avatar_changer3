<template>
  <section class="card stack">
    <div>
      <p class="section-kicker">Settings</p>
    </div>

    <label>
      <span class="label">OSC Host</span>
      <input v-model.trim="localHost" class="text-input" type="text" placeholder="127.0.0.1" />
    </label>
    <label>
      <span class="label">OSC Port</span>
      <input v-model.number="localPort" class="text-input" type="number" min="1" max="65535" />
    </label>
    <label class="row">
      <span>OSC Enabled</span>
      <input v-model="localEnabled" type="checkbox" />
    </label>
    <label class="row">
      <span>タグ機能を使う</span>
      <input v-model="localTagsEnabled" type="checkbox" />
    </label>
    <label class="row">
      <span>Switchボタンを表示</span>
      <input v-model="localSwitchButtonsEnabled" type="checkbox" />
    </label>
    <label>
      <span class="label">最新取得件数</span>
      <select v-model.number="localLatestFetchCount" class="text-input">
        <option :value="20">20</option>
        <option :value="50">50</option>
        <option :value="100">100</option>
      </select>
    </label>

    <div class="card-actions">
      <button class="primary-button" type="button" :disabled="busy" @click="handleSave">
        {{ busy ? "Saving..." : "Save Settings" }}
      </button>
    </div>
    <p v-if="props.message" class="muted">{{ props.message }}</p>
  </section>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

import type { OscState, UiSettings } from "@/types";

const props = defineProps<{
  osc: OscState;
  ui: UiSettings;
  message?: string;
}>();

const emit = defineEmits<{
  save: [payload: { osc: OscState; ui: UiSettings }];
}>();

const localEnabled = ref(props.osc.enabled);
const localHost = ref(props.osc.host);
const localPort = ref(props.osc.port);
const localTagsEnabled = ref(props.ui.tagsEnabled);
const localSwitchButtonsEnabled = ref(props.ui.switchButtonsEnabled);
const localLatestFetchCount = ref<20 | 50 | 100>(props.ui.latestFetchCount);
const busy = ref(false);

watch(
  () => props.osc,
  (osc) => {
    localEnabled.value = osc.enabled;
    localHost.value = osc.host;
    localPort.value = osc.port;
  },
  { deep: true },
);

watch(
  () => props.ui,
  (ui) => {
    localTagsEnabled.value = ui.tagsEnabled;
    localSwitchButtonsEnabled.value = ui.switchButtonsEnabled;
    localLatestFetchCount.value = ui.latestFetchCount;
  },
  { deep: true },
);

function handleSave() {
  busy.value = true;
  const normalizedHost = localHost.value.trim() === "" ? "127.0.0.1" : localHost.value.trim();
  const normalizedPort = Number(localPort.value) || 9000;

  emit("save", {
    osc: {
      enabled: localEnabled.value,
      host: normalizedHost,
      port: normalizedPort,
    },
    ui: {
      tagsEnabled: localTagsEnabled.value,
      switchButtonsEnabled: localSwitchButtonsEnabled.value,
      latestFetchCount: localLatestFetchCount.value,
    },
  });

  localHost.value = normalizedHost;
  localPort.value = normalizedPort;
  busy.value = false;
}
</script>
