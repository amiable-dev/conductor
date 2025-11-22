# MIDIMon shadcn-ui Migration Guide

**Goal:** Incrementally migrate MIDIMon's custom UI to shadcn-ui + Tailwind CSS while maintaining full functionality.

**Timeline:** 2-3 days (can be done over multiple sessions)

**Risk Level:** Low (incremental approach preserves working app at each step)

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Phase 1: Setup & Configuration](#phase-1-setup--configuration)
3. [Phase 2: Simple Component Migration](#phase-2-simple-component-migration)
4. [Phase 3: Layout & Utilities](#phase-3-layout--utilities)
5. [Phase 4: Complex Components](#phase-4-complex-components)
6. [Phase 5: Polish & Cleanup](#phase-5-polish--cleanup)
7. [Troubleshooting](#troubleshooting)
8. [Rollback Plan](#rollback-plan)

---

## Prerequisites

**Before starting:**
- ✅ Current MIDIMon GUI is working (`cargo tauri dev` runs successfully)
- ✅ Git commit your current state: `git add -A && git commit -m "Pre-shadcn migration checkpoint"`
- ✅ Node.js 18+ installed
- ✅ Familiarity with Tailwind CSS basics (optional but helpful)

**Estimated Time:** 10 minutes

---

## Phase 1: Setup & Configuration

**Goal:** Install and configure Tailwind CSS and shadcn-svelte without breaking existing UI.

**Estimated Time:** 1-2 hours

### Step 1.1: Install Tailwind CSS

```bash
cd midimon-gui/ui
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

**Expected output:** Creates `tailwind.config.js` and `postcss.config.js`

### Step 1.2: Configure Tailwind

Edit `midimon-gui/ui/tailwind.config.js`:

```js
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // MIDIMon dark theme colors
        background: {
          DEFAULT: '#1e1e1e',
          dark: '#1a1a1a',
          card: '#2a2a2a',
          elevated: '#252525',
        },
        foreground: {
          DEFAULT: '#e0e0e0',
          muted: '#999',
          dimmed: '#666',
        },
        primary: {
          DEFAULT: '#4a9eff',
          hover: '#3a8eef',
        },
        border: '#333',
        success: '#6ad98b',
        warning: '#ffa500',
        error: '#ff6b6b',
      },
    },
  },
  plugins: [],
}
```

### Step 1.3: Create Tailwind CSS Entry Point

Create `midimon-gui/ui/src/app.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Global base styles */
@layer base {
  :root {
    --background: 30 30 30; /* #1e1e1e */
    --foreground: 224 224 224; /* #e0e0e0 */

    --card: 42 42 42; /* #2a2a2a */
    --card-foreground: 224 224 224;

    --primary: 74 158 255; /* #4a9eff */
    --primary-foreground: 255 255 255;

    --border: 51 51 51; /* #333 */
    --input: 51 51 51;

    --ring: 74 158 255;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
  }
}
```

### Step 1.4: Import Tailwind in App.svelte

Edit `midimon-gui/ui/src/App.svelte`, add at the top of the `<script>` section:

```svelte
<script>
  import './app.css'; // Add this line
  import { onMount } from 'svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  // ... rest of imports
</script>
```

### Step 1.5: Test Build

```bash
npm run dev
```

**Expected:** App still looks the same (Tailwind base styles applied but custom CSS still active)

**Checkpoint:** Commit changes
```bash
git add -A
git commit -m "feat(ui): Add Tailwind CSS configuration"
```

---

### Step 1.6: Install shadcn-svelte

```bash
npx shadcn-svelte@latest init
```

**Interactive prompts - Choose:**
- ✅ **TypeScript?** No (we're keeping JavaScript for now)
- ✅ **Style:** New York (modern, clean)
- ✅ **Base color:** Slate (matches dark theme)
- ✅ **CSS variables:** Yes
- ✅ **Tailwind config location:** `tailwind.config.js`
- ✅ **Components location:** `src/lib/components/ui`
- ✅ **Utils location:** `src/lib/utils`

**Expected output:**
- Creates `src/lib/components/ui/` directory
- Creates `src/lib/utils.js`
- Updates `tailwind.config.js`

### Step 1.7: Install First Components

```bash
npx shadcn-svelte@latest add button
npx shadcn-svelte@latest add card
npx shadcn-svelte@latest add badge
```

**Expected:** Component files appear in `src/lib/components/ui/`

### Step 1.8: Verify Setup

Create `midimon-gui/ui/src/lib/components/TestShadcn.svelte`:

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
</script>

<Card class="w-96">
  <CardHeader>
    <CardTitle>shadcn-ui Test</CardTitle>
  </CardHeader>
  <CardContent class="flex gap-2">
    <Button>Primary</Button>
    <Button variant="outline">Outline</Button>
    <Badge>Success</Badge>
  </CardContent>
</Card>
```

Temporarily add to `App.svelte` before `<Sidebar />`:

```svelte
<script>
  import TestShadcn from './lib/components/TestShadcn.svelte'; // Add
  // ...
</script>

<div class="app">
  <TestShadcn /> <!-- Add this temporarily -->
  <Sidebar />
  <!-- ... -->
</div>
```

**Expected:** See a card with buttons and badge in top-left corner

**Remove the test** once verified:
```svelte
<!-- Remove TestShadcn import and component -->
```

**Checkpoint:** Commit
```bash
git add -A
git commit -m "feat(ui): Install shadcn-svelte components"
```

---

## Phase 2: Simple Component Migration

**Goal:** Replace custom buttons and badges with shadcn components.

**Estimated Time:** 4-6 hours

### Step 2.1: Migrate StatusBar Badges

**Before:** `midimon-gui/ui/src/lib/components/StatusBar.svelte`

```svelte
<div class="status-section">
  <span class="status-label">Status:</span>
  <span class="status-value">{getStatusText($statusStore.status)}</span>
</div>
```

**After:**

```svelte
<script>
  import { Badge } from "$lib/components/ui/badge";
  // ... existing imports

  function getStatusVariant(status) {
    if (!status || !status.running) return 'destructive';
    if (!status.connected) return 'secondary';
    if (status.error) return 'secondary';
    return 'default';
  }
</script>

<div class="flex items-center gap-2">
  <span class="text-sm text-muted-foreground">Status:</span>
  <Badge variant={getStatusVariant($statusStore.status)}>
    {getStatusText($statusStore.status)}
  </Badge>
</div>
```

**Update the component:**
1. Import `Badge` from shadcn
2. Replace status value spans with `<Badge>`
3. Convert container divs to Tailwind flex utilities
4. Update CSS classes to use Tailwind equivalents

**Full StatusBar.svelte migration:**

```svelte
<script>
  import { onMount, onDestroy } from 'svelte';
  import { Badge } from "$lib/components/ui/badge";
  import { statusStore } from '../stores.js';

  // ... keep existing functions ...
</script>

<div class="flex items-center gap-8 px-6 py-3 bg-background-elevated border-b border-border text-sm">
  <!-- Status Indicator -->
  <div class="flex items-center gap-2">
    <div
      class="w-2 h-2 rounded-full animate-pulse"
      style="background-color: {getStatusColor($statusStore.status)}"
    ></div>
    <span class="text-muted-foreground font-medium">Status:</span>
    <Badge variant={getStatusVariant($statusStore.status)}>
      {getStatusText($statusStore.status)}
    </Badge>
  </div>

  <!-- Mode -->
  <div class="flex items-center gap-2">
    <span class="text-muted-foreground font-medium">Mode:</span>
    <span class="text-foreground">{getCurrentMode($statusStore.status)}</span>
  </div>

  <!-- Device -->
  <div class="flex items-center gap-2">
    <span class="text-muted-foreground font-medium">Device:</span>
    <code class="text-foreground text-xs font-mono">
      {getConnectedDevice($statusStore.status)}
    </code>
  </div>

  <!-- Uptime (conditional) -->
  {#if $statusStore.status && $statusStore.status.uptime_secs !== null}
    <div class="flex items-center gap-2">
      <span class="text-muted-foreground font-medium">Uptime:</span>
      <span class="text-foreground">{formatUptime($statusStore.status.uptime_secs)}</span>
    </div>
  {/if}

  <!-- Events (conditional) -->
  {#if $statusStore.status && $statusStore.status.events_processed !== null}
    <div class="flex items-center gap-2">
      <span class="text-muted-foreground font-medium">Events:</span>
      <span class="text-foreground">{$statusStore.status.events_processed.toLocaleString()}</span>
    </div>
  {/if}

  <!-- Error (conditional) -->
  {#if $statusStore.error}
    <div class="ml-auto flex items-center gap-2 text-destructive">
      <span class="text-base">⚠️</span>
      <span class="text-xs max-w-xs truncate">{$statusStore.error}</span>
    </div>
  {/if}
</div>

<!-- Remove <style> block - all Tailwind now -->
```

**Test:** Status bar should look the same but use shadcn Badge

---

### Step 2.2: Migrate DevicesView Buttons

**Install button-related components:**
```bash
npx shadcn-svelte@latest add button
```

**Before:** `midimon-gui/ui/src/lib/views/DevicesView.svelte`

```svelte
<button class="btn-secondary" on:click={openTemplateSelector}>
  📋 Device Templates
</button>
```

**After:**

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
  // ... other imports
</script>

<Button variant="outline" on:click={openTemplateSelector}>
  📋 Device Templates
</Button>
```

**Migrate all buttons in DevicesView:**

```svelte
<div class="flex items-center justify-between mb-6">
  <div>
    <h2 class="text-2xl font-semibold text-foreground">Devices & Profiles</h2>
    <p class="text-sm text-muted-foreground mt-1">
      Manage MIDI device connections, templates, and per-app profiles
    </p>
  </div>
  <div class="flex gap-3 items-center">
    {#if loading}
      <div class="flex items-center gap-2 text-primary text-sm">
        <span class="inline-block w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin"></span>
        Loading...
      </div>
    {/if}
    <Button variant="outline" on:click={openTemplateSelector}>
      📋 Device Templates
    </Button>
    <Button variant="outline" on:click={openProfileManager}>
      🔄 Profiles
    </Button>
  </div>
</div>
```

---

### Step 2.3: Migrate DeviceList Component

**Install additional components:**
```bash
npx shadcn-svelte@latest add card
npx shadcn-svelte@latest add separator
```

**Update:** `midimon-gui/ui/src/lib/components/DeviceList.svelte`

```svelte
<script>
  import { onMount, onDestroy } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  import { Separator } from "$lib/components/ui/separator";
  import { devicesStore } from '../stores.js';

  async function handleRefresh() {
    await devicesStore.fetch();
  }

  onMount(() => {
    devicesStore.startAutoRefresh();
  });

  onDestroy(() => {
    devicesStore.stopAutoRefresh();
  });
</script>

<Card>
  <CardHeader>
    <div class="flex items-center justify-between">
      <CardTitle>Available MIDI Devices</CardTitle>
      <Button
        variant="ghost"
        size="sm"
        on:click={handleRefresh}
        disabled={$devicesStore.loading}
      >
        <span class="mr-2">{$devicesStore.loading ? '⏳' : '🔄'}</span>
        Refresh
      </Button>
    </div>
  </CardHeader>
  <CardContent>
    {#if $devicesStore.loading && $devicesStore.devices.length === 0}
      <div class="flex items-center justify-center py-8 text-muted-foreground">
        <p>Scanning for MIDI devices...</p>
      </div>
    {:else if $devicesStore.error}
      <div class="flex flex-col items-center justify-center gap-4 py-8">
        <p class="text-destructive">{$devicesStore.error}</p>
        <Button variant="outline" on:click={handleRefresh}>
          Try Again
        </Button>
      </div>
    {:else if $devicesStore.devices.length === 0}
      <div class="flex flex-col items-center justify-center gap-3 py-12 text-center">
        <span class="text-6xl opacity-50">🎹</span>
        <h3 class="text-lg font-medium text-foreground">No MIDI Devices Found</h3>
        <p class="text-sm text-muted-foreground max-w-sm">
          Please connect a MIDI device and click refresh.
        </p>
      </div>
    {:else}
      <div class="space-y-3">
        {#each $devicesStore.devices as device, index (device.port_index)}
          <div class="flex items-center justify-between p-4 rounded-lg border border-border bg-background-card hover:bg-background-elevated transition-colors">
            <div class="flex-1">
              <div class="flex items-center gap-3">
                <h4 class="font-medium text-foreground">{device.port_name}</h4>
                {#if device.connected}
                  <Badge variant="default">Connected</Badge>
                {:else}
                  <Badge variant="outline">Available</Badge>
                {/if}
              </div>
              <div class="flex items-center gap-4 mt-2 text-sm text-muted-foreground">
                <span>Port {device.port_index}</span>
                {#if device.manufacturer}
                  <Separator orientation="vertical" class="h-4" />
                  <span>{device.manufacturer}</span>
                {/if}
              </div>
            </div>
            <Button variant={device.connected ? "outline" : "default"} size="sm">
              {device.connected ? 'Disconnect' : 'Connect'}
            </Button>
          </div>
        {/each}
      </div>
    {/if}
  </CardContent>
</Card>

<!-- Remove <style> block -->
```

**Test:** Device list should render with shadcn cards and buttons

---

### Step 2.4: Update DevicesView Layout

**Full DevicesView with Tailwind utilities:**

```svelte
<script>
  import { onMount } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "$lib/components/ui/card";
  import { Alert, AlertDescription } from "$lib/components/ui/alert";
  import DeviceList from '$lib/components/DeviceList.svelte';
  import TemplateSelector from '$lib/components/TemplateSelector.svelte';
  import ProfileManager from '$lib/components/ProfileManager.svelte';
  import { statusStore, devicesStore, appStore } from '$lib/stores.js';
  import api from '$lib/api.js';

  // ... keep existing logic ...

  onMount(async () => {
    await loadTemplates();
    statusStore.startAutoRefresh();
    devicesStore.startAutoRefresh();

    return () => {
      statusStore.stopAutoRefresh();
      devicesStore.stopAutoRefresh();
    };
  });
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <header class="px-10 pt-8 pb-6 border-b border-border bg-background">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-semibold text-foreground">Devices & Profiles</h2>
        <p class="text-sm text-muted-foreground mt-1">
          Manage MIDI device connections, templates, and per-app profiles
        </p>
      </div>
      <div class="flex gap-3 items-center">
        {#if loading}
          <div class="flex items-center gap-2 text-primary text-sm">
            <span class="inline-block w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin"></span>
            Loading...
          </div>
        {/if}
        <Button variant="outline" on:click={openTemplateSelector}>
          📋 Device Templates
        </Button>
        <Button variant="outline" on:click={openProfileManager}>
          🔄 Profiles
        </Button>
      </div>
    </div>
  </header>

  <!-- Content -->
  <div class="flex-1 px-10 py-8 overflow-y-auto space-y-8">
    <!-- Error Banner -->
    {#if error}
      <Alert variant="destructive">
        <span class="text-lg mr-2">⚠️</span>
        <AlertDescription class="flex items-center justify-between">
          <span>{error}</span>
          <Button
            variant="ghost"
            size="sm"
            on:click={() => { error = null; appStore.clearError(); }}
          >
            ✕
          </Button>
        </AlertDescription>
      </Alert>
    {/if}

    <!-- Daemon Status Section -->
    <section>
      <h3 class="text-xl font-semibold text-foreground mb-4">Daemon Status</h3>
      {#if $statusStore.status}
        <Card>
          <CardContent class="pt-6">
            <div class="space-y-3">
              <div class="flex items-center justify-between py-2 border-b border-border">
                <span class="text-sm font-medium text-muted-foreground">Running:</span>
                <Badge variant={$statusStore.status.running ? 'default' : 'destructive'}>
                  {$statusStore.status.running ? '✅ Yes' : '❌ No'}
                </Badge>
              </div>
              <div class="flex items-center justify-between py-2 border-b border-border">
                <span class="text-sm font-medium text-muted-foreground">Connected:</span>
                <Badge variant={$statusStore.status.connected ? 'default' : 'destructive'}>
                  {$statusStore.status.connected ? '✅ Yes' : '❌ No'}
                </Badge>
              </div>
              {#if $statusStore.status.lifecycle_state}
                <div class="flex items-center justify-between py-2 border-b border-border">
                  <span class="text-sm font-medium text-muted-foreground">State:</span>
                  <span class="text-sm text-foreground">{$statusStore.status.lifecycle_state}</span>
                </div>
              {/if}
              {#if $statusStore.status.uptime_secs !== null && $statusStore.status.uptime_secs !== undefined}
                <div class="flex items-center justify-between py-2 border-b border-border">
                  <span class="text-sm font-medium text-muted-foreground">Uptime:</span>
                  <span class="text-sm text-foreground">
                    {Math.floor($statusStore.status.uptime_secs / 60)}m {$statusStore.status.uptime_secs % 60}s
                  </span>
                </div>
              {/if}
              {#if $statusStore.status.events_processed !== null && $statusStore.status.events_processed !== undefined}
                <div class="flex items-center justify-between py-2">
                  <span class="text-sm font-medium text-muted-foreground">Events Processed:</span>
                  <span class="text-sm text-foreground">{$statusStore.status.events_processed.toLocaleString()}</span>
                </div>
              {/if}
            </div>
          </CardContent>
        </Card>
      {:else if $statusStore.error}
        <Alert variant="destructive">
          <AlertDescription>{$statusStore.error}</AlertDescription>
        </Alert>
      {:else}
        <Card>
          <CardContent class="py-6">
            <p class="text-sm text-muted-foreground italic">Loading status...</p>
          </CardContent>
        </Card>
      {/if}
    </section>

    <!-- MIDI Devices Section -->
    <section>
      <h3 class="text-xl font-semibold text-foreground mb-4">MIDI Devices</h3>
      <DeviceList />
    </section>
  </div>
</div>

<!-- Modals -->
{#if showTemplateSelector}
  <TemplateSelector
    {templates}
    on:selected={handleTemplateSelected}
    on:close={() => showTemplateSelector = false}
  />
{/if}

{#if showProfileManager}
  <ProfileManager
    on:profileSwitch={handleProfileSwitch}
    on:close={() => showProfileManager = false}
  />
{/if}

<!-- Remove <style> block -->
```

**Install Alert component:**
```bash
npx shadcn-svelte@latest add alert
```

**Checkpoint:** Commit
```bash
git add -A
git commit -m "feat(ui): Migrate DevicesView to shadcn components"
```

---

## Phase 3: Layout & Utilities

**Goal:** Convert remaining layouts to Tailwind, keep Sidebar custom.

**Estimated Time:** 3-4 hours

### Step 3.1: Update App.svelte Global Styles

**Edit:** `midimon-gui/ui/src/App.svelte`

Remove the `<style>` block and replace with Tailwind utilities:

```svelte
<script>
  import './app.css';
  import { onMount } from 'svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  import DevicesView from './lib/views/DevicesView.svelte';
  import ModesView from './lib/views/ModesView.svelte';
  import MappingsView from './lib/views/MappingsView.svelte';
  import SettingsView from './lib/views/SettingsView.svelte';
  import { currentSection, restoreNavigationState, SECTIONS } from './lib/stores/navigation.js';

  onMount(() => {
    restoreNavigationState();
  });
</script>

<div class="flex h-screen overflow-hidden bg-background text-foreground">
  <Sidebar />
  <div class="flex-1 flex flex-col overflow-hidden">
    <StatusBar />
    <main class="flex-1 flex flex-col bg-background overflow-hidden">
      {#if $currentSection === SECTIONS.DEVICES}
        <DevicesView />
      {:else if $currentSection === SECTIONS.MODES}
        <ModesView />
      {:else if $currentSection === SECTIONS.MAPPINGS}
        <MappingsView />
      {:else if $currentSection === SECTIONS.SETTINGS}
        <SettingsView />
      {/if}
    </main>
  </div>
</div>

<!-- Remove <style> block entirely -->
```

**Update app.css** to remove conflicting global styles:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 30 30 30;
    --foreground: 224 224 224;
    --card: 42 42 42;
    --card-foreground: 224 224 224;
    --primary: 74 158 255;
    --primary-foreground: 255 255 255;
    --border: 51 51 51;
    --input: 51 51 51;
    --ring: 74 158 255;
  }
}

@layer base {
  * {
    @apply border-border;
  }

  html,
  body {
    @apply h-full overflow-hidden;
  }

  body {
    @apply bg-background text-foreground font-sans;
  }
}
```

### Step 3.2: Keep Sidebar Custom (Optional Minor Updates)

The Sidebar already looks good with custom styling. Optionally add Tailwind utilities for consistency:

**Minor updates to Sidebar.svelte:**

```svelte
<!-- Keep mostly the same, just ensure colors use Tailwind variables -->
<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 240px;
    height: 100vh;
    background: theme('colors.background.dark'); /* Use Tailwind theme */
    border-right: 1px solid theme('colors.border');
    overflow-y: auto;
  }

  /* ... keep rest of styles ... */
</style>
```

---

## Phase 4: Complex Components

**Goal:** Migrate ModesView, MappingsView, SettingsView, and modal components.

**Estimated Time:** 8-10 hours

### Step 4.1: Install Remaining Components

```bash
npx shadcn-svelte@latest add dialog
npx shadcn-svelte@latest add table
npx shadcn-svelte@latest add tabs
npx shadcn-svelte@latest add select
npx shadcn-svelte@latest add input
npx shadcn-svelte@latest add textarea
npx shadcn-svelte@latest add switch
npx shadcn-svelte@latest add label
npx shadcn-svelte@latest add form
npx shadcn-svelte@latest add progress
npx shadcn-svelte@latest add tooltip
npx shadcn-svelte@latest add skeleton
```

### Step 4.2: Migrate TemplateSelector Modal

**Before:** Custom modal with CSS

**After:** shadcn Dialog component

```svelte
<script>
  import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  import { createEventDispatcher } from 'svelte';

  export let templates = [];

  const dispatch = createEventDispatcher();
  let open = true;

  function handleSelect(template) {
    dispatch('selected', { template });
  }

  function handleClose() {
    open = false;
    dispatch('close');
  }
</script>

<Dialog bind:open onOpenChange={(isOpen) => { if (!isOpen) handleClose(); }}>
  <DialogContent class="max-w-4xl max-h-[80vh] overflow-y-auto">
    <DialogHeader>
      <DialogTitle>Device Templates</DialogTitle>
      <DialogDescription>
        Choose a pre-configured template for your MIDI device
      </DialogDescription>
    </DialogHeader>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
      {#each templates as template}
        <Card class="cursor-pointer hover:border-primary transition-colors" on:click={() => handleSelect(template)}>
          <CardHeader>
            <div class="flex items-start justify-between">
              <CardTitle class="text-base">{template.name}</CardTitle>
              {#if template.featured}
                <Badge variant="default">Featured</Badge>
              {/if}
            </div>
          </CardHeader>
          <CardContent>
            <p class="text-sm text-muted-foreground mb-3">{template.description}</p>
            <div class="flex flex-wrap gap-1">
              {#each template.tags || [] as tag}
                <Badge variant="outline" class="text-xs">{tag}</Badge>
              {/each}
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>

    <div class="flex justify-end gap-3 mt-6">
      <Button variant="outline" on:click={handleClose}>Cancel</Button>
    </div>
  </DialogContent>
</Dialog>
```

### Step 4.3: Migrate ModesView

**Pattern:** Similar to DevicesView - use Cards, Buttons, Badges

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  // ... imports
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <header class="px-10 pt-8 pb-6 border-b border-border bg-background">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-semibold">Modes</h2>
        <p class="text-sm text-muted-foreground mt-1">
          Manage different control modes for your MIDI device
        </p>
      </div>
      <Button on:click={handleCreateMode}>
        + New Mode
      </Button>
    </div>
  </header>

  <div class="flex-1 px-10 py-8 overflow-y-auto">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each modes as mode}
        <Card class="hover:border-primary transition-colors cursor-pointer">
          <CardHeader>
            <div class="flex items-start justify-between">
              <CardTitle class="text-lg">{mode.name}</CardTitle>
              <div
                class="w-4 h-4 rounded-full"
                style="background-color: {mode.color}"
              ></div>
            </div>
          </CardHeader>
          <CardContent>
            <p class="text-sm text-muted-foreground mb-3">
              {mode.mappings?.length || 0} mappings
            </p>
            <div class="flex gap-2">
              <Button variant="outline" size="sm" on:click={() => handleEdit(mode)}>
                Edit
              </Button>
              <Button variant="ghost" size="sm" on:click={() => handleDelete(mode)}>
                Delete
              </Button>
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>
  </div>
</div>
```

### Step 4.4: Migrate MappingsView with Table

```bash
npx shadcn-svelte@latest add table
```

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
  import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "$lib/components/ui/table";
  import { Badge } from "$lib/components/ui/badge";
  // ... imports
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <header class="px-10 pt-8 pb-6 border-b border-border bg-background">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-semibold">Mappings</h2>
        <p class="text-sm text-muted-foreground mt-1">
          Configure MIDI event to action mappings
        </p>
      </div>
      <Button on:click={handleCreateMapping}>
        + New Mapping
      </Button>
    </div>
  </header>

  <div class="flex-1 px-10 py-8 overflow-y-auto">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Trigger</TableHead>
          <TableHead>Action</TableHead>
          <TableHead>Mode</TableHead>
          <TableHead>Description</TableHead>
          <TableHead class="text-right">Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each mappings as mapping}
          <TableRow>
            <TableCell>
              <Badge variant="outline">{mapping.trigger.type}</Badge>
              <span class="ml-2 text-sm">{mapping.trigger.note || mapping.trigger.cc || '-'}</span>
            </TableCell>
            <TableCell>
              <Badge>{mapping.action.type}</Badge>
            </TableCell>
            <TableCell>
              <span class="text-sm text-muted-foreground">{mapping.mode || 'Global'}</span>
            </TableCell>
            <TableCell>
              <span class="text-sm">{mapping.description}</span>
            </TableCell>
            <TableCell class="text-right">
              <div class="flex justify-end gap-2">
                <Button variant="ghost" size="sm" on:click={() => handleEdit(mapping)}>
                  Edit
                </Button>
                <Button variant="ghost" size="sm" on:click={() => handleDelete(mapping)}>
                  Delete
                </Button>
              </div>
            </TableCell>
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>
</div>
```

### Step 4.5: Migrate SettingsView with Form Components

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Switch } from "$lib/components/ui/switch";
  import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "$lib/components/ui/select";
  // ... imports
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <header class="px-10 pt-8 pb-6 border-b border-border bg-background">
    <h2 class="text-2xl font-semibold">Settings</h2>
    <p class="text-sm text-muted-foreground mt-1">
      Configure global application settings
    </p>
  </header>

  <div class="flex-1 px-10 py-8 overflow-y-auto space-y-6">
    <!-- Device Settings -->
    <Card>
      <CardHeader>
        <CardTitle>Device Settings</CardTitle>
        <CardDescription>Configure MIDI device connection parameters</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="device-name">Device Name</Label>
          <Input id="device-name" placeholder="Mikro MK3" bind:value={settings.deviceName} />
        </div>

        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label>Auto-connect on startup</Label>
            <p class="text-sm text-muted-foreground">Automatically connect to last used device</p>
          </div>
          <Switch bind:checked={settings.autoConnect} />
        </div>
      </CardContent>
    </Card>

    <!-- Advanced Settings -->
    <Card>
      <CardHeader>
        <CardTitle>Advanced Settings</CardTitle>
        <CardDescription>Fine-tune event detection timings</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="chord-timeout">Chord Timeout (ms)</Label>
          <Input
            id="chord-timeout"
            type="number"
            bind:value={settings.chordTimeoutMs}
          />
        </div>

        <div class="space-y-2">
          <Label for="double-tap-timeout">Double-tap Timeout (ms)</Label>
          <Input
            id="double-tap-timeout"
            type="number"
            bind:value={settings.doubleTapTimeoutMs}
          />
        </div>

        <div class="space-y-2">
          <Label for="hold-threshold">Hold Threshold (ms)</Label>
          <Input
            id="hold-threshold"
            type="number"
            bind:value={settings.holdThresholdMs}
          />
        </div>
      </CardContent>
    </Card>

    <!-- Actions -->
    <div class="flex justify-end gap-3">
      <Button variant="outline" on:click={handleReset}>Reset to Defaults</Button>
      <Button on:click={handleSave}>Save Settings</Button>
    </div>
  </div>
</div>
```

**Checkpoint:** Commit
```bash
git add -A
git commit -m "feat(ui): Migrate all views to shadcn components"
```

---

## Phase 5: Polish & Cleanup

**Goal:** Add finishing touches, remove old CSS, improve UX.

**Estimated Time:** 3-4 hours

### Step 5.1: Add Loading States

Install Skeleton component:
```bash
npx shadcn-svelte@latest add skeleton
```

**Add to DeviceList.svelte:**

```svelte
{#if $devicesStore.loading && $devicesStore.devices.length === 0}
  <div class="space-y-3">
    {#each [1, 2, 3] as _}
      <div class="flex items-center justify-between p-4 rounded-lg border border-border">
        <div class="flex-1 space-y-2">
          <Skeleton class="h-5 w-48" />
          <Skeleton class="h-4 w-24" />
        </div>
        <Skeleton class="h-9 w-20" />
      </div>
    {/each}
  </div>
{:else if $devicesStore.error}
  <!-- ... existing error state ... -->
{/if}
```

### Step 5.2: Add Tooltips

Install Tooltip:
```bash
npx shadcn-svelte@latest add tooltip
```

**Add to StatusBar:**

```svelte
<script>
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
</script>

<Tooltip>
  <TooltipTrigger>
    <Badge variant={getStatusVariant($statusStore.status)}>
      {getStatusText($statusStore.status)}
    </Badge>
  </TooltipTrigger>
  <TooltipContent>
    <p>Daemon is {$statusStore.status?.running ? 'running' : 'stopped'}</p>
    {#if $statusStore.status?.uptime_secs}
      <p class="text-xs text-muted-foreground">
        Uptime: {formatUptime($statusStore.status.uptime_secs)}
      </p>
    {/if}
  </TooltipContent>
</Tooltip>
```

### Step 5.3: Remove Old CSS Files

**Delete or clean up custom CSS:**

```bash
# Remove if components are fully migrated
rm midimon-gui/ui/src/lib/components/*.css  # If any exist
```

**Verify no `<style>` blocks remain** in migrated components.

### Step 5.4: Add Animations

Install additional utilities if needed:

```bash
npm install -D @tailwindcss/forms @tailwindcss/typography
```

**Update tailwind.config.js:**

```js
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      // ... existing colors ...
      animation: {
        'fade-in': 'fadeIn 0.2s ease-in',
        'slide-up': 'slideUp 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}
```

**Add transitions to cards:**

```svelte
<Card class="transition-all duration-200 hover:shadow-lg hover:scale-[1.02]">
  <!-- ... -->
</Card>
```

### Step 5.5: Accessibility Improvements

**Add aria-labels:**

```svelte
<Button
  variant="ghost"
  size="sm"
  on:click={handleRefresh}
  aria-label="Refresh device list"
>
  🔄
</Button>
```

**Add focus states:**

```svelte
<Card class="focus-within:ring-2 focus-within:ring-primary focus-within:ring-offset-2">
  <!-- ... -->
</Card>
```

### Step 5.6: Final Testing Checklist

- [ ] All views render correctly
- [ ] No console errors
- [ ] Buttons are clickable and styled
- [ ] Forms submit properly
- [ ] Modals open/close correctly
- [ ] Tables are sortable/filterable
- [ ] Loading states show skeletons
- [ ] Error states display alerts
- [ ] Dark theme looks consistent
- [ ] Tooltips appear on hover
- [ ] Keyboard navigation works
- [ ] No custom CSS conflicts

**Final Checkpoint:**
```bash
git add -A
git commit -m "feat(ui): Complete shadcn-ui migration with polish and accessibility"
```

---

## Troubleshooting

### Issue: Components not found

**Error:** `Cannot find module '$lib/components/ui/button'`

**Solution:**
```bash
# Reinstall component
npx shadcn-svelte@latest add button

# Check alias in vite.config.js
resolve: {
  alias: {
    $lib: path.resolve('./src/lib'),
  },
}
```

---

### Issue: Tailwind classes not applying

**Error:** Styles don't appear

**Solution:**
1. Check `tailwind.config.js` content paths include `.svelte` files
2. Ensure `app.css` is imported in `App.svelte`
3. Restart dev server: `Ctrl+C` then `npm run dev`
4. Clear browser cache

---

### Issue: Dark theme colors wrong

**Error:** Components use light theme

**Solution:**
Update `app.css` CSS variables to match dark theme:

```css
:root {
  --background: 30 30 30; /* HSL values, not hex! */
  --foreground: 224 224 224;
}
```

---

### Issue: Buttons look broken

**Error:** Button text/padding incorrect

**Solution:**
Check button variant usage:

```svelte
<!-- Correct -->
<Button variant="outline">Text</Button>

<!-- Incorrect -->
<Button class="btn-outline">Text</Button>
```

---

## Rollback Plan

If migration fails or introduces too many bugs:

### Option 1: Revert to Last Commit

```bash
# See commits
git log --oneline

# Revert to before migration
git reset --hard <commit-hash-before-migration>
```

### Option 2: Keep Tailwind, Remove shadcn

```bash
# Uninstall shadcn components
rm -rf src/lib/components/ui
rm src/lib/utils.js

# Keep Tailwind for utility classes
# Restore custom CSS components
```

### Option 3: Gradual Rollback

Revert one component at a time:

```bash
git checkout HEAD~1 -- src/lib/components/DeviceList.svelte
```

---

## Post-Migration Notes

**What Changed:**
- ✅ All buttons use shadcn `<Button>` component
- ✅ All cards use shadcn `<Card>` component
- ✅ Forms use shadcn form components
- ✅ Modals use shadcn `<Dialog>` component
- ✅ Tables use shadcn `<Table>` component
- ✅ Custom CSS removed (except Sidebar)
- ✅ Tailwind utilities for all layouts
- ✅ Dark theme preserved with CSS variables
- ✅ Accessibility improved with Radix primitives

**What Stayed the Same:**
- ✅ All Rust backend code unchanged
- ✅ All stores and state management unchanged
- ✅ All API calls unchanged
- ✅ All business logic unchanged
- ✅ Sidebar design (custom CSS preserved)

**Performance:**
- Bundle size may increase by ~50-100KB (shadcn components + Tailwind)
- Runtime performance should be identical or slightly better
- Build time may increase by ~5-10 seconds

---

## Next Steps

After migration is complete:

1. **Test thoroughly** on all views
2. **Add more polish:** animations, transitions, micro-interactions
3. **Consider TypeScript migration** for type safety
4. **Add more shadcn components** as needed (Popover, Command, DropdownMenu)
5. **Document component usage** for team members
6. **Create component showcase** page for testing

---

## Resources

- **shadcn-ui Docs:** https://www.shadcn-ui.com/docs
- **shadcn-svelte Docs:** https://www.shadcn-svelte.com
- **Tailwind CSS Docs:** https://tailwindcss.com/docs
- **Radix UI Docs:** https://www.radix-ui.com/primitives/docs
- **Svelte Docs:** https://svelte.dev/docs

---

## Conclusion

This migration guide provides a **low-risk, incremental approach** to modernizing MIDIMon's UI with shadcn-ui components and Tailwind CSS.

**Key Benefits:**
- ✅ Professional, consistent design
- ✅ Better accessibility out of the box
- ✅ Faster future development (less custom CSS)
- ✅ Responsive design built-in
- ✅ Active community and regular updates

**Estimated Total Time:** 24-30 hours over 3-4 days

Good luck with the migration! 🚀
