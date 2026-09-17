
r := chi.NewRouter()

r.Get("/maticas", getMaticas)

func getMaticas(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello World!"))
}