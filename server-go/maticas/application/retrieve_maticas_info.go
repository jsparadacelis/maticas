package application

import "context"

type RetrieveMaticasInformation struct {
	finder MaticasFinder
}

func NewRetrieveMaticasInformation(finder MaticasFinder) *RetrieveMaticasInformation {
	return &RetrieveMaticasInformation{finder: finder}
}

func (self *RetrieveMaticasInformation) Execute(ctx context.Context, id int) string {
	return self.finder.FindByID(ctx, id)
}