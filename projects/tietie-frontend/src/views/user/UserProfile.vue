<template>
  <div class="user-profile">
    <div class="user-header">
      <user-info :user="user" />
    </div>
    <div class="user-content">
      <h2 class="section-title">用户动态</h2>
      <post-list :posts="userPosts" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import type { User, Post } from '@/types'
import { UserInfo, PostList } from '@/components'

const route = useRoute()
const userId = route.params.userId

const user = ref<User>({
  id: 1,
  username: '示例用户',
  avatar: 'https://via.placeholder.com/48',
  createdAt: new Date()
})

const userPosts = ref<Post[]>([
  {
    id: 1,
    title: '示例帖子',
    content: '这是一个示例帖子内容',
    createdAt: new Date(),
    author: user.value
  }
])

onMounted(() => {
  // TODO: 根据userId获取用户信息和帖子列表
  console.log('Fetching user data for:', userId)
})
</script>

<style lang="scss" scoped>
.user-profile {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;

  .user-header {
    margin-bottom: 2rem;
  }

  .section-title {
    font-size: 1.5rem;
    color: #333;
    margin-bottom: 1rem;
  }

  .user-content {
    background-color: #fff;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
}
</style>