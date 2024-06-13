<template>
    <div class="flex flex-col align-center justify-center">
        <div class="self-center">
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
            <font-awesome-icon icon="fa-brands fa-github"/>
        </div>

        <Carousel
        class="self-center w-5/6 m-1"
        :plugins="[autoplay]"
        @mouseenter="autoplay.stop"
        @mouseleave="[autoplay.reset(), autoplay.play(), console.log('Running')];">
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
    import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel'
    import { Card, CardContent } from '@/components/ui/card'

    const autoplay = Autoplay({
        delay: 2000,
        stopOnMouseEnter: true,
        stopOnInteraction: false,
    })
</script>

<style scoped>

</style>