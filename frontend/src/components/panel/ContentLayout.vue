<script setup lang="ts">
import Navbar from '@/components/panel/Navbar.vue'
</script>

<template>
  <div>
    <Navbar />
    <div class="container pt-8 pb-8 px-4 sm:px-8">
      <router-view />
    </div>
  </div>
</template>
