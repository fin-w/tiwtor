# tiwtor
Retrieve grammatical and relational data about Welsh words.

## To use
First set up lecsicon-db, then clone this repository in an adjacent directory. Set the paths to the lecsicon_cc0.txt and the desired location of the database in the .env file. Build this and it should use lecsicon-db. The first run will generate the SQLite database, subsequent runs will start up immediately and only use the database. Perform searches for Welsh words, with search suggestions as you type and a search history. Search results contain words matching the search term. Grammatical data is displayed about each entry, and words related to the search term are also printed.
