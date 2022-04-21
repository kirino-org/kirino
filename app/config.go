package app

import "github.com/kirino-org/kirino/core"

var Service = &core.Service{
	ID:   "app",
	Name: "Kiririn",
	Func: AppRouter,
}

var config = renderConfig{
	Title:   "Kiririn",
	BaseURL: "/app",
}

type renderConfig struct {
	Title   string
	BaseURL string
}

type rendererData struct {
	Config renderConfig
	Data   interface{}
}
