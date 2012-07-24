//! Integration tests for the ORM

use danki_orm::{
    connect_test_db,
    entity::prelude::*,
    migration,
    repository::{user::UserRepository, post::PostRepository, comment::CommentRepository},
};

#[tokio::test]
async fn test_user_crud() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to in-memory SQLite database
    let db = connect_test_db().await?;
    
    // Apply migrations
    migration::migrate(&db).await?;
    
    // Create user repository
    let repo = UserRepository::new(db.clone());
    
    // Create a user
    let user = repo.create(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "password_hash".to_string(),
        Some("https://example.com/avatar.jpg".to_string()),
        Some("Test user bio".to_string()),
    ).await?;
    
    // Verify user was created
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    
    // Find user by ID
    let found_user = repo.find_by_id(user.id).await?.unwrap();
    assert_eq!(found_user.id, user.id);
    
    // Find user by username
    let found_user = repo.find_by_username("testuser").await?.unwrap();
    assert_eq!(found_user.id, user.id);
    
    // Find user by email
    let found_user = repo.find_by_email("test@example.com").await?.unwrap();
    assert_eq!(found_user.id, user.id);
    
    // Update user
    let mut update_model = user.clone().into_active_model();
    update_model.bio = Set(Some("Updated bio".to_string()));
    let updated_user = repo.update(user.id, update_model).await?;
    assert_eq!(updated_user.bio, Some("Updated bio".to_string()));
    
    // Delete user
    repo.delete(user.id).await?;
    
    // Verify user was deleted
    let deleted_user = repo.find_by_id(user.id).await?;
    assert!(deleted_user.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_post_crud() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to in-memory SQLite database
    let db = connect_test_db().await?;
    
    // Apply migrations
    migration::migrate(&db).await?;
    
    // Create repositories
    let user_repo = UserRepository::new(db.clone());
    let post_repo = PostRepository::new(db.clone());
    
    // Create a user first
    let user = user_repo.create(
        "postuser".to_string(),
        "post@example.com".to_string(),
        "password_hash".to_string(),
        None,
        None,
    ).await?;
    
    // Create a post
    let post = post_repo.create(
        "Test Post".to_string(),
        "This is a test post content".to_string(),
        user.id,
    ).await?;
    
    // Verify post was created
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.user_id, user.id);
    
    // Find post by ID
    let found_post = post_repo.find_by_id(post.id).await?.unwrap();
    assert_eq!(found_post.id, post.id);
    
    // Find posts by user ID
    let user_posts = post_repo.find_by_user_id(user.id).await?;
    assert_eq!(user_posts.len(), 1);
    assert_eq!(user_posts[0].id, post.id);
    
    // Update post
    let mut update_model = post.clone().into_active_model();
    update_model.content = Set("Updated content".to_string());
    let updated_post = post_repo.update(post.id, update_model).await?;
    assert_eq!(updated_post.content, "Updated content");
    
    // Delete post
    post_repo.delete(post.id).await?;
    
    // Verify post was deleted
    let deleted_post = post_repo.find_by_id(post.id).await?;
    assert!(deleted_post.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_comment_crud() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to in-memory SQLite database
    let db = connect_test_db().await?;
    
    // Apply migrations
    migration::migrate(&db).await?;
    
    // Create repositories
    let user_repo = UserRepository::new(db.clone());
    let post_repo = PostRepository::new(db.clone());
    let comment_repo = CommentRepository::new(db.clone());
    
    // Create a user first
    let user = user_repo.create(
        "commentuser".to_string(),
        "comment@example.com".to_string(),
        "password_hash".to_string(),
        None,
        None,
    ).await?;
    
    // Create a post
    let post = post_repo.create(
        "Comment Test Post".to_string(),
        "This is a post for testing comments".to_string(),
        user.id,
    ).await?;
    
    // Create a parent comment
    let parent_comment = comment_repo.create(
        "Parent comment".to_string(),
        user.id,
        post.id,
        None,
    ).await?;
    
    // Create a reply comment
    let reply_comment = comment_repo.create(
        "Reply comment".to_string(),
        user.id,
        post.id,
        Some(parent_comment.id),
    ).await?;
    
    // Verify comments were created
    assert_eq!(parent_comment.content, "Parent comment");
    assert_eq!(reply_comment.content, "Reply comment");
    assert_eq!(reply_comment.parent_id, Some(parent_comment.id));
    
    // Find comment by ID
    let found_comment = comment_repo.find_by_id(parent_comment.id).await?.unwrap();
    assert_eq!(found_comment.id, parent_comment.id);
    
    // Find comments by post ID
    let post_comments = comment_repo.find_by_post_id(post.id).await?;
    assert_eq!(post_comments.len(), 2);
    
    // Find comments by user ID
    let user_comments = comment_repo.find_by_user_id(user.id).await?;
    assert_eq!(user_comments.len(), 2);
    
    // Find replies to a comment
    let replies = comment_repo.find_replies(parent_comment.id).await?;
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].id, reply_comment.id);
    
    // Update comment
    let mut update_model = parent_comment.clone().into_active_model();
    update_model.content = Set("Updated parent comment".to_string());
    let updated_comment = comment_repo.update(parent_comment.id, update_model).await?;
    assert_eq!(updated_comment.content, "Updated parent comment");
    
    // Delete comments
    comment_repo.delete(reply_comment.id).await?;
    comment_repo.delete(parent_comment.id).await?;
    
    // Verify comments were deleted
    let deleted_comment = comment_repo.find_by_id(parent_comment.id).await?;
    assert!(deleted_comment.is_none());
    
    Ok(())
}