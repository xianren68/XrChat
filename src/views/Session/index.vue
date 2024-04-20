<template>
    <div class="session">
        <div class="left">
            <Search v-model="search" @search="searchSession"></Search>
            <div class="sessions">
                <Info v-for="i in 5" avatar="''" name="xianren" msg="hello" :key="i" :class="{'notice-active':i===1}">
                    <Notice lastTime="2022-12-10" unRead="10"></Notice>
                </Info>
            </div>
        </div>
        <div class="right"></div>
    </div>
</template>

<script setup lang="ts">
import Search from '@/components/Search.vue'
import Info from '@/components/Info.vue'
import Notice from '@/components/Notice.vue'
import { ref } from 'vue'
const search = ref('')
const searchSession = () => {
    console.log(search.value)
}
</script>

<style scoped>
.session {
    box-sizing: border-box;
    width:calc(100% - 60px);
    height: 100%;
    display: flex;
    .left {
        height:100%;
        width:300px;
        padding:10px;
        box-sizing: border-box;
        .notice-active {
            transform: scale(1.03);
            box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.2);
        }
    }
    .right {
        height:100%;
        flex:1;
    }
}
</style>