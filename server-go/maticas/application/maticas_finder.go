package application

import "context"

type MaticasFinder interface {
	FindByID(ctx context.Context, id int) string
}
