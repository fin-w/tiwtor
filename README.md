# tiwtor
Retrieve grammatical and relational data about Welsh words through a CLI.

## To use
First clone and set up lecsicon-db, then clone this repository from the same directory so the crates are adjacent. Set the paths to the lecsicon_cc0.txt and the desired location of the database in the .env file: the initial generation of the database is best done in RAM so follow the instructions in the lecsicon-db README, setting the CSV_PATH and DATABASE_PATH in the .env file in this repository, ensuring DATABASE_PATH here matches DATABASE_URL in the lecsicon-db .env file.

Then, cargo build this and it should use lecsicon-db. The first run will generate the SQLite database, subsequent runs will start up immediately and only use the database. Perform searches for Welsh words, with search suggestions as you type and a search history. Search results contain words matching the search term. Grammatical data is displayed about each entry, and words related to the search term are also printed.
