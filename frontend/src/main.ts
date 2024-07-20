import "./assets/index.css";

import { createApp } from "vue";
import { Promised } from "vue-promised";
import { createNotivue } from "notivue";
import { createPinia } from "pinia";
import router from "./router/router";
import { AuthPlugin } from "./iam";
import { components } from "./utils/components";
import Main from "./Main.vue";
import i18n from "@/i18n";

// Notivue
import "notivue/notification.css"; // Only needed if using built-in notifications
import "notivue/animations.css"; // Only needed if using built-in animations
import "notivue/notification-progress.css";

const notivue = createNotivue({
  position: "top-right",
  limit: 4,
  enqueue: true,
  avoidDuplicates: true,
  animations: {
    enter: "Notivue__enter",
    leave: "Notivue__leave",
    clearAll: "Notivue__clearAll",
  },
  pauseOnHover: true,
  transition: "transform 0.35s cubic-bezier(0.5, 1, 0.25, 1)",
});

const pinia = createPinia();
const app = createApp(Main);

app.use(pinia);
app.use(router);
app.use(i18n);
app.use(AuthPlugin);
app.use(notivue);

app.component("Promised", Promised);

for (const [name, component] of Object.entries(components)) {
  app.component(name, component);
}

initSettings();

app.mount("#app");

function initSettings() {
  const language = localStorage.getItem("language");
  if (language) {
    i18n.global.locale.value = language;
  } else {
    localStorage.setItem("language", i18n.global.locale.value);
  }
}
