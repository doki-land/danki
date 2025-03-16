import { defineComponent } from 'vue'
import type { User } from '../../index'

// 用户信息组件
export const UserInfo = defineComponent({
  name: 'UserInfo',
  props: {
    user: {
      type: Object as () => User,
      required: true
    }
  },
  setup(props) {
    return () => (
      <div class="user-info">
        <img src={props.user.avatar} alt="avatar" class="user-avatar" />
        <div class="user-meta">
          <span class="username">{props.user.username}</span>
          <span class="join-time">加入时间：{props.user.createdAt.toLocaleString()}</span>
        </div>
      </div>
    )
  }
})

// 用户列表组件
export const UserList = defineComponent({
  name: 'UserList',
  props: {
    users: {
      type: Array as () => User[],
      required: true
    }
  },
  setup(props) {
    return () => (
      <div class="user-list">
        {props.users.map(user => (
          <div key={user.id} class="user-item">
            <UserInfo user={user} />
          </div>
        ))}
      </div>
    )
  }
})