package core

import (
	"io/fs"
	"path/filepath"
)

type LibraryType int

const (
	LibraryTypeAnime    LibraryType = 1
	LibraryTypeManga    LibraryType = 2
	LibraryTypeMusic    LibraryType = 3
	LibraryTypePodcasts LibraryType = 4
)

type Library struct {
	Id        string      `json:"id"`
	Type      LibraryType `json:"type"`
	Name      string      `json:"name"`
	MediaDirs []string    `json:"media_dirs"`
}

func (core *Core) NewLibrary(library *Library) {
	core.libraries = append(core.libraries, library)
}

func (core *Core) Libraries() []*Library {
	return core.libraries
}

func (l *Library) Scan() {
	for _, dir := range l.MediaDirs {
		filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
			filepath.Ext(path)
		})
	}
}
