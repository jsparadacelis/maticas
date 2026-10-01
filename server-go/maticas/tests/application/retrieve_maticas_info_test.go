package application

import (
	"context"
	"testing"

	"github.com/jsparadacelis/maticas/maticas/application"
)

type findByIDCall struct {
	ctx context.Context
	id  int
}

type maticasFinderSpy struct {
	calls  []findByIDCall
	result string
}

func (spy *maticasFinderSpy) FindByID(ctx context.Context, id int) string {
	spy.calls = append(spy.calls, findByIDCall{ctx: ctx, id: id})
	return spy.result
}

func TestRetrieveMaticasInformation_Execute(test *testing.T) {
	finder := &maticasFinderSpy{result: "matica-info"}
	action := application.NewRetrieveMaticasInformation(finder)

	ctx := context.Background()
	id := 42

	result := action.Execute(ctx, id)

	if len(finder.calls) != 1 {
		test.Fatalf("FindByID called %d times; expected 1", len(finder.calls))
	}

	call := finder.calls[0]
	if call.id != id {
		test.Errorf("FindByID called with id = %d; expected %d", call.id, id)
	}
	if call.ctx != ctx {
		test.Errorf("FindByID called with unexpected context")
	}

	if result != finder.result {
		test.Errorf("Execute() = %q; expected %q", result, finder.result)
	}
}
