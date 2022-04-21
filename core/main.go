// Package core is the core of Kirino Media Server.
// core is to Kirino as OpenRC or *cringes in disgust* systemd is to Linux.
package core

type Core struct {
	providers []*Provider
	libraries []*Library
	services  []*Service
	fetchers  []*Fetcher
	scanners  []*Scanner

	provider map[string]*Provider
	library  map[string]*Library
	service  map[string]*Service
	fetcher  map[string]*Fetcher
	scanner  map[string]*Scanner

	Config map[string]string
}

func New() *Core {
	return &Core{
		provider: make(map[string]*Provider),
		library:  make(map[string]*Library),
		service:  make(map[string]*Service),
		fetcher:  make(map[string]*Fetcher),
		scanner:  make(map[string]*Scanner),
	}
}
