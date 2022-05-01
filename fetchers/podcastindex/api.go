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

// Set request headers
func setHeaders(req *http.Request) {
	// Get current UNIX time
	authTime := scsc.Int64Str(
		time.Now().Unix(),
	)

	/*
		Hash APIKey, APISecret, and authTime to authHash
	*/
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

	// Set user agent
	req.Header.Set("User-Agent", "github.com/kirino-org/kirino")
	// Set auth time
	req.Header.Set("X-Auth-Date", authTime)
	// Set auth key
	req.Header.Set("X-Auth-Key", ApiKey)
	// Set authorization
	req.Header.Set("Authorization", authHash)
}

// Search by term
func searchByTerm(query string) []*core.Series {
	// Create request
	req, err := http.NewRequest("GET", "https://api.podcastindex.org/api/1.0/search/byterm?q="+query, nil)
	if err != nil {
		panic(err)
	}

	// Set request headers
	setHeaders(req)

	// Send the request
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(err)
	}

	// Read response body to rawJson
	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	// Unmarshal rawJson
	var results Results
	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	/*
		Prepare feeds to return
	*/
	var feeds []*core.Series

	for _, f := range results.Feeds {
		feeds = append(feeds, &core.Series{
			ID:          f.Id,
			Image:       f.Image,
			Title:       f.Title,
			Description: f.Description,
		})
	}

	// Return feeds
	return feeds
}

// Get feed by ID
func getFeedByID(id string) Feed {
	// Create request
	req, err := http.NewRequest("GET", "https://api.podcastindex.org/api/1.0/podcasts/byfeedid?id="+id, nil)
	if err != nil {
		panic(err)
	}

	// Set request headers
	setHeaders(req)

	// Send request
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(err)
	}

	// Read response to rawJson
	rawJson, err := io.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}

	/*
		Unmarshal rawJson
	*/
	var results struct {
		Feed Feed `json:"feed"`
	}
	if err := json.Unmarshal(rawJson, &results); err != nil {
		panic(err)
	}

	/*
		Return feed
	*/
	// f := results.Feed
	// return &core.Feed{
	// 	ID:          scsc.IntStr(f.Id),
	// 	CoverImage:  f.Image,
	// 	Title:       f.Title,
	// 	Description: f.Description,
	// }
	return results.Feed
}
