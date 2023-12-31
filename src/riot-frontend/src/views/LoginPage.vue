<template>
  <a-flex :style="loginStyle" vertical>
    <a-card title="登录" :bordered="false">
      <a-form
        :model="formState"
        name="normal_login"
        class="login-form"
        @finish="onFinish"
        @finishFailed="onFinishFailed"
      >
        <a-form-item label="账号" name="username" :rules="[{ required: true, message: '必填' }]">
          <a-input v-model:value="formState.username" placeholder="用户名或邮箱">
            <template #prefix>
              <UserOutlined class="site-form-item-icon" />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item label="密码" name="password" :rules="[{ required: true, message: '必填' }]">
          <a-input-password v-model:value="formState.password">
            <template #prefix>
              <LockOutlined class="site-form-item-icon" />
            </template>
          </a-input-password>
        </a-form-item>

        <a-form-item>
          <a-form-item name="remember" no-style>
            <a-checkbox v-model:checked="formState.remember">记住我</a-checkbox>
            <!-- TODO -->
          </a-form-item>
        </a-form-item>

        <a-form-item>
          <a-button
            :disabled="disabled"
            type="primary"
            html-type="submit"
            class="login-form-button"
          >
            登录
          </a-button>
          Or...
          <router-link to="/register">现在注册！</router-link>
        </a-form-item>
      </a-form>
    </a-card>
  </a-flex>
</template>
<script lang="ts" setup>
import { reactive, computed, type CSSProperties } from 'vue'
import { UserOutlined, LockOutlined } from '@ant-design/icons-vue'
import { useUserStore } from '@/stores/user'
import router from '@/router'
import { message } from 'ant-design-vue'

interface FormState {
  username: string
  password: string
  remember: boolean
}
const formState = reactive<FormState>({
  username: '',
  password: '',
  remember: true
})

const onFinish = async (values: any) => {
  const userState = useUserStore()
  const result = await userState.login(values.username, values.password)
  if (result) {
    message.success('登录成功！')
    router.push('/dashboard')
  } else {
    message.error('登陆失败')
  }
}

const onFinishFailed = (errorInfo: any) => {
  console.log('Failed:', errorInfo)
}

const disabled = computed(() => {
  return !(formState.username && formState.password)
})
const loginStyle: CSSProperties = {
  minWidth: '100vw',
  minHeight: '100vh',
  paddingLeft: '20vw',
  paddingRight: '20vw',
  paddingTop: '10vh',
  paddingBottom: '10vh',
  background: '#ececec'
}
</script>
