<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { push } from "notivue";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import Error404 from "../others/Error404.vue";
import { resetPassword } from "./UserServices";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import type { Problem } from "@/http";
import { displayProblem } from "@/http";
import router from "@/router/router";
import { routes } from "@/router/routes";
import { Button } from "@/components/ui/button";

const token = ref<string>("");

const { t } = useI18n({ useScope: "global" });
const formSchema = toTypedSchema(
  z.object({
    new_password: z
      .string()
      .min(1, t("reset_password_page.new_password.errors.too_short")),
    confirm_password: z
      .string()
      .min(1, t("reset_password_page.confirm_password.errors.too_short")),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: {
  new_password: string;
  confirm_password: string;
}) {
  if (!values.new_password || !values.confirm_password) {
    return push.warning({
      title: t("reset_password_page.errors.missing_fields.title"),
      message: t("reset_password_page.errors.missing_fields.message"),
      duration: 5000,
    });
  }

  if (values.new_password !== values.confirm_password) {
    return push.warning({
      title: t("reset_password_page.errors.not_match.title"),
      message: t("reset_password_page.errors.not_match.message"),
      duration: 5000,
    });
  }

  if (!token.value) {
    return push.error({
      title: t("reset_password_page.errors.required_token.title"),
      message: t("reset_password_page.errors.required_token.message"),
      duration: 5000,
    });
  }

  await resetPassword(token.value, values.new_password)
    .then(() => {
      push.success({
        title: t("reset_password_page.success_notification.title"),
        message: t("reset_password_page.success_notification.message"),
        duration: 5000,
      });
      return router.push({ name: routes.Login });
    })
    .catch((problem: Problem) => {
      displayProblem(problem);
    });
}

function _load() {
  token.value = router.currentRoute.value.query.token as string;
  if (!token.value) {
    push.error({
      title: t("reset_password_page.errors.required_token.title"),
      message: t("reset_password_page.errors.required_token.message"),
      duration: 5000,
    });
  }
}

onMounted(() => {
  _load();
});
</script>

<template>
  <div v-if="token" class="flex items-center justify-center min-h-screen">
    <Card class="mx-auto">
      <CardHeader>
        <CardTitle>{{ t("reset_password_page.card.title") }}</CardTitle>
        <CardDescription>{{
          t("reset_password_page.card.description")
        }}</CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <div class="mb-4">
            <FormField v-slot="{ componentField }" name="new_password">
              <FormItem v-auto-animate>
                <FormLabel>{{
                  t("reset_password_page.new_password.label")
                }}</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="************"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>

          <div class="mb-4">
            <FormField
              v-slot="{ componentField }"
              name="confirm_password"
              class="mb-6"
            >
              <FormItem v-auto-animate>
                <FormLabel>{{
                  t("reset_password_page.confirm_password.label")
                }}</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="************"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>

          <Button type="submit" class="w-full">
            {{ t("reset_password_page.reset_password_button") }}
          </Button>
        </form>

        <div class="mt-4 text-center text-sm">
          {{ t("reset_password_page.begin_reset_password.label") }}
          <router-link
            :to="{ name: routes.BeginResetpassword }"
            class="underline"
          >
            {{ t("reset_password_page.begin_reset_password.button_label") }}
          </router-link>
        </div>
      </CardContent>
    </Card>
  </div>
  <div v-else class="flex items-center justify-center min-h-screen">
    <component :is="Error404" />
  </div>
</template>
