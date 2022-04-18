// Package core is the core of Kirino Media Server.
// core is to Kirino as OpenRC or *cringes in disgust* systemd is to Linux.
package core

type Core struct {
	providers []*Provider
	libraries []*Library
	services  []*Service
	fetchers  []*Fetcher

	provider map[string]*Provider
	library  map[string]*Library
	service  map[string]*Service
	fetcher  map[string]*Fetcher
}

func New() *Core {
	return &Core{}
}
