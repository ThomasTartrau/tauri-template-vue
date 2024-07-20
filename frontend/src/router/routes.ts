import HomePage from "@/pages/HomePage.vue";
import LoginPage from "@/pages/users/LoginPage.vue";
import RegisterPage from "@/pages/users/RegisterPage.vue";
import VerifyEmail from "@/pages/users/VerifyEmail.vue";
import UserSettings from "@/pages/users/UserSettings.vue";
import Error404 from "@/pages/others/Error404.vue";
import BeginResetpassword from "@/pages/users/BeginResetpassword.vue";
import ResetPassword from "@/pages/users/ResetPassword.vue";

export type TemplateRoutes = string;

export const routes: Record<TemplateRoutes, string> = {
  Home: "Home",

  Login: "Login",
  Register: "Register",
  VerifyEmail: "VerifyEmail",
  BeginResetpassword: "BeginResetpassword",
  ResetPassword: "ResetPassword",

  Settings: "Settings",
  SecuritySettings: "SecuritySettings",
  DeleteAccountSettings: "DeleteAccountSettings",

  Error404: "Error404",
};

export default [
  {
    name: routes.Home,
    path: "/",
    component: HomePage,
  },
  {
    name: routes.Login,
    path: "/login",
    component: LoginPage,
    meta: {
      requiresAuth: false,
    },
  },
  {
    name: routes.Register,
    path: "/register",
    component: RegisterPage,
    meta: { requiresAuth: false },
  },
  {
    name: routes.VerifyEmail,
    path: "/verify-email",
    component: VerifyEmail,
    meta: { requiresAuth: false },
  },
  {
    name: routes.BeginResetpassword,
    path: "/begin-reset-password",
    component: BeginResetpassword,
    meta: {
      requiresAuth: false,
      redirectIfLoggedIn: false,
    },
  },
  {
    name: routes.ResetPassword,
    path: "/reset-password",
    component: ResetPassword,
    meta: {
      requiresAuth: false,
      redirectIfLoggedIn: false,
    },
  },
  {
    name: routes.Settings,
    path: "/settings",
    component: UserSettings,
  },
  {
    name: routes.SecuritySettings,
    path: "/settings/security",
    component: UserSettings,
  },
  {
    name: routes.DeleteAccountSettings,
    path: "/settings/delete-account",
    component: UserSettings,
  },
  {
    name: routes.Error404,
    path: "/:pathMatch(.*)*",
    component: Error404,
  },
];

export function useRoute(routeName: TemplateRoutes): string {
  return routes[routeName];
}
