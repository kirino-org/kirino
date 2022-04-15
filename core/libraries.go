package core

type LibraryType int

const (
	LibraryTypeAnime    LibraryType = 1
	LibraryTypeManga    LibraryType = 2
	LibraryTypeMusic    LibraryType = 3
	LibraryTypePodcasts LibraryType = 4
)

type Library struct {
	Type LibraryType `json:"type"`
	Name string      `json:"name"`
}
