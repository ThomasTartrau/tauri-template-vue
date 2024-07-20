<script lang="ts" setup>
import { useColorMode } from "@vueuse/core";
import { Moon, Sun } from "lucide-vue-next";
import {
  Tooltip,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import Button from "@/components/ui/button/Button.vue";

const mode = useColorMode();

function handleThemeChange() {
  mode.value = mode.value !== "light" ? "light" : "dark";
}
</script>

<template>
  <TooltipProvider disable-hoverable-content>
    <Tooltip :delay-duration="100">
      <TooltipTrigger as-child>
        <Button
          class="rounded-full w-8 h-8 bg-background"
          variant="outline"
          size="icon"
          @click="handleThemeChange"
        >
          <Sun
            class="w-[1.2rem] h-[1.2rem] rotate-90 scale-0 transition-transform ease-in-out duration-500 dark:rotate-0 dark:scale-100"
          />
          <Moon
            class="absolute w-[1.2rem] h-[1.2rem] rotate-0 scale-100 transition-transform ease-in-out duration-500 dark:rotate-90 dark:scale-0"
          />
        </Button>
      </TooltipTrigger>
    </Tooltip>
  </TooltipProvider>
</template>
