<script setup lang="ts">
import { Menu as MenuIcon, Code } from "lucide-vue-next";
import { routes } from "@/router/routes";
import { Button } from "@/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTrigger,
} from "@/components/ui/sheet";
import Menu from "@/components/panel/Menu.vue";
import { config } from "@/lib/config";
</script>

<template>
  <Sheet>
    <SheetTrigger class="lg:hidden" as-child>
      <Button class-name="h-8" variant="outline" size="icon">
        <MenuIcon :size="18" />
      </Button>
    </SheetTrigger>
    <SheetContent class="sm:w-72 px-3 h-full flex flex-col" side="left">
      <SheetHeader>
        <Button
          class="flex justify-center items-center pb-2 pt-1"
          variant="link"
          as-child
        >
          <router-link :to="{ name: routes.Home }">
            <Code class="w-6 h-6 mr-1" />
            <h1 class="font-bold text-lg">
              {{ config.SITE_NAME }}
            </h1>
          </router-link>
        </Button>
      </SheetHeader>
      <Menu is-open />
    </SheetContent>
  </Sheet>
</template>
