SELECT id, org_repo, logfile, created_at
	FROM public.osb_github_logfiles;

SELECT 
	logfile->>'createdAt' as createdAt,
	logfile->>'diskUsage' as diskUsage,
	logfile->>'forksCount' as forksCount,
	logfile->>'openPRsCount' as openPRsCount,
	logfile->>'watchersCount' as watchersCount,
	logfile->>'openIssuesCount' as openIssuesCount,
	logfile->>'stargazersCount' as stargazersCount,
	created_at
FROM public.osb_github_logfiles
WHERE
    logfile->>'org' ILIKE 'MicrosoftDocs' AND
    logfile->>'repo' ILIKE 'node-essentials'
ORDER BY created_at desc
LIMIT 30;

SELECT 
    key,
    value
FROM 
    public.osb_github_logfiles,
    jsonb_each_text(logfile)
WHERE
    logfile->>'org' ILIKE 'MicrosoftDocs' AND
    logfile->>'repo' ILIKE 'node-essentials'
ORDER BY created_at DESC
LIMIT 30;