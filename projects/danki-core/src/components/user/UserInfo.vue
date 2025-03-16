<template>
  <div class="user-info">
    <img :src="user.avatar" alt="avatar" class="user-avatar"/>
    <div class="user-meta">
      <span class="username">{{ user.username }}</span>
      <span class="join-time">加入时间：{{ user.createdAt.toLocaleString() }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {User} from '../../index'

defineProps<{
  user: User
}>()
</script>


<style lang="scss" scoped>
.user-info {
  display: flex;
  align-items: center;
  padding: 1rem;
  border: 1px solid #eee;
  border-radius: 4px;

  .user-avatar {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    margin-right: 1rem;
  }

  .user-meta {
    display: flex;
    flex-direction: column;

    .username {
      font-weight: bold;
      font-size: 1.1rem;
      margin-bottom: 0.25rem;
    }

    .join-time {
      color: #666;
      font-size: 0.9rem;
    }
  }
}
</style>