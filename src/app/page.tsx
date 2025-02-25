"use client";

import { PlayerControls } from "@/components/player-controls";
import { PlaylistView } from "@/components/playlist-view";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";

export default function MusicPlayer() {
  function ThemeToggle() {
    const { theme, setTheme } = useTheme();

    return (
      <Button
        variant="ghost"
        size="icon"
        className="hover:bg-transparent mr-2 hover:cursor-pointer"
        onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
      >
        <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span className="sr-only">Toggle theme</span>
      </Button>
    );
  }
  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <Tabs
        defaultValue="playlist"
        className="flex-1 flex flex-col overflow-hidden"
      >
        <TabsList className="w-full justify-start rounded-none border-b bg-background p-0 z-50 shadow">
          <TabsTrigger
            value="playlist"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Playlist
          </TabsTrigger>
          <TabsTrigger
            value="lyrics"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Lyrics
          </TabsTrigger>
          <TabsTrigger
            value="bio"
            className="rounded-none border-b-2 border-transparent px-4 py-2 data-[state=active]:border-primary hover:cursor-pointer"
          >
            Bio
          </TabsTrigger>
          <div className="flex-1" />
          <ThemeToggle />
        </TabsList>
        <TabsContent value="playlist" className="flex-1 p-0 overflow-hidden">
          <ScrollArea className="h-full">
            <PlaylistView />
          </ScrollArea>
        </TabsContent>
      </Tabs>
      <PlayerControls />
    </div>
  );
}
