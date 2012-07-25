<template>
  <div class="admin-layout">
    <aside class="sidebar">
      <nav class="nav-menu">
        <router-link to="/admin/pages" class="nav-item">页面管理</router-link>
        <router-link to="/admin/users" class="nav-item">用户管理</router-link>
      </nav>
    </aside>
    <main class="main-content">
      <router-view></router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
// 组件逻辑
</script>

<style lang="scss" scoped>
.admin-layout {
  display: flex;
  min-height: 100vh;

  .sidebar {
    width: 200px;
    background-color: #2c3e50;
    padding: 2rem 0;

    .nav-menu {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      
      .nav-item {
        padding: 0.75rem 1.5rem;
        text-decoration: none;
        color: #fff;
        transition: all 0.3s ease;

        &:hover {
          background-color: #34495e;
        }

        &.router-link-active {
          background-color: #34495e;
          border-left: 4px solid #42b983;
        }
      }
    }
  }

  .main-content {
    flex: 1;
    padding: 2rem;
    background-color: #f5f5f5;
  }
}
</style>