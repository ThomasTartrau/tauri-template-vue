<script setup lang="ts">
import { vAutoAnimate } from "@formkit/auto-animate/vue";

import { push } from "notivue";
import type { AxiosError, AxiosResponse } from "axios";
import { useRouter } from "vue-router";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { z } from "zod";
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
import { login } from "@/iam";
import type { Problem } from "@/http";
import { displayError } from "@/http";

const router = useRouter();
const { t } = useI18n({ useScope: "global" });

const formSchema = toTypedSchema(
  z.object({
    email: z.string().email(t("login_page.email.errors.type")),
    password: z.string().min(1, t("login_page.password.errors.to_short")),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: { email: string; password: string }) {
  await login(values.email, values.password)
    .then(() => {
      push.success({
        title: t("login_page.success_notification.title"),
        message: t("login_page.success_notification.message"),
        duration: 5000,
      });
      return router.push({ name: routes.Home });
    })
    .catch((err: AxiosError<AxiosResponse<Problem>>) => {
      displayError(err);
    });
}
</script>

<template>
  <form
    class="flex items-center justify-center min-h-screen"
    @submit.prevent="onSubmit"
  >
    <Card class="mx-auto max-w-sm">
      <CardHeader>
        <CardTitle class="text-2xl">
          {{ t("login_page.card.title") }}
        </CardTitle>
        <CardDescription>
          {{ t("login_page.card.description") }}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4">
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>{{ t("login_page.email.label") }}</FormLabel>
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
              <FormLabel>
                <div class="text-left">
                  {{ t("login_page.password.label") }}
                </div>
                <div class="text-right text-xs">
                  <router-link
                    :to="{ name: routes.BeginResetpassword }"
                    class="underline"
                  >
                    {{ t("login_page.forgot_password_label") }}
                  </router-link>
                </div>
              </FormLabel>
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
            {{ t("login_page.login_button") }}
          </Button>
          <!-- <Button variant="outline" class="w-full">
            Login with Google
          </Button> -->
        </div>
        <div class="mt-4 text-center text-sm">
          {{ t("login_page.sign_up.label") }}
          <router-link :to="{ name: routes.Register }" class="underline">
            {{ t("login_page.sign_up.button_label") }}
          </router-link>
        </div>
      </CardContent>
    </Card>
  </form>
</template>
