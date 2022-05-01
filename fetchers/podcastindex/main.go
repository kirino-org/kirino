package podcastindex

import (
	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/internal/scsc"
	"github.com/mmcdole/gofeed"
)

var Fetcher = &core.Fetcher{
	ID:          "podcastindex",
	Type:        core.LibraryTypePodcasts,
	Name:        "PodcastIndex.org",
	Description: "Non-profit podcast index",

	SearchFunc: searchByTerm,
	FetchFunc:  fetchFunc,
}

func searchFunc(query string) []*core.Series {
	return searchByTerm(query)
}

func fetchFunc(s *core.Series) {
	f := getFeedByID(s.OriginalID)

	s.Title = f.Title
	s.Description = f.Description
	s.Image = f.Image

	fp := gofeed.NewParser()
	feed, err := fp.ParseURL(f.Url)
	if err != nil {
		panic(err)
	}

	for _, i := range feed.Items {
		s.AddItem(&core.Item{
			OriginalID:    scsc.IntStr(f.Id),
			OriginalTitle: i.Title,

			Title: i.Title,
			Path:  i.Enclosures[0].URL,
		})
	}
}
