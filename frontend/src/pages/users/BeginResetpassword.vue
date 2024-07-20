<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { push } from "notivue";
import { useI18n } from "vue-i18n";
import { beginResetPassword } from "./UserServices";
import Button from "@/components/ui/button/Button.vue";
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
import { displayError } from "@/http";
import router from "@/router/router";

const { t } = useI18n({ useScope: "global" });
const formSchema = toTypedSchema(
  z.object({
    email: z.string().email(t("begin_reset_password_page.email.errors.type")),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: { email: string }) {
  await beginResetPassword(values.email)
    .then(() => {
      push.success({
        title: t("begin_reset_password_page.success_notification.title"),
        message: t("begin_reset_password_page.success_notification.message"),
        duration: 5000,
      });
    })
    .catch(displayError);
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen">
    <Card class="mx-auto">
      <CardHeader>
        <CardTitle>{{ t("begin_reset_password_page.card.title") }}</CardTitle>
        <CardDescription>
          {{ t("begin_reset_password_page.card.description") }}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>{{
                t("begin_reset_password_page.email.label")
              }}</FormLabel>
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
          <div class="flex justify-end mt-4 space-x-4">
            <Button type="button" variant="secondary" @click="router.back">
              {{ t("begin_reset_password_page.back_button_label") }}
            </Button>
            <Button type="submit">
              {{ t("begin_reset_password_page.reset_password_button_label") }}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
