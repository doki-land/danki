import { defineComponent } from 'vue'
import type { Post } from '../../index'

// 帖子列表组件
export const PostList = defineComponent({
  name: 'PostList',
  props: {
    posts: {
      type: Array as () => Post[],
      required: true
    }
  },
  setup(props) {
    return () => (
      <div class="post-list">
        {props.posts.map(post => (
          <div key={post.id} class="post-item">
            <h3>{post.title}</h3>
            <p>{post.content}</p>
            <div class="post-meta">
              <span>作者: {post.author.username}</span>
              <span>点赞: {post.likes}</span>
              <span>评论: {post.comments.length}</span>
            </div>
          </div>
        ))}
      </div>
    )
  }
})

// 帖子详情组件
export const PostDetail = defineComponent({
  name: 'PostDetail',
  props: {
    post: {
      type: Object as () => Post,
      required: true
    }
  },
  setup(props) {
    return () => (
      <div class="post-detail">
        <h2>{props.post.title}</h2>
        <div class="post-content">{props.post.content}</div>
        <div class="post-meta">
          <img src={props.post.author.avatar} alt="avatar" class="author-avatar" />
          <span class="author-name">{props.post.author.username}</span>
          <span class="post-time">{props.post.createdAt.toLocaleString()}</span>
          <span class="post-likes">点赞: {props.post.likes}</span>
        </div>
      </div>
    )
  }
})