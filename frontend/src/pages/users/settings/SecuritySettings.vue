<script setup lang="ts">
import { push } from 'notivue'
import { ref } from 'vue'
import { changePassword } from '../UserServices'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import http, { displayError, displayProblem } from '@/http'
import { AlertDialog, AlertDialogContent, AlertDialogTrigger, AlertDialogHeader, AlertDialogTitle, AlertDialogDescription, AlertDialogAction, AlertDialogCancel, AlertDialogFooter } from '@/components/ui/alert-dialog'

const isPasswordDialogOpen = ref(false)
const closePasswordDialog = () => isPasswordDialogOpen.value = false

const isA2FDialogOpen = ref(false)
const closeA2FDialog = () => isA2FDialogOpen.value = false

const new_password = ref<string>('')
const confirm_password = ref<string>('')

async function doChangePassword() {
  if (new_password.value !== confirm_password.value) {
    return push.error({
      title: 'Invalid password',
      message: 'Passwords do not match',
      duration: 5000,
    })
  }

  await changePassword(new_password.value)
    .then(() => {
      push.success({
        title: 'Password changed',
        message: 'Your password has been changed successfully',
        duration: 5000,
      })
      closePasswordDialog()
    })
    .catch(displayProblem)
}

async function doEnableA2F() {
  await http.post('/user/security/a2f')
    .then(() => {
      push.success({
        title: 'A2F enabled',
        message: 'Two-factor authentication has been enabled successfully',
        duration: 5000,
      })
      closeA2FDialog()
    })
    .catch(displayError)
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
      <div class="flex justify-end mt-4">
        <Dialog v-model:open="isPasswordDialogOpen">
          <form>
            <DialogTrigger as-child>
              <Button variant="outline">
                Change password
              </Button>
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
                <Button @click="doChangePassword">
                  Change password
                </Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </CardContent>
  </Card>

  <Card>
    <CardHeader>
      <CardTitle> A2F </CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription>
        Enable two-factor authentication for your account. This will add an
        additional layer of security to your account.
      </CardDescription>
      <div class="flex justify-end">
        <AlertDialog v-model:open="isA2FDialogOpen">
          <AlertDialogTrigger class>
            <Button variant="outline">
              Enable A2F
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>
                Enable two-factor authentication
              </AlertDialogTitle>
              <AlertDialogDescription>
                By enabling two-factor authentication, you need to enter a code sent to your email to log in.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction @click="doEnableA2F">Enable</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>
    </CardContent>
  </Card>
</template>
