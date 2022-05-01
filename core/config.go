package core

import (
	"fmt"
	"io"
	"os"
)

// TODO: look into something better than JSON for config

type Config struct {
	Port      int         `json:"port"`
	Services  []*Service  `json:"services"`
	Fetchers  []*Fetcher  `json:"fetchers"`
	Providers []*Provider `json:"providers"`
}

func DecodeConfig(path string) {
	configDir, _ := os.UserConfigDir()
	f, err := os.Open(configDir)
	if err != nil {
		//panic(err)
	}

	c, err := io.ReadAll(f)
	if err != nil {
		//panic(err)
	}

	fmt.Println(c)
}
