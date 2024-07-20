<script setup lang="ts">
import { push } from "notivue";
import { ref } from "vue";
import { changePassword } from "../UserServices";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { displayProblem } from "@/http";

const isPasswordDialogOpen = ref<boolean>(false);
const closePasswordDialog = () => (isPasswordDialogOpen.value = false);

const new_password = ref<string>("");
const confirm_password = ref<string>("");

async function submit() {
  if (new_password.value !== confirm_password.value) {
    return push.error({
      title: "Invalid password",
      message: "Passwords do not match",
      duration: 5000,
    });
  }

  await changePassword(new_password.value)
    .then(() => {
      push.success({
        title: "Password changed",
        message: "Your password has been changed successfully",
        duration: 5000,
      });
    })
    .catch(displayProblem);
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle> Security settings </CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription>
        Change your password here. Please enter your new password and confirm
        it. You need to use an secure password with minimum 12 characters.
      </CardDescription>
      <div class="flex justify-end">
        <Dialog v-model:open="isPasswordDialogOpen">
          <form>
            <DialogTrigger as-child>
              <Button variant="outline" class="mt-8"> Change password </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Change password</DialogTitle>
                <DialogDescription>
                  Change your password here. Please enter your new password and
                  confirm it. You need to use an secure password with minimum 12
                  characters.
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="newPassword" class="text-right">
                    New password
                  </Label>
                  <Input
                    id="newPassword"
                    v-model="new_password"
                    type="password"
                    class="col-span-3"
                  />
                </div>
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="confirmPassword" class="text-right">
                    Confirm password
                  </Label>
                  <Input
                    id="confirmPassword"
                    v-model="confirm_password"
                    type="password"
                    class="col-span-3"
                  />
                </div>
              </div>
              <DialogFooter>
                <Button variant="secondary" @click="closePasswordDialog">
                  Cancel
                </Button>
                <Button @click="submit"> Change password </Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </CardContent>
  </Card>
</template>
