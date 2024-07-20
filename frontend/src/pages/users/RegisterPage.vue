<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import { vAutoAnimate } from "@formkit/auto-animate/vue";

import { push } from "notivue";
import type { AxiosError, AxiosResponse } from "axios";
import { useI18n } from "vue-i18n";
import { Button } from "@/components/ui/button";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { routes } from "@/router/routes";
import { register } from "@/iam";
import type { Problem } from "@/http";
import { displayError } from "@/http";
import router from "@/router/router";

const { t } = useI18n({ useScope: "global" });
const formSchema = toTypedSchema(
  z.object({
    firstName: z.string().min(1, t("sign_up_page.first_name.errors.too_short")),
    lastName: z.string().min(1, t("sign_up_page.last_name.errors.too_short")),
    email: z.string().email(t("sign_up_page.email.errors.type")),
    password: z.string().min(12, t("sign_up_page.password.errors.too_short")),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
}) {
  await register(
    values.email,
    values.firstName,
    values.lastName,
    values.password,
  )
    .then(() => {
      push.success({
        title: t("sign_up_page.success_notification.title"),
        message: t("sign_up_page.success_notification.message"),
        duration: 5000,
      });
      return router.push({ name: routes.Login });
    })
    .catch((err: AxiosError<AxiosResponse<Problem>>) => {
      displayError(err);
    });
}
</script>

<template>
  <form
    class="flex items-center justify-center min-h-screen"
    @submit="onSubmit"
  >
    <Card class="mx-auto max-w-sm">
      <CardHeader>
        <CardTitle class="text-2xl">
          {{ t("sign_up_page.card.title") }}
        </CardTitle>
        <CardDescription>
          {{ t("sign_up_page.card.description") }}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4">
          <div class="grid grid-cols-2 gap-4">
            <FormField v-slot="{ componentField }" name="firstName">
              <FormItem v-auto-animate>
                <FormLabel>{{ t("sign_up_page.first_name.label") }}</FormLabel>
                <FormControl>
                  <Input
                    type="text"
                    placeholder="John"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
            <FormField v-slot="{ componentField }" name="lastName">
              <FormItem v-auto-animate>
                <FormLabel>{{ t("sign_up_page.last_name.label") }}</FormLabel>
                <FormControl>
                  <Input
                    type="text"
                    placeholder="Doe"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>{{ t("sign_up_page.email.label") }}</FormLabel>
              <FormControl>
                <Input
                  type="email"
                  placeholder="johndoe@example.com"
                  v-bind="componentField"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
          <FormField v-slot="{ componentField }" name="password">
            <FormItem v-auto-animate>
              <FormLabel>{{ t("sign_up_page.password.label") }}</FormLabel>
              <FormControl>
                <Input
                  type="password"
                  placeholder="********"
                  v-bind="componentField"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
          <Button type="submit" class="w-full">
            {{ t("sign_up_page.sign_up_button") }}
          </Button>
        </div>
        <div class="mt-4 text-center text-sm">
          {{ t("sign_up_page.sign_in.label") }}
          <router-link :to="{ name: routes.Login }" class="underline">
            {{ t("sign_up_page.sign_in.button_label") }}
          </router-link>
        </div>
      </CardContent>
    </Card>
  </form>
</template>
