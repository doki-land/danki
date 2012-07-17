<script setup lang="ts">
import type { User } from '../../index'
import { UserInfo } from './UserInfo.vue'

defineProps<{
  users: User[]
}>()
</script>

<template>
  <div class="user-list">
    <div v-for="user in users" :key="user.id" class="user-item">
      <UserInfo :user="user" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.user-list {
  .user-item {
    margin-bottom: 1rem;
  }
}
</style>