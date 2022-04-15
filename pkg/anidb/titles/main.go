/*
	anidb_titles

	Client for AniDB.net's title dump API
*/
package anidb_titles

import (
	"compress/gzip"
	"encoding/xml"
	"io"
	"net/http"
	"os"

	"github.com/kirino-org/kirino/pkg/anidb"
)

func FetchTitles() {
	req, err := http.NewRequest("GET", anidb.TitleDumpURL, nil)
	if err != nil {
		panic(err)
	}

	req.Header.Add("User-Agent", anidb.UserAgent)

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(res)
	}

	defer res.Body.Close()

	r, err := gzip.NewReader(res.Body)
	if err != nil {
		panic(err)
	}

	f, err := os.Create(anidb.TitlesPath)
	if err != nil {
		panic(err)
	}

	io.Copy(f, r)
}

func DecodeTitles() map[int][]Title {
	rawXmlData, err := os.ReadFile(anidb.TitlesPath)
	if err != nil {
		panic(err)
	}

	var Data titles

	if err := xml.Unmarshal(rawXmlData, &Data); err != nil {
		panic(err)
	}

	var retData map[int][]Title
	for _, t := range Data.Anime {
		retData[t.ID] = []Title{}

		for i, v := range t.Titles {
			retData[t.ID][i].Title = v.Title

			switch v.Type {
			case "official":
				retData[t.ID][i].Type = TitleTypeOfficial
			case "main":
				retData[t.ID][i].Type = TitleTypeMain
			case "syn":
				retData[t.ID][i].Type = TitleTypeSynonym
			default:
				retData[t.ID][i].Type = TitleTypeOther
			}
		}
	}

	return retData
}
