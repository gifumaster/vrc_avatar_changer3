<template>
  <Teleport to="body">
    <div v-if="open" class="dialog-backdrop" @click.self="emit('close')">
      <div class="dialog-frame eye-height-dialog-frame">
        <button class="dialog-close-button" type="button" aria-label="Close eye height dialog" @click="emit('close')">
          ✕
        </button>
        <section class="dialog-card eye-height-dialog-card">
          <div class="eye-height-panel">
            <div class="eye-height-header">
              <div>
                <p class="section-kicker">OSC</p>
                <h3>Avatar Eye Height</h3>
                <p class="muted">Scroll to choose a value, then send it to `/avatar/eyeheight`.</p>
              </div>
              <div class="eye-height-value">{{ selectedEyeHeight.toFixed(2) }} m</div>
            </div>

            <div class="eye-height-picker-wrap">
              <button class="ghost-button eye-height-step-button" type="button" @click="stepEyeHeight(-1)">
                -0.01
              </button>
              <div class="eye-height-picker-frame" @wheel.prevent="handleEyeHeightWheel">
                <div class="eye-height-picker-list">
                  <button
                    v-for="value in eyeHeightOptions"
                    :key="value"
                    :ref="(element) => setEyeHeightOptionRef(value, element)"
                    class="eye-height-option"
                    :class="{ 'eye-height-option-active': value === selectedEyeHeight }"
                    type="button"
                    @click="selectEyeHeight(value)"
                  >
                    {{ value.toFixed(2) }} m
                  </button>
                </div>
              </div>
              <button class="ghost-button eye-height-step-button" type="button" @click="stepEyeHeight(1)">
                +0.01
              </button>
            </div>

            <div class="eye-height-actions">
              <button class="primary-button" type="button" @click="emit('send', selectedEyeHeight)">
                Send Eye Height
              </button>
              <p class="muted eye-height-note">Range: 1.00m to 2.00m in 0.01m steps.</p>
            </div>
          </div>
        </section>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from "vue";

const props = defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  close: [];
  send: [eyeHeightMeters: number];
}>();

const EYE_HEIGHT_MIN = 1.0;
const EYE_HEIGHT_MAX = 2.0;
const EYE_HEIGHT_STEP = 0.01;
const DEFAULT_EYE_HEIGHT = 1.44;
const eyeHeightOptions = Array.from(
  { length: Math.round((EYE_HEIGHT_MAX - EYE_HEIGHT_MIN) / EYE_HEIGHT_STEP) + 1 },
  (_, index) => Number((EYE_HEIGHT_MIN + index * EYE_HEIGHT_STEP).toFixed(2)),
);

const eyeHeightOptionRefs = new Map<number, HTMLElement>();
const selectedEyeHeight = ref(DEFAULT_EYE_HEIGHT);

watch(
  () => props.open,
  (open) => {
    if (!open) {
      return;
    }

    void scrollSelectedEyeHeightIntoView();
  },
);

function setEyeHeightOptionRef(value: number, element: Element | null) {
  if (element instanceof HTMLElement) {
    eyeHeightOptionRefs.set(value, element);
    return;
  }

  eyeHeightOptionRefs.delete(value);
}

function handleEyeHeightWheel(event: WheelEvent) {
  stepEyeHeight(event.deltaY > 0 ? 1 : -1);
}

function stepEyeHeight(direction: -1 | 1) {
  const currentIndex = eyeHeightOptions.indexOf(selectedEyeHeight.value);
  const nextIndex = Math.min(eyeHeightOptions.length - 1, Math.max(0, currentIndex + direction));
  selectedEyeHeight.value = eyeHeightOptions[nextIndex];
  void scrollSelectedEyeHeightIntoView();
}

function selectEyeHeight(value: number) {
  selectedEyeHeight.value = value;
  void scrollSelectedEyeHeightIntoView();
}

async function scrollSelectedEyeHeightIntoView() {
  await nextTick();
  eyeHeightOptionRefs.get(selectedEyeHeight.value)?.scrollIntoView({
    block: "center",
    behavior: "smooth",
  });
}
</script>

<style scoped>
.eye-height-dialog-frame {
  max-width: 38rem;
}

.eye-height-dialog-card {
  padding: 1.5rem;
}

.eye-height-panel {
  display: grid;
  gap: 1rem;
}

.eye-height-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
}

.eye-height-header h3 {
  margin: 0.2rem 0 0.35rem;
}

.eye-height-header .muted {
  margin: 0;
}

.eye-height-value {
  min-width: 6rem;
  padding: 0.6rem 0.85rem;
  border-radius: 999px;
  background: rgba(15, 23, 42, 0.78);
  color: #f8fafc;
  font-size: 1.2rem;
  font-weight: 700;
  text-align: center;
}

.eye-height-picker-wrap {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  gap: 0.75rem;
  align-items: center;
}

.eye-height-picker-frame {
  position: relative;
  height: 18rem;
  overflow: hidden;
  border: 1px solid rgba(148, 163, 184, 0.24);
  border-radius: 1rem;
  background: linear-gradient(180deg, rgba(15, 23, 42, 0.94) 0%, rgba(15, 23, 42, 0.7) 100%);
}

.eye-height-picker-frame::before,
.eye-height-picker-frame::after {
  content: "";
  position: absolute;
  inset-inline: 0;
  height: 2.5rem;
  pointer-events: none;
  z-index: 1;
}

.eye-height-picker-frame::before {
  top: 0;
  background: linear-gradient(180deg, rgba(15, 23, 42, 0.98), rgba(15, 23, 42, 0));
}

.eye-height-picker-frame::after {
  bottom: 0;
  background: linear-gradient(0deg, rgba(15, 23, 42, 0.98), rgba(15, 23, 42, 0));
}

.eye-height-picker-list {
  display: grid;
  gap: 0.4rem;
  height: 18rem;
  padding: 4.2rem 0.7rem;
  overflow-y: auto;
  scroll-snap-type: y proximity;
  scrollbar-width: thin;
}

.eye-height-option {
  width: 100%;
  padding: 0.85rem 1rem;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 0.9rem;
  background: rgba(30, 41, 59, 0.54);
  color: #cbd5e1;
  font: inherit;
  font-size: 1rem;
  text-align: center;
  cursor: pointer;
  scroll-snap-align: center;
  transition: transform 140ms ease, border-color 140ms ease, background 140ms ease, color 140ms ease;
}

.eye-height-option:hover {
  border-color: rgba(125, 211, 252, 0.42);
  color: #f8fafc;
}

.eye-height-option-active {
  border-color: rgba(56, 189, 248, 0.72);
  background: linear-gradient(180deg, rgba(14, 165, 233, 0.3), rgba(56, 189, 248, 0.16));
  color: #f8fafc;
  transform: scale(1.02);
}

.eye-height-step-button {
  min-width: 4.75rem;
  min-height: 3.25rem;
  font-weight: 700;
}

.eye-height-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
}

.eye-height-note {
  margin: 0;
}

@media (max-width: 720px) {
  .eye-height-header,
  .eye-height-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .eye-height-picker-wrap {
    grid-template-columns: 1fr;
  }

  .eye-height-step-button {
    width: 100%;
  }
}
</style>
