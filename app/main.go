package app

import (
	"bytes"
	"fmt"
	"html/template"
	"io"
	"net/http"
	"os"
	"runtime"
	"strings"

	"github.com/kirino-org/kirino/core"
	"github.com/kirino-org/kirino/internal/scsc"
)

func renderTmpl(w http.ResponseWriter, r *http.Request, tmpl string, data interface{}) {
	var render bytes.Buffer
	t := template.New("").Funcs(template.FuncMap{
		"GoVersion": func() string {
			return runtime.Version()
		},
		"GoNumGoroutine": func() string {
			return scsc.IntStr(
				runtime.NumGoroutine(),
			)
		},
		"GoNumCPU": func() string {
			return scsc.IntStr(
				runtime.NumCPU(),
			)
		},
	})

	_, err := t.ParseFiles("./app/views/"+tmpl+".html", "./app/views/base.html")
	if err != nil {
		panic(err)
	}

	t.ExecuteTemplate(&render, "base.html", data)

	w.WriteHeader(200)
	w.Write(render.Bytes())

	render.Reset()
}

func addRoute(r *http.ServeMux, path, formTmpl, actionTmpl string, params []string, dataFunc func(vals map[string]string) interface{}) {
	r.HandleFunc(path, func(w http.ResponseWriter, r *http.Request) {
		vals := make(map[string]string)

		for _, v := range params {
			vals[v] = r.FormValue(v)
		}

		data := dataFunc(vals)

		var tmpl string
		switch r.Method {
		case "GET":
			tmpl = formTmpl
		case "POST":
			tmpl = actionTmpl
		default:
			tmpl = "dash"
		}

		renderTmpl(w, r, tmpl, rendererData{
			Config: config,
			Data:   data,
		})
	})
}

func dataFuncNil(vals map[string]string) interface{} {
	return nil
}

func AppRouter(c *core.Core) *http.ServeMux {
	r := http.NewServeMux()

	addRoute(r, "/dash", "dash", "dash", nil, dataFuncNil)

	addRoute(r, "/about", "about", "dash", nil, func(vals map[string]string) interface{} {
		return License
	})

	addRoute(r, "/manage", "manage", "dash", nil, dataFuncNil)

	addRoute(r, "/search", "search", "results", []string{
		"fetcher",
		"query",
	}, func(v map[string]string) interface{} {
		if v["fetcher"] != "" && v["query"] != "" {
			return c.Fetcher(v["fetcher"]).SearchFunc(v["query"])
		} else {
			return c.Fetchers()
		}
	})

	addRoute(r, "/libraries", "libraries", "dash", nil, func(vals map[string]string) interface{} {
		return c.Libraries()
	})
	addRoute(r, "/libraries/new", "new_library", "libraries", []string{
		"name",
	}, func(v map[string]string) interface{} {
		if v["name"] != "" {
			c.NewLibrary(&core.Library{
				ID:   len(c.Libraries()),
				Type: core.LibraryTypeMusic,
				Name: v["name"],
			})

			return c.Libraries()
		} else {
			return nil
		}
	})

	addRoute(r, "/library", "library", "dash", []string{
		"id",
	}, func(v map[string]string) interface{} {
		if v["id"] != "" {
			return c.Library(
				scsc.StrInt(v["id"]),
			)
		} else {
			return nil
		}
	})

	addRoute(r, "/series", "series", "dash", []string{
		"lid",
		"sid",
	}, func(v map[string]string) interface{} {
		series := c.Library(
			scsc.StrInt(v["lid"]),
		).Series(
			scsc.StrInt(v["sid"]),
		)

		return struct {
			Series  *core.Series
			Library string
		}{
			Series:  series,
			Library: v["lid"],
		}
	})

	addRoute(r, "/play", "play", "play", []string{
		"lid",
		"sid",
		"iid",
	}, func(v map[string]string) interface{} {
		return struct {
			StreamURL string
			Series    *core.Series
		}{
			StreamURL: fmt.Sprintf(`/app/stream?lid=%s&sid=%s&iid=%s`, v["lid"], v["sid"], v["iid"]),
			Series:    c.Library(scsc.StrInt(v["lid"])).Series(scsc.StrInt(v["sid"])),
		}
	})

	r.HandleFunc("/stream", func(w http.ResponseWriter, r *http.Request) {
		lid := scsc.StrInt(r.FormValue("lid"))
		sid := scsc.StrInt(r.FormValue("sid"))
		iid := scsc.StrInt(r.FormValue("iid"))

		mediaPath := c.Library(lid).
			Series(sid).
			Item(iid).
			Path

		w.WriteHeader(200)

		if strings.HasPrefix(mediaPath, "https://") || strings.HasPrefix(mediaPath, "http://") {
			res, err := http.Get(mediaPath)
			if err != nil {
				panic(err)
			}

			io.Copy(w, res.Body)
		} else {
			f, err := os.Open(mediaPath)
			if err != nil {
				panic(err)
			}

			io.Copy(w, f)
		}
	})

	r.Handle(
		"/assets/",
		http.StripPrefix(
			strings.TrimRight("/assets/", "/"),
			http.FileServer(http.Dir("./app/assets")),
		),
	)

	return r
}
