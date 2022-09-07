<template>
  <div>
    <el-tag
        v-for="tag in dynamicTags"
        :key="tag"
        class="el-tag"
        closable
        :disable-transitions="false"
        @close="handleClose(tag)"
    >
      {{ tag }}
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
import { nextTick, ref } from 'vue'
import { ElInput } from 'element-plus'

const inputValue = ref('')
const dynamicTags = ref([])
const inputVisible = ref(false)
const InputRef = ref<InstanceType<typeof ElInput>>()

const handleClose = (tag: string) => {
  dynamicTags.value.splice(dynamicTags.value.indexOf(tag), 1)
}

const showInput = () => {
  inputVisible.value = true
  nextTick(() => {
    InputRef.value!.input!.focus()
  })
}

const handleInputConfirm = () => {
  if (inputValue.value) {
    dynamicTags.value.push(inputValue.value)
  }
  inputVisible.value = false
  inputValue.value = ''
}
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