import type { MetaFunction } from "@remix-run/node";
import { useEffect, useId, useState } from "react";

export const meta: MetaFunction = () => {
  return [
    { title: "New Remix SPA" },
    { name: "description", content: "Welcome to Remix (SPA Mode)!" },
  ];
};

function Emote({ emote }) {
  let url = "";
  if (emote.mode === "bttv") {
    url = `https://cdn.betterttv.net/emote/${emote.id}/1x`;
  } else if (emote.mode === "7tv") {
    url = `https://cdn.7tv.app/emote/${emote.id}/1x.webp`;
  }

  return (
    <li>
      <img aria-label="asd" src={url} />
    </li>
  );
}

export default function Index() {
  const [msgs, setMsgs] = useState<Array<{ word: string; id: string }>>([]);
  const [lookup, setLookup] = useState(new Map());

  useEffect(() => {
    const set = new Map();
    let eventSource: EventSource;
    async function getLookup() {
      const res = await fetch("http://localhost:8006/lookup");
      const data = await res.json();

      for (const emote of data["7tv"].Emotes) {
        set.set(emote.Name, {
          code: emote.Name,
          id: emote.Id,
          mode: "7tv",
          _id: Math.random(),
        });
      }
      for (const emote of data.bttv.ChannelEmotes) {
        set.set(emote.Code, {
          code: emote.Code,
          id: emote.Id,
          mode: "bttv",
          _id: Math.random(),
          // gif: emote.Animated,
        });
      }
      for (const emote of data.bttv.SharedEmotes) {
        set.set(emote.Code, {
          code: emote.Code,
          id: emote.Id,
          mode: "bttv",
          _id: Math.random(),
          // gif: emote.Animated,
        });
      }

      for (const emote of data.bttv_global) {
        set.set(emote.Code, {
          code: emote.Code,
          id: emote.Id,
          mode: "bttv",
          _id: Math.random(),
          // gif: emote.Animated,
        });
      }

      setLookup(set);
    }

    getLookup().then(() => {
      eventSource = new EventSource("http://localhost:8006/events");
      eventSource.onmessage = (e) => {
        setMsgs((m) =>
          [
            ...(e.data as string)
              .split(" ")
              .filter((word) => set.has(word))
              .map((word) => ({ word, id: `${word}-${Math.random()}` })),
            ...m,
          ].slice(0, 50)
        );
      };
    });

    return () => {
      eventSource?.close();
    };
  }, []);

  return (
    <div className="font-sans p-4">
      <h1 className="text-3xl">Welcome to Remix (SPA Mode)</h1>
      <ul className="list-disc mt-4 pl-6 space-y-2">
        {msgs.map((msg) => {
          const match = lookup.get(msg.word);
          if (match) {
            return <Emote emote={match} key={msg.id} />;
          }
          return null;
          // return msg.msg.split(" ").map((word, i) => {
        })}
      </ul>
    </div>
  );
}
