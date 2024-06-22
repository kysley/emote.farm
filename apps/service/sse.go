package main

import (
	"github.com/tmaxmax/go-sse"
)

var joe = &sse.Joe{}

var sseHandler = &sse.Server{
	Provider: joe,
}
