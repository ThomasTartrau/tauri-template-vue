<script setup lang="ts">
import { LayoutGrid, User } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { onMounted, onUpdated, ref } from "vue";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { UserInfo } from "@/iam";
import { emptyUserInfo, getUserInfo, logout } from "@/iam";
import CustomRouterLink from "@/components/CustomRouterLink.vue";

const userInfo: UserInfo = getUserInfo().value || emptyUserInfo;
const { t } = useI18n({ useScope: "global" });

const profile_picture = ref<string>("");

function _load() {
  profile_picture.value = `/profile-pictures/${userInfo.user_id}.jpeg`;
}

onMounted(() => {
  _load();
});
onUpdated(() => {
  _load();
});
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="relative h-8 w-8 rounded-full">
        <Avatar class="h-8 w-8">
          <AvatarImage :src="profile_picture" alt="Avatar" />
          <AvatarFallback class="bg-transparent">
            {{ userInfo.firstName.charAt(0).toUpperCase()
            }}{{ userInfo.lastName.charAt(0).toUpperCase() }}
          </AvatarFallback>
        </Avatar>
      </Button>
    </DropdownMenuTrigger>
    <!-- <TooltipProvider>
      <Tooltip>
        <TooltipTrigger as-child>

        </TooltipTrigger>
        <TooltipContent side="bottom">
          Profile
        </TooltipContent>
      </Tooltip>
    </TooltipProvider> -->
    <DropdownMenuContent class="w-56" align="end">
      <DropdownMenuLabel class="font-normal">
        <div class="flex flex-col space-y-1">
          <p class="text-sm font-medium leading-none">
            {{ userInfo.name }}
          </p>
          <p class="text-xs leading-none text-muted-foreground">
            {{ userInfo.email }}
          </p>
        </div>
      </DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuItem class="hover:cursor-pointer" as-child>
          <CustomRouterLink route="Home">
            <LayoutGrid class="w-4 h-4 mr-3 text-muted-foreground" />
            {{ t("navbar.dashboard_label") }}
          </CustomRouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem class="hover:cursor-pointer" as-child>
          <CustomRouterLink route="Settings">
            <User class="w-4 h-4 mr-3 text-muted-foreground" />
            {{ t("navbar.account_label") }}
          </CustomRouterLink>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuItem class="hover:cursor-pointer" @click="logout">
        <User class="w-4 h-4 mr-3 text-muted-foreground" />
        {{ t("sidebar.sign_out_label") }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
