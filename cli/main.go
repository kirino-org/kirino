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

	core.RunServices(c)
}
