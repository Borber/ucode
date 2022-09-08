<template>
  <div>
    <el-tag
        v-for="tag in dynamicTags"
        :key="tag.value.id"
        class="el-tag"
        closable
        :disable-transitions="false"
        @close="handleClose(tag)"
    >
      {{ tag.value.name }}
    </el-tag>
      <el-input
        v-if="inputVisible"
        ref="InputRef"
        v-model="inputValue"
        class="input-new-tag"
        size="small"
        @keyup.enter="handleInputConfirm"
        @blur="handleInputConfirm"
      />
      <el-button v-else class="button-new-tag" size="small" @click="showInput">
        + 新标签
      </el-button>
  </div>
</template>

<script lang="ts" setup>
import {nextTick, onMounted, Ref, ref} from 'vue'
import {ElInput} from 'element-plus'
import {invoke} from "@tauri-apps/api/tauri";

const inputValue = ref('')
const dynamicTags = ref([] as Ref<Tag>[])
const inputVisible = ref(false)
const InputRef = ref<InstanceType<typeof ElInput>>()
const allTags = ref([])

interface Tag {
  id: number;
  name: string;
  flag: number;
}

const handleClose = (tag: Ref<Tag>) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1);
  console.log(dynamicTags.value.map( (id) => { return id.value; }));
}

const showInput = () => {
  inputVisible.value = true
  nextTick(() => {
    InputRef.value!.input!.focus()
  })
}

const handleInputConfirm = async () => {
  if (inputValue.value) {
    let ctag = ref({
      id: 0,
      name:  inputValue.value,
      flag: 0
    });
    dynamicTags.value.push(ctag)
    await invoke("add_tag", {
      name: inputValue.value
    }).then(async (id) => {
      ctag.value.id = id as number;
      console.log(dynamicTags.value.map( (id) => { return id.value; }));
    });
  }
  inputVisible.value = false
  inputValue.value = ''
}

onMounted(async () => {
  let allTags = await invoke("all_tag");
  console.log(allTags);
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