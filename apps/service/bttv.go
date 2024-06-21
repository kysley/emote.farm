package main

type BTTVResponse struct {
	Id            string
	Bots          []string
	Avatar        string
	ChannelEmotes []BTTVChannelEmote
	SharedEmotes  []BTTVSharedEmote
}

type BTTVSharedEmote struct {
	Id        string
	Code      string
	ImateType string
	Animated  bool
	User      BTTVUser
}

type BTTVChannelEmote struct {
	Id        string
	Code      string
	ImageType string
	Animated  bool
	UserId    string
}

type BTTVUser struct {
	Id          string
	Name        string
	DisplayName string
	ProviderId  string
}

var bttv_url = "https://api.betterttv.net/3/cached/users/twitch/121059319"

var bttv_global_url = "https://api.betterttv.net/3/cached/emotes/global"
