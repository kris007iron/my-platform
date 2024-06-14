<template>
    <div class="flex flex-col align-center justify-center m-10">
        <Carousel
        class="relative w-full max-w-xs"
        @init-api="(val) => (emblaThumbnailApi = val)"
        :opts="{
        loop: true,
        }"
        >
            <CarouselContent class="flex gap-1 ml-0">
                <CarouselItem
                v-for="(_, index) in 10"
                :key="index"
                class="pl-0 basis-1/4 cursor-pointer"
                @click="onThumbClick(index)"
                >
                    <div class="p-1" :class="index === selectedIndex ? '' : 'opacity-50'">
                        <Card>
                            <CardContent
                            class="flex aspect-square items-center justify-center p-6"
                            >
                                <span class="text-4xl font-semibold"><font-awesome-icon icon="fa-brands fa-github"/></span>
                            </CardContent>
                        </Card>
                    </div>
                </CarouselItem>
            </CarouselContent>
        </Carousel>
                
        <Carousel
        class="self-center w-5/6 m-1"
        :plugins="[autoplay]"
        @mouseenter="autoplay.stop"
        @mouseleave="[autoplay.reset(), autoplay.play()];">
            <CarouselContent>
                <CarouselItem v-for="(_, index) in 5" :key="index">
                    <div class="p-1">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-6">
                                <span class="text-4xl font-semibold">{{ index + 1 }}</span>
                            </CardContent>
                        </Card>
                    </div>
                </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
        </Carousel>
    </div>
</template>

<script setup lang="ts">
    import Autoplay from 'embla-carousel-autoplay'
    import { Carousel, CarouselContent, type CarouselApi, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel'
    import { Card, CardContent } from '@/components/ui/card'
    import { ref } from 'vue';
    const autoplay = Autoplay({
        delay: 2000,
        stopOnMouseEnter: true,
        stopOnInteraction: false,
    })

    const emblaThumbnailApi = ref<CarouselApi>();
    const selectedIndex = ref(0);

    function onThumbClick(index: number) {
    if (!emblaThumbnailApi.value) return;
    if (index === selectedIndex.value) 
    {
        //unselect
        selectedIndex.value = -1;
        emblaThumbnailApi.value.scrollTo(0);
        return;
    }
    selectedIndex.value = index;
    emblaThumbnailApi.value.scrollTo(index - 1);
    }
</script>

<style scoped>

</style>