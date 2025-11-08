import { type RouteRecordNameGeneric, createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/auth";
import AuthView from "@/views/AuthView.vue";
import HomeView from "@/views/HomeView.vue";
import SettingsView from "@/views/SettingsView.vue";
import WalletsView from "@/views/WalletsView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/wallets",
      name: "wallets",
      component: WalletsView,
    },
    {
      path: "/transactions",
      name: "transactions",
      component: () => import("@/views/TransactionsView.vue"),
    },
    {
      path: "/add",
      name: "add",
      component: HomeView,
    },
    {
      path: "/analytics",
      name: "analytics",
      component: HomeView,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
    },
    {
      path: "/login",
      name: "login",
      component: AuthView,
    },
    {
      path: "/forgot-password/:id",
      name: "forgot-password",
      component: AuthView,
    },
    {
      path: "/activate/:id",
      name: "activate",
      component: AuthView,
    },
  ],
});

router.beforeEach((to) => {
  if (!useAuthStore().isAuthenticated && isAuthRoute(to.name)) {
    return { name: "login" };
  }
});

export function isAuthRoute(name: RouteRecordNameGeneric) {
  switch (name) {
    case "activate":
    case "forgot-password":
    case "login":
      return false;
    default:
      return true;
  };
};

export default router;
