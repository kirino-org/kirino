package core

import "github.com/kirino-org/kirino/core/types"

type Provider struct {
	ID          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	QueryFunc func(query string) []types.Song `json:"-"`
}

func (c *Core) RegisterProvider(provider *Provider) {
	c.providers = append(c.providers, provider)
	c.provider[provider.ID] = provider
}

func (c *Core) Providers() []*Provider {
	return c.providers
}

func (c *Core) Provider(id string) *Provider {
	return c.provider[id]
}
