import { createRouter, createWebHistory, Router } from "vue-router";
import HomeView from "@/pages/Home.vue";
import Projects from "@/pages/Projects.vue";
import Project from "@/pages/Project.vue";
import NotFound from "@/pages/NotFound.vue";

const routes = [
    { path: "/", component: HomeView },
    { path: "/projects", component: Projects },
    { path: "/projects/:id", component: Project },
    { path: "/:pathMatch(.*)*", component: NotFound }
]

const router: Router = createRouter({ history: createWebHistory(import.meta.env.BASE_URL), routes })

export default router