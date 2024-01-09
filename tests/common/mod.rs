pub const ISSUE_PAYLOAD: &str = "
{
    \"action\": \"opened\",
    \"issue\": {
        \"url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3\",
        \"repository_url\": \"https://api.github.com/repos/JerboaBurrow/test\",
        \"labels_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3/labels{/name}\",
        \"comments_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3/comments\",
        \"events_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3/events\",
        \"html_url\": \"https://github.com/JerboaBurrow/test/issues/3\",
        \"id\": 2064624373,
        \"node_id\": \"I_kwDOLAecns57D6r1\",
        \"number\": 3,
        \"title\": \"a\",
        \"user\": {
        \"login\": \"Jerboa-app\",
        \"id\": 84378622,
        \"node_id\": \"MDQ6VXNlcjg0Mzc4NjIy\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/84378622?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/Jerboa-app\",
        \"html_url\": \"https://github.com/Jerboa-app\",
        \"followers_url\": \"https://api.github.com/users/Jerboa-app/followers\",
        \"following_url\": \"https://api.github.com/users/Jerboa-app/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/Jerboa-app/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/Jerboa-app/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/Jerboa-app/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/Jerboa-app/orgs\",
        \"repos_url\": \"https://api.github.com/users/Jerboa-app/repos\",
        \"events_url\": \"https://api.github.com/users/Jerboa-app/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/Jerboa-app/received_events\",
        \"type\": \"User\",
        \"site_admin\": false
        },
        \"labels\": [
    
        ],
        \"state\": \"open\",
        \"locked\": false,
        \"assignee\": null,
        \"assignees\": [
    
        ],
        \"milestone\": null,
        \"comments\": 0,
        \"created_at\": \"2024-01-03T20:44:50Z\",
        \"updated_at\": \"2024-01-03T20:44:50Z\",
        \"closed_at\": null,
        \"author_association\": \"NONE\",
        \"active_lock_reason\": null,
        \"body\": null,
        \"reactions\": {
        \"url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3/reactions\",
        \"total_count\": 0,
        \"+1\": 0,
        \"-1\": 0,
        \"laugh\": 0,
        \"hooray\": 0,
        \"confused\": 0,
        \"heart\": 0,
        \"rocket\": 0,
        \"eyes\": 0
        },
        \"timeline_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/3/timeline\",
        \"performed_via_github_app\": null,
        \"state_reason\": null
    },
    \"repository\": {
        \"id\": 738696350,
        \"node_id\": \"R_kgDOLAecng\",
        \"name\": \"test\",
        \"full_name\": \"JerboaBurrow/test\",
        \"private\": true,
        \"owner\": {
        \"login\": \"JerboaBurrow\",
        \"id\": 109722648,
        \"node_id\": \"O_kgDOBoo8GA\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/109722648?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/JerboaBurrow\",
        \"html_url\": \"https://github.com/JerboaBurrow\",
        \"followers_url\": \"https://api.github.com/users/JerboaBurrow/followers\",
        \"following_url\": \"https://api.github.com/users/JerboaBurrow/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/JerboaBurrow/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/JerboaBurrow/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/JerboaBurrow/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/JerboaBurrow/orgs\",
        \"repos_url\": \"https://api.github.com/users/JerboaBurrow/repos\",
        \"events_url\": \"https://api.github.com/users/JerboaBurrow/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/JerboaBurrow/received_events\",
        \"type\": \"Organization\",
        \"site_admin\": false
        },
        \"html_url\": \"https://github.com/JerboaBurrow/test\",
        \"description\": null,
        \"fork\": false,
        \"url\": \"https://api.github.com/repos/JerboaBurrow/test\",
        \"forks_url\": \"https://api.github.com/repos/JerboaBurrow/test/forks\",
        \"keys_url\": \"https://api.github.com/repos/JerboaBurrow/test/keys{/key_id}\",
        \"collaborators_url\": \"https://api.github.com/repos/JerboaBurrow/test/collaborators{/collaborator}\",
        \"teams_url\": \"https://api.github.com/repos/JerboaBurrow/test/teams\",
        \"hooks_url\": \"https://api.github.com/repos/JerboaBurrow/test/hooks\",
        \"issue_events_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/events{/number}\",
        \"events_url\": \"https://api.github.com/repos/JerboaBurrow/test/events\",
        \"assignees_url\": \"https://api.github.com/repos/JerboaBurrow/test/assignees{/user}\",
        \"branches_url\": \"https://api.github.com/repos/JerboaBurrow/test/branches{/branch}\",
        \"tags_url\": \"https://api.github.com/repos/JerboaBurrow/test/tags\",
        \"blobs_url\": \"https://api.github.com/repos/JerboaBurrow/test/git/blobs{/sha}\",
        \"git_tags_url\": \"https://api.github.com/repos/JerboaBurrow/test/git/tags{/sha}\",
        \"git_refs_url\": \"https://api.github.com/repos/JerboaBurrow/test/git/refs{/sha}\",
        \"trees_url\": \"https://api.github.com/repos/JerboaBurrow/test/git/trees{/sha}\",
        \"statuses_url\": \"https://api.github.com/repos/JerboaBurrow/test/statuses/{sha}\",
        \"languages_url\": \"https://api.github.com/repos/JerboaBurrow/test/languages\",
        \"stargazers_url\": \"https://api.github.com/repos/JerboaBurrow/test/stargazers\",
        \"contributors_url\": \"https://api.github.com/repos/JerboaBurrow/test/contributors\",
        \"subscribers_url\": \"https://api.github.com/repos/JerboaBurrow/test/subscribers\",
        \"subscription_url\": \"https://api.github.com/repos/JerboaBurrow/test/subscription\",
        \"commits_url\": \"https://api.github.com/repos/JerboaBurrow/test/commits{/sha}\",
        \"git_commits_url\": \"https://api.github.com/repos/JerboaBurrow/test/git/commits{/sha}\",
        \"comments_url\": \"https://api.github.com/repos/JerboaBurrow/test/comments{/number}\",
        \"issue_comment_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues/comments{/number}\",
        \"contents_url\": \"https://api.github.com/repos/JerboaBurrow/test/contents/{+path}\",
        \"compare_url\": \"https://api.github.com/repos/JerboaBurrow/test/compare/{base}...{head}\",
        \"merges_url\": \"https://api.github.com/repos/JerboaBurrow/test/merges\",
        \"archive_url\": \"https://api.github.com/repos/JerboaBurrow/test/{archive_format}{/ref}\",
        \"downloads_url\": \"https://api.github.com/repos/JerboaBurrow/test/downloads\",
        \"issues_url\": \"https://api.github.com/repos/JerboaBurrow/test/issues{/number}\",
        \"pulls_url\": \"https://api.github.com/repos/JerboaBurrow/test/pulls{/number}\",
        \"milestones_url\": \"https://api.github.com/repos/JerboaBurrow/test/milestones{/number}\",
        \"notifications_url\": \"https://api.github.com/repos/JerboaBurrow/test/notifications{?since,all,participating}\",
        \"labels_url\": \"https://api.github.com/repos/JerboaBurrow/test/labels{/name}\",
        \"releases_url\": \"https://api.github.com/repos/JerboaBurrow/test/releases{/id}\",
        \"deployments_url\": \"https://api.github.com/repos/JerboaBurrow/test/deployments\",
        \"created_at\": \"2024-01-03T20:41:54Z\",
        \"updated_at\": \"2024-01-03T20:41:54Z\",
        \"pushed_at\": \"2024-01-03T20:41:54Z\",
        \"git_url\": \"git://github.com/JerboaBurrow/test.git\",
        \"ssh_url\": \"git@github.com:JerboaBurrow/test.git\",
        \"clone_url\": \"https://github.com/JerboaBurrow/test.git\",
        \"svn_url\": \"https://github.com/JerboaBurrow/test\",
        \"homepage\": null,
        \"size\": 0,
        \"stargazers_count\": 0,
        \"watchers_count\": 0,
        \"language\": null,
        \"has_issues\": true,
        \"has_projects\": true,
        \"has_downloads\": true,
        \"has_wiki\": false,
        \"has_pages\": false,
        \"has_discussions\": false,
        \"forks_count\": 0,
        \"mirror_url\": null,
        \"archived\": false,
        \"disabled\": false,
        \"open_issues_count\": 1,
        \"license\": null,
        \"allow_forking\": false,
        \"is_template\": false,
        \"web_commit_signoff_required\": false,
        \"topics\": [
    
        ],
        \"visibility\": \"private\",
        \"forks\": 0,
        \"open_issues\": 1,
        \"watchers\": 0,
        \"default_branch\": \"main\",
        \"custom_properties\": {
    
        }
    },
    \"organization\": {
        \"login\": \"JerboaBurrow\",
        \"id\": 109722648,
        \"node_id\": \"O_kgDOBoo8GA\",
        \"url\": \"https://api.github.com/orgs/JerboaBurrow\",
        \"repos_url\": \"https://api.github.com/orgs/JerboaBurrow/repos\",
        \"events_url\": \"https://api.github.com/orgs/JerboaBurrow/events\",
        \"hooks_url\": \"https://api.github.com/orgs/JerboaBurrow/hooks\",
        \"issues_url\": \"https://api.github.com/orgs/JerboaBurrow/issues\",
        \"members_url\": \"https://api.github.com/orgs/JerboaBurrow/members{/member}\",
        \"public_members_url\": \"https://api.github.com/orgs/JerboaBurrow/public_members{/member}\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/109722648?v=4\",
        \"description\": \"Creating apps and games\"
    },
    \"sender\": {
        \"login\": \"Jerboa-app\",
        \"id\": 84378622,
        \"node_id\": \"MDQ6VXNlcjg0Mzc4NjIy\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/84378622?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/Jerboa-app\",
        \"html_url\": \"https://github.com/Jerboa-app\",
        \"followers_url\": \"https://api.github.com/users/Jerboa-app/followers\",
        \"following_url\": \"https://api.github.com/users/Jerboa-app/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/Jerboa-app/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/Jerboa-app/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/Jerboa-app/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/Jerboa-app/orgs\",
        \"repos_url\": \"https://api.github.com/users/Jerboa-app/repos\",
        \"events_url\": \"https://api.github.com/users/Jerboa-app/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/Jerboa-app/received_events\",
        \"type\": \"User\",
        \"site_admin\": false
    }
}
";

pub const RELEASE_PAYLOAD: &str = "
{
    \"action\": \"published\",
    \"release\": {
        \"url\": \"https://api.github.com/repos/JerboaBurrow/jGL/releases/135576460\",
        \"assets_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/releases/135576460/assets\",
        \"upload_url\": \"https://uploads.github.com/repos/JerboaBurrow/jGL/releases/135576460/assets{?name,label}\",
        \"html_url\": \"https://github.com/JerboaBurrow/jGL/releases/tag/v0.0.5\",
        \"id\": 135576460,
        \"author\": {
        \"login\": \"github-actions[bot]\",
        \"id\": 41898282,
        \"node_id\": \"MDM6Qm90NDE4OTgyODI=\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/in/15368?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/github-actions%5Bbot%5D\",
        \"html_url\": \"https://github.com/apps/github-actions\",
        \"followers_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/followers\",
        \"following_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/orgs\",
        \"repos_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/repos\",
        \"events_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/received_events\",
        \"type\": \"Bot\",
        \"site_admin\": false
        },
        \"node_id\": \"RE_kwDOKhzwN84IFLuM\",
        \"tag_name\": \"v0.0.5\",
        \"target_commitish\": \"main\",
        \"name\": \"jGL-0-0.0.5\",
        \"draft\": false,
        \"prerelease\": false,
        \"created_at\": \"2024-01-02T19:34:33Z\",
        \"published_at\": \"2024-01-03T07:01:06Z\",
        \"assets\": [
        {
            \"url\": \"https://api.github.com/repos/JerboaBurrow/jGL/releases/assets/143456215\",
            \"id\": 143456215,
            \"node_id\": \"RA_kwDOKhzwN84IjPfX\",
            \"name\": \"jGL.zip\",
            \"label\": \"\",
            \"uploader\": {
            \"login\": \"github-actions[bot]\",
            \"id\": 41898282,
            \"node_id\": \"MDM6Qm90NDE4OTgyODI=\",
            \"avatar_url\": \"https://avatars.githubusercontent.com/in/15368?v=4\",
            \"gravatar_id\": \"\",
            \"url\": \"https://api.github.com/users/github-actions%5Bbot%5D\",
            \"html_url\": \"https://github.com/apps/github-actions\",
            \"followers_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/followers\",
            \"following_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}\",
            \"gists_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}\",
            \"starred_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}\",
            \"subscriptions_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/subscriptions\",
            \"organizations_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/orgs\",
            \"repos_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/repos\",
            \"events_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}\",
            \"received_events_url\": \"https://api.github.com/users/github-actions%5Bbot%5D/received_events\",
            \"type\": \"Bot\",
            \"site_admin\": false
            },
            \"content_type\": \"application/zip\",
            \"state\": \"uploaded\",
            \"size\": 2064839,
            \"download_count\": 0,
            \"created_at\": \"2024-01-02T19:39:03Z\",
            \"updated_at\": \"2024-01-02T19:39:03Z\",
            \"browser_download_url\": \"https://github.com/JerboaBurrow/jGL/releases/download/v0.0.5/jGL.zip\"
        }
        ],
        \"tarball_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/tarball/v0.0.5\",
        \"zipball_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/zipball/v0.0.5\",
        \"body\": \"### Adds\r\n\r\n- supports android ndk builds\r\n\r\n### Fixes\r\n\r\n- stackoverflow in font bitmap\"
    },
    \"repository\": {
        \"id\": 706539575,
        \"node_id\": \"R_kgDOKhzwNw\",
        \"name\": \"jGL\",
        \"full_name\": \"JerboaBurrow/jGL\",
        \"private\": false,
        \"owner\": {
        \"login\": \"JerboaBurrow\",
        \"id\": 109722648,
        \"node_id\": \"O_kgDOBoo8GA\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/109722648?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/JerboaBurrow\",
        \"html_url\": \"https://github.com/JerboaBurrow\",
        \"followers_url\": \"https://api.github.com/users/JerboaBurrow/followers\",
        \"following_url\": \"https://api.github.com/users/JerboaBurrow/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/JerboaBurrow/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/JerboaBurrow/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/JerboaBurrow/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/JerboaBurrow/orgs\",
        \"repos_url\": \"https://api.github.com/users/JerboaBurrow/repos\",
        \"events_url\": \"https://api.github.com/users/JerboaBurrow/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/JerboaBurrow/received_events\",
        \"type\": \"Organization\",
        \"site_admin\": false
        },
        \"html_url\": \"https://github.com/JerboaBurrow/jGL\",
        \"description\": \"A Graphics Library with OpenGL or Vulkan backends\",
        \"fork\": false,
        \"url\": \"https://api.github.com/repos/JerboaBurrow/jGL\",
        \"forks_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/forks\",
        \"keys_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/keys{/key_id}\",
        \"collaborators_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/collaborators{/collaborator}\",
        \"teams_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/teams\",
        \"hooks_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/hooks\",
        \"issue_events_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/issues/events{/number}\",
        \"events_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/events\",
        \"assignees_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/assignees{/user}\",
        \"branches_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/branches{/branch}\",
        \"tags_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/tags\",
        \"blobs_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/git/blobs{/sha}\",
        \"git_tags_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/git/tags{/sha}\",
        \"git_refs_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/git/refs{/sha}\",
        \"trees_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/git/trees{/sha}\",
        \"statuses_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/statuses/{sha}\",
        \"languages_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/languages\",
        \"stargazers_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/stargazers\",
        \"contributors_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/contributors\",
        \"subscribers_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/subscribers\",
        \"subscription_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/subscription\",
        \"commits_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/commits{/sha}\",
        \"git_commits_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/git/commits{/sha}\",
        \"comments_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/comments{/number}\",
        \"issue_comment_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/issues/comments{/number}\",
        \"contents_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/contents/{+path}\",
        \"compare_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/compare/{base}...{head}\",
        \"merges_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/merges\",
        \"archive_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/{archive_format}{/ref}\",
        \"downloads_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/downloads\",
        \"issues_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/issues{/number}\",
        \"pulls_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/pulls{/number}\",
        \"milestones_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/milestones{/number}\",
        \"notifications_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/notifications{?since,all,participating}\",
        \"labels_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/labels{/name}\",
        \"releases_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/releases{/id}\",
        \"deployments_url\": \"https://api.github.com/repos/JerboaBurrow/jGL/deployments\",
        \"created_at\": \"2023-10-18T06:40:56Z\",
        \"updated_at\": \"2024-01-02T14:19:54Z\",
        \"pushed_at\": \"2024-01-02T19:35:10Z\",
        \"git_url\": \"git://github.com/JerboaBurrow/jGL.git\",
        \"ssh_url\": \"git@github.com:JerboaBurrow/jGL.git\",
        \"clone_url\": \"https://github.com/JerboaBurrow/jGL.git\",
        \"svn_url\": \"https://github.com/JerboaBurrow/jGL\",
        \"homepage\": \"\",
        \"size\": 28185,
        \"stargazers_count\": 0,
        \"watchers_count\": 0,
        \"language\": \"C++\",
        \"has_issues\": true,
        \"has_projects\": true,
        \"has_downloads\": true,
        \"has_wiki\": false,
        \"has_pages\": false,
        \"has_discussions\": false,
        \"forks_count\": 0,
        \"mirror_url\": null,
        \"archived\": false,
        \"disabled\": false,
        \"open_issues_count\": 11,
        \"license\": {
        \"key\": \"mit\",
        \"name\": \"MIT License\",
        \"spdx_id\": \"MIT\",
        \"url\": \"https://api.github.com/licenses/mit\",
        \"node_id\": \"MDc6TGljZW5zZTEz\"
        },
        \"allow_forking\": true,
        \"is_template\": false,
        \"web_commit_signoff_required\": false,
        \"topics\": [
        \"2d-game\",
        \"2d-graphics\",
        \"cross-platform\",
        \"cross-platform-library\",
        \"graphics\",
        \"graphics-2d\",
        \"graphics-library\",
        \"graphics-programming\",
        \"linux\",
        \"macos\",
        \"opengl\",
        \"vulkan\",
        \"windows\"
        ],
        \"visibility\": \"public\",
        \"forks\": 0,
        \"open_issues\": 11,
        \"watchers\": 0,
        \"default_branch\": \"main\",
        \"custom_properties\": {
    
        }
    },
    \"organization\": {
        \"login\": \"JerboaBurrow\",
        \"id\": 109722648,
        \"node_id\": \"O_kgDOBoo8GA\",
        \"url\": \"https://api.github.com/orgs/JerboaBurrow\",
        \"repos_url\": \"https://api.github.com/orgs/JerboaBurrow/repos\",
        \"events_url\": \"https://api.github.com/orgs/JerboaBurrow/events\",
        \"hooks_url\": \"https://api.github.com/orgs/JerboaBurrow/hooks\",
        \"issues_url\": \"https://api.github.com/orgs/JerboaBurrow/issues\",
        \"members_url\": \"https://api.github.com/orgs/JerboaBurrow/members{/member}\",
        \"public_members_url\": \"https://api.github.com/orgs/JerboaBurrow/public_members{/member}\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/109722648?v=4\",
        \"description\": \"Creating apps and games\"
    },
    \"sender\": {
        \"login\": \"Jerboa-app\",
        \"id\": 84378622,
        \"node_id\": \"MDQ6VXNlcjg0Mzc4NjIy\",
        \"avatar_url\": \"https://avatars.githubusercontent.com/u/84378622?v=4\",
        \"gravatar_id\": \"\",
        \"url\": \"https://api.github.com/users/Jerboa-app\",
        \"html_url\": \"https://github.com/Jerboa-app\",
        \"followers_url\": \"https://api.github.com/users/Jerboa-app/followers\",
        \"following_url\": \"https://api.github.com/users/Jerboa-app/following{/other_user}\",
        \"gists_url\": \"https://api.github.com/users/Jerboa-app/gists{/gist_id}\",
        \"starred_url\": \"https://api.github.com/users/Jerboa-app/starred{/owner}{/repo}\",
        \"subscriptions_url\": \"https://api.github.com/users/Jerboa-app/subscriptions\",
        \"organizations_url\": \"https://api.github.com/users/Jerboa-app/orgs\",
        \"repos_url\": \"https://api.github.com/users/Jerboa-app/repos\",
        \"events_url\": \"https://api.github.com/users/Jerboa-app/events{/privacy}\",
        \"received_events_url\": \"https://api.github.com/users/Jerboa-app/received_events\",
        \"type\": \"User\",
        \"site_admin\": false
    }
    }
    ";