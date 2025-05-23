use crate::db::repo::NewsRepo;
use crate::db::models::NewsModel;

#[derive(Clone)]  // 新增 Clone 派生
pub struct NewsDao {
    repo: NewsRepo,
}

impl NewsDao {
    pub fn new(repo: NewsRepo) -> Self {
        Self { repo }
    }

    pub async fn create_news(
        &self,
        news_type: &str,
        href: &str,
        title: &str,
        content: &str,
    ) -> Result<NewsModel, sqlx::Error> {
        self.repo
            .create_news(news_type, href, title, content)
            .await
    }

    pub async fn get_paginated(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<NewsModel>, sqlx::Error> {
        self.repo
            .get_paginated(page, page_size)
            .await
    }
}


// 在 news/dao.rs 底部添加以下测试代码
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_pool;

    #[actix_rt::test]
    async fn test_dao_create_news() {
        let pool = init_pool().await.unwrap();
        let repo = NewsRepo::new(pool.clone());
        let dao = NewsDao::new(repo);

        let mut tx = pool.begin().await.unwrap();

        // 测试 DAO 方法
        let result = dao
            .create_news("tech", "https://dao.test", "DAO Test", "Content")
            .await;
        assert!(result.is_ok());

        tx.rollback().await.unwrap();
    }
}