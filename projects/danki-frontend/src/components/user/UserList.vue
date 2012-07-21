<template>
  <div class="user-list">
    <div v-for="user in users" :key="user.id" class="user-item">
      <user-info :user="user"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {User} from '@/types'
import {UserInfo} from "@/components";

defineProps<{
  users: User[]
}>()
</script>


<style lang="scss" scoped>
.user-list {
  .user-item {
    margin-bottom: 1rem;
  }
}
</style>