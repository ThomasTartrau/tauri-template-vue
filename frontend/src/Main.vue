<script setup lang="ts">
import { CircleUser, Menu, Book, Search } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { routes } from "@/router/routes.ts";
import { Icon } from "@iconify/vue";
import { useColorMode } from "@vueuse/core";
import {
  darkTheme,
  lightTheme,
  Notivue,
  Notification,
  NotificationProgress,
} from "notivue";
import { logout as doLogout, getAccessToken } from "@/iam";
import router from "./router/router";

const mode = useColorMode();
const notivueTheme = mode.value === "dark" ? darkTheme : lightTheme;
const is_logged_in = getAccessToken();

const navbar = [{ name: "Home", to: { name: routes.Home } }];

async function logout() {
  await doLogout();
}
</script>

<template>
  <Notivue v-slot="item">
    <Notification :theme="notivueTheme" :item="item">
      <NotificationProgress :item="item" />
    </Notification>
  </Notivue>

  <!-- If the user is logged in, show the app -->
  <div v-if="is_logged_in" class="flex min-h-screen w-full flex-col">
    <header
      class="sticky top-0 flex h-16 items-center gap-4 border-b bg-background px-4 md:px-6"
    >
      <nav
        class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6"
      >
        <router-link
          class="flex items-center gap-2 text-lg font-semibold md:text-base"
          :to="{ name: routes.Home }"
        >
          <Book class="h-6 w-6" />
          <span class="sr-only">MyTemplate</span>
        </router-link>
        <router-link
          v-for="item in navbar"
          :key="item.name"
          :to="item.to"
          class="text-foreground transition-colors hover:text-foreground"
        >
          {{ item.name }}
        </router-link>
      </nav>
      <Sheet>
        <SheetTrigger as-child>
          <Button variant="outline" size="icon" class="shrink-0 md:hidden">
            <Menu class="h-5 w-5" />
            <span class="sr-only">Menu</span>
          </Button>
        </SheetTrigger>
        <SheetContent side="left">
          <nav class="grid gap-6 text-lg font-medium">
            <router-link
              class="flex items-center gap-2 text-lg font-semibold md:text-base"
              :to="{ name: routes.Home }"
            >
              <Book class="h-6 w-6" />
              <span class="sr-only">MyTemplate</span>
            </router-link>
            <router-link
              v-for="item in navbar"
              :key="item.name"
              :to="item.to"
              class="text-foreground transition-colors hover:text-foreground"
            >
              {{ item.name }}
            </router-link>
          </nav>
        </SheetContent>
      </Sheet>
      <div class="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
        <form class="ml-auto flex-1 sm:flex-initial">
          <div class="relative">
            <Search
              class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground"
            />
            <Input
              type="search"
              placeholder="Search products..."
              class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]"
            />
          </div>
        </form>
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button variant="outline">
              <Icon
                icon="radix-icons:moon"
                class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
              />
              <Icon
                icon="radix-icons:sun"
                class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
              />
              <span class="sr-only">Theme</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem @click="mode = 'light'"> Light </DropdownMenuItem>
            <DropdownMenuItem @click="mode = 'dark'"> Dark </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button variant="secondary" size="icon" class="rounded-full">
              <CircleUser class="h-5 w-5" />
              <span class="sr-only">Menu</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>Account</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem @click="router.push({ name: routes.Settings })"
              >Settings</DropdownMenuItem
            >
            <DropdownMenuItem @click="logout">Logout</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </header>
    <router-view />
  </div>

  <!-- If the user is not logged in, show the page without the header -->
  <router-view v-else />
</template>
