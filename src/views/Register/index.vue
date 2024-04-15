<template>
  <div class="register">
    <p class="title">REGISTER</p>
    <div class="info">
      <div class="name item">
        <svg class="icon">
          <use xlink:href="#icon-user"></use>
        </svg>
        <input type="text" placeholder="用户名" v-model="name">
        <span class="verify" v-show="verName">用户名不能为空</span>
      </div>
      <div class="pwd item">
        <svg class="icon">
          <use xlink:href="#icon-pwd"></use>
        </svg>
        <input type="password" placeholder="密码" v-model="pwd">
        <span class="verify" v-show="verPwd">密码不能为空</span>
      </div>
      <div class="email item">
        <svg class="icon">
          <use xlink:href="#icon-youxiang"></use>
        </svg>
        <input type="email" placeholder="输入邮箱" v-model="email" v-if="sendOrSetTime==='邮箱验证'">
        <input type="text" placeholder="输入验证码" v-model="code" v-else>
        <span class="verify" v-show="verEmail">邮箱是必须的</span>
        <span class="verify" v-show="verCode">输入验证码</span>
        <span class="vef" @click="sendEmail">{{sendOrSetTime}}</span>
      </div>
      <button @click="register">REGISTER</button>
      <div class="login">
        <span>已有账号？ </span>
        <a class="link" @click="router.push({ name: 'login' })">去登录</a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useRouter} from "vue-router"
import {ref} from 'vue'
import {VerifyEmail,Register} from "@/api"
import { user} from "@/pb/user.ts"
import {ElMessage} from "element-plus"
import {verifyEmailAddress} from "@/utils"

const router = useRouter()
const name = ref('')
const pwd = ref('')
const email = ref('')
const verEmail = ref(false)
const verPwd = ref(false)
const verName = ref(false)
const code = ref('')
const verCode = ref(false)
const sendOrSetTime = ref<string|number>("邮箱验证")

// control whether email verification can be sent.
let timer:any = null
// send Email address for verify.
const sendEmail = async () => {
  if(timer){
    return
  }
  if (!email.value) {
    verEmail.value = true
    setTimeout(()=>{
      verEmail.value = false
    },1000)
    return
  }
  if(!verifyEmailAddress(email.value)){
      ElMessage.warning("邮箱格式有误")
  }
  const msg = new user.EmailVerifyRequest({email:email.value})
  const res = await VerifyEmail(msg)
  const resp = user.Response.deserialize(res as Uint8Array)
  if(resp.code !== 200){
    ElMessage.error(resp.message)
    return
  }
  ElMessage.success(resp.message)
  sendOrSetTime.value = 300
  timer = setInterval(() => {
    sendOrSetTime.value = sendOrSetTime.value as number - 1
    if (sendOrSetTime.value === 0) {
        clearInterval(timer)
        timer = null
        sendOrSetTime.value = "邮箱验证"
    }
  },1000)
}
// register
const register = async()=>{
  if(!verify()){
    return
  }
  console.log(code.value)
  const arrayBuffer = await Register(new user.RegisterRequest({
    username: name.value,
    password: pwd.value,
    code: code.value,
    email: email.value
  }))
  const resp = user.Response.deserialize(arrayBuffer as Uint8Array)
  if(resp.code !== 200){
    ElMessage.error(resp.message)
    return
  }
  ElMessage.success(resp.message)
}
// verify input.
const verify = ()=>{
  let flag = false
  if(!name){
    verName.value = true
    setTimeout(()=>{
      verName.value = false
    },1000)
  }
  if(!pwd.value){
    verPwd.value = true
    setTimeout(()=>{
      verPwd.value = false
    },1000)
  }
  if(!email.value){
    verEmail.value = true
    setTimeout(()=>{
      verEmail.value = false
    },1000)
  }
  if(sendOrSetTime.value === "邮箱验证"){
    ElMessage.warning("请先完成邮箱验证")
  } else if(!code.value && sendOrSetTime.value !== "邮箱验证"){
    verCode.value = true
    setTimeout(()=>{
      verCode.value = false
    },1000)
  }
  flag = true
  return flag
}



</script>

<style scoped>
.register {
  margin: auto;
  height: 380px;
  width: 350px;
  background-color: #ffffff11;
  box-shadow: rgba(0, 0, 0, 0.1) 0px 10px 15px -3px, rgba(0, 0, 0, 0.05) 0px 4px 6px -2px;
  border-radius: 10px;

  .title {
    margin-top: 20px;
    font-size: 20px;
    text-align: center;
    font-weight: bolder;
  }

  .info {
    display: flex;
    flex-direction: column;
    align-items: center;
    line-height: 16px;

    .item {
      position: relative;
      border-bottom: #ccc 2px solid;
      height: 30px;
      width: 280px;

      .icon {
        display: inline-block;
        width: 20px;
        height: 20px;
      }
    }

    .name {
      margin-top: 60px;
    }

    .pwd {
      margin-top: 35px;
    }
    .email {
      margin-top: 35px;
      position:relative;
      .vef{
        position: absolute;
        right: 0;
        height: 30px;
        width: 70px;
        font-size:12px;
        line-height: 30px;
        background-color:#ffffcf08;
        text-align: center;
      }
    }

    input {
      display: inline-block;
      height: 30px;
      width: 160px;
      margin-left: 10px;
      border: none;
      outline: none;
      background-color: transparent;
      padding: 0 0 10px 10px;
    }

    button {
      margin-top: 20px;
      background-color: #0090fc;
      font-weight: bolder;
      border-radius: 8px;
      box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
      width: 200px;
      height: 40px;
      color: #fff;
      border: none;
    }

    .login {
      margin-top: 23px;
      font-size: 12px;
      color: #ccc;

      .link {
        color: #8cc63e;
      }
    }

    .verify {
      position: absolute;
      display: inline-block;
      width: 200px;
      color: red;
      font-size: 10px;
      bottom: -20px;
      left: 10px;
    }
  }
}</style>