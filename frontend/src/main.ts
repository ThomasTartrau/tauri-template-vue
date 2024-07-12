import "./assets/index.css";

import { createApp } from "vue";
import { Promised } from "vue-promised";
import router from "./router/router";
import { createNotivue } from "notivue";
import { AuthPlugin } from "./iam";

import { components } from "./utils/components";

import Main from "./Main.vue";

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

const app = createApp(Main);

app.use(router);
app.use(AuthPlugin);
app.use(notivue);

app.component("Promised", Promised);

for (const [name, component] of Object.entries(components)) {
  app.component(name, component);
}

app.mount("#app");
