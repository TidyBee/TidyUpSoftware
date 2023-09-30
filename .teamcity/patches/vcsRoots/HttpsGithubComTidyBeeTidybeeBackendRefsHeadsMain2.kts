package patches.vcsRoots

import jetbrains.buildServer.configs.kotlin.*
import jetbrains.buildServer.configs.kotlin.ui.*
import jetbrains.buildServer.configs.kotlin.vcs.GitVcsRoot

/*
This patch script was generated by TeamCity on settings change in UI.
To apply the patch, create a vcsRoot with id = 'HttpsGithubComTidyBeeTidybeeBackendRefsHeadsMain2'
in the root project, and delete the patch script.
*/
create(DslContext.projectId, GitVcsRoot({
    id("HttpsGithubComTidyBeeTidybeeBackendRefsHeadsMain2")
    name = "https://github.com/TidyBee/tidybee-backend#refs/heads/main (2)"
    url = "https://github.com/TidyBee/tidybee-backend"
    branch = "refs/heads/main"
    branchSpec = "refs/heads/*"
    authMethod = password {
        userName = "Cavonstavant"
        password = "credentialsJSON:c1633f86-9483-42f9-b41a-46ab8cf6c21b"
    }
}))

