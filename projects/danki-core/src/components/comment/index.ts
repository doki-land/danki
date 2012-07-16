import { defineComponent, ref } from 'vue'
import type { Comment } from '../../index'

// 评论列表组件
export const CommentList = defineComponent({
  name: 'CommentList',
  props: {
    comments: {
      type: Array as () => Comment[],
      required: true
    }
  },
  setup(props) {
    return () => (
      <div class="comment-list">
        {props.comments.map(comment => (
          <div key={comment.id} class="comment-item">
            <div class="comment-header">
              <img src={comment.author.avatar} alt="avatar" class="author-avatar" />
              <span class="author-name">{comment.author.username}</span>
              <span class="comment-time">{comment.createdAt.toLocaleString()}</span>
            </div>
            <div class="comment-content">{comment.content}</div>
            <div class="comment-footer">
              <span class="comment-likes">点赞: {comment.likes}</span>
            </div>
          </div>
        ))}
      </div>
    )
  }
})

// 评论编辑组件
export const CommentEditor = defineComponent({
  name: 'CommentEditor',
  emits: ['submit'],
  setup(_, { emit }) {
    const content = ref('')

    const handleSubmit = () => {
      if (content.value.trim()) {
        emit('submit', content.value)
        content.value = ''
      }
    }

    return () => (
      <div class="comment-editor">
        <textarea
          v-model={content.value}
          placeholder="请输入评论内容"
          class="comment-textarea"
        />
        <button onClick={handleSubmit} class="submit-btn">
          发表评论
        </button>
      </div>
    )
  }
})