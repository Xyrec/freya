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
    { id: 6, title: "Bohemian Rhapsody", artist: "Queen", duration: "5:55" },
    {
      id: 7,
      title: "Like a Rolling Stone",
      artist: "Bob Dylan",
      duration: "6:13",
    },
    {
      id: 8,
      title: "Smells Like Teen Spirit",
      artist: "Nirvana",
      duration: "4:38",
    },
    { id: 9, title: "Imagine", artist: "John Lennon", duration: "3:04" },
    { id: 10, title: "Hotel California", artist: "Eagles", duration: "6:30" },
    {
      id: 11,
      title: "Billie Jean",
      artist: "Michael Jackson",
      duration: "4:54",
    },
    { id: 12, title: "Hey Jude", artist: "The Beatles", duration: "7:11" },
    {
      id: 13,
      title: "Stairway to Heaven",
      artist: "Led Zeppelin",
      duration: "8:02",
    },
    {
      id: 14,
      title: "Born to Run",
      artist: "Bruce Springsteen",
      duration: "4:29",
    },
    { id: 15, title: "Yesterday", artist: "The Beatles", duration: "2:05" },
  ];

  return (
    <div className="space-y-1 p-4">
      {playlist.map((track) => (
        <div
          key={track.id}
          className={`flex items-center justify-between rounded-lg px-4 py-2 hover:bg-accent ${
            track.current ? "bg-accent" : ""
          }`}
        >
          <div className="flex items-center gap-4">
            <span className="text-sm text-muted-foreground">{track.id}</span>
            <div>
              <div className="font-medium leading-none">{track.title}</div>
              <div className="text-sm text-muted-foreground">
                {track.artist}
              </div>
            </div>
          </div>
          <span className="text-sm text-muted-foreground">
            {track.duration}
          </span>
        </div>
      ))}
    </div>
  );
}
