// @generated automatically by Diesel CLI.

diesel::table! {
    emote_occurrences (id) {
        id -> Nullable<Integer>,
        emote_name -> Text,
        chatter_name -> Text,
        channel_name -> Text,
        occurrence_timestamp -> Nullable<Text>,
    }
}

diesel::table! {
    emote_usage (id) {
        id -> Nullable<Integer>,
        emote_name -> Text,
        chatter_name -> Text,
        channel_name -> Text,
        usage_count -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    emote_occurrences,
    emote_usage,
);
