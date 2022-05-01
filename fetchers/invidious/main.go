package invidious

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"

	"github.com/kirino-org/kirino/core"
)

type channel struct {
	Type   string `json:"type"`
	Name   string `json:"author"`
	Id     string `json:"authorId"`
	Url    string `json:"authorUrl"`
	Avatar []struct {
		URL    string `json:"url"`
		Width  int    `json:"width"`
		Height int    `json:"height"`
	} `json:"authorThumbnails"`
	SubCount        int    `json:"subCount"`
	VideoCount      int    `json:"videoCount"`
	Description     string `json:"description"`
	DescriptionHtml string `json:"descriptionHtml"`
}

type video struct {
	Type            string `json:"type"`
	Title           string `json:"title"`
	Id              string `json:"videoId"`
	Author          string `json:"author"`
	AuthorID        string `json:"authorId"`
	AuthorURL       string `json:"authorUrl"`
	VideoThumbnails []struct {
		Quality string `json:"quality"`
		Url     string `json:"url"`
		Width   int    `json:"width"`
		Height  int    `json:"height"`
	} `json:"videoThumbnails"`
	Description     string `json:"description"`
	DescriptionHtml string `json:"descriptionHtml"`
	ViewCount       int    `json:"viewCount"`
	Published       int    `json:"published"`
	PublishedText   string `json:"publishedText"`
	LengthSeconds   int    `json:"lengthSeconds"`
	LiveNow         bool   `json:"liveNow"`
	Premium         bool   `json:"premium"`
	IsUpcoming      bool   `json:"isUpcoming"`
}

var Fetcher = &core.Fetcher{
	ID:          "invidious",
	Type:        core.LibraryTypeTubeVideos,
	Name:        "Invidious",
	Description: "Alternate frontend for YouTube",
	SearchFunc:  searchChannel,
	FetchFunc: func(s *core.Series) {
		for _, v := range videosForChannel(s.OriginalID) {
			s.AddItem(
				&core.Item{
					OriginalID:    v.Id,
					Title:         v.Title,
					OriginalTitle: v.Title,
				},
			)
		}
	},
}

func searchChannel(query string) []*core.Series {
	urlValues := url.Values{
		"q": {
			query,
		},

		"type": {
			"channel",
		},
	}

	res, err := http.Get(InvidiousURL + "/api/v1/search?" + urlValues.Encode())
	if err != nil {
		panic(err)
	}

	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	var results []channel

	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	var feeds []*core.Series

	for _, c := range results {
		feeds = append(feeds, &core.Series{
			OriginalID:    c.Id,
			Image:         c.Avatar[len(c.Avatar)-1].URL,
			Title:         c.Name,
			OriginalTitle: c.Name,
			Description:   c.Description,
		})
	}

	return feeds
}

func videosForChannel(chID string) []video {
	res, err := http.Get(InvidiousURL + "/api/v1/channels/" + chID + "/videos")
	if err != nil {
		panic(err)
	}

	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	var results []video
	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	for _, c := range results {
		fmt.Println(c.Title, "("+c.Id+")")
	}

	return results
}
