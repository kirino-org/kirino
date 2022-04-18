package podcastindex

type Results struct {
	Status      string `json:"status"`
	Feeds       []Feed `json:"feeds"`
	Count       int    `json:"count"`
	Query       string `json:"query"`
	Description string `json:"description"`
}

type Feed struct {
	Id                     int    `json:"id"`
	Title                  string `json:"title"`
	Url                    string `json:"url"`
	OriginalUrl            string `json:"originalUrl"`
	Link                   string `json:"link"`
	Description            string `json:"description"`
	Author                 string `json:"author"`
	OwnerName              string `json:"ownerName"`
	Image                  string `json:"image"`
	Artwork                string `json:"artwork"`
	LastUpdateTime         int    `json:"lastUpdateTime"`
	LastCrawlTime          int    `json:"lastCrawlTime"`
	LastParseTime          int    `json:"lastParseTime"`
	LastGoodHttpStatusTime int    `json:"lastGoodHttpStatusTime"`
	LastHttpStatus         int    `json:"lastHttpStatus"`
	ContentType            string `json:"contentType"`
	ItunesId               int    `json:"itunesId"`
	Generator              string `json:"generator"`
	Language               string `json:"language"`
	Type                   int    `json:"type"`
	Dead                   int    `json:"dead"`
	CrawlErrors            int    `json:"crawlErrors"`
	ParseErrors            int    `json:"parseErrors"`
	Locked                 int    `json:"locked"`
	ImageUrlHash           int64  `json:"imageUrlHash"`
}
