package api

import (
	"net/http"

	"github.com/jsparadacelis/maticas/maticas/application"
)

func NewGetMaticas(action *application.RetrieveMaticasInformation) http.HandlerFunc {
	return func(responseWriter http.ResponseWriter, request *http.Request) {
		result := action.Execute(request.Context(), 1)
		responseWriter.Write([]byte(result))
	}
}
