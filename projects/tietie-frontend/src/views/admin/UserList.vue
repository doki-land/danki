<template>
  <div class="user-list-admin">
    <div class="page-header">
      <h2>用户管理</h2>
    </div>
    <table class="user-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>用户名</th>
          <th>头像</th>
          <th>注册时间</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users" :key="user.id">
          <td>{{ user.id }}</td>
          <td>{{ user.username }}</td>
          <td>
            <img :src="user.avatar" alt="avatar" class="user-avatar"/>
          </td>
          <td>{{ user.createdAt.toLocaleString() }}</td>
          <td>
            <button class="edit-btn" @click="handleEditUser(user)">编辑</button>
            <button class="delete-btn" @click="handleDeleteUser(user)">删除</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { User } from '@/types'

const users = ref<User[]>([
  {
    id: 1,
    username: '测试用户',
    avatar: 'https://via.placeholder.com/48',
    createdAt: new Date()
  }
])

const handleEditUser = (user: User) => {
  // 实现编辑用户逻辑
}

const handleDeleteUser = (user: User) => {
  // 实现删除用户逻辑
}
</script>

<style lang="scss" scoped>
.user-list-admin {
  .page-header {
    margin-bottom: 2rem;

    h2 {
      margin: 0;
    }
  }

  .user-table {
    width: 100%;
    border-collapse: collapse;

    th, td {
      padding: 1rem;
      text-align: left;
      border-bottom: 1px solid #eee;
    }

    th {
      background-color: #f8f9fa;
      font-weight: 600;
    }

    .user-avatar {
      width: 32px;
      height: 32px;
      border-radius: 50%;
    }

    .edit-btn, .delete-btn {
      padding: 0.25rem 0.5rem;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      margin-right: 0.5rem;
    }

    .edit-btn {
      background-color: #ffc107;
      color: #000;

      &:hover {
        background-color: #e0a800;
      }
    }

    .delete-btn {
      background-color: #dc3545;
      color: white;

      &:hover {
        background-color: #c82333;
      }
    }
  }
}
</style>