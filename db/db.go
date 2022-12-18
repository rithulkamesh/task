package db

import (
	"database/sql"
	"fmt"
	"github.com/mattn/go-sqlite3"
)

type Database struct {
	Conn sqlite3.SQLiteConn
	Ctx  sqlite3.SQLiteContext
}

func CreateConnection(path string) (*sql.DB, error) {
	db, err := sql.Open("sqlite3", path)
	if err != nil {
		fmt.Println(err.Error())
	}
	return db, err
}
