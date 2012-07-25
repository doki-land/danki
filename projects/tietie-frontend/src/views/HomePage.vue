<template>
  <div class="home-page">
    <nav class="nav-menu">
      <router-link to="/users" class="nav-item">用户列表</router-link>
      <router-link to="/posts" class="nav-item">帖子列表</router-link>
    </nav>
    <div class="welcome-section">
      <h1>欢迎来到 TieTie</h1>
      <p>这是一个简单的社交平台，您可以在这里浏览用户和帖子。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 组件逻辑
</script>

<style lang="scss" scoped>
.home-page {
  padding: 2rem;

  .nav-menu {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    
    .nav-item {
      padding: 0.5rem 1rem;
      text-decoration: none;
      color: #333;
      border: 1px solid #ddd;
      border-radius: 4px;
      transition: all 0.3s ease;

      &:hover {
        background-color: #f5f5f5;
      }

      &.router-link-active {
        background-color: #e6e6e6;
        border-color: #ccc;
      }
    }
  }

  .welcome-section {
    text-align: center;
    
    h1 {
      margin-bottom: 1rem;
      color: #333;
    }

    p {
      color: #666;
      font-size: 1.1rem;
    }
  }
}
</style>