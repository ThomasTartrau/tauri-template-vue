<script setup lang="ts">
import { useColorMode } from "@vueuse/core";
import {
  Notification,
  NotificationProgress,
  Notivue,
  darkTheme,
  lightTheme,
} from "notivue";
import { useAdminPanel } from "./utils/useAdminPanel";
import Sidebar from "./components/panel/Sidebar.vue";
import { cn } from "./lib/utils";
import ContentLayout from "./components/panel/ContentLayout.vue";
import { getAccessToken } from "@/iam";

const mode = useColorMode();
const notivueTheme = mode.value === "dark" ? darkTheme : lightTheme;
const is_logged_in = getAccessToken();
const { isOpen } = useAdminPanel();
</script>

<template>
  <Notivue v-slot="item">
    <Notification :theme="notivueTheme" :item="item">
      <NotificationProgress :item="item" />
    </Notification>
  </Notivue>

  <!-- If the user is logged in, show the app -->
  <div v-if="is_logged_in" class="flex min-h-screen w-full flex-col">
    <Sidebar />
    <main
      :class="
        cn(
          'min-h-screen bg-zinc-50 dark:bg-muted/30 transition-[margin-left] ease-in-out duration-300',
          !isOpen ? 'lg:ml-[90px]' : 'lg:ml-72',
        )
      "
    >
      <ContentLayout />
    </main>
  </div>

  <!-- If the user is not logged in, show the page without the header -->
  <router-view v-else />
</template>
