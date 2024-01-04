<template>
  <a-flex :style="loginStyle" vertical>
    <a-card title="注册" :bordered="false">
      <a-form :model="formState" name="normal_login" class="login-form" :rules="rules" @finish="onFinish"
        @finishFailed="onFinishFailed">
        <a-form-item label="账号" name="username">
          <a-input v-model:value="formState.username" placeholder="用户名">
            <template #prefix>
              <UserOutlined class="site-form-item-icon" />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item label="邮箱" name="email">
          <a-input v-model:value="formState.email" placeholder="邮箱">
            <template #prefix>
              <MailOutlined class="site-form-item-icon" />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item label="密码" name="password">
          <a-input-password v-model:value="formState.password">
            <template #prefix>
              <LockOutlined class="site-form-item-icon" />
            </template>
          </a-input-password>
        </a-form-item>

        <a-form-item>
          <a-button :disabled="disabled" type="primary" html-type="submit" class="login-form-button">
            注册
          </a-button>
        </a-form-item>
      </a-form>
    </a-card>
  </a-flex>
</template>
<script lang="ts" setup>
import { reactive, computed, type CSSProperties, inject } from 'vue'
import { UserOutlined, LockOutlined, MailOutlined } from '@ant-design/icons-vue'
import router from '@/router'
import { message } from 'ant-design-vue'
import type { Rule } from 'ant-design-vue/es/form'
import axios, { type AxiosResponse } from 'axios'
import { API_BASE_SYMBOL } from '@/type'
import { theme } from 'ant-design-vue';
const { useToken } = theme;
const { token } = useToken();

const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base,
  headers: {
    'Content-Type': 'application/json'
  }
})

interface FormState {
  username: string
  email: string
  password: string
}
const formState = reactive<FormState>({
  username: '',
  password: '',
  email: ''
})
const validateUsername = async (_rule: Rule, username: string) => {
  if ([...username].every((c) => /^[a-zA-Z0-9]$/.test(c))) {
    return Promise.resolve()
  } else {
    return Promise.reject('用户名只能包含字母数字')
  }
}
const validPassword = async (_rule: Rule, password: string) => {
  const hasUppercase = [...password].some((c) => c === c.toUpperCase() && /[A-Z]/.test(c))
  const hasLowercase = [...password].some((c) => c === c.toLowerCase() && /[a-z]/.test(c))
  const hasDigit = [...password].some((c) => /[0-9]/.test(c))
  const hasSpecialChar = [...password].some((c) => !/[a-zA-Z0-9]/.test(c))

  if ((hasUppercase || hasLowercase) && hasDigit && hasSpecialChar) {
    return Promise.resolve()
  } else {
    return Promise.reject('不允许弱密码。至少包含字母与数字与其他特殊符号。')
  }
}
const rules: Record<string, Rule[]> = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'change' },
    { min: 4, max: 64, message: '用户名长度需要为4-64', trigger: 'blur' },
    { validator: validateUsername, trigger: 'blur' }
  ],
  email: [{ required: true, message: '请输入合法邮箱', trigger: 'change', type: 'email' }],
  password: [
    { required: true, message: '请输入密码', trigger: 'change' },
    { min: 8, max: 64, message: '密码长度需要为8-64', trigger: 'blur' },
    { validator: validPassword, trigger: 'blur' }
  ]
}
async function register(form: FormState): Promise<AxiosResponse<any, any>> {
  try {
    const response = await api.post('/accounts/register', form, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    return response
  } catch (error: any) {
    console.log(error)
    return error.response
  }
}
const onFinish = async (_values: any) => {
  const response = await register(formState)
  if (response.status === 200) {
    message.success('注册成功：请查看邮箱验证邮件以激活！')
    router.push('/login')
  } else {
    message.error('注册失败: ' + response.data.message)
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
  background: token.value.colorBgBase,
}
</script>
