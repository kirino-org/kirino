package app

import (
	"bytes"
	"html/template"
	"net/http"
	"strings"

	"github.com/kirino-org/kirino/core"
)

type renderConfig struct {
	Title   string
	BaseURL string
}

type rendererData struct {
	Config renderConfig
	Data   interface{}
}

func renderTmpl(w http.ResponseWriter, r *http.Request, tmpl string, data interface{}) {
	var render bytes.Buffer
	t := template.New("").Funcs(template.FuncMap{})

	_, err := t.ParseFiles("./app/views/"+tmpl+".html", "./app/views/base.html")
	if err != nil {
		panic(err)
	}

	t.ExecuteTemplate(&render, "base.html", data)

	w.WriteHeader(200)
	w.Write(render.Bytes())

	render.Reset()
}

var Service = &core.Service{
	Id:   "app",
	Name: "Kirino Web",
	Mux:  AppRouter(),
}

func AppRouter() *http.ServeMux {
	r := http.NewServeMux()

	config := renderConfig{
		Title:   "Kirino Web",
		BaseURL: "/app",
	}

	r.HandleFunc("/app/home", func(w http.ResponseWriter, r *http.Request) {
		renderTmpl(w, r, "home", rendererData{
			Config: config,
		})
	})

	r.HandleFunc("/app/about", func(w http.ResponseWriter, r *http.Request) {
		renderTmpl(w, r, "about", rendererData{
			Config: config,
		})
	})

	r.Handle(
		"/app/assets/",

		http.StripPrefix(
			strings.TrimRight("/app/assets/", "/"),
			http.FileServer(
				http.Dir("./app/assets"),
			),
		),
	)

	return r
}
