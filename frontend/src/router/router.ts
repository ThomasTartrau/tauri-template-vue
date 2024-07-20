import { createRouter, createWebHistory } from "vue-router";
import routes from "@/router/routes";

const router = createRouter({
  // Provide the history implementation to use
  history: createWebHistory(),
  routes,
});

export default router;
