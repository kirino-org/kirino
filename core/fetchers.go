package core

// A feed
type Feed struct {
	ID          string `json:"id"`
	CoverImage  string `json:"image"`
	Title       string `json:"title"`
	Description string `json:"description"`

	Fetcher  string `json:"fetcher"`
	FetchDir string `json:"fetch_dir"`
}

// A fetcher
type Fetcher struct {
	ID          string      `json:"id"`
	Type        LibraryType `json:"type"`
	Name        string      `json:"name"`
	Description string      `json:"description"`

	SearchFunc func(query string) []*Feed `json:"-"`
	FetchFunc  func(id string)            `json:"-"`
}

// Register Fetcher fetcher for Core c
func (c *Core) RegisterFetcher(fetcher *Fetcher) {
	c.fetchers = append(c.fetchers, fetcher)
	c.fetcher[fetcher.ID] = fetcher
}

// Get all Fetchers for Core c
func (c *Core) Fetchers() []*Fetcher {
	return c.fetchers
}

// Get Fetcher with ID id
func (c *Core) Fetcher(id string) *Fetcher {
	return c.fetcher[id]
}

// Fetch anything new for Feed f
func (c *Core) Fetch(f *Feed) {
	fetcher := c.fetcher[f.Fetcher]
	fetcher.FetchFunc(f.ID)
}
