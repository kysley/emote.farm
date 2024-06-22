package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/gempir/go-twitch-irc/v4"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/tmaxmax/go-sse"
)

type Emote struct {
	Id       string
	Platform string
	Code     string
}

func findEmotes(message string, bttv_lookup map[string]Emote, stv_lookup map[string]Emote) {
	words := strings.Fields(message)

	for _, word := range words {
		_, isBttv := bttv_lookup[word]
		_, isStv := stv_lookup[word]

		if isBttv && isStv {
			// fmt.Printf("Emote '%s' found in both lookups.\n", word)
		} else if isBttv {
			// fmt.Printf("Emote '%s' found in BTTV.\n", word)
		} else if isStv {
			// fmt.Printf("Emote '%s' found in 7TV.\n", word)
		} else {
			fmt.Printf("Emote '%s' not found in any lookup.\n", word)
		}
	}
}

func getEmotes(url string, strct any) {
	res, err := http.Get(url)
	if err != nil {
		panic(err.Error())
	}

	body, err := io.ReadAll(res.Body)

	if err != nil {
		panic(err.Error())
	}

	json.Unmarshal(body, &strct)
}

func main() {

	r := gin.Default()

	var bttv_data BTTVResponse
	getEmotes(bttv_url, &bttv_data)

	var bttv_global_data []BTTVChannelEmote
	getEmotes(bttv_global_url, &bttv_global_data)

	var stv_data STVResponse
	getEmotes(sevenTv_url, &stv_data)

	r.Use(cors.New(cors.Config{
		AllowOrigins: []string{"*"}, // Adjust origins as needed
	}))

	r.GET("/lookup", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"7tv":         stv_data,
			"bttv":        bttv_data,
			"bttv_global": bttv_global_data,
		})
	})

	r.GET("/events", func(ctx *gin.Context) {
		handler := sseHandler
		handler.ServeHTTP(ctx.Writer, ctx.Request)
	})

	// or client := twitch.NewAnonymousClient() for an anonymous user (no write capabilities)

	// moonmoon

	bttv_lookup := make(map[string]Emote)
	stv_lookup := make(map[string]Emote)

	for _, emote := range bttv_data.ChannelEmotes {
		bttv_lookup[emote.Code] = Emote{Code: emote.Code, Platform: "bttv", Id: emote.Id}
	}

	for _, emote := range bttv_data.SharedEmotes {
		bttv_lookup[emote.Code] = Emote{Code: emote.Code, Platform: "bttv", Id: emote.Id}
	}

	for _, emote := range bttv_global_data {
		bttv_lookup[emote.Code] = Emote{Code: emote.Code, Platform: "bttv", Id: emote.Id}
	}

	for _, emote := range stv_data.Emotes {
		stv_lookup[emote.Name] = Emote{Code: emote.Name, Platform: "7tv", Id: emote.Id}
	}

	client := twitch.NewAnonymousClient()

	client.OnPrivateMessage(func(message twitch.PrivateMessage) {
		// fmt.Println(message.Message)
		findEmotes(message.Message, bttv_lookup, stv_lookup)
		m := &sse.Message{}
		m.AppendData(message.Message)
		_ = sseHandler.Publish(m)
	})

	client.Join("moonmoon")
	go func() {
		err := client.Connect()

		if err != nil {
			panic(err)
		}
	}()

	// mux := http.NewServeMux()
	// mux.HandleFunc("/lookup", func(w http.ResponseWriter, r *http.Request) {

	// })

	// mux.Handle("/events", sseHandler)

	// handler := cors.Default().Handler(mux)

	// if err := .ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
	// 	return err
	// }

	// if err := http.ListenAndServe(":8006", handler); err != nil {
	// 	log.Fatalln(err)
	// }

	r.Run(":8006")

}
