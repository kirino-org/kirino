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

	c.AddFetcher(invidious.Fetcher)
	c.AddFetcher(podcastindex.Fetcher)

	c.RegisterService(app.Service)
	c.RegisterService(api.Service)

	c.NewLibrary(&core.Library{
		ID:   0,
		Type: core.LibraryTypeAnime,
		Name: "Example Library",
		MediaDirs: []string{
			"./test",
			"/mnt/downloads",
			"/mnt/anime/to-process",
			"/mnt3",
		},
	})

	c.Library(0).Scan()

	c.NewLibrary(&core.Library{
		ID:   1,
		Type: core.LibraryTypePodcasts,
		Name: "Podcasts",
	})

	core.RunServices(c)
}
