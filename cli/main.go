package main

import (
	"github.com/kirino-org/kirino/app"
	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/fetchers/invidious"
)

func main() {
	c := core.New()

	c.RegisterFetcher(invidious.Fetcher)

	c.RegisterService(app.Service)

	c.RunServices()
}
