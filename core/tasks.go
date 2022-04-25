package core

import (
	"net/http"
	"strings"
)

type Task struct {
	ID   int    `json:"id"`
	Name string `json:"name"`

	Func func(c *Core) `json:"-"`
}

func runServices(c *Core) {
	for _, s := range c.services {
		http.Handle(
			"/"+s.ID+"/",

			http.StripPrefix(
				strings.TrimRight("/"+s.ID+"/", "/"),
				s.Func(c),
			),
		)
	}

	go http.ListenAndServe(":6319", nil)
}
