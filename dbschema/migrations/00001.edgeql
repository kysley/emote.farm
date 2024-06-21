CREATE MIGRATION m1am3tnuhxocepa7wnvz2nrpowpnkj33b5n57lx3q2s6dayvphausq
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::EmoteOccurance {
      CREATE REQUIRED PROPERTY channel_name -> std::str;
      CREATE REQUIRED PROPERTY chatter_name -> std::str;
      CREATE REQUIRED PROPERTY emote_name -> std::str;
      CREATE REQUIRED PROPERTY timestamp -> cal::local_datetime;
  };
  CREATE TYPE default::EmoteUsage {
      CREATE REQUIRED PROPERTY channel_name -> std::str;
      CREATE REQUIRED PROPERTY emote_name -> std::str;
      CREATE CONSTRAINT std::exclusive ON ((.emote_name, .channel_name));
      CREATE REQUIRED PROPERTY usage_count -> std::int64;
  };
};
