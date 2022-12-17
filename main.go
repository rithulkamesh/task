package main

import (
	//	"database/sql"
	//	"github.com/mattn/go-sqlite3"
	"github.com/urfave/cli"
	"os"
	"path/filepath"
)

var app = cli.NewApp()

func main() {
	task_init()
}

func task_init() {
	/*

		Initialise the task program, this handles the creation of the tasks folder under config,
		Checking if the database exists or not
		Creating the database if it does not exist

	*/

	var homedir, _ = os.UserHomeDir()
	path := filepath.Join(homedir, ".config", "task")
	os.MkdirAll(path, os.ModePerm)

	return
}

func create_task() {
}
