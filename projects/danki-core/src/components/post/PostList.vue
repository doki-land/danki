<template>
  <div class="post-list">
    <div v-for="post in posts" :key="post.id" class="post-item">
      <h3>{{ post.title }}</h3>
      <p>{{ post.content }}</p>
      <div class="post-meta">
        <span>作者: {{ post.author.username }}</span>
        <span>点赞: {{ post.likes }}</span>
        <span>评论: {{ post.comments.length }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '../../index'

defineProps<{
  posts: Post[]
}>()
</script>



<style lang="scss" scoped>
.post-list {
  .post-item {
    margin-bottom: 1.5rem;
    padding: 1rem;
    border: 1px solid #eee;
    border-radius: 4px;

    h3 {
      margin: 0 0 1rem;
      font-size: 1.25rem;
    }

    p {
      margin: 0 0 1rem;
      line-height: 1.5;
    }

    .post-meta {
      display: flex;
      gap: 1rem;
      color: #666;
      font-size: 0.9rem;
    }
  }
}
</style>