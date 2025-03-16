<template>
  <div class="comment-editor">
    <textarea
        v-model="content"
        placeholder="请输入评论内容"
        class="comment-textarea"
    />
    <button @click="handleSubmit" class="submit-btn">
      发表评论
    </button>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'

const emit = defineEmits<{
  submit: [content: string]
}>()

const content = ref('')

const handleSubmit = () => {
  if (content.value.trim()) {
    emit('submit', content.value)
    content.value = ''
  }
}
</script>


<style lang="scss" scoped>
.comment-editor {
  padding: 1rem;
  border: 1px solid #eee;
  border-radius: 4px;

  .comment-textarea {
    width: 100%;
    min-height: 100px;
    padding: 0.5rem;
    margin-bottom: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    resize: vertical;

    &:focus {
      outline: none;
      border-color: #666;
    }
  }

  .submit-btn {
    padding: 0.5rem 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;

    &:hover {
      background-color: #0056b3;
    }
  }
}
</style>