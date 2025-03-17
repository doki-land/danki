<template>
  <div class="post-detail">
    <h2>{{ post.title }}</h2>
    <div class="post-content">{{ post.content }}</div>
    <div class="post-meta">
      <img :src="post.author.avatar" alt="avatar" class="author-avatar" />
      <span class="author-name">{{ post.author.username }}</span>
      <span class="post-time">{{ post.createdAt.toLocaleString() }}</span>
      <span class="post-likes">点赞: {{ post.likes }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '../../index'

defineProps<{
  post: Post
}>()
</script>



<style lang="scss" scoped>
.post-detail {
  padding: 1rem;
  border: 1px solid #eee;
  border-radius: 4px;

  h2 {
    margin: 0 0 1rem;
    font-size: 1.5rem;
  }

  .post-content {
    margin-bottom: 1rem;
    line-height: 1.6;
  }

  .post-meta {
    display: flex;
    align-items: center;
    gap: 1rem;
    color: #666;
    font-size: 0.9rem;

    .author-avatar {
      width: 32px;
      height: 32px;
      border-radius: 50%;
    }

    .author-name {
      font-weight: bold;
    }
  }
}
</style>