package podcastindex

import (
	"crypto/sha1"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/internal/scsc"
)

var Fetcher = &core.Fetcher{
	ID:          "podcastindex",
	Type:        core.LibraryTypePodcasts,
	Name:        "PodcastIndex.org",
	Description: "Non-profit podcast index",

	SearchFunc: searchByTerm,
}

func searchByTerm(query string) []*core.Feed {
	req, err := http.NewRequest("GET", "https://api.podcastindex.org/api/1.0/search/byterm?q="+query, nil)
	if err != nil {
		panic(err)
	}

	authTime := scsc.Int64Str(
		time.Now().Unix(),
	)

	authHasher := sha1.New()
	authHasher.Write(
		[]byte(
			ApiKey + ApiSecret + authTime,
		),
	)
	authHash := fmt.Sprintf(
		"%x",
		authHasher.Sum(nil),
	)

	req.Header.Set("User-Agent", "github.com/kirino-org/kirino")
	req.Header.Set("X-Auth-Date", authTime)
	req.Header.Set("X-Auth-Key", ApiKey)
	req.Header.Set("Authorization", authHash)

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(err)
	}

	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	var results Results
	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	var feeds []*core.Feed

	for _, f := range results.Feeds {
		feeds = append(feeds, &core.Feed{
			ID:          scsc.IntStr(f.Id),
			CoverImage:  f.Image,
			Title:       f.Title,
			Description: f.Description,
		})
	}

	return feeds
}
