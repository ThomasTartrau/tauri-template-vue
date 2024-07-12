<script setup lang="ts">
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
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { beginResetPassword } from "./UserServices";
import { push } from "notivue";
import { displayError } from "@/http";

const formSchema = toTypedSchema(
  z.object({
    email: z.string().email("Invalid email address"),
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
        title: "Success",
        message:
          "Email sent successfully. Please check your email to reset your password.",
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
        <CardTitle>Reset my password</CardTitle>
        <CardDescription>
          Enter the email address associated with your account and we'll send
          you an email with instructions to reset your password.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>Email</FormLabel>
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
          <div class="flex justify-end mt-4">
            <Button type="submit"> Reset Password </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
