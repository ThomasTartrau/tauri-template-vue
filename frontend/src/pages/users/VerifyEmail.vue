<script setup lang="ts">
import { push } from "notivue";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { resendVerificationEmail, verifyEmail } from "./UserServices";
import Separator from "@/components/ui/separator/Separator.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import router from "@/router/router";
import { routes } from "@/router/routes";
import { displayProblem } from "@/http";

const route = useRoute();

const token = ref<string>("");

async function _load() {
  if (!route.query.token) {
    return push.error({
      title: "Missing field",
      message: "Token is missing, please check your email link",
      duration: 5000,
    });
  }

  token.value = route.query.token as string;

  await verifyEmail(token.value)
    .then(() => {
      push.success({
        title: "Email verified",
        message: "Your email has been verified successfully",
        duration: 5000,
      });
      return router.push({ name: routes.Login });
    })
    .catch(displayProblem);
}

async function submit() {
  await resendVerificationEmail(token.value)
    .then(() => {
      push.success({
        title: "Email sent",
        message: "Email has been sent successfully. Please check your inbox",
        duration: 5000,
      });
    })
    .catch(displayProblem);
}

onMounted(() => {
  _load();
});
</script>

<template>
  <div class="flex items-center justify-center min-h-screen">
    <Card>
      <CardHeader>
        <CardTitle> Verify your email </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col gap-2 mb-4">
          <div class="flex flex-col gap-1">
            <div class="text-sm">
              Please check your email to verify your account
            </div>
            <div class="text-sm">
              If you didn't receive an email, click here to resend it
            </div>
          </div>
        </div>

        <Separator class="my-4" />

        <div class="flex flex-col gap-2 mt-4">
          <Button variant="outline" @click="submit"> Resend email </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
