<script setup lang="ts">
import {onMounted, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const lan = ref("");
const options: any[] = reactive([]);

onMounted(async () => {
  await invoke<string[]>("lans").then(async (s) => {
    console.log(s);
    s.forEach((item) => {
      options.push({
        value: item,
        label: item
      });
    })
  })
})
</script>

<template>
  <div>
    <el-select v-model="lan" class="lan-select" filterable placeholder="语言">
      <el-option
          v-for="item in options"
          :key="item.value"
          :label="item.label"
          :value="item.value"
      />
    </el-select>
  </div>
</template>
<style scoped>
.lan-select {
  width: 140px;
}
</style>