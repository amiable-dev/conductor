# Native Instruments Device Profiles

## Overview

The Maschine Mikro MK3 uses `.ncmm3` XML files to define MIDI mappings for its pads. These profiles are created with the NI Controller Editor application and define how MIDI notes map to the 16 physical pads across 8 different "pages."

## File Format: .ncmm3

### Structure

`.ncmm3` files are XML documents with the following hierarchy:

```xml
<DeviceProfile>
  <DeviceProperties>
    <Name>Profile Name</Name>
    <Type>MASCHINE_MIKRO_MK3</Type>
    <SerialNumber>...</SerialNumber>
  </DeviceProperties>
  
  <Mapping>
    <PageList>
      <Page name="Pad Page A">
        <ControlList>
          <MidiControl id="12">
            <MidiNote channel="0" note="12"/>
          </MidiControl>
          <!-- 15 more controls for pads 13-27 -->
        </ControlList>
      </Page>
      <!-- 7 more pages: B, C, D, E, F, G, H -->
    </PageList>
  </Mapping>
</DeviceProfile>
```

### Key Elements

- **Page**: Each of 8 pages (A-H) defines a complete 16-pad mapping
- **MidiControl**: Maps a control ID to MIDI parameters
  - `id`: Control identifier (typically 12-27 for pads)
  - `MidiNote channel`: MIDI channel (0-15)
  - `MidiNote note`: MIDI note number (0-127)

## Pad Page System

### Page Layout

The MK3 has 8 pad pages accessible via hardware buttons:

```
Pad Pages: A | B | C | D | E | F | G | H
Each page: 16 pads (4x4 grid)
Total:     128 unique MIDI mappings possible
```

### Common Page Configurations

**Example 1: Chromatic Layout (Page A)**
- Pad 0 (bottom-left): Note 12 (C)
- Pad 1: Note 13 (C#)
- Pad 2: Note 14 (D)
- ...sequential chromatic notes...
- Pad 15 (top-right): Note 27 (Eb)

**Example 2: Drum Kit (Page B)**
- Pad 0: Note 36 (Kick)
- Pad 1: Note 38 (Snare)
- Pad 2: Note 42 (Closed Hi-hat)
- ...custom drum mapping...

**Example 3: Scale Mode (Page C)**
- Only notes in a specific scale
- E.g., C Major: C, D, E, F, G, A, B, C...

## Implementation Details

### Profile Parsing

The `DeviceProfile` struct in `src/device_profile.rs` handles parsing:

```rust
pub struct DeviceProfile {
    name: String,
    device_type: String,
    pages: Vec<PadPage>,
}

pub struct PadPage {
    name: String,
    mappings: HashMap<u8, u8>,  // note -> pad_index
}
```

### Parsing Process

1. **Load XML**: Read `.ncmm3` file into memory
2. **Extract Pages**: Parse each `<Page>` element
3. **Build Mappings**: Create note → pad_index lookup tables
4. **Validation**: Ensure all 16 pads are mapped per page

```rust
// Parse a page
for control in page.find("ControlList").find_all("MidiControl") {
    let id = control.attr("id").parse()?;
    let note = control.find("MidiNote").attr("note").parse()?;
    
    // Map note to pad index (0-15)
    let pad_index = id - 12;  // Assuming IDs 12-27
    mappings.insert(note, pad_index);
}
```

### Auto-Detection

The system automatically detects which page is active by matching incoming MIDI notes:

```rust
pub fn detect_page_for_note(&self, note: u8) -> Option<usize> {
    self.pages.iter().position(|page| {
        page.mappings.contains_key(&note)
    })
}
```

**Behavior**:
- On first MIDI note received, detect which page contains that note
- Switch to that page's mappings
- All subsequent notes use that page until a note from another page arrives

### Note-to-Pad Conversion

Critical function for LED feedback:

```rust
pub fn note_to_pad_index(&self, note: u8, page: usize) -> Option<u8> {
    self.pages.get(page)
        .and_then(|page| page.mappings.get(&note).copied())
}
```

**Usage Flow**:
```
MIDI Note 14 arrives
    ↓
detect_page_for_note(14) → Page 0 (if note 14 is in Page A)
    ↓
note_to_pad_index(14, 0) → Pad index 2
    ↓
map_pad_to_led_position(2) → LED position 14
    ↓
Light LED 14
```

## Creating Custom Profiles

### Using NI Controller Editor

1. **Open Controller Editor**: Installed with Native Access
2. **Select Device**: Choose "Maschine Mikro MK3"
3. **Edit Pages**: Click each page (A-H) to edit
4. **Assign Notes**: Click pads and set MIDI note numbers
5. **Save Profile**: Export as `.ncmm3` file

### Profile Template

A minimal valid profile structure:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<DeviceProfile>
  <DeviceProperties>
    <Name>Custom Profile</Name>
    <Type>MASCHINE_MIKRO_MK3</Type>
  </DeviceProperties>
  
  <Mapping>
    <PageList>
      <Page name="Pad Page A">
        <ControlList>
          <MidiControl id="12">
            <MidiNote channel="0" note="60"/>
          </MidiControl>
          <!-- Repeat for IDs 13-27 -->
        </ControlList>
      </Page>
    </PageList>
  </Mapping>
</DeviceProfile>
```

## Best Practices

### Profile Design

1. **Consistent Layouts**: Keep similar pages across profiles for muscle memory
2. **Note Ranges**: Use MIDI notes 0-127, avoid conflicts with other devices
3. **Channel Assignment**: Use different MIDI channels for different page types
4. **Documentation**: Name pages descriptively (e.g., "Drums", "Chords", "Bass")

### Implementation Guidelines

1. **Always Use Profile Mapping**: Never assume note 12 = pad 0
   ```rust
   // ❌ WRONG
   let pad_index = midi_note - 12;
   
   // ✅ CORRECT
   let pad_index = profile.note_to_pad_index(midi_note, current_page)?;
   ```

2. **Handle Missing Mappings**: Not all notes may be mapped
   ```rust
   match profile.note_to_pad_index(note, page) {
       Some(pad) => light_led(pad),
       None => eprintln!("Note {} not mapped in page {}", note, page),
   }
   ```

3. **Cache Current Page**: Store active page to avoid repeated detection
   ```rust
   if let Some(detected_page) = profile.detect_page_for_note(note) {
       if detected_page != current_page {
           println!("Switched to page {}", detected_page);
           current_page = detected_page;
       }
   }
   ```

## Profile Locations

### Default Profile Path

**macOS**:
```
~/Documents/Native Instruments/Controller Editor/Profiles/
```

**Windows**:
```
%USERPROFILE%\Documents\Native Instruments\Controller Editor\Profiles\
```

### Built-in Profiles

The MK3 hardware has built-in profiles that don't require files:
- Factory default (chromatic layout starting at C)
- Drum mode
- Hardware control mode (for Maschine software)

### Loading Profiles

```bash
# Specify profile via CLI
./midimon --profile "path/to/profile.ncmm3"

# Use relative path
./midimon --profile "./profiles/my-custom.ncmm3"

# Full path
./midimon --profile "$HOME/Documents/NI/my-profile.ncmm3"
```

## Troubleshooting

### Profile Not Loading

**Symptoms**: Error message "Failed to load profile"

**Checks**:
- File exists and is readable
- XML is well-formed (validate with xmllint)
- Profile contains at least one page
- MidiControl IDs are in expected range (12-27)

### Wrong Pads Lighting

**Symptoms**: LEDs light on incorrect pads

**Checks**:
- Profile mappings are correct (check with Controller Editor)
- Using `note_to_pad_index()` not direct note arithmetic
- LED position mapping is applied (`map_pad_to_led_position()`)
- Correct page is active (check page detection)

### Page Detection Issues

**Symptoms**: Page doesn't switch when expected

**Checks**:
- Note is actually in target page's mapping
- No overlap between pages (same note in multiple pages)
- Page detection logic is called on every note

## Advanced Topics

### Multi-Channel Profiles

Assign different pages to different MIDI channels:

```xml
<Page name="Drums (Ch 1)">
  <MidiControl id="12">
    <MidiNote channel="0" note="36"/>  <!-- Channel 1 in 0-indexed -->
  </MidiControl>
</Page>

<Page name="Bass (Ch 2)">
  <MidiControl id="12">
    <MidiNote channel="1" note="36"/>  <!-- Channel 2, same note -->
  </MidiControl>
</Page>
```

Requires channel-aware page detection:

```rust
pub fn detect_page_for_note_and_channel(&self, note: u8, channel: u8) 
    -> Option<usize>
{
    self.pages.iter().position(|page| {
        page.mappings.get(&note)
            .map(|mapping| mapping.channel == channel)
            .unwrap_or(false)
    })
}
```

### Dynamic Profile Switching

Switch profiles at runtime:

```rust
let mut profile = DeviceProfile::from_file("profile1.ncmm3")?;

// Later, reload different profile
profile = DeviceProfile::from_file("profile2.ncmm3")?;
current_page = 0;  // Reset to first page
```

### Profile Validation

Ensure profile completeness:

```rust
pub fn validate(&self) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    for (i, page) in self.pages.iter().enumerate() {
        if page.mappings.len() != 16 {
            errors.push(format!("Page {} has {} pads (expected 16)", 
                               i, page.mappings.len()));
        }
        
        let pad_indices: Vec<_> = page.mappings.values().collect();
        for expected in 0..16 {
            if !pad_indices.contains(&&expected) {
                errors.push(format!("Page {} missing pad {}", i, expected));
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

## Reference Implementation

See `src/device_profile.rs` for the complete implementation, including:
- XML parsing with `roxmltree`
- Note-to-pad mapping with `HashMap`
- Page detection and auto-switching
- Error handling and validation

---

**Last Updated**: November 10, 2025
**Format Version**: `.ncmm3` (Controller Editor 2.x compatible)
