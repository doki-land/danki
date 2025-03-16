<template>
  <div class="comment-list">
    <div v-for="comment in comments" :key="comment.id" class="comment-item">
      <div class="comment-header">
        <img :src="comment.author.avatar" alt="avatar" class="author-avatar"/>
        <span class="author-name">{{ comment.author.username }}</span>
        <span class="comment-time">{{ comment.createdAt.toLocaleString() }}</span>
      </div>
      <div class="comment-content">{{ comment.content }}</div>
      <div class="comment-footer">
        <span class="comment-likes">点赞: {{ comment.likes }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {Comment} from '../../index'

defineProps<{
  comments: Comment[]
}>()
</script>


<style lang="scss" scoped>
.comment-list {
  .comment-item {
    margin-bottom: 1rem;
    padding: 1rem;
    border-bottom: 1px solid #eee;

    .comment-header {
      display: flex;
      align-items: center;
      margin-bottom: 0.5rem;

      .author-avatar {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        margin-right: 0.5rem;
      }

      .author-name {
        font-weight: bold;
        margin-right: 0.5rem;
      }

      .comment-time {
        color: #666;
        font-size: 0.9rem;
      }
    }

    .comment-content {
      margin: 0.5rem 0;
      line-height: 1.5;
    }

    .comment-footer {
      color: #666;
      font-size: 0.9rem;
    }
  }
}
</style>