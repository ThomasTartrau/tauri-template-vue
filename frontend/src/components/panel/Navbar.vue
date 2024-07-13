<script setup lang="ts">
import ColorModeToggle from '@/components/ColorModeToggle.vue';
import UserDropdown from '@/components/panel/UserDropdown.vue';
import SheetMenu from './SheetMenu.vue';
</script>

<template>
  <header class="sticky top-0 z-10 w-full bg-background/95 shadow backdrop-blur supports-[backdrop-filter]:bg-background/60 dark:shadow-secondary">
    <div class="mx-4 sm:mx-8 flex h-14 items-center">
      <div class="flex items-center space-x-4 lg:space-x-0">
        <SheetMenu />
        <h1 class="font-bold">
          Brand
        </h1>
      </div>
      <div class="flex flex-1 items-center space-x-2 justify-end">
        <ColorModeToggle />
        <UserDropdown />
      </div>
    </div>
  </header>
</template>

<style scoped>

</style>
