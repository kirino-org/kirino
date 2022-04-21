package core

import (
	"fmt"
	"net/http"
	"strings"
)

type Service struct {
	ID   string
	Name string
	Func func(c *Core) *http.ServeMux `json:"-"`
}

// Register a service with Core c
func (c *Core) RegisterService(service *Service) {
	c.services = append(c.services, service)
	c.service[service.ID] = service
}

// RunServices starts up all services on Core c registered with RegisterService
func RunServices(c *Core) {
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

	fmt.Println(`KMS is running on port 6319`)

	select {}
}
