<script setup lang="ts">

import { onMounted } from "vue";
import init, { start } from "../../pkg/wasm_test.js";
import { useWindowSize } from '@vueuse/core'

const { width, height } = useWindowSize()

onMounted(async () => {
  await init();
  start()
});
</script>

<template>
  <div style="overflow: hidden">
    <canvas id="canvas" :width="width" :height="height * 2"></canvas>
  </div>
</template>

<style scoped></style>
