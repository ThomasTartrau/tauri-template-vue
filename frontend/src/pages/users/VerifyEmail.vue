<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import Separator from "@/components/ui/separator/Separator.vue";
import { push } from "notivue";
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import { verifyEmail } from "./UserServices";
import router from "@/router/router";
import { routes } from "@/router/routes";
import { displayProblem } from "@/http";

const route = useRoute();

async function _load() {
  if (!route.query.token) {
    return push.error({
      title: "Missing field",
      message: "Token is missing, please check your email link",
      duration: 5000,
    });
  }

  const token = route.query.token as string;

  await verifyEmail(token)
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
          <Button variant="outline"> Resend email </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
