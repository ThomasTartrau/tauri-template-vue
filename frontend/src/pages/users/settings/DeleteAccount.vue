<script setup lang="ts">
import { push } from "notivue";
import { deleteUser } from "../UserServices";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { displayProblem } from "@/http";
import { removeStateFromStorage } from "@/iam";

async function submit() {
  await deleteUser()
    .then(() => {
      removeStateFromStorage();
      push.success({
        title: "Account deleted",
        message: "Your account has been successfully deleted",
        duration: 5000,
      });
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    })
    .catch(displayProblem);
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle> Delete account </CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription>
        By deleting your account, you will lose access to all of our services
        and data. You will not be able to recover your account.
      </CardDescription>
      <div class="flex justify-end mt-6">
        <AlertDialog>
          <AlertDialogTrigger as-child>
            <Button variant="destructive"> Delete account </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
              <AlertDialogDescription>
                This action cannot be undone. This will permanently delete your
                account and remove your data from our servers.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction @click="submit"> Delete </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>
    </CardContent>
  </Card>
</template>
