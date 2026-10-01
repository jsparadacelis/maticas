package finders

import "context"

type InMemoryMaticasFinder struct{}

func NewInMemoryMaticasFinder() *InMemoryMaticasFinder {
	return &InMemoryMaticasFinder{}
}

func (finder *InMemoryMaticasFinder) FindByID(ctx context.Context, id int) string {
	return "matica stub"
}
