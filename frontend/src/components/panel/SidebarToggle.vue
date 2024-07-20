<script lang="ts" setup>
import { ChevronLeft } from "lucide-vue-next";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

defineProps<{
  isOpen: boolean;
  setIsOpen?: () => void;
}>();
</script>

<template>
  <div class="invisible lg:visible absolute top-[12px] -right-[16px] z-20">
    <Button
      class="rounded-md w-8 h-8"
      variant="outline"
      size="icon"
      @click="setIsOpen"
    >
      <ChevronLeft
        :class="
          cn(
            'h-4 w-4 transition-transform ease-in-out duration-700',
            !isOpen ? 'rotate-180' : 'rotate-0',
          )
        "
      />
    </Button>
  </div>
</template>
