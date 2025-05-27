# Tasks Plan: Data-Driven Entity System Implementation

## Current Status: PLANNING PHASE
**Overall Progress**: 0% (Planning Complete, Ready for Implementation)

## Phase 1: Foundation (Core Data Structures)
**Target Timeline**: 2-3 hours
**Status**: 🔴 NOT STARTED

### Task 1.1: Create Entity Definition Data Structures ✅
**Status**: ✅ COMPLETED
**Actual Time**: 45 minutes
**Dependencies**: None

**Details**:
- Create `src/model/entities/` module
- Implement `EntityDefinition` struct with serde support
- Create component data structs:
  - `TurnActorData`
  - `ViewShedData`
  - `TileSpriteData`
- Add trait implementations for data-to-component conversion
- Write unit tests for serialization/deserialization

**Files Created/Modified**:
- ✅ `src/model/entities/mod.rs`
- ✅ `src/model/entities/definition.rs`
- ✅ `src/model/entities/components.rs`
- ✅ Updated `src/model/mod.rs`

**Acceptance Criteria**:
- ✅ All data structures compile without errors
- ✅ RON serialization/deserialization works
- ✅ Unit tests pass for all conversions (8/8 tests passing)
- ✅ Type safety is maintained

---

### Task 1.2: Asset Loading Integration
**Status**: 🔴 PENDING
**Estimated Time**: 1 hour
**Dependencies**: Task 1.1

**Details**:
- Extend existing asset loading system
- Add `EntityDefinitions` resource
- Integrate with `bevy_asset_loader`
- Set up folder loading for entity definitions

**Files to Create/Modify**:
- `src/model/entities/loader.rs`
- `assets/textures.ron` (add entity_definitions)
- Update asset loading configuration

**Acceptance Criteria**:
- [ ] Entity definitions load at startup
- [ ] Resource is accessible in systems
- [ ] Error handling for missing files
- [ ] Hot reloading works in dev mode

---

### Task 1.3: Basic Entity RON Files ✅
**Status**: ✅ COMPLETED
**Actual Time**: 15 minutes
**Dependencies**: Task 1.1

**Details**:
- Create directory structure for entity assets
- Write player.ron definition matching current hardcoded values
- Write basic enemy definitions (whale, basic_enemy)
- Validate RON syntax and structure

**Files Created**:
- ✅ `assets/entities/player.ron`
- ✅ `assets/entities/enemies/whale.ron`
- ✅ `assets/entities/enemies/basic_enemy.ron`

**Acceptance Criteria**:
- ✅ RON files have valid syntax
- ✅ Definitions match current hardcoded components
- ✅ Files load without errors (verified with tests)
- ✅ All required components are specified

---

## Phase 2: Core Migration (Replace Hardcoded Spawning)
**Target Timeline**: 3-4 hours
**Status**: 🔴 NOT STARTED

### Task 2.1: Data-Driven Spawning System
**Status**: 🔴 PENDING
**Estimated Time**: 2 hours
**Dependencies**: Tasks 1.1, 1.2

**Details**:
- Create new spawning functions that use entity definitions
- Implement component building from data
- Add entity lookup by name/id
- Maintain exact component compatibility with existing system

**Files to Create/Modify**:
- `src/model/entities/spawner.rs`
- `src/model/systems/spawner.rs` (refactor existing)

**Acceptance Criteria**:
- [ ] Can spawn entities from definitions
- [ ] Component creation matches hardcoded values
- [ ] Position and turn queue integration works
- [ ] No regression in entity behavior

---

### Task 2.2: Player Spawning Migration
**Status**: 🔴 PENDING
**Estimated Time**: 1 hour
**Dependencies**: Task 2.1

**Details**:
- Replace hardcoded player spawning with data-driven version
- Update `spawn_player` system to use EntityDefinitions resource
- Ensure exact behavioral compatibility
- Add fallback to hardcoded if data loading fails

**Files to Modify**:
- `src/model/systems/spawner.rs`
- `src/view/screens/gameplay.rs` (if needed)

**Acceptance Criteria**:
- [ ] Player spawns with same components as before
- [ ] Turn system integration preserved
- [ ] Visual rendering unchanged
- [ ] Input system still works
- [ ] Fallback mechanism works

---

### Task 2.3: Enemy Spawning Migration
**Status**: 🔴 PENDING
**Estimated Time**: 1 hour
**Dependencies**: Task 2.2

**Details**:
- Replace hardcoded enemy spawning with data-driven version
- Support multiple enemy types from definitions
- Maintain AI and turn system integration
- Add enemy type selection logic

**Files to Modify**:
- `src/model/systems/spawner.rs`

**Acceptance Criteria**:
- [ ] Enemies spawn with correct components
- [ ] AI system integration preserved
- [ ] Multiple enemy types supported
- [ ] Turn timing and behavior unchanged

---

## Phase 3: Testing & Validation
**Target Timeline**: 1-2 hours
**Status**: 🔴 NOT STARTED

### Task 3.1: Integration Testing
**Status**: 🔴 PENDING
**Estimated Time**: 1 hour
**Dependencies**: All Phase 2 tasks

**Details**:
- Write integration tests for full spawning pipeline
- Test hot reloading functionality
- Validate performance vs hardcoded system
- Test error scenarios and recovery

**Files to Create**:
- `tests/integration/entity_spawning.rs`
- `benches/spawning_performance.rs`

**Acceptance Criteria**:
- [ ] All integration tests pass
- [ ] Performance is equivalent to hardcoded
- [ ] Hot reloading works correctly
- [ ] Error recovery is robust

---

### Task 3.2: Component Registration & Type Safety
**Status**: 🔴 PENDING
**Estimated Time**: 30 minutes
**Dependencies**: Task 3.1

**Details**:
- Ensure all new types are registered with Bevy
- Add reflection support for debugging
- Validate serialization compatibility
- Check save/load compatibility

**Files to Modify**:
- `src/model/mod.rs`

**Acceptance Criteria**:
- [ ] All types registered for reflection
- [ ] Debug inspector works with new components
- [ ] No serialization errors
- [ ] Save/load functionality preserved

---

## Phase 4: Enhancement & Polish
**Target Timeline**: 2-3 hours (OPTIONAL)
**Status**: 🔴 NOT STARTED

### Task 4.1: Template System (OPTIONAL)
**Status**: 🔴 PENDING
**Estimated Time**: 2 hours
**Dependencies**: All Phase 3 tasks

**Details**:
- Implement entity inheritance/template system
- Add base template support
- Support component overrides
- Create template examples

**Files to Create/Modify**:
- `src/model/entities/template.rs`
- `assets/entities/templates/`

**Acceptance Criteria**:
- [ ] Template inheritance works
- [ ] Component overrides apply correctly
- [ ] Template validation prevents cycles
- [ ] Examples demonstrate functionality

---

### Task 4.2: Development Tools (OPTIONAL)
**Status**: 🔴 PENDING
**Estimated Time**: 1 hour
**Dependencies**: Task 4.1

**Details**:
- Add RON validation tools
- Implement entity definition viewer
- Add performance profiling
- Create documentation

**Files to Create**:
- `tools/entity_validator.rs`
- `docs/entity_system_guide.md`

**Acceptance Criteria**:
- [ ] RON validation catches errors
- [ ] Entity viewer shows definitions clearly
- [ ] Performance tools provide insights
- [ ] Documentation is complete

---

## Risk Assessment & Mitigation

### High Risk Items
1. **Component Compatibility**: Ensure exact behavioral match
   - **Mitigation**: Extensive testing, component-by-component validation

2. **Performance Regression**: Data-driven might be slower
   - **Mitigation**: Benchmarking, caching strategies, profiling

3. **Asset Loading Failures**: RON files might be corrupted
   - **Mitigation**: Robust error handling, fallback mechanisms

### Medium Risk Items
1. **Type Safety**: Serde deserialization errors
   - **Mitigation**: Comprehensive error handling, validation

2. **Development Workflow**: Hot reloading complexity
   - **Mitigation**: Feature flags, gradual rollout

## Known Issues & Constraints

### Current System Limitations
- Hardcoded spawning makes entity variety difficult
- No designer-friendly way to modify entities
- Testing new entity types requires code changes

### Technical Constraints
- Must maintain Bevy 0.16 compatibility
- Performance should not regress
- Existing save files must remain compatible
- Hot reloading should work in development

## Success Metrics

### Functional Success
- [ ] Player spawns from RON data with identical behavior
- [ ] Multiple enemy types spawn from data files
- [ ] No regressions in gameplay or performance
- [ ] Hot reloading works for rapid iteration

### Quality Success
- [ ] Code coverage >80% for new modules
- [ ] All linter checks pass
- [ ] Performance benchmarks within 5% of baseline
- [ ] Error handling covers all failure modes

### Development Success
- [ ] New entity types can be added with only data changes
- [ ] Designer workflow is improved
- [ ] Code is more maintainable and modular
- [ ] Documentation enables easy extension

## Dependencies & Blockers

### External Dependencies
- Bevy 0.16 ECS system
- serde crate for serialization
- bevy_asset_loader for asset management

### Internal Dependencies
- Existing component definitions
- Current spawning system architecture
- Asset loading pipeline
- Turn system integration

### Potential Blockers
- Complex component relationships might be hard to serialize
- Performance requirements might conflict with flexibility
- Hot reloading might introduce complexity
- Error handling edge cases might be difficult to cover
