package api

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/internal/scsc"
)

var Service = &core.Service{
	ID:   "api",
	Name: "KMS REST API",
	Func: APIRouter,
}

func setHeaders(w http.ResponseWriter, r *http.Request) {
	w.Header().Add("Content-Type", "application/json")
	w.WriteHeader(200)
}

func APIRouter(c *core.Core) *http.ServeMux {
	ro := mux.NewRouter()

	ro.Path("/library/{lid}/fetch").HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		v := mux.Vars(r)
		setHeaders(w, r)

		l := c.Library(
			scsc.StrInt(
				v["lid"],
			),
		)
		c.Fetch(l)

		jsonData, err := json.Marshal(map[string]interface{}{
			"kms_version": "0.1",
			"api_version": "1",
		})
		if err != nil {
			panic(err)
		}

		w.Write(jsonData)
	})

	ro.Path("/libraries/{lid}/add").HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		v := mux.Vars(r)

		jsonData, err := io.ReadAll(r.Body)
		if err != nil {
			panic(err)
		}

		var data *core.Series
		if err := json.Unmarshal(jsonData, &data); err != nil {
			panic(err)
		}

		c.Library(
			scsc.StrInt(v["lid"]),
		).AddSeries(data)
	})

	ro.Path("/libraries").HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		setHeaders(w, r)

		jsonData, err := json.Marshal(c.Libraries())
		if err != nil {
			panic(err)
		}

		w.Write(jsonData)
	})

	ro.Path("/version").HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		jsonData, err := json.Marshal(map[string]interface{}{
			"kms_version": "0.1",
			"api_version": "1",
		})
		if err != nil {
			panic(err)
		}

		w.Write(jsonData)
	})

	sm := http.NewServeMux()
	sm.HandleFunc("/", ro.ServeHTTP)
	return sm
}
