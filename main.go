package main

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"

	_ "github.com/mattn/go-sqlite3"
	"github.com/urfave/cli"
)

var db *sql.DB

func main() {
	TaskInit()
	app := &cli.App{
		Name:  "task",
		Usage: "A simple task manager",
		Commands: []cli.Command{
			{
				Name:    "add",
				Aliases: []string{"a"},
				Usage:   "Add a new task",
				Action: func(c *cli.Context) error {
					// Insert args as title into db
					args := c.Args().Get(0)
					var res, err = db.Exec("INSERT INTO tasks (title) VALUES (?)", args)
					fmt.Println(res)
					return err

				},
			},
		},
	}
	app.Run(os.Args)

}

func TaskInit() {
	var homedir, _ = os.UserHomeDir()
	path := filepath.Join(homedir, ".config", "task")
	_ = os.MkdirAll(path, os.ModePerm)

	var _, err = os.Stat(path + "/tasks.db")
	if os.IsNotExist(err) {
		os.Create(path + "/tasks.db")
	}
	create_conn(path + "/tasks.db")
}

func create_conn(path string) {
	db, err := sql.Open("sqlite3", path)
	if err != nil {
		fmt.Println(err.Error())
	}

	db.Exec(`CREATE TABLE if not exists [tasks] (
		id integer primary key autoincrement,
		title text not null,
		description text,
		completed integer not null default 0
	)`)
}
