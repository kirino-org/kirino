// Package core is the core of Kirino Media Server.
// core is to Kirino as OpenRC or *cringes in disgust* systemd is to Linux.
package core

import (
	"fmt"
)

type Core struct {
	providers []*Provider
	libraries []*Library
	services  []*Service
	fetchers  []*Fetcher
	scanners  []*Scanner
	tasks     []*Task

	provider map[string]*Provider
	library  map[int]*Library
	service  map[string]*Service
	fetcher  map[string]*Fetcher
	scanner  map[string]*Scanner
	task     map[int]*Task

	toPlay string

	Config map[string]string
}

func New() *Core {
	return &Core{
		provider: make(map[string]*Provider),
		library:  make(map[int]*Library),
		service:  make(map[string]*Service),
		fetcher:  make(map[string]*Fetcher),
		scanner:  make(map[string]*Scanner),
		task:     make(map[int]*Task),
	}
}

func (c *Core) Run() {
	fmt.Println("Starting services...")
	runServices(c)

	fmt.Println("Starting task runner...")
	//
}
