<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { Ellipsis, LogOut } from "lucide-vue-next";
import CustomRouterLink from "../CustomRouterLink.vue";
import CollapseMenu from "./CollapseMenu.vue";
import { cn } from "@/lib/utils";
import { useMenu } from "@/utils/useMenu";
import { ScrollArea } from "@/components/ui/scroll-area";
import TooltipProvider from "@/components/ui/tooltip/TooltipProvider.vue";
import Tooltip from "@/components/ui/tooltip/Tooltip.vue";
import TooltipTrigger from "@/components/ui/tooltip/TooltipTrigger.vue";

import Button from "@/components/ui/button/Button.vue";
import TooltipContent from "@/components/ui/tooltip/TooltipContent.vue";
import { logout } from "@/iam";

defineProps<{
  isOpen: boolean;
}>();

const { menuList } = useMenu();
const { t } = useI18n({ useScope: "global" });
</script>

<template>
  <ScrollArea class="[&>div>div[style]]:!block">
    <nav class="mt-8 h-full w-full">
      <ul
        class="flex w-full flex-col min-h-[calc(100vh-48px-36px-16px-32px)] lg:min-h-[calc(100vh-32px-40px-32px)] items-start space-y-1 px-2"
      >
        <li
          v-for="({ groupLabel, menus }, index) in menuList"
          :key="index"
          :class="cn('w-full', groupLabel ? 'pt-5' : '')"
        >
          <template v-if="(isOpen && groupLabel) || isOpen === undefined">
            <p
              class="text-sm font-medium text-muted-foreground px-4 pb-2 max-w-[248px] truncate"
            >
              {{ groupLabel }}
            </p>
          </template>
          <template v-else-if="!isOpen && isOpen !== undefined && groupLabel">
            <TooltipProvider>
              <Tooltip :delay-duration="100">
                <TooltipTrigger class="w-full">
                  <div class="w-full flex justify-center items-center">
                    <Ellipsis />
                  </div>
                </TooltipTrigger>
                <TooltipContent side="right" align="start">
                  <p>{{ groupLabel }}</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </template>
          <template v-else>
            <p class="pb-2" />
          </template>
          <template
            v-for="({ route, label, icon, active, submenus }, m_index) in menus"
          >
            <template v-if="submenus.length === 0">
              <div :key="m_index" class="w-full">
                <TooltipProvider disable-hoverable-content>
                  <Tooltip :delay-duration="100">
                    <TooltipTrigger as-child>
                      <Button
                        :variant="active ? 'secondary' : 'ghost'"
                        class="w-full justify-start h-10 mb-1"
                        as-child
                      >
                        <CustomRouterLink :route="route">
                          <div
                            :class="
                              cn(
                                'flex justify-center items-center',
                                !isOpen ? '' : 'mr-4',
                              )
                            "
                          >
                            <component :is="icon" :size="18" />
                          </div>
                          <p
                            :class="
                              cn(
                                'max-w-[200px] flex justify-center items-center truncate',
                                !isOpen
                                  ? '-translate-x-96 opacity-0'
                                  : 'translate-x-0 opacity-100',
                              )
                            "
                          >
                            {{ label }}
                          </p>
                        </CustomRouterLink>
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent v-if="!isOpen" side="right">
                      {{ label }}
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
            </template>
            <template v-else>
              <div :key="m_index" class="w-full">
                <CollapseMenu
                  :icon="icon"
                  :label="label"
                  :active="active"
                  :submenus="submenus"
                />
              </div>
            </template>
          </template>
        </li>
        <li class="w-full grow flex items-end">
          <TooltipProvider disable-hoverable-content>
            <Tooltip :delay-duration="100">
              <TooltipTrigger as-child>
                <Button
                  variant="outline"
                  class="w-full justify-center h-10 mt-5"
                  @click="logout"
                >
                  <span
                    :class="
                      cn(
                        'flex justify-center items-center',
                        isOpen === false ? '' : 'mr-4',
                      )
                    "
                  >
                    <LogOut :size="18" />
                  </span>
                  <p
                    :class="
                      cn(
                        'whitespace-nowrap',
                        !isOpen ? 'opacity-0 hidden' : 'opacity-100',
                      )
                    "
                  >
                    {{ t("sidebar.sign_out_label") }}
                  </p>
                </Button>
              </TooltipTrigger>
              <TooltipContent v-if="!isOpen" side="right">
                {{ t("sidebar.sign_out_label") }}
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </li>
      </ul>
    </nav>
  </ScrollArea>
</template>
