<template>
    <div class="self">
        <div class="main">
            <div class="avatar">
                <img src="" alt="">
            </div>
            <input type="text" class="name" placeholder="请输入新昵称" v-if="editName" @keydown.enter="editNameFn"
                @blur="editNameFn">
            <div class="name" v-else>闲人</div>
            <div class="line">less is more</div>
            <div class="phone">
                <svg class="icon">
                    <use xlink:href="#icon-shouji"></use>
                </svg>
                <div>18827262115</div>
            </div>
            <div class="gender">
                <svg class="icon">
                    <use xlink:href="#icon-xingbie"></use>
                </svg>
                <div class="select">
                    <input type="radio" name="gender" id="male" value="male" @change="editGenderFn">
                    <label for="male">男</label>
                </div>
                <div class="select">
                    <input type="radio" name="gender" id="female" value="female" @change="editGenderFn">
                    <label for="female">女</label>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { updateName, updateGender } from '@/api'
import { userStore } from '@/store'
import { option } from '@/pb/option'
import { ElMessage } from 'element-plus'
import { user } from '@/pb/user'
const useUserStore = userStore()
const editName = ref(true)
const editPhone = ref(false)
const editLine = ref(false)

// send editname request.
const editNameFn = async (e: Event) => {
    const value = (e.target as HTMLInputElement).value
    if (useUserStore.userInfo!.username === value) {
        return
    }
    const reqInfo = new option.updateName({
        id: useUserStore.userInfo?.id,
        name: value
    })
    try {
        const resp = await updateName(reqInfo)
        const res = user.Response.deserialize(resp as Uint8Array)
        if (res.code !== 200) {
            ElMessage.error(res.message)
            return
        }
        useUserStore.userInfo!.username = value
        console.log(value)
        ElMessage.success('修改成功')
    } catch (e) {
        console.log(e)
    }
    editName.value = false
}

// send editGender request.
const editGenderFn = async (e: Event) => {
    const value = (e.target as HTMLInputElement).value === 'male' ? true : false
    const reqInfo = new option.updateGender({
        id: useUserStore.userInfo?.id,
        gender: value
    })
    try {
        const resp = await updateGender(reqInfo)
        const res = user.Response.deserialize(resp as Uint8Array)
        if (res.code !== 200) {
            ElMessage.error(res.message)
            return
        }
        useUserStore.userInfo!.gender = value
        console.log(value)
        ElMessage.success('修改成功')
    } catch (e) {
        console.log(e)
    }
}


</script>

<style scoped>
.self {
    height: 100%;
    width: calc(100% - 60px);
    display: flex;
    justify-content: center;
    align-items: center;

    .main {
        box-sizing: border-box;
        position: relative;
        height: 400px;
        width: 600px;
        background-color: var(--primary-color);
        border-radius: 5px;
        box-shadow: 4px 4px 4px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        align-items: center;

        input {
            background-color: transparent;
            border: none;
            outline: none;
            border-bottom: 1px solid var(--text-color-secondary)
        }

        .avatar {
            position: absolute;
            height: 100px;
            width: 100px;
            top: -50px;
            left: 50%;
            transform: translateX(-50%);
            border-radius: 50%;
            box-shadow: 0px 0px 4px rgba(0, 0, 0, 0.2);
            overflow: hidden;

            img {
                height: 100%;
                width: 100%;
            }

        }

        .name {
            margin-top: 80px;
            height: 14px;
            line-height: 14px;
            text-align: center;
            color: var(--text-color);
        }

        .line {
            margin-top: 20px;
            height: 30px;
            font-size: 20px;
            line-height: 30px;
            font-weight: 600;
            font-style: italic;
            color: var(--text-color-secondary);
        }

        .icon {
            height: 30px;
            width: 30px;
            fill: var(--text-color-secondary);
            margin-right: 10px;
        }

        .phone {
            display: flex;
            align-items: center;
            margin-top: 20px;
            color: var(--text-color);
        }

        .gender {
            display: flex;
            align-items: center;
            margin-top: 20px;
            color: var(--text-color);

            .select {
                margin-left: 10px;
                display: flex;
            }
        }
    }


}
</style>