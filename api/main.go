package api

import (
	"encoding/json"
	"net/http"

	"github.com/kirino-org/kirino/core"
)

var Service = &core.Service{
	ID:   "api",
	Name: "KMS REST API",
	Func: APIRouter,
}

func addAPIEndpoint(r *http.ServeMux, path string, data func() interface{}) {
	r.HandleFunc(path, func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-Type", "application/json")
		w.WriteHeader(200)

		jsonData, err := json.Marshal(data())
		if err != nil {
			panic(err)
		}

		w.Write(jsonData)
	})
}

func APIRouter(c *core.Core) *http.ServeMux {
	r := http.NewServeMux()

	addAPIEndpoint(r, "/version", func() interface{} {
		return map[string]interface{}{
			"kms_version": "0.1",
			"api_version": "1",
		}
	})

	addAPIEndpoint(r, "/fetchers", func() interface{} {
		return c.ListFetchers()
	})

	addAPIEndpoint(r, "/libraries", func() interface{} {
		return c.ListLibraries()
	})

	return r
}
