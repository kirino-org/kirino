package core

type Feed struct {
	Id          string
	Title       string
	Description string
}

type Fetcher struct {
	Id          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	SearchFunc func(query string)
}

func (c *Core) RegisterFetcher(fetcher *Fetcher) {
	c.fetchers = append(c.fetchers, fetcher)
}
