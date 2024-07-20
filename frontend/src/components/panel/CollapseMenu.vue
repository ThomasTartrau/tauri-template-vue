<script setup lang="ts">
import { DropdownMenuArrow } from "radix-vue";
import { ref } from "vue";
import { ChevronDown, Dot } from "lucide-vue-next";
import CustomRouterLink from "../CustomRouterLink.vue";
import { cn } from "@/lib/utils";
import type { CollapseMenuButtonProps } from "@/lib/menu";
import { useAdminPanel } from "@/utils/useAdminPanel";
import Collapsible from "@/components/ui/collapsible/Collapsible.vue";
import CollapsibleTrigger from "@/components/ui/collapsible/CollapsibleTrigger.vue";
import Button from "@/components/ui/button/Button.vue";
import CollapsibleContent from "@/components/ui/collapsible/CollapsibleContent.vue";
import { DropdownMenu } from "@/components/ui/dropdown-menu";
import DropdownMenuTrigger from "@/components/ui/dropdown-menu/DropdownMenuTrigger.vue";
import DropdownMenuContent from "@/components/ui/dropdown-menu/DropdownMenuContent.vue";
import DropdownMenuLabel from "@/components/ui/dropdown-menu/DropdownMenuLabel.vue";
import DropdownMenuSeparator from "@/components/ui/dropdown-menu/DropdownMenuSeparator.vue";
import DropdownMenuItem from "@/components/ui/dropdown-menu/DropdownMenuItem.vue";

const props = defineProps<CollapseMenuButtonProps>();

const isSubmenuActive = props.submenus.some((submenu) => submenu.active);
const isCollapsed = ref(isSubmenuActive);

const { isOpen } = useAdminPanel();
</script>

<template>
  <div>
    <Collapsible v-if="isOpen" v-model:open="isCollapsed" class="w-full">
      <CollapsibleTrigger
        class="[&[data-state=open]>div>div>svg]:rotate-180 mb-1"
        as-child
      >
        <Button
          :variant="active ? 'secondary' : 'ghost'"
          class="w-full items-center justify-start h-10"
        >
          <div class="w-full flex items-center justify-between">
            <div class="flex items-center justify-center">
              <span class="mr-4 flex justify-center items-center">
                <component :is="icon" :size="18" />
              </span>
              <p
                :class="
                  cn(
                    'max-w-[150px] flex justify-center items-center truncate',
                    isOpen
                      ? 'translate-x-0 opacity-100'
                      : '-translate-x-96 opacity-0',
                  )
                "
              >
                {{ label }}
              </p>
            </div>
            <div
              :class="
                cn(
                  'whitespace-nowrap flex justify-center items-center',
                  isOpen
                    ? 'translate-x-0 opacity-100'
                    : '-translate-x-96 opacity-0',
                )
              "
            >
              <ChevronDown
                class="transition-transform duration-200"
                :size="18"
              />
            </div>
          </div>
        </Button>
      </CollapsibleTrigger>
      <CollapsibleContent
        class="overflow-hidden data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down"
      >
        <Button
          v-for="({ route, label, active }, index) in submenus"
          :key="index"
          :variant="active ? 'secondary' : 'ghost'"
          class="w-full justify-start h-10 mb-1"
          as-child
        >
          <CustomRouterLink :route="route" class="flex items-center">
            <span class="mr-4 ml-2">
              <Dot :size="18" />
            </span>
            <p
              :class="
                cn(
                  'max-w-[170px] truncate',
                  isOpen
                    ? 'translate-x-0 opacity-100'
                    : '-translate-x-96 opacity-0',
                )
              "
            >
              {{ label }}
            </p>
          </CustomRouterLink>
        </Button>
      </CollapsibleContent>
    </Collapsible>
    <DropdownMenu v-else>
      <DropdownMenuTrigger as-child>
        <Button
          :variant="active ? 'secondary' : 'ghost'"
          class="w-full justify-start h-10 mb-1"
        >
          <div class="w-full items-center flex justify-between">
            <div class="flex justify-center items-center">
              <span
                :class="
                  cn('flex justify-center items-center', !isOpen ? '' : 'mr-4')
                "
              >
                <component :is="icon" :size="18" />
              </span>
              <p
                :class="
                  cn(
                    'max-w-[200px] truncate flex justify-center items-center',
                    !isOpen ? 'opacity-0' : 'opacity-100',
                  )
                "
              >
                {{ label }}
              </p>
            </div>
          </div>
        </Button>
      </DropdownMenuTrigger>
      <!-- <TooltipProvider disable-hoverable-content>
        <Tooltip :delay-duration="100">
          <TooltipTrigger as-child>

          </TooltipTrigger>
          <TooltipContent side="right" align="start" :align-offset="2">
            {{ label }}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider> -->
      <DropdownMenuContent side="right" :side-offset="25" align="start">
        <DropdownMenuLabel class="max-w-[190px] truncate">
          {{ label }}
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          v-for="({ route, label }, index) in submenus"
          :key="index"
          as-child
        >
          <CustomRouterLink class="cursor-pointer" :route="route">
            <p class="max-w-[180px] truncate">
              {{ label }}
            </p>
          </CustomRouterLink>
        </DropdownMenuItem>
        <DropdownMenuArrow class="fill-border" />
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>
