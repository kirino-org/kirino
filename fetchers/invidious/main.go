package invidious

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"

	"github.com/kirino-org/kirino/core"
)

type Channel struct {
	Type            string `json:"type"`
	Name            string `json:"author"`
	Id              string `json:"authorId"`
	Url             string `json:"authorUrl"`
	SubCount        int    `json:"subCount"`
	VideoCount      int    `json:"videoCount"`
	Description     string `json:"description"`
	DescriptionHtml string `json:"descriptionHtml"`
}

type Video struct {
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

func Init(c *core.Core) {
	c.Libraries()
}

func searchChannel(query string) (results []Channel) {
	urlValues := url.Values{
		"q": {
			query,
		},

		"type": {
			"channel",
		},
	}

	res, err := http.Get("https://iv.lncn.dev/api/v1/search?" + urlValues.Encode())
	if err != nil {
		panic(err)
	}

	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	for _, c := range results {
		fmt.Println(c.Name, "("+c.Id+")")
		videosForChannel(c)
	}

	return
}

func videosForChannel(channel Channel) []Video {
	res, err := http.Get("https://iv.lncn.dev/api/v1/channels/" + channel.Id + "/videos")
	if err != nil {
		panic(err)
	}

	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	var results []Video
	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	for _, c := range results {
		fmt.Println(c.Title, "("+c.Id+")")
	}

	return results
}
