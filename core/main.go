/*
	core

	This package is the core of Kirino Media Server.
	core is to Kirino as OpenRC or *cringes* systemd is to Linux.
*/
package core

type Core struct {
	providers []*Provider
	libraries []*Library
	services  []*Service
	fetchers  []*Fetcher
}

func New() *Core {
	return &Core{}
}
