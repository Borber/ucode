<script setup lang="ts">
import { ref, reactive, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// 普通变量
const msg = 'Hello!'

// 响应式变量
const greetMsg = ref("");// ref声明基本类型变量
const obj = reactive({        // reactive声明对象类型变量，如Object、Array、Date...
  key: 'this is a object'
})
// 函数
function log() {
  console.log(msg)          // Hello
  console.log(greetMsg.value)    // 1111（可根据input输入值而改变）
  console.log(obj.key)      // this is a object
}
const name = ref("");
const arr = invoke("lans");

const value = ref("");
// const options = reactive([
// ]);
const options: any[] = []
// console.log(invoke("lans"));
async function greet() {
  greetMsg.value = await invoke("lans");
}
// options.push(invoke("lans").value)
// invoke("lans").then()
onBeforeMount(async () => {
  await invoke("lans").then(async (s: any) => {
    console.log(s);
    s.forEach( (item: string) => {
      const arr = reactive({
        value: item,
        label: item
      })
      options.push(arr);
    });
    console.log(options);
  })
})
</script>

<template>
  <div>
    <el-select class="lan-select" v-model="value" filterable placeholder="Select">
      <el-option
          v-for="item in options"
          :key="item.value"
          :label="item.label"
          :value="item.value"
      />
    </el-select>
  </div>
</template>
<style>
.lan-select{
  width: 140px;
}
</style>