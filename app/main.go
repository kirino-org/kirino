package app

import (
	"bytes"
	"html/template"
	"net/http"
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

	addRoute(r, "/about", "about", "dash", nil, dataFuncNil)

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
				ID:   scsc.IntStr(len(c.Libraries())),
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
			return c.Library(v["id"])
		} else {
			return nil
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
