<script setup lang="ts">
  import { Avatar, AvatarFallback, AvatarImage } from './components/ui/avatar';
</script>

<template>
  <header class="sticky z-40 top-0 bg-background/20 backdrop-blur-lg border-b border-border">    
  <nav class="flex py-4 items-center">
    <router-link to="/" class="text-primary font-bold text-lg px-4 justify-self-start">
      <Avatar>
        <AvatarImage src="https://avatars.githubusercontent.com/u/e?email=kris007.iron@gmail.com" alt="Avatar" />
        <AvatarFallback>Chris</AvatarFallback>
      </Avatar>
    </router-link>  
    <div class="flex-grow"></div>  
    <div class="flex space-x-4 justify-self-end">
      <router-link to="/projects" class="text-primary font-bold text-lg px-4">Projects</router-link>
      <router-link to="/about" class="text-primary font-bold text-lg px-4">About</router-link>
      <router-link to="/contact" class="text-primary font-bold text-lg px-4">Contact</router-link>
    </div>
</nav>

  </header>
  <RouterView />
</template>

<style scoped>

</style>
