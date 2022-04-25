package core

import (
	"io/fs"
	"os"
	"path"
	"path/filepath"
	"regexp"
)

type LibraryType string

// Library types
const (
	LibraryTypeAnime      LibraryType = "anime"
	LibraryTypeManga      LibraryType = "manga"
	LibraryTypeMusic      LibraryType = "music"
	LibraryTypePodcasts   LibraryType = "podcasts"
	LibraryTypeTubeVideos LibraryType = "tube_videos"
)

// A library
type Library struct {
	ID        int         `json:"id"`
	Type      LibraryType `json:"type"`
	Name      string      `json:"name"`
	MediaDirs []string    `json:"media_dirs"`

	all_series []*Series       `json:"-"`
	series     map[int]*Series `json:"-"`

	Scanner  *Scanner `json:"scanner"`
	Provider string   `json:"provider"`
	Fetcher  string   `json:"fetcher"`
}

type Series struct {
	ID            int    `json:"id"`
	Image         string `json:"image"`
	Title         string `json:"title"`
	OriginalTitle string `json:"original_title"`

	items []*Item       `json:"-"`
	item  map[int]*Item `json:"-"`
}

type Item struct {
	ID            int
	Title         string
	OriginalTitle string

	Path string
}

// Create new Library library
func (c *Core) NewLibrary(l *Library) {
	l.series = make(map[int]*Series)
	c.libraries = append(c.libraries, l)
	c.library[l.ID] = l
}

// Get all Libraries for Core c
func (c *Core) Libraries() []*Library {
	return c.libraries
}

// Get Library with ID id
func (c *Core) Library(id int) *Library {
	return c.library[id]
}

func (l *Library) Series(id int) *Series {
	return l.series[id]
}

func (l *Library) AllSeries() []*Series {
	return l.all_series
}

func (l *Library) AddSeries(s *Series) {
	s.item = make(map[int]*Item)
	s.ID = len(l.all_series)
	l.all_series = append(l.all_series, s)
	l.series[s.ID] = s
}

func (s *Series) Item(id int) *Item {
	return s.item[id]
}

func (s *Series) Items() []*Item {
	return s.items
}

func (s *Series) AddItem(item *Item) {
	item.ID = len(s.items)
	s.items = append(s.items, item)
	s.item[item.ID] = item
}

// Scan Library l
func (l *Library) Scan() {
	regex := regexp.MustCompile(`\.(flac|ogg|mp3|mkv|mp4)`)
	for _, mDir := range l.MediaDirs {
		dirEntries, _ := os.ReadDir(mDir)
		for _, entry := range dirEntries {
			if entry.Type().IsDir() {
				sID := len(l.all_series)

				l.AddSeries(&Series{
					ID:            sID,
					OriginalTitle: entry.Name(),
				})

				filepath.WalkDir(
					path.Join(mDir, entry.Name()),
					func(path string, d fs.DirEntry, err error) error {
						if d.Type().IsRegular() && regex.MatchString(d.Name()) {
							iID := len(l.Series(sID).Items())
							l.Series(sID).AddItem(&Item{
								ID:            iID,
								OriginalTitle: d.Name(),
								Path:          path,
							})
						}
						return nil
					},
				)
			}
		}
	}
}
