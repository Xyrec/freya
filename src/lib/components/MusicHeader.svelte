<script lang="ts">
  import { Button } from "./ui/button";
  import { Settings } from "lucide-svelte";
</script>

<header class="flex items-center justify-between border-b px-4 py-2">
  <div class="flex items-center gap-2">
    <Button variant="ghost" size="icon">
      <Settings class="h-4 w-4" />
    </Button>
  </div>
</header>
