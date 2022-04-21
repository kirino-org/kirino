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
}

func searchChannel(query string) []*core.Feed {
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

	var feeds []*core.Feed

	for _, c := range results {
		feeds = append(feeds, &core.Feed{
			ID:          c.Id,
			CoverImage:  c.Avatar[len(c.Avatar)-1].URL,
			Title:       c.Name,
			Description: c.Description,
		})
	}

	return feeds
}

func videosForChannel(channel channel) []video {
	res, err := http.Get(InvidiousURL + "/api/v1/channels/" + channel.Id + "/videos")
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
