package core

import "github.com/kirino-org/kirino/models"

type ProviderFuncs struct {
	Query func(query string) []models.Series
}

type Provider struct {
	Id          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`

	QueryFunc func(query string) []models.Series
}

func (core *Core) RegisterProvider(provider *Provider) {
	core.providers = append(core.providers, provider)
}
