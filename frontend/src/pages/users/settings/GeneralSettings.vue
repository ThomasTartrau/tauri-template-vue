<script setup lang="ts">
import { push } from 'notivue'
import { onMounted, onUpdated, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import type { UserInfo } from '@/iam'
import { getUserInfo } from '@/iam'

const user_info = ref<UserInfo | null>(null)

const first_name = ref<string>('')
const last_name = ref<string>('')

function _load() {
  user_info.value = getUserInfo().value
  first_name.value = user_info.value?.firstName || ''
  last_name.value = user_info.value?.lastName || ''
}

function submit() {
  if (
    first_name.value !== user_info.value?.firstName
    || last_name.value !== user_info.value?.lastName
  ) {
    if (first_name.value.length < 2 || last_name.value.length < 2) {
      push.error({
        title: 'Invalid name',
        message: 'First and last name must be at least 2 characters long',
        duration: 5000,
      })
    }

    push.success({
      title: 'Profile updated',
      message: 'Your profile has been updated successfully',
      duration: 5000,
    })
  }
  else {
    push.error({
      title: 'No changes',
      message: 'No changes have been made to your profile',
      duration: 5000,
    })
  }
}
onMounted(_load)
onUpdated(_load)
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle> Personnal information </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="mb-6">
        <FormField name="Email">
          <FormItem>
            <FormLabel>Email</FormLabel>
            <FormControl>
              <Input
                disabled
                type="email"
                placeholder="Email"
                :model-value="user_info?.email"
              />
            </FormControl>
            <FormDescription>
              This is the email address associated with your account.
              Unfortuntely, you cannot change this.
            </FormDescription>
          </FormItem>
        </FormField>
      </div>

      <div class="mb-6">
        <FormField name="First name">
          <FormItem>
            <FormLabel>First name</FormLabel>
            <FormControl>
              <Input
                disabled
                type="text"
                placeholder="First name"
                :model-value="user_info?.firstName"
              />
            </FormControl>
            <FormDescription>
              That's the first name you've used to register.
            </FormDescription>
          </FormItem>
        </FormField>
      </div>

      <div class="mb-6">
        <FormField name="Last name">
          <FormItem>
            <FormLabel>Last name</FormLabel>
            <FormControl>
              <Input
                disabled
                type="text"
                placeholder="Last name"
                :model-value="user_info?.lastName"
              />
            </FormControl>
            <FormDescription>
              That's the last name you've used to register.
            </FormDescription>
          </FormItem>
        </FormField>
      </div>

      <div class="flex justify-end">
        <Dialog>
          <form>
            <DialogTrigger as-child>
              <Button variant="outline">
                Edit profile
              </Button>
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
              <DialogHeader>
                <DialogTitle>Edit profile</DialogTitle>
                <DialogDescription>
                  Make changes to your profile here. Click save when you're
                  done.
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="firstName" class="text-right"> First name </Label>
                  <Input
                    id="firstName"
                    v-model="first_name"
                    class="col-span-3"
                  />
                </div>
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="lastName" class="text-right"> Last name </Label>
                  <Input id="lastName" v-model="last_name" class="col-span-3" />
                </div>
              </div>
              <DialogFooter>
                <Button @click="submit">
                  Save changes
                </Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </CardContent>
  </Card>
</template>
