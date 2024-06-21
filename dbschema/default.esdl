module default {
  type EmoteOccurance {
    required property emote_name -> str;
    required property channel_name -> str;
    required property timestamp -> cal::local_datetime;
    required property chatter_name -> str;
  }

  type EmoteUsage {
    required property emote_name -> str;
    required property channel_name -> str;
    required property usage_count -> int64;

    constraint exclusive on ((.emote_name, .channel_name));
  }
}
