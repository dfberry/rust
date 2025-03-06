# Query Postgresql JsonB column with Diesel ORM

Using a custom sql statement, returning data out of JSON (not the Jsonb itself).

## Indexes

```sql
CREATE INDEX IF NOT EXISTS idx_org_repo ON public.osb_github_logfiles(org_repo);
CREATE INDEX IF NOT EXISTS idx_created_at ON public.osb_github_logfiles(created_at);
```

## Diesel schem

```rs
diesel::table! {
    osb_github_logfiles (id) {
        id -> Uuid,
        org_repo -> Text,
        logfile -> Jsonb,
        created_at -> Timestamptz,
    }
}
```

## JSONB object

```json
{
  "org": "azure-samples",
  "url": "https://github.com/Azure-Samples/js-e2e",
  "name": "js-e2e",
  "repo": "js-e2e",
  "log_time": "2025-03-06T00:29:59.024695851+00:00",
  "pushedAt": "2023-07-07T18:01:50Z",
  "createdAt": "2021-02-05T21:34:01Z",
  "diskUsage": 1298,
  "updatedAt": "2024-06-23T07:07:00Z",
  "forksCount": 18,
  "description": "JavaScript end-to-end",
  "openPRsCount": 3,
  "watchersCount": 13,
  "openIssuesCount": 1,
  "stargazersCount": 9
}
```

## Example JSONB queries

```sql
SELECT
  id,
  org_repo,
  logfile -> 'openIssuesCount' openIssuesCount,
  created_at
FROM
  public.osb_github_logfiles
WHERE
  org_repo = 'azure-samples/azure-typescript-e2e-apps'
ORDER BY
  created_at DESC
```

```sql
WITH T as (
  SELECT
  org_repo,
  logfile
  created_at
FROM
  public.osb_github_logfiles
WHERE
  org_repo = 'azure-samples/azure-typescript-e2e-apps'
ORDER BY
  created_at DESC
  LIMIT 30
)
SELECT
logfile ->> 'diskUsage' as "disk_usage",
logfile ->> 'forkCounts' as "fork_count",
logfile ->> 'openPRsCount' as "open_prs_count",
logfile ->> 'watchesCount' as "watches_count",
logfile ->> 'openIssuesCount' as "issues_count",
logfile ->> 'starsCount' as "stars_count",
created_at
from T
ORDER BY created_at DESC;
```

## HTTP request

```http
GET http://localhost:3001
```

## HTTP response

```
HTTP/1.1 200 OK
content-type: application/json
content-length: 1921
connection: close
date: Thu, 06 Mar 2025 14:40:30 GMT

[
  {
    "disk_usage": "1417",
    "created_at": "2025-03-06T00:29:54.822699"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-03-05T00:29:58.404899"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-03-04T00:29:33.168411"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-03-03T00:30:11.236243"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-03-02T00:31:15.672126"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-03-01T00:31:31.827998"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-28T00:29:17.302565"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-27T00:28:58.279227"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-26T00:28:58.770971"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-25T00:29:22.745950"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-24T00:29:41.004819"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-23T00:30:57.535252"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-22T00:27:37.793738"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-21T00:28:42.600424"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-20T00:29:11.119951"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-19T00:28:42.349562"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-18T00:28:16.791162"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-17T00:29:57.221307"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-16T00:30:52.669536"
  },
  {
    "disk_usage": "1417",
    "created_at": "2025-02-15T00:28:09.848712"
  },
  {
    "disk_usage": "1401",
    "created_at": "2025-02-14T02:18:09.374818"
  },
  {
    "disk_usage": "1401",
    "created_at": "2025-02-14T00:26:05.317473"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-13T00:26:17.302849"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-12T00:26:09.004316"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-11T00:26:09.856009"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-10T00:27:07.199429"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-09T00:28:09.857041"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-08T00:25:24.142630"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-07T00:26:20.796519"
  },
  {
    "disk_usage": "1421",
    "created_at": "2025-02-06T00:26:19.596837"
  }
]
```