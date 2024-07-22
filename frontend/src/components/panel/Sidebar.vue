<script setup>
import { Code } from "lucide-vue-next";
import { cn } from "@/lib/utils";
import { useAdminPanel } from "@/utils/useAdminPanel";
import Menu from "@/components/panel/Menu.vue";
import SidebarToggle from "@/components/panel/SidebarToggle.vue";
import { Button } from "@/components/ui/button";
import { routes } from "@/router/routes";
import { config } from "@/lib/config";

const { isOpen, setOpen } = useAdminPanel();
</script>

<template>
  <aside
    :class="
      cn(
        'fixed top-0 left-0 z-20 h-screen -translate-x-full lg:translate-x-0 transition-[width] ease-in-out duration-300',
        isOpen ? 'w-72' : 'w-[90px]'
      )
    "
  >
    <SidebarToggle :is-open="isOpen" :set-is-open="setOpen" />
    <div
      class="relative h-full flex flex-col px-3 py-4 overflow-y-auto shadow-md dark:shadow-zinc-800"
    >
      <Button variant="link" as-child>
        <router-link :to="{ name: routes.Home }">
          <Code class="w-6 h-6 mr-1" />
          <h1
            :class="
              cn(
                'font-bold text-lg whitespace-nowrap transition-[transform,opacity,display] ease-in-out duration-300',
                isOpen
                  ? 'translate-x-0 opacity-100'
                  : '-translate-x-96 opacity-0 hidden'
              )
            "
          >
            {{ config.SITE_NAME }}
          </h1>
        </router-link>
      </Button>
      <Menu :is-open="isOpen" />
    </div>
  </aside>
</template>
