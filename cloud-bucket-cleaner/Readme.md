# Description

This Lambda reads from dynamodb once a day and finds all files that have been marked as downloaded during that time. Afterwards it looks in the wav bucket to find all files that have been downloaded and deletes them. It also deletes all files from a bucket which are older than a day.