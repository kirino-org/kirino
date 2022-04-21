package core

type Scanner struct {
	ID   string      `json:"id"`
	Type LibraryType `json:"type"`
	Name string      `json:"name"`

	Func func(library *Library) []string `json:"-"`
}
