```markdown
# 新闻平台系统文档

## 运行命令
```bash
$env:DATABASE_URL="postgres://postgres:your_password@localhost/News"; cargo run
```

## 运行前准备
1. 在 PostgreSQL 数据库中建立数据表：
   - 新闻表 `news`
   - 用户表 `users`
   > 表结构详见：`/src/db/PostgreSQL_table.md`
   
2. 数据爬取与处理：
   - 进入 `src/spider/` 目录
   - 根据目录内`文件说明.md`执行爬取
   - 将数据插入数据库
   - 整理数据确保**新数据优先展示**

## 平台功能展示


### 核心功能
- **时间感知**
- <img width="441" alt="image" src="https://github.com/user-attachments/assets/689ba07e-0382-4e82-a020-782347f8866c" />

  - 右上角实时日期显示
  - 新闻按时效性排序
- **智能栏目**
- <img width="471" alt="image" src="https://github.com/user-attachments/assets/16b229b3-14f2-4ad4-b74b-2a9f442065cc" />

  - 分类栏目切换
  - 进入栏目自动更新题头
- **交互优化**
- <img width="171" alt="image" src="https://github.com/user-attachments/assets/93b1fc29-8f2f-4194-b970-6a0651389f0f" />

- <img width="415" alt="image" src="https://github.com/user-attachments/assets/6900de47-9ac3-49cf-b9cc-cfae60b099a2" />

  - 动态浮动分类块
  - 脉冲色彩条视觉引导
  - 标题变色效果
- **分页系统**
- <img width="472" alt="image" src="https://github.com/user-attachments/assets/8e646877-dea3-465f-b051-6bd3602f8901" />

- <img width="452" alt="image" src="https://github.com/user-attachments/assets/62d6ab55-6c89-49f7-86f2-5166c4f5b8a4" />

  - 自动居中当前页码
  - 流畅翻页体验
- **数据实时性**
  - 爬虫针对腾讯新闻持续更新
  - 最新新闻优先展示
- **用户系统**
- <img width="424" alt="image" src="https://github.com/user-attachments/assets/eca0a265-76df-4f24-a632-8d344f065ae7" />

- <img width="408" alt="image" src="https://github.com/user-attachments/assets/78a531b9-6d86-4455-b500-1b8e0a2355cd" />

  - 注册邮箱格式校验
  - 防止非法邮箱地址
- **新闻浏览**
  - 点击跳转至腾讯新闻源页面

> 注：所有功能需在完成数据库初始化后生效
```
