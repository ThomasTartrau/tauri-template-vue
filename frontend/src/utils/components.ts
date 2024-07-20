import GeneralSettings from "@/pages/users/settings/GeneralSettings.vue";
import SecuritySettings from "@/pages/users/settings/SecuritySettings.vue";
import DeleteAccount from "@/pages/users/settings/DeleteAccount.vue";

import Error404 from "@/pages/others/Error404.vue";

export const components = {
  "general-settings": GeneralSettings,
  "security-settings": SecuritySettings,
  "delete-account": DeleteAccount,

  "error-404": Error404,
};
