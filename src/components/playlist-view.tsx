"use client";

export function PlaylistView() {
  const playlist = [
    { id: 1, title: "Love Has Gone", artist: "Netsky", duration: "4:11" },
    {
      id: 2,
      title: "The Whistle Song",
      artist: "Netsky feat. Dynamic",
      duration: "4:39",
    },
    {
      id: 3,
      title: "Wanna Die For You",
      artist: "Netsky feat. Diane Char",
      duration: "4:17",
    },
    {
      id: 4,
      title: "Come Alive",
      artist: "Netsky",
      duration: "3:11",
      current: true,
    },
    { id: 5, title: "Give & Take", artist: "Netsky", duration: "4:08" },
  ];
  return (
    <div className="px-3 space-y-1 my-2 overscroll-auto">
      {playlist.map((track) => (
        <div
          key={track.id}
          className={`grid grid-cols-[40px_1fr_auto] items-center rounded-lg px-4 py-2 hover:bg-accent cursor-pointer ${
            track.current ? "bg-accent shadow-inner shadow-border" : ""
          }`}
          role="button"
          tabIndex={0}
        >
          <span className="text-sm text-muted-foreground font-mono w-[40px]">
            {track.id}
          </span>
          <div>
            <div className="font-medium leading-none">{track.title}</div>
            <div className="text-sm text-muted-foreground">{track.artist}</div>
          </div>
          <span className="text-sm text-muted-foreground font-mono">
            {track.duration}
          </span>
        </div>
      ))}
    </div>
  );
}
