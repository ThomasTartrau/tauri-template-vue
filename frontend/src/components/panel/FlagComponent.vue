<script setup lang="ts">
import CountryFlag from "vue-country-flag-next";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { getLanguageLabel, supportedLanguages } from "@/lib/config";
import type { Language } from "@/lib/config";

const { locale } = useI18n();
const availableLanguages = ref<Language[]>(supportedLanguages());

watch(
  locale,
  (newLocale, oldLocale) => {
    if (newLocale && newLocale !== oldLocale) handleLanguageSelect(newLocale);
  },
  { immediate: true }
);

function handleLanguageSelect(newLocale: string) {
  if (
    !newLocale ||
    !availableLanguages.value.some((sl) => sl.value === newLocale)
  )
    return;
  localStorage.setItem("language", newLocale);
}
</script>

<template>
  <div class="space-y-8">
    <div class="flex items-center space-x-2">
      <Select id="language-select" v-model="locale">
        <SelectTrigger class="w-[85px]">
          <SelectValue :placeholder="getLanguageLabel(locale)" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem
              v-for="language in availableLanguages"
              :key="language.value"
              :value="language.value"
            >
              <CountryFlag :country="language.flag" />
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  </div>
</template>
