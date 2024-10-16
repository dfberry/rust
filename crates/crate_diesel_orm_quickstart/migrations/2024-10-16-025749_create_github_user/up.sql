-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "github_user_sessions" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"expires_at" timestamp with time zone NOT NULL,
    "created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "github_user_tokens" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"encrypted_access_token" text NOT NULL,
    "created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "github_users" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"github_id" text NOT NULL,
	"username" text NOT NULL,
    "created_at" timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE "github_users" ADD CONSTRAINT "github_user_id_unique" UNIQUE("id");--> statement-breakpoint
ALTER TABLE "github_users" ADD CONSTRAINT "github_user_github_id_unique" UNIQUE("github_id");
ALTER TABLE "github_users" ADD CONSTRAINT "github_user_username_unique" UNIQUE("username");

CREATE TABLE IF NOT EXISTS "app_user_watch_repos" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"repo_name" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
ALTER TABLE "app_user_watch_repos" ADD CONSTRAINT "app_user_watch_repo_user_id_github_user_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."github_users"("id") ON DELETE no action ON UPDATE no action;
CREATE UNIQUE INDEX IF NOT EXISTS "unique_user_repo" ON "app_user_watch_repos" USING btree ("user_id","repo_name");