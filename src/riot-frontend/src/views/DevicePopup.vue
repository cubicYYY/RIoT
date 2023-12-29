<template>
    <a-form :model="formState" name="basic" :label-col="{ span: 6 }" :wrapper-col="{ span: 12 }" autocomplete="off"
        @finish="onFinish" @finishFailed="onFinishFailed">
        <a-form-item label="设备名称" name="name">
            <a-input v-model:value="formState.name" />
        </a-form-item>

        <a-form-item label="设备描述" name="description">
            <a-textarea v-model:value="formState.description" />
        </a-form-item>

        <a-form-item label="设备类型" name="dtype" :span="8">
            <a-input v-model:value="formState.dtype" />
        </a-form-item>

        <a-form-item label="纬度（可选）" name="latitude">
            <a-input-number v-model:value="formState.latitude" />
        </a-form-item>

        <a-form-item label="经度（可选）" name="longitude">
            <a-input-number v-model:value="formState.longitude" />
        </a-form-item>

        <a-form-item label="topic" name="topic">
            <a-input-group compact>
                <a-tooltip title="API Key为固定前缀">
                    <a-input value="/test/topic" disabled style="width: 30%;"/>
                </a-tooltip>
                <a-input v-model:value="formState.topic" style="width: 70%;"/>
            </a-input-group>
        </a-form-item>
    </a-form>
</template>
<script lang="ts" setup>
import { reactive } from 'vue';
const apiKey = "114514";
interface FormState {
    name: string;
    description: string;
    dtype: Number,
    latitude: Number;
    longitude: Number;
    topic: string;

}

const formState = reactive<FormState>({
    name: '',
    description: '',
    dtype: 1,
    latitude: 1.1,
    longitude: 1.1,
    topic: '/test'
});
const onFinish = (values: any) => {
    console.log('Success:', values);
};

const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
};
</script>
  
  