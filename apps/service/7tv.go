package main

type STVResponse struct {
	Id          string
	Name        string
	Emotes      []STVEmote
	Emote_count int
}

type STVEmote struct {
	Id   string
	Name string
}

// its like a third party variable or whatever
var sevenTv_url = "https://7tv.io/v3/emote-sets/6372cdb3541612cf475f545c"
