-- This file should undo anything in `up.sql`
-- Drop the unique index on app_user_watch_repos
DROP INDEX IF EXISTS "unique_user_repo";

-- Drop the foreign key constraint on app_user_watch_repos
ALTER TABLE "app_user_watch_repos" DROP CONSTRAINT IF EXISTS "app_user_watch_repo_user_id_github_user_id_fk";

-- Drop the app_user_watch_repos table
DROP TABLE IF EXISTS "app_user_watch_repos";

-- Drop the unique constraints on github_users
ALTER TABLE "github_users" DROP CONSTRAINT IF EXISTS "github_user_id_unique";
ALTER TABLE "github_users" DROP CONSTRAINT IF EXISTS "github_user_github_id_unique";
ALTER TABLE "github_users" DROP CONSTRAINT IF EXISTS "github_user_username_unique";

-- Drop the github_users table
DROP TABLE IF EXISTS "github_users";

-- Drop the github_user_tokens table
DROP TABLE IF EXISTS "github_user_tokens";

-- Drop the github_user_sessions table
DROP TABLE IF EXISTS "github_user_sessions";