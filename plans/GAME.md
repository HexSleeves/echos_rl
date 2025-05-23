# Echos in the Dark - Game Design Ideas

This document outlines core concepts, mechanics, and programming considerations for the roguelike game "Echos in the Dark."

## I. Core Roguelike Concepts

### 1. The "Echo" Mechanic (Sound & Visibility)

*   **Concept:**
    *   Light is scarce and unreliable. The primary way of "seeing" is through sound.
    *   **Echolocation Ping:** An active ability. Sends out a "ping" revealing nearby layout (walls, ore, enemies, items) as outlines or temporary tiles. This *creates sound*, potentially attracting enemies.
    *   **Passive Sounds:** Moving, mining, combat, and enemies generate sound, visualized as ripples or temporary "blips" even in darkness.
    *   **Light Sources:** Torches, lanterns, bioluminescent fungi provide limited, constant light. They might run out or be extinguished. Some enemies might be drawn to/repelled by light.
*   **Fun Factor:**
    *   Creates tension: Risk a loud ping for visibility or creep slowly?
    *   Sound management becomes as crucial as health management.
*   **Programming Ideas:**
    *   **Grid-based map:** Store tile types (wall, floor, ore, enemy, item).
    *   **Fog of War:** Tiles initially hidden; revealed by light or echolocation.
    *   **Echolocation:** Raycasting from the player. Mark hit tiles as temporarily visible.
    *   **Sound Propagation:** Actions have a "sound value." Enemies within a radius of a sound event might investigate.

### 2. Ore & Crafting

*   **Concept:**
    *   Different tiers and types of ore:
        *   **Basic Ore (Iron, Copper):** For early tools (pickaxes, basic weapons/armor).
        *   **Resonant Ore (e.g., "Sonorite," "Echo-Crystal"):** Crafts gear enhancing sound mechanics (quieter movement, better pings, sound-based attacks/defenses).
        *   **Luminous Ore (e.g., "Glimmerstone"):** Crafts better/longer-lasting light sources or gear emitting a faint glow.
        *   **Exotic/Cursed Ore:** Found deeper; powerful but with drawbacks (e.g., "Whispering Iron" - great weapons, but constantly noisy).
*   **Fun Factor:**
    *   Meaningful choices in resource gathering.
    *   Crafting provides clear progression.
*   **Programming Ideas:**
    *   **Item System:** Objects/classes for items with properties (type, durability, stats, effects).
    *   **Crafting Recipes:** Data structure (dictionary/list of objects) mapping input items/quantities to output items.
    *   **Inventory System:** Manage player's items.
    *   **Procedural Ore Generation:** Sprinkle ore veins based on depth and rarity rules.

### 3. The Mine's "Personality" - Environmental Storytelling & Hazards

*   **Concept:**
    *   The mine reacts, changes, and has history.
    *   **Cave-ins:** Triggered by loud noises or randomly. Can block/reveal paths, damage entities.
    *   **Gas Pockets:** Mining can release flammable or poisonous gas.
    *   **Ancient Machinery/Relics:** Hint at past civilizations; some might be reactivatable.
    *   **Echoes of the Past:** Occasional ghostly apparitions reenacting moments or offering clues.
*   **Fun Factor:**
    *   Makes the environment a character. Adds unpredictability and lore.
*   **Programming Ideas:**
    *   **Event System:** Trigger events based on player actions or randomness.
    *   **Tile Properties:** Flags like "unstable," "gas_pocket."
    *   **Pathfinding:** Must adapt to changing map layouts (e.g., due to cave-ins).

### 4. Enemies of the Dark

*   **Concept:**
    *   Creatures adapted to darkness and sound.
    *   **Cave Critters:** Giant bats (echolocate you!), spiders (webs slow you), rock mimics.
    *   **Sound-Sensitive Hunters:** "Stalkers" - blind but with acute hearing, move towards sound.
    *   **Light-Averse Shades:** Weak in light, dangerous in darkness.
    *   **Ore Golems:** Animated by ores, camouflaged until disturbed.
    *   **The "Echo Eater":** Rare, dangerous enemy that dampens sound or feeds off pings.
*   **Fun Factor:**
    *   Diverse enemy behaviors require different tactics.
*   **Programming Ideas:**
    *   **AI Behaviors:**
        *   Pathfinding (A* is common).
        *   State machines (idle, wander, chase, attack, flee).
        *   Sensory systems (detect player by sight radius, sound radius).

### 5. Player Progression & Skills

*   **Concept:**
    *   Player hones mining and survival skills beyond gear.
    *   **Skills:**
        *   Mining Speed
        *   Silent Movement
        *   Echo Focus (range/duration/detail)
        *   Geology (chance for rare ores, identify ore types)
        *   Trap Disarm
    *   **Perks/Mutations:** Found deeper or after near-death, offering permanent (for that run) buffs, possibly with trade-offs (e.g., "Bat's Ears: +Echolocation Range, -Light Tolerance").
*   **Fun Factor:**
    *   Character customization and adapting playstyle.
*   **Programming Ideas:**
    *   **Character Stats:** Store player attributes.
    *   **Skill Tree/System:** Allow player to spend XP or find skill points.
    *   **Status Effect System:** For perks, curses, temporary buffs/debuffs.

## II. Programming Ideas - Getting Started

### 1. Core Game Loop:

1.  **Initialize game** (load resources, generate first level).
2.  **Player turn:**
    *   Get player input (move, attack, use item, ping).
    *   Process player action.
    *   Update game state (move player, resolve combat, etc.).
3.  **Enemy turn(s):**
    *   For each enemy:
        *   Process AI (decide action).
        *   Execute action.
        *   Update game state.
4.  **Update screen.**
5.  **Check win/lose conditions.**
6.  **Repeat.**

### 2. Start Simple (Minimum Viable Product - MVP):

*   Player can move on a procedurally generated map.
*   Basic light source (fixed radius).
*   One type of ore to mine.
*   One type of enemy that moves towards the player.
*   Player can attack enemy.
*   Win/lose condition (e.g., reach stairs / die).
*   **Then add features one by one:** Echolocation, crafting, more enemy types, etc.

## III. Making it Fun - Key Roguelike Principles

*   **Meaningful Choices:** Every decision should have weight and consequence.
*   **High Replayability:** Procedural generation, random drops, varied encounters.
*   **Risk vs. Reward:** Pushing deeper or using powerful but risky abilities.
*   **Fairness (mostly):** Deaths should feel like a result of player decisions or calculated risks, not arbitrary punishment.
*   **Sense of Discovery:** Uncovering new ores, enemies, secrets, and mechanics.
