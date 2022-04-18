package core

import "net/http"

type Service struct {
	Id   string
	Name string
	Mux  http.ServeMux
}

func (core *Core) RegisterService(service *Service) {
	core.services = append(core.services, service)
}

// RunServices starts up all services registered on core
func (core *Core) RunServices() {
	mux := http.NewServeMux()

	for _, s := range core.services {
		mux.HandleFunc(
			"/"+s.Id+"/",
			func(w http.ResponseWriter, r *http.Request) {
				s.Mux.ServeHTTP(w, r)
			},
		)
	}

	go http.ListenAndServe(":6319", mux)

	select {}
}
