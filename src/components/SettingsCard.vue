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

    <div class="card-actions">
      <button class="primary-button" type="button" :disabled="busy" @click="handleSave">
        {{ busy ? "Saving..." : "Save OSC" }}
      </button>
    </div>
    <p v-if="props.message" class="muted">{{ props.message }}</p>
  </section>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

import type { OscState } from "@/types";

const props = defineProps<{
  osc: OscState;
  message?: string;
}>();

const emit = defineEmits<{
  save: [osc: OscState];
}>();

const localEnabled = ref(props.osc.enabled);
const localHost = ref(props.osc.host);
const localPort = ref(props.osc.port);
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

function handleSave() {
  busy.value = true;
  const normalizedHost = localHost.value.trim() === "" ? "127.0.0.1" : localHost.value.trim();
  const normalizedPort = Number(localPort.value) || 9000;

  emit("save", {
    enabled: localEnabled.value,
    host: normalizedHost,
    port: normalizedPort,
  });

  localHost.value = normalizedHost;
  localPort.value = normalizedPort;
  busy.value = false;
}
</script>
