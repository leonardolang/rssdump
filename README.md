# Description

The sole purpose of this code is to extract useful data from the Activity Feed of Jira to aid the filling of a SR&ED/R&D Timesheet.

Result will be show on console, each entry alternating between red/green colours, to improve readability.

# Instructions

1. Open this on your browser (after logging in on Jira):

        https://<your-jira-server>/activity?maxResults=1000&streams=user+IS+<your-username>&os_authType=basic&title=undefined

    Replace `<your-jira-server>` by your server address (e.g. issues.my.server) and `<your-username>` by your username (e.g. user1234).

    Save the resulting file as `activity.xml`.

2. Run:

    `$ ./rssdump activity.xml | less -R`

    Check the resulting data for filling your SR&ED/R&D Timesheet.

# Notes

You may want to increase `maxResults=1000` to something larger if you are really active on Jira.
