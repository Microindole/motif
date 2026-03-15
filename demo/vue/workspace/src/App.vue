<script setup lang="ts">
import FluentWorkspaceSection from "./sections/FluentWorkspaceSection.vue";
import MaterialWorkspaceSection from "./sections/MaterialWorkspaceSection.vue";
</script>

<template>
  <main class="shell">
    <FluentWorkspaceSection />
    <MaterialWorkspaceSection />
  </main>
</template>

<style>
.shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
  padding: 32px;
  background: linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%);
}

.row {
  display: grid;
  gap: 10px;
}

.cluster {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.compact-cluster {
  gap: 8px;
}

.spinner-row {
  align-items: center;
}

.tall-skeleton {
  min-height: 4rem;
}

.entry {
  width: 100%;
  box-sizing: border-box;
}

.button-reset {
  cursor: pointer;
}
</style>
