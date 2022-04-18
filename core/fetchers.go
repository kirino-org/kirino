package core

type Feed struct {
	Id          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
}

type Fetcher struct {
	Id          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	SearchFunc func(query string) []Feed
}

func (c *Core) RegisterFetcher(fetcher *Fetcher) {
	c.fetchers = append(c.fetchers, fetcher)
}
