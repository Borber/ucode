<template>
  <div>
    <el-tag
      v-for="tag in dynamicTags"
      :key="tag.id"
      class="el-tag"
      closable
      :disable-transitions="false"
      @close="handleClose(tag)"
    >
      {{ tag.value }}
    </el-tag>
      <el-autocomplete
        class="input-new-tag"
        v-if="inputVisible"
        ref="InputRef"
        size="small"
        @keyup.enter="handleInputConfirm"
        @blur="handleInputConfirm"
        v-model="inputValue"
        :fetch-suggestions="querySearch"
        :trigger-on-focus="false"
        clearable
        placeholder="标签"
        @select="handleSelect"
      />
      <el-button v-else class="button-new-tag" size="small" @click="showInput">
        + 新标签
      </el-button>
  </div>
</template>

<script lang="ts" setup>
import {nextTick, onMounted,reactive, Ref, ref} from 'vue'
import {ElInput} from 'element-plus'
import {invoke} from "@tauri-apps/api/tauri";

const inputValue = ref('')
const dynamicTags = ref<Tag[]>([])
const inputVisible = ref(false)
const InputRef = ref()
let allTags: Tag[] = [];

// 标签实体
// id: 标签id
// name: 标签名
// flag: 是否被删除 
interface Tag {
  id: number;
  value: string;
  flag: number;
}

// 返回建议输入的方法,querySearch(queryString, cb) 返回建议通过 cb(data) 自动完成建议。
const querySearch = (queryString: string, cb: any, restaurant: Tag) => {
  const results = queryString
    ? allTags.filter(createFilter(queryString))
    : restaurant.value
  // call callback function to return suggestions
  cb(results)
}

const createFilter = (queryString: string) => {
  return (restaurant: Tag) => {
    return (
      restaurant.value.toLowerCase().indexOf(queryString.toLowerCase()) === 0
    )
  }
}


const handleSelect = (item: Tag) => {
  console.log(item)
}

const handleClose = (tag: Tag) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1);
  console.log(dynamicTags.value.map( (id) => { return id.value; }));
}

const showInput = () => {
  inputVisible.value = true
  nextTick(() => {
    InputRef.value.focus()
  })
}

const handleInputConfirm = async () => {
  if (inputValue.value) {
    let ctag = {
      id: 0,
      value: inputValue.value,
      flag: 0
    };
    dynamicTags.value.push(ctag)
    await invoke<number>("add_tag", {
      value: inputValue.value
    }).then(async (id) => {
      ctag.id = id;
      console.log(dynamicTags.value.map( (id) => { return id.value; }));
    });
  }
  inputVisible.value = false
  inputValue.value = ''
}

onMounted(async () => {
  await invoke<Tag[]>("all_tag").then(async (tags) => { 
    allTags = tags;
   });
})
</script>


<style scoped>
.el-tag{
  margin-right: 5px;
}
.input-new-tag {
  width: 90px;
  vertical-align: bottom;
}
</style>