# Echos in the Dark - Next Steps Implementation Plan

## Current State Analysis

### ✅ What's Already Implemented
- **Basic Bevy Setup**: App structure with states (Loading, Gameplay)
- **Tilemap System**: Using `bevy_ecs_tilemap` for world representation
- **Map Generation**: Comprehensive procedural generation for mines, caves, and dark adventure areas
- **Tile System**: Complete tile types including ores, special features, and underground types
- **Basic UI Framework**: Camera, interaction system, widget utilities
- **Asset Loading**: Texture and audio asset management
- **Settings System**: Configurable app settings with persistence

### ❌ Critical Missing Components
1. **Screens Module Integration**: References to `ScreenState` and `theme::prelude` but missing actual modules
2. **Player Entity**: No player character implementation
3. **Core Game Loop**: Missing turn-based mechanics
4. **Input Handling**: No player movement or interaction systems
5. **Echo/Sound System**: Core mechanic not implemented
6. **Entity Systems**: No enemies, items, or interactive objects

---

## Phase 1: Foundation & MVP (Priority: Critical)

### 1.1 Fix Module Structure
**Goal**: Get the game running without compilation errors

- [ ] **Create missing modules**:
  - `src/screens/mod.rs` - Screen state management
  - `src/theme/mod.rs` - UI theme system (or alias to ui::utils)
  - `src/gameplay/player/mod.rs` - Player entity and systems
  - `src/gameplay/entities/mod.rs` - Game entities (enemies, items)

- [ ] **Integrate screens into main.rs**:
  ```rust
  pub mod screens;
  pub mod gameplay;

  // Add to plugins
  app.add_plugins(screens::ScreensPlugin);
  ```

- [ ] **Define ScreenState enum**:
  ```rust
  #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
  pub enum ScreenState {
      #[default]
      Loading,
      Gameplay,
  }
  ```

### 1.2 Implement Player Entity
**Goal**: Basic player that can exist on the map

- [ ] **Player Component**:
  ```rust
  #[derive(Component)]
  pub struct Player {
      pub health: i32,
      pub position: IVec2,
      pub light_radius: f32,
  }
  ```

- [ ] **Position Component**:
  ```rust
  #[derive(Component)]
  pub struct Position {
      pub x: i32,
      pub y: i32,
  }
  ```

- [ ] **Player spawn system**: Place player on map when gameplay starts

### 1.3 Basic Input & Movement
**Goal**: Player can move around the map

- [ ] **Input handling system**:
  - Arrow keys/WASD for movement
  - Turn-based movement (one tile per keypress)
  - Collision detection with walls

- [ ] **Movement validation**:
  - Check if target tile is walkable
  - Update player position
  - Update camera to follow player

---

## Phase 2: Core Gameplay Loop (Priority: High)

### 2.1 Turn-Based System
**Goal**: Implement the core roguelike turn structure

- [ ] **Turn Manager Resource**:
  ```rust
  #[derive(Resource)]
  pub struct TurnManager {
      pub current_turn: u32,
      pub player_turn: bool,
  }
  ```

- [ ] **Turn processing**:
  - Player input → Player action → Enemy turns → Update world
  - Action validation and execution
  - Turn counter and state management

### 2.2 Basic Visibility System
**Goal**: Implement fog of war and basic light

- [ ] **Visibility Component**:
  ```rust
  #[derive(Component)]
  pub struct Visibility {
      pub visible_tiles: HashSet<IVec2>,
      pub explored_tiles: HashSet<IVec2>,
  }
  ```

- [ ] **Light source system**:
  - Basic circular light radius around player
  - Tile visibility calculation
  - Fog of war rendering

### 2.3 Basic Mining System
**Goal**: Player can mine ores from walls

- [ ] **Mining action**:
  - Check if adjacent tile is mineable
  - Remove ore tile, add to inventory
  - Replace with floor tile

- [ ] **Basic Inventory**:
  ```rust
  #[derive(Component)]
  pub struct Inventory {
      pub items: HashMap<ItemType, u32>,
      pub capacity: u32,
  }
  ```

---

## Phase 3: Echo Mechanics (Priority: High)

### 3.1 Sound System Foundation
**Goal**: Implement the core "echo" mechanic

- [ ] **Sound Event System**:
  ```rust
  #[derive(Event)]
  pub struct SoundEvent {
      pub position: IVec2,
      pub intensity: f32,
      pub sound_type: SoundType,
  }

  pub enum SoundType {
      Movement,
      Mining,
      EcholocationPing,
      Combat,
  }
  ```

- [ ] **Sound propagation**:
  - Calculate sound radius based on intensity
  - Mark affected tiles temporarily
  - Visual representation of sound waves

### 3.2 Echolocation System
**Goal**: Player can "ping" to reveal map areas

- [ ] **Echolocation ability**:
  - Spacebar or E key to activate
  - Raycast from player position
  - Temporarily reveal tiles in radius
  - Generate sound event that can attract enemies

- [ ] **Revealed tile tracking**:
  - Different visual states: unseen, revealed by echo, currently lit
  - Fade effect for echo-revealed tiles

---

## Phase 4: Enemies & AI (Priority: Medium)

### 4.1 Basic Enemy System
**Goal**: Implement simple enemies that respond to sound

- [ ] **Enemy Components**:
  ```rust
  #[derive(Component)]
  pub struct Enemy {
      pub enemy_type: EnemyType,
      pub health: i32,
      pub ai_state: AIState,
  }

  pub enum EnemyType {
      Stalker,    // Sound-sensitive
      CaveBat,    // Echolocates player
      RockMimic,  // Camouflaged
  }

  pub enum AIState {
      Idle,
      Investigating(IVec2),
      Chasing(Entity),
      Attacking,
  }
  ```

- [ ] **Basic AI behaviors**:
  - Idle: Random movement or stationary
  - Sound response: Move toward sound events
  - Simple pathfinding (A* or basic line-of-sight)

### 4.2 Combat System
**Goal**: Basic turn-based combat

- [ ] **Combat mechanics**:
  - Melee attacks when adjacent
  - Health and damage calculation
  - Death and respawn handling

---

## Phase 5: Items & Crafting (Priority: Medium)

### 5.1 Item System
**Goal**: Implement items and basic crafting

- [ ] **Item definitions**:
  ```rust
  pub enum ItemType {
      // Raw materials
      IronOre,
      CopperOre,
      SonoriteOre,
      GlimmerstoneOre,
      WhisperingIronOre,

      // Tools
      BasicPickaxe,
      SilentPickaxe,

      // Light sources
      Torch,
      Lantern,
      GlimmerLamp,
  }
  ```

- [ ] **Crafting system**:
  - Recipe definitions
  - Crafting interface
  - Tool durability and effectiveness

### 5.2 Equipment System
**Goal**: Player can equip tools and gear

- [ ] **Equipment slots**:
  - Tool (pickaxe, weapon)
  - Light source
  - Armor (future)

- [ ] **Equipment effects**:
  - Mining speed modifiers
  - Light radius changes
  - Sound generation modifiers

---

## Phase 6: Advanced Features (Priority: Low)

### 6.1 Environmental Hazards
**Goal**: Implement mine personality features

- [ ] **Cave-ins**: Triggered by loud sounds
- [ ] **Gas pockets**: Released when mining
- [ ] **Unstable walls**: Collapse over time

### 6.2 Progression System
**Goal**: Player skills and upgrades

- [ ] **Skill system**:
  - Mining Speed
  - Silent Movement
  - Echo Focus
  - Geology

- [ ] **Experience and leveling**

### 6.3 Advanced AI
**Goal**: More sophisticated enemy behaviors

- [ ] **Complex AI states**
- [ ] **Enemy coordination**
- [ ] **Special abilities** (bat echolocation, etc.)

---

## Technical Debt & Improvements

### Code Organization
- [ ] **Separate concerns**: Move map generation to dedicated module
- [ ] **Component organization**: Group related components
- [ ] **System organization**: Logical system sets and ordering

### Performance
- [ ] **Efficient visibility calculations**
- [ ] **Spatial indexing** for entities
- [ ] **Sound propagation optimization**

### User Experience
- [ ] **UI improvements**: Health display, inventory UI
- [ ] **Audio integration**: Sound effects and ambient audio
- [ ] **Visual polish**: Animations, particle effects

---

## Immediate Next Actions (This Week)

1. **Fix compilation issues** by creating missing modules
2. **Implement basic player entity** and spawning
3. **Add simple movement system** with collision detection
4. **Create basic turn manager** for game loop
5. **Implement simple visibility system** with fog of war

This plan follows the MVP approach suggested in the game design document, starting with the most basic playable version and incrementally adding the unique "echo" mechanics that make this game special.
