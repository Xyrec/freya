"use client";

import { Play, SkipBack, SkipForward, Volume2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import Image from "next/image";

export function PlayerControls() {
  return (
    <div className="w-full border-t bg-background p-4 z-10">
      <div className="flex items-center gap-4">
        <Image
          src="album-art.jpg"
          alt="Album art"
          className="aspect-square w-24 rounded-md"
          width={96}
          height={96}
          placeholder="empty"
        />
        <div className="flex-1">
          <div className="mb-2 flex items-center justify-between">
            <div>
              <div className="font-medium leading-none">Come Alive</div>
              <div className="text-sm text-muted-foreground">Netsky</div>
            </div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground font-mono">
              <span>0:00</span>
              <span>/</span>
              <span>0:00</span>
            </div>
          </div>
          <Slider
            defaultValue={[70]}
            max={100}
            step={0.01}
            className="mb-3 mt-2 w-[calc(100%)]"
          />
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <Button variant="ghost" size="icon">
                <SkipBack className="h-4 w-4" />
              </Button>
              <Button size="icon">
                <Play className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon">
                <SkipForward className="h-4 w-4" />
              </Button>
            </div>
            <div className="flex items-center gap-2">
              <Volume2 className="h-4 w-4" />
              <Slider
                defaultValue={[70]}
                max={100}
                step={1}
                className="w-[100px]"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
