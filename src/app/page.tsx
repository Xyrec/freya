"use client";

import { PlayerControls } from "@/components/player-controls";
import { PlaylistView } from "@/components/playlist-view";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Settings } from "lucide-react";

export default function MusicPlayer() {
  return (
    <div className="flex h-screen flex-col bg-background text-foreground">
      <Tabs defaultValue="playlist" className="flex-1 relative">
        <div className="flex items-center justify-between border-b bg-background">
          <TabsList className="w-full justify-start rounded-none p-0">
            <TabsTrigger
              value="playlist"
              className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
            >
              Playlist
            </TabsTrigger>
            <TabsTrigger
              value="lyrics"
              className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
            >
              Lyrics
            </TabsTrigger>
            <TabsTrigger
              value="bio"
              className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary"
            >
              Bio
            </TabsTrigger>
          </TabsList>
          <div className="flex items-center gap-2 pr-4">
            <Button variant="ghost" size="icon">
              <Settings className="h-4 w-4" />
            </Button>
            {/* <ThemeToggle /> */}
          </div>
        </div>
        <TabsContent value="playlist" className="absolute">
          <ScrollArea className="h-full">
            <PlaylistView />
          </ScrollArea>
        </TabsContent>
      </Tabs>
      <div className="fixed bottom-0 left-0 right-0">
        <PlayerControls />
      </div>
    </div>
  );
}

// function ThemeToggle() {
//   const { theme, setTheme } = useTheme();

//   return (
//     <Button
//       variant="ghost"
//       size="icon"
//       onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
//     >
//       <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
//       <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
//       <span className="sr-only">Toggle theme</span>
//     </Button>
//   );
// }
