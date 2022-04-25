package main

import (
	"github.com/kirino-org/kirino/api"
	"github.com/kirino-org/kirino/app"
	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/fetchers/invidious"
	"github.com/kirino-org/kirino/fetchers/podcastindex"
)

func main() {
	c := core.New()

	c.RegisterFetcher(invidious.Fetcher)
	c.RegisterFetcher(podcastindex.Fetcher)

	c.RegisterService(app.Service)
	c.RegisterService(api.Service)

	c.NewLibrary(&core.Library{
		ID:   0,
		Type: core.LibraryTypeAnime,
		Name: "Example Library",
		MediaDirs: []string{
			"./test",
		},
	})

	c.Library(0).Scan()

	core.RunServices(c)
}
