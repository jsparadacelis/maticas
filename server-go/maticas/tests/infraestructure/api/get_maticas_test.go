package api

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/jsparadacelis/maticas/maticas/application"
	"github.com/jsparadacelis/maticas/maticas/infraestructure/api"
)

type maticasFinderFake struct {
	result string
}

func (fake *maticasFinderFake) FindByID(ctx context.Context, id int) string {
	return fake.result
}

func TestGetMaticas(test *testing.T) {
	finder := &maticasFinderFake{result: "matica-info"}
	action := application.NewRetrieveMaticasInformation(finder)
	handler := api.NewGetMaticas(action)

	request := httptest.NewRequest(http.MethodGet, "/maticas", nil)
	recorder := httptest.NewRecorder()

	handler(recorder, request)

	response := recorder.Result()
	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		test.Errorf("status code = %d; expected %d", response.StatusCode, http.StatusOK)
	}

	body, err := io.ReadAll(response.Body)
	if err != nil {
		test.Fatalf("unexpected error reading body: %v", err)
	}

	if string(body) != finder.result {
		test.Errorf("body = %q; expected %q", string(body), finder.result)
	}
}
