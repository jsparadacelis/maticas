package main

import (
    "net/http"

    "github.com/go-chi/chi/v5"
    "github.com/go-chi/chi/v5/middleware"

    "github.com/jsparadacelis/maticas/maticas/application"
    "github.com/jsparadacelis/maticas/maticas/infraestructure/api"
    "github.com/jsparadacelis/maticas/maticas/infraestructure/finders"
)

func main() {
    retrieveMaticasInformation := buildRetrieveMaticasInformation()
    router := buildRouter(retrieveMaticasInformation)
    http.ListenAndServe(":3000", router)
}

func buildRetrieveMaticasInformation() *application.RetrieveMaticasInformation {
    finder := finders.NewInMemoryMaticasFinder()
    return application.NewRetrieveMaticasInformation(finder)
}

func buildRouter(retrieveMaticasInformation *application.RetrieveMaticasInformation) *chi.Mux {
    router := chi.NewRouter()
    router.Use(middleware.Logger)
    router.Get("/", handle)
    router.Get("/maticas", api.NewGetMaticas(retrieveMaticasInformation))
    return router
}


func handle(responseWriter http.ResponseWriter, request *http.Request) {
        responseWriter.Write([]byte("Hello World!"))
    }