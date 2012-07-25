import type {RouteRecordRaw} from 'vue-router'
import {createRouter, createWebHistory} from 'vue-router'
import {PostDetail, PostList, UserList} from "@/components";
import HomePage from '@/views/HomePage.vue';
import AdminLayout from '@/views/admin/AdminLayout.vue';
import AdminPageList from '@/views/admin/PageList.vue';
import AdminUserList from '@/views/admin/UserList.vue';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: HomePage
    },
    {
        path: '/users',
        name: 'UserList',
        component: UserList
    },
    {
        path: '/posts',
        name: 'PostList',
        component: PostList
    },
    {
        path: '/posts/:id',
        name: 'post-detail',
        component: PostDetail
    },
    {
        path: '/admin',
        component: AdminLayout,
        children: [
            {
                path: 'pages',
                name: 'admin-pages',
                component: AdminPageList
            },
            {
                path: 'users',
                name: 'admin-users',
                component: AdminUserList
            }
        ]
    }
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})