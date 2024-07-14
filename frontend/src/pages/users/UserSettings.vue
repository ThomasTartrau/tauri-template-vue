<script setup lang="ts">
import { ref } from 'vue'

const tabs = ref([
  { name: 'General', component: 'general-settings' },
  { name: 'Security', component: 'security-settings' },
  { name: 'Delete account', component: 'delete-account' },
])

const currentTab = ref(tabs.value[0].component)
</script>

<template>
  <main
    class="flex flex-1 flex-col"
  >
    <div
      class="mx-auto grid w-full items-start gap-6 md:grid-cols-[180px_1fr] lg:grid-cols-[250px_1fr]"
    >
      <nav class="grid gap-4 text-sm text-muted-foreground">
        <a
          v-for="tab in tabs"
          :key="tab.name"
          href="#"
          class="font-semibold"
          :class="{ 'text-primary': currentTab === tab.component }"
          @click.prevent="currentTab = tab.component"
        >
          {{ tab.name }}
        </a>
      </nav>
      <div class="grid gap-6">
        <component :is="currentTab" />
      </div>
    </div>
  </main>
</template>
