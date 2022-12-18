package main

import (
	"database/sql"
	"github.com/rithulkamesh/task/db"
	"github.com/urfave/cli"
	"os"
	"path/filepath"
)

var _ = cli.NewApp()
var dat *sql.DB

func main() {
	TaskInit()

}

func TaskInit() {
	/*

		Initialise the task program, this handles the creation of the tasks folder under config,
		Checking if the database exists or not
		Creating the database if it does not exist

	*/

	var homedir, _ = os.UserHomeDir()
	path := filepath.Join(homedir, ".config", "task")
	_ = os.MkdirAll(path, os.ModePerm)

	var _, err = os.Stat(path + "/tasks.db")
	if os.IsNotExist(err) {
		os.Create(path + "/tasks.db")

	}

	dat, _ = db.CreateConnection(path + "/tasks.db")

	dat.Exec(`CREATE TABLE if not exists [tasks] (
		id integer primary key autoincrement,
		title text not null,
		description text,
		completed integer not null default 0
	)`)
}
