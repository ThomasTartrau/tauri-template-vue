<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import { vAutoAnimate } from "@formkit/auto-animate/vue";

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
import { push } from "notivue";
import { Problem, displayError } from "@/http";
import { AxiosError, AxiosResponse } from "axios";
import router from "@/router/router";

const formSchema = toTypedSchema(
  z.object({
    firstName: z.string().min(1, "First name is too short"),
    lastName: z.string().min(1, "Last name is too short"),
    email: z.string().email("Invalid email address"),
    password: z
      .string()
      .min(12, "Password is too short, minimum 12 characters"),
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
        title: "Success",
        message:
          "You're successfully registered. You need to confirm your email address before using Hook0. Check your mailbox!",
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
        <CardTitle class="text-2xl"> Sign Up </CardTitle>
        <CardDescription>
          Enter your information to create an account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4">
          <div class="grid grid-cols-2 gap-4">
            <FormField v-slot="{ componentField }" name="firstName">
              <FormItem v-auto-animate>
                <FormLabel>First name</FormLabel>
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
                <FormLabel>Last name</FormLabel>
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
          <FormField v-slot="{ componentField }" name="password">
            <FormItem v-auto-animate>
              <FormLabel>Password</FormLabel>
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
          <Button type="submit" class="w-full"> Create an account </Button>
        </div>
        <div class="mt-4 text-center text-sm">
          Already have an account?
          <router-link :to="{ name: routes.Login }" class="underline">
            Sign in
          </router-link>
        </div>
      </CardContent>
    </Card>
  </form>
</template>
