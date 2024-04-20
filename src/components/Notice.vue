<template>
    <div class="notice">
        <div class="last_time">{{lastTime}}</div>
        <div class="unread">{{unRead}}</div>
    </div>
</template>

<script setup lang="ts">
defineProps({
    unRead:String,
    lastTime:String
})
</script>

<style scoped>
.notice {
    height: 40px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: end;
    justify-content: space-between;
    .last_time {
        font-size: 12px;
        color:var(--text-color-secondary);
    }
    .unread {
        color: var(--text-color);
        font-size: 10px;
        height: 18px;
        width: 18px;
        margin-right: 15px;
        line-height: 18px;
        text-align: center;
        border-radius: 50%;
        background-color: #c40000;
    }
}
</style>