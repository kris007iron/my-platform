<template>
  <div class="flex flex-col align-center rounded-lg justify-around py-10 m-10 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
    <Carousel
      class="flex self-center w-5/6 m-1 h-60 flex-wrap justify-center gap-4"
      @init-api="(val) => (emblaThumbnailApi = val)"
      :plugins="[autoplay2]"
      @mouseenter="autoplay2.stop"
      @mouseleave="[autoplay2.reset(), autoplay2.play()]"
      :opts="{ loop: true }"
    >
      <CarouselContent class="flex gap-1 ml-0">
        <CarouselItem
          v-for="(tag, index) in store.getTags"
          :key="tag.name"
          class="pl-0 basis-1/6 cursor-pointer"
          @click="onThumbClick(index)"
        >
          <div class="p-1 transition-opacity duration-300 ease-in-out" :class="index === selectedIndex ? 'opacity-100' : 'opacity-50'">
            <Card class="hover:shadow-lg transform hover:-translate-y-1 transition-transform duration-300">
              <CardContent class="flex aspect-square items-center justify-center p-6">
                <span class="text-4xl font-semibold"><font-awesome-icon :icon="tag.classes" /></span>
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>

    <div class="flex flex-wrap justify-center gap-4 w-5/6 self-center mt-6">
      <div
        v-for="project in store.getProjectsByTag(selectedTag)"
        :key="project._id.$oid"
        class="p-2 w-full sm:w-1/2 lg:w-1/3 xl:w-1/4"
      >
        <Card class="overflow-hidden rounded-lg shadow-lg p-2 hover:shadow-2xl transition-shadow duration-300">
          <div class="relative h-64">
            <!-- Image -->
            <img
              :src="exampleImageSrc"
              :alt="project.title"
              class=" object-scale-down w-full h-full transform hover:scale-105 p-1 transition-transform duration-300"
            />
            <!-- Gradient Overlay -->
            <div class="absolute inset-0 bg-gradient-to-t from-black to-transparent opacity-75"></div>
            <!-- Title and Description -->
            <div class="absolute bottom-0 left-0 right-0 p-4 text-white">
              <h3 class="text-lg font-semibold">{{ project.title }}</h3>
              <p class="mt-2 text-sm">{{ project.description }}</p>
              <!-- Tags -->
              <div class="flex flex-wrap mt-2">
                <span
                  v-for="tag in project.tags"
                  :key="tag"
                  class="inline-block px-2 py-1 mr-2 mt-1 text-xs font-semibold text-gray-800 bg-white rounded"
                >
                  #{{ tag }}
                </span>
              </div>
            </div>
          </div>
          <CardContent class="p-4 text-center">
            <a
              :href="project.link"
              target="_blank"
              class="inline-block px-4 py-2 mt-2 text-sm font-semibold text-white bg-blue-600 rounded hover:bg-blue-700 transition-colors duration-300"
            >
              View Project
            </a>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
    import Autoplay from 'embla-carousel-autoplay'
    import { Carousel, CarouselContent, type CarouselApi, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel'
    import { Card, CardContent } from '@/components/ui/card'
    import { ref } from 'vue'
    import { useStore } from '@/store'
    import exampleImage from '@/assets/exampleIMG/csharp.png'
    const exampleImageSrc = ref(exampleImage);
    const store = useStore();

    const autoplay2 = Autoplay({
        delay: 2000,
        stopOnMouseEnter: true,
        stopOnInteraction: false,
    })

    const emblaThumbnailApi = ref<CarouselApi>();
    const selectedIndex = ref(-1);
    const selectedTag = ref('');

    function onThumbClick(index: number) {
    if (!emblaThumbnailApi.value) return;
    if (index === selectedIndex.value) 
    {
        //unselect
        selectedIndex.value = -1;
        selectedTag.value = '';
        emblaThumbnailApi.value.scrollTo(0);
        return;
    }
    selectedIndex.value = index;
    selectedTag.value = store.getTags[index].name;
    emblaThumbnailApi.value.scrollTo(index);
    }
</script>

<style scoped>

</style>