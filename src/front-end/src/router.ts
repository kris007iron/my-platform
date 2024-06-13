import { createRouter, createWebHistory, Router } from "vue-router";
import HomeView from "@/pages/Home.vue";
import NotFound from "@/pages/NotFound.vue";

const routes = [
    { path: "/", component: HomeView },
    //
    { path: "/:pathMatch(.*)*", component: NotFound }
]

const router: Router = createRouter({ history: createWebHistory(import.meta.env.BASE_URL), routes })

export default router