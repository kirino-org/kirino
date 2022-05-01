// Package core is the core of Kirino Media Server.
// core is to Kirino as OpenRC or *cringes in disgust* systemd is to Linux.
package core

import (
	"net/http"
	"strings"
	"time"
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

func (c *Core) Start() {
	/*
		Set up the services
	*/
	for _, s := range c.services {
		http.Handle(
			"/"+s.ID+"/",

			http.StripPrefix(
				strings.TrimRight("/"+s.ID+"/", "/"),
				s.Func(c),
			),
		)
	}

	// Listen on port 6319, serve default mux
	go http.ListenAndServe(":6319", nil)

	// Repeat forever or until the program exits
	for {
		// Iterate over all tasks
		for _, t := range c.tasks {
			nextRun := t.LastRun.Add(t.RunEvery)
			if time.Now().After(nextRun) {
				t.Func(c)
			}
		}
	}
}
