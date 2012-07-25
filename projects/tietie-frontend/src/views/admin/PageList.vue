<template>
  <div class="page-list">
    <div class="page-header">
      <h2>页面管理</h2>
      <button class="add-btn" @click="handleAddPage">新增页面</button>
    </div>
    <table class="page-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>标题</th>
          <th>路径</th>
          <th>创建时间</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="page in pages" :key="page.id">
          <td>{{ page.id }}</td>
          <td>{{ page.title }}</td>
          <td>{{ page.path }}</td>
          <td>{{ page.createdAt.toLocaleString() }}</td>
          <td>
            <button class="edit-btn" @click="handleEditPage(page)">编辑</button>
            <button class="delete-btn" @click="handleDeletePage(page)">删除</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Page {
  id: number
  title: string
  path: string
  createdAt: Date
}

const pages = ref<Page[]>([
  {
    id: 1,
    title: '首页',
    path: '/',
    createdAt: new Date()
  }
])

const handleAddPage = () => {
  // 实现新增页面逻辑
}

const handleEditPage = (page: Page) => {
  // 实现编辑页面逻辑
}

const handleDeletePage = (page: Page) => {
  // 实现删除页面逻辑
}
</script>

<style lang="scss" scoped>
.page-list {
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;

    h2 {
      margin: 0;
    }

    .add-btn {
      padding: 0.5rem 1rem;
      background-color: #42b983;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;

      &:hover {
        background-color: #3aa876;
      }
    }
  }

  .page-table {
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