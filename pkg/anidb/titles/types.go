package anidb_titles

/*
	Title types
*/
type TitleType int

const (
	TitleTypeMain     TitleType = 1
	TitleTypeOfficial TitleType = 2
	TitleTypeSynonym  TitleType = 3

	TitleTypeOther TitleType = 0
)

/*
	Title languages
*/
type TitleLanguage int

const (
	TitleLanguageXJat TitleLanguage = 1
	TitleLanguageJa   TitleLanguage = 2
	TitleLanguageEn   TitleLanguage = 3

	TitleLanguageOther TitleLanguage = 0
)

// An anime title
type Title struct {
	Type     TitleType
	Language TitleLanguage
	Title    string
}

/*
	XML garbage
*/
type title struct {
	ID     int `xml:"aid,attr"`
	Titles []struct {
		Title    string `xml:",chardata"`
		Type     string `xml:"type,attr"`
		Language string `xml:"lang,attr"`
	} `xml:"title"`
}

type titles struct {
	Anime []title `xml:"anime"`
}
