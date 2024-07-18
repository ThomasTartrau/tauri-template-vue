<script setup lang="ts">
import { onMounted, onUpdated, ref } from 'vue'
import { useRouter } from 'vue-router'

const tabs = ref([
  { name: 'General', component: 'general-settings', path: '/settings' },
  { name: 'Security', component: 'security-settings', path: '/settings/security' },
  { name: 'Delete account', component: 'delete-account', path: '/settings/delete-account' },
])

const route = useRouter()
const currentTab = ref<string>('general-settings')

function _onLoad() {
  const tab = tabs.value.find(tab => tab.path === route.currentRoute.value.path)
  if (tab) {
    currentTab.value = tab.component
  }
}
onMounted(() => {
  _onLoad()
})
onUpdated(() => {
  _onLoad()
})
</script>

<template>
  <main
    class="flex flex-1 flex-col"
  >
    <div
      class="mx-auto grid w-full items-start gap-6 md:grid-cols-[180px_1fr] lg:grid-cols-[250px_1fr]"
    >
      <nav class="grid gap-4 text-sm text-muted-foreground">
        <router-link
          v-for="tab in tabs"
          :key="tab.name"
          :to="tab.path"
          class="font-semibold"
          :class="{ 'text-primary': currentTab === tab.component }"
        >
          {{ tab.name }}
        </router-link>
      </nav>
      <div class="grid gap-6">
        <component :is="currentTab" />
      </div>
    </div>
  </main>
</template>
