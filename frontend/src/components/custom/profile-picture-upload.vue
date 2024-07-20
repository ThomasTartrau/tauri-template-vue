<script setup lang="ts">
import { useDropzone } from "vue3-dropzone";
import { push } from "notivue";
import { UploadCloud } from "lucide-vue-next";
import http, { displayError } from "@/http";

const props = defineProps({
  accept: {
    type: String,
    default: "*",
  },
});

const emit = defineEmits(["closeProfileDialog"]);

async function onDrop(files: File[]) {
  if (files.length > 1 || files.length <= 0) {
    return push.error({
      title: "Invalid file count",
      message: "Only one file can be uploaded at a time",
      duration: 5000,
    });
  }
  const file = files[0];
  if (!["image/jpeg", "image/jpg", "image/png"].includes(file.type)) {
    push.error({
      title: "Unsupported file type",
      message: "Only .jpeg, .jpg, and .png image are supported",
      duration: 5000,
    });
    return;
  }

  if (file.size > 10 * 1024 * 1024) {
    push.error({
      title: "File size too large",
      message: "File size should be under 10 MB",
      duration: 5000,
    });
    return;
  }

  const fd = new FormData();
  fd.append("picture", file);
  await http
    .post("/user/profile-picture", fd, {
      headers: {
        "Content-Type": "multipart/form-data; ",
      },
    })
    .then(() => {
      push.success({
        title: "Profile picture updated",
        message: "Your profile picture has been updated successfully",
        duration: 5000,
      });
    })
    .catch(displayError);
  emit("closeProfileDialog");
}

const { getRootProps, getInputProps } = useDropzone({
  onDrop,
  multiple: false,
});
</script>

<template>
  <div>
    <div>
      <label
        v-bind="getRootProps()"
        class="relative flex flex-col items-center justify-center w-full py-6 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-muted/30 hover:bg-background/10"
      >
        <div class="text-center">
          <div class="border p-2 rounded-md max-w-min mx-auto">
            <UploadCloud :size="20" />
          </div>

          <p class="mt-2 text-sm">
            <span class="font-semibold">Drag files</span>
          </p>
          <p class="text-xs">
            Click to upload files (files should be under 10 MB)
          </p>
        </div>
      </label>

      <form enctype="multipart/form-data">
        <input
          v-bind="getInputProps()"
          id="dropzone-file"
          :accept="props.accept"
          type="file"
          class="hidden"
        />
      </form>
    </div>
  </div>
</template>
