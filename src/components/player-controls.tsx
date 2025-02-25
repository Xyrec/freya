import { Play, SkipBack, SkipForward, Volume2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import Image from "next/image";

export function PlayerControls() {
  return (
    <div className="border-t bg-background p-4">
      <div className="flex items-center gap-4">
        <Image
          src="/placeholder.svg?height=48&width=48"
          alt="Album art"
          className="h-12 w-12 rounded-md"
          height={48}
          width={48}
        />
        <div className="flex-1">
          <div className="mb-2 flex items-center justify-between">
            <div>
              <div className="font-medium leading-none">Come Alive</div>
              <div className="text-sm text-muted-foreground">Netsky</div>
            </div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <span>2:14</span>
              <span>/</span>
              <span>3:11</span>
            </div>
          </div>
          <Slider defaultValue={[70]} max={100} step={1} className="my-2" />
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
