package main

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	_ "github.com/mattn/go-sqlite3"
	"github.com/urfave/cli"
)

func main() {
	var db *sql.DB = TaskInit()
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
					args := strings.Join(c.Args()[:], " ")
					fmt.Println(args)
					var res, err = db.Exec("INSERT INTO tasks (title) VALUES (?)", args)
					fmt.Println(res.LastInsertId())
					return err

				},
			},
		},
	}
	app.Run(os.Args)

}

func TaskInit() *sql.DB {
	var homedir, _ = os.UserHomeDir()
	path := filepath.Join(homedir, ".config", "task")
	_ = os.MkdirAll(path, os.ModePerm)

	var _, err = os.Stat(path + "/tasks.db")
	if os.IsNotExist(err) {
		os.Create(path + "/tasks.db")
	}
	return create_conn(path + "/tasks.db")
}

func create_conn(path string) *sql.DB {
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

	return db
}
