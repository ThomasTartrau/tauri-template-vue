<script setup lang="ts">
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
import { resetPassword } from "./UserServices";
import { push } from "notivue";
import { onMounted, ref } from "vue";
import { Problem, displayProblem } from "@/http";
import router from "@/router/router";
import { routes } from "@/router/routes";
import { Button } from "@/components/ui/button";

const token = ref<string>("");

const formSchema = toTypedSchema(
  z.object({
    new_password: z.string().min(1, "Password is too short"),
    confirm_password: z.string().min(1, "Password is too short"),
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
      title: "Warning",
      message: "Please fill in all fields",
      duration: 5000,
    });
  }

  if (values.new_password !== values.confirm_password) {
    return push.warning({
      title: "Warning",
      message: "Passwords do not match",
      duration: 5000,
    });
  }

  if (!token.value) {
    return push.error({
      title: "Error",
      message: "Token is required to reset password",
      duration: 5000,
    });
  }

  await resetPassword(token.value, values.new_password)
    .then(() => {
      push.success({
        title: "Success",
        message: "Your password has been reset successfully. Please login.",
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
      title: "Invalid token",
      message: "Token is required to reset password",
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
        <CardTitle>Reset Password</CardTitle>
        <CardDescription> Please enter your new password </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <div class="mb-4">
            <FormField v-slot="{ componentField }" name="new_password">
              <FormItem v-auto-animate>
                <FormLabel>New Password</FormLabel>
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
                <FormLabel>Confirm Password</FormLabel>
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

          <Button type="submit" class="w-full"> Reset Password </Button>
        </form>

        <div class="mt-4 text-center text-sm">
          Something went wrong?
          <router-link :to="{ name: routes.BeginResetpassword }" class="underline">
            Resend Email
          </router-link>
        </div>
      </CardContent>
    </Card>
  </div>
  <div v-else class="flex items-center justify-center min-h-screen">
    <component is="error-404" />
  </div>
</template>
