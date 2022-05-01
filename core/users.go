package core

import (
	"github.com/segmentio/ksuid"
)

type User struct {
	//
}

func (c *Core) AddUser(u *User) {
	ksuid.New()
}
