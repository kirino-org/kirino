package core

import "github.com/kirino-org/kirino/models"

type Provider struct {
	Id          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	QueryFunc func(query string) []models.Series
}

func (c *Core) RegisterProvider(provider *Provider) {
	c.providers = append(c.providers, provider)
}
