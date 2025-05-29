# Active Context: AI Enemy System Implementation

## Current Status: COMPLETED ✅

**Date**: January 2, 2025
**Mode**: ACT MODE - Implementation Complete

## What Was Just Completed

### ✅ AI Enemy Spawning and Interaction System

Successfully implemented a comprehensive AI system using big-brain utility AI for enemy behavior:

#### 1. **Big-Brain Integration**

- Added `big-brain = "0.22.0"` dependency for utility AI
- Integrated BigBrainPlugin with Bevy's PreUpdate schedule
- Set up proper system scheduling for scorers and actions

#### 2. **AI Behavior Components**

- `AIBehavior`: Core AI component with behavior type and detection range
- `AIBehaviorType`: Enum for Hostile, Passive, and Neutral behaviors
- `AIState`: Tracks current AI action and target position

#### 3. **AI Scorer Systems** (AI "Eyes")

- `chase_player_scorer_system`: Hostile enemies detect and score chasing opportunities
- `flee_from_player_scorer_system`: Passive enemies detect threats and score fleeing
- `wander_scorer_system`: Base wandering behavior for all AI
- `player_visibility_scorer_system`: General player detection utility

#### 4. **AI Action Systems** (AI "Hands")

- `chase_player_action_system`: Pathfinding toward player with obstacle avoidance
- `flee_from_player_action_system`: Pathfinding away from player
- `wander_action_system`: Random movement in walkable directions
- `idle_action_system`: Fallback behavior when no actions are viable

#### 5. **Enemy Entity Definitions**

Created data-driven enemy definitions:

- **Hostile Guard**: Aggressive, chases player (speed: 110, detection: 6 tiles)
- **Passive Critter**: Flees from player (speed: 90, detection: 5 tiles)
- **Neutral Wanderer**: Ignores player, wanders (speed: 80, detection: 3 tiles)

#### 6. **AI Spawning System**

- `spawn_multiple_ai_enemies`: Spawns mixed enemy types from definitions
- Automatic behavior type detection from entity names
- Integration with existing entity definition system
- Proper big-brain Thinker creation for each behavior type

#### 7. **Pathfinding and Movement**

- Direction calculation for chase/flee behaviors
- Alternative pathfinding when direct routes are blocked
- Random walkable direction finding with collision avoidance
- Integration with existing WalkBuilder action system

## Technical Implementation Details

### Big-Brain Thinker Configuration

```rust
// Hostile enemies prioritize chasing when they see the player
Thinker::build()
    .picker(FirstToScore { threshold: 0.6 })
    .when(ChasePlayerScorer, ChasePlayerAction)
    .when(WanderScorer, WanderAction)
    .otherwise(IdleAction)

// Passive enemies prioritize fleeing from threats
Thinker::build()
    .picker(FirstToScore { threshold: 0.5 })
    .when(FleeFromPlayerScorer, FleeFromPlayerAction)
    .when(WanderScorer, WanderAction)
    .otherwise(IdleAction)

// Neutral enemies just wander around
Thinker::build()
    .picker(FirstToScore { threshold: 0.3 })
    .when(WanderScorer, WanderAction)
    .otherwise(IdleAction)
```

### FOV-Based Detection

- AI enemies use the existing FOV system to detect the player
- Detection ranges vary by enemy type (3-6 tiles)
- Line-of-sight requirements for player detection
- Last known position tracking for chase behavior

### Integration Points

- **Turn System**: AI actions integrate with existing TurnActor system
- **Map System**: Pathfinding uses CurrentMap for walkability checks
- **FOV System**: Player detection uses existing FovMap resource
- **Entity Definitions**: Leverages data-driven entity spawning system

## Files Modified/Created

### Core AI System

- `src/model/components/ai_behavior.rs` - AI behavior components
- `src/model/systems/ai_systems.rs` - Scorer and action systems
- `src/model/systems/ai_spawning.rs` - AI entity spawning logic
- `src/model/mod.rs` - BigBrainPlugin integration

### Enemy Definitions

- `assets/entities/enemies/hostile_guard.definition.ron`
- `assets/entities/enemies/passive_critter.definition.ron`
- `assets/entities/enemies/neutral_wanderer.definition.ron`

### Integration Updates

- `src/view/screens/gameplay.rs` - Added AI spawning to initialization
- `src/controller/systems/process.rs` - Updated monster turn processing
- `Cargo.toml` - Added big-brain and regex dependencies

## Current Game State

### Player Experience

- **Hostile Guards**: Will chase the player when spotted, creating tension
- **Passive Critters**: Will flee when the player approaches, adding variety
- **Neutral Wanderers**: Provide ambient life without direct threat

### AI Behaviors Working

- ✅ FOV-based player detection
- ✅ Pathfinding with obstacle avoidance
- ✅ Behavior-specific responses (chase/flee/wander)
- ✅ Integration with turn-based system
- ✅ Data-driven enemy spawning

## Next Development Opportunities

### Immediate Enhancements

1. **Combat System**: Add attack actions for hostile enemies
2. **Sound/Visual Feedback**: Alert indicators when enemies spot player
3. **AI Difficulty Scaling**: Adjust detection ranges and speeds by level

### Advanced Features

1. **Group Behavior**: Coordinated AI actions between enemies
2. **Memory System**: AI remembers player's last known location longer
3. **Patrol Routes**: Predefined movement patterns for guards
4. **Dynamic Spawning**: Spawn enemies based on player actions/location

## Performance Notes

- Big-brain systems run in PreUpdate for optimal performance
- Scorer systems are highly parallelizable
- Action systems integrate seamlessly with existing turn queue
- No performance regressions observed during testing

## Success Metrics Achieved

- ✅ Enemies spawn with distinct behaviors
- ✅ Player detection works via FOV system
- ✅ Pathfinding avoids obstacles and other entities
- ✅ Turn-based integration maintains game flow
- ✅ Data-driven configuration allows easy enemy creation
- ✅ Build compiles successfully with no errors

The AI enemy system is now fully functional and ready for gameplay testing!
