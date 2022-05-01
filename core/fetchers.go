package core

// A fetcher
type Fetcher struct {
	ID          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	SearchFunc func(query string) []*Series `json:"-"`
	FetchFunc  func(series *Series)         `json:"-"`
}

// Get Fetcher with ID id
func (c *Core) Fetcher(id string) *Fetcher {
	return c.fetcher[id]
}

// Get all Fetchers for Core c
func (c *Core) Fetchers() []*Fetcher {
	return c.fetchers
}

// Register Fetcher fetcher for Core c
func (c *Core) AddFetcher(fetcher *Fetcher) {
	c.fetchers = append(c.fetchers, fetcher)
	c.fetcher[fetcher.ID] = fetcher
}

// Fetch anything new for Feed f
func (c *Core) Fetch(l *Library) {
	for _, s := range l.AllSeries() {
		if s.Fetcher != "" {
			c.
				Fetcher(s.Fetcher).
				FetchFunc(s)
		}
	}
}
