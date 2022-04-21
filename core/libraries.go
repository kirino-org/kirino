package core

import (
	"io/fs"
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
	ID        string      `json:"id"`
	Type      LibraryType `json:"type"`
	Name      string      `json:"name"`
	MediaDirs []string    `json:"media_dirs"`

	Scanner  string `json:"scanner"`
	Provider string `json:"provider"`
	Fetcher  string `json:"fetcher"`
}

// Create new Library library
func (c *Core) NewLibrary(library *Library) {
	c.libraries = append(c.libraries, library)
	c.library[library.ID] = library
}

// Get all Libraries for Core c
func (c *Core) Libraries() []*Library {
	return c.libraries
}

// Get Library with ID id
func (c *Core) Library(id string) *Library {
	return c.library[id]
}

// Scan Library l
func (l *Library) Scan() {
	for _, dir := range l.MediaDirs {
		r := regexp.MustCompile(`\.(flac|ogg|mp3)`)
		filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
			if r.MatchString(filepath.Ext(path)) {
				//
			}
			return nil
		})
	}
}
