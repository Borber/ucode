<template>
	<div>
    <el-input v-model="codeTitle" placeholder="Please input title" />
    <el-select v-model="lan" class="lan-select" filterable placeholder="语言">
      <el-option
        v-for="item in options"
        :key="item.value"
        :label="item.label"
        :value="item.value"
      />
    </el-select>
    <div class="tag">
      <el-tag
        v-for="tag in tags"
        :key="tag.id"
        class="el-tag"
        closable
        size="large"
        :disable-transitions="false"
        @close="handleClose(tag)"
      >
        {{ tag.value }}
      </el-tag>
      <div class="input-new-tag">
        <el-autocomplete
          v-if="inputVisible"
          ref="InputRef"
          @keyup.enter="addTag"
          @blur="addTag"
          v-model="inputValue"
          :fetch-suggestions="querySearch"
          :trigger-on-focus="false"
          clearable
          placeholder="标签"
          @select="handleSelect"
        />
        <el-button v-else class="button-new-tag" @click="showInput">
          + 新标签
        </el-button>
      </div>
    </div>
    <el-input
      v-model="codeBody"
      :autosize="{ minRows: 2 }"
      type="textarea"
      placeholder="Please input body"
    />
    <el-button
      type="primary"
      @click="submit"
    >提交</el-button>
  </div>
</template>
  
<script lang="ts" setup>
  import {onMounted, reactive, ref, nextTick} from "vue";
  import {invoke} from "@tauri-apps/api/tauri";

  const codeTitle = ref('')
  const lan = ref('');
  // 语言选项
  const options: any[] = reactive([]);
  // 添加标签输入框
  const inputValue = ref('')
  // 标签
  const tags = ref<Tag[]>([])
  const inputVisible = ref(false)
  const InputRef = ref()
  let allTags: Tag[] = []
  const codeBody = ref('')

  // 标签实体
  // id: 标签id
  // value: 标签内容
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
  // ！==-1实现模糊查询
  const createFilter = (queryString: string) => {
    return (restaurant: Tag) => {
      return (
        restaurant.value.toLowerCase().indexOf(queryString.toLowerCase()) !== -1
      )
    }
  }

  const handleSelect = (item: Tag) => {
    console.log(item)
  }

  const handleClose = (tag: Tag) => {
    tags.value.splice(tags.value.indexOf(tag), 1);
    console.log(tags.value); 
  }

  const showInput = () => {
    inputVisible.value = true
    nextTick(() => {
      InputRef.value.focus()
    })
  }

  const addTag = async () => {
    // 当输入的值不在已添加的tag时才可以添加
    if (tags.value.map(item=>item.value).indexOf(inputValue.value)===-1) {
      let ctag = {
        id: 0,
        value: inputValue.value,
        flag: 0
      };
      tags.value.push(ctag)
      // 后端方法
      await invoke<number>("add_tag", {
        value: inputValue.value
      }).then(async (id) => {
        ctag.id = id;
        console.log(tags.value);
      });
    }
    inputVisible.value = false
    inputValue.value = ''
  }

  const submit = async () => {
    await invoke<boolean>("add_code", { 
      code: {
        desc: codeTitle.value,
        lan: lan.value,
        tags: tags.value.map( items => items.id),
        body: codeBody.value
      }
    }).then(async (suc) => {
      console.log(suc);
    });
  }

  onMounted(async () => {
    await invoke<string[]>("lans").then(async (s) => {
      s.forEach((item) => {
        options.push({
          value: item,
          label: item
        });
      })
    })
    await invoke<Tag[]>("all_tag").then(async (tags) => { 
    allTags = tags;
    console.log("所有标签:", allTags);
   });
  })
</script>
<style scoped>
  .el-tag{
    margin-right: 5px;
  }
  .input-new-tag {
    width: 88px;
    display: inline-block;
    vertical-align: bottom;
}
</style>
  