package main

import (
	"context"
	"crypto/tls"
	"io"
	"log"
	"os"
	"time"

	"github.com/quic-go/quic-go"
)

func main() {
	tlsConf := &tls.Config{
		InsecureSkipVerify: true,
		NextProtos:         []string{"sample"},
	}
	conn, err := quic.DialAddr(context.Background(), "127.0.0.1:8080", tlsConf, nil)
	if err != nil {
		log.Println(err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	stream, err := conn.AcceptUniStream(ctx)
	io.Copy(os.Stdout,stream)
	if err != nil {
		log.Println(err)
	}
	// new_conn
}

func chk(err error) {
	if err != nil {
		panic(err)
	}
}
