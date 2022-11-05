# Rustikub
A simulation of the tabletop game Rummikub, built entirely in Rust. 
Someday, may or may not include a UI as well. 

# Design
High level diagrams of the events in the game and the basic logic for how they interact. See /resources for the official Rummikub rules.

## Basic Game Loop
```mermaid
graph TD
StartGame --> SetUpShuffleBoneyard
SetUpShuffleBoneyard --> D[EachPlayerDrawsInitialSet]
D --> N1[Player1 Turn]
N1 -->|Win?| Win
N1 --> N2[Player N+1 Turn]
N2 -->|Win?| Win
N2 --> N3[PlayerN Turn]
N3 -->|Win?| Win
N3 --> N1
Win[Game Ends] --> Score[Score And Store/Sum for Multiple Rounds!]
``` 

## Individual Player Turn
```mermaid
graph TD
start(Previous player's turn ends) --> LaidDown?{Have I Laid Down First 30?}
LaidDown? -->|Yes| RackEmpty
LaidDown? -->|No| FirstMeld{ScanRack Has Sets <br>:group+run: >= 30pts?}
FirstMeld -->|Yes, Lay Down Initial Set| RackEmpty
FirstMeld -->|No| Draw(Draw Tile from Boneyard)
Draw --> EndTurn(End Turn Pass to Next Player)

Placed?{Have I Already<br> Placed >=1 Tile?}
Placed? -->|Yes| EndTurn
Placed? -->|No| Draw

RackEmpty{Is Rack Empty?} -->|Yes Empty| Win
RackEmpty -->|No >0 Tile| Think
Think[Table and Rack not Empty]
Think --> RackSet{Do I have one or<br> more sets on Rack?}
PlaceSet --> RackEmpty
RackSet -->|No| RearrangeTable{Can I Rearrange Table <br> and place >0 Tiles}
RackSet -->|Yes| PlaceSet[Place Set From Rack]
RearrangeTable -->|No| Placed?
RearrangeTable -->|Yes| Change[Rearrange Table and Place Tile]
Change --> RackEmpty
Win[Rack Empty Win!]
EndTurn[No More Available Moves! End Turn Pass to Next PlayerEnd Logic]
```

## Domain Model Types
```typescript
enum Number {One,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten,Eleven,Twelve,Thirteen}
enum Color {Red,Blue,Orange,Black}

interface RegularTile {
    color: Color,
    number: Number,
}
interface Joker {};
type Tile = RegularTile | Joker;

type Group = {
    // Only one from each color of the same number
    members: Map<Color, boolean>
}
type Run = {
    // Must be a sequantially ordered set of tiles with same color
    color: Color,
    members: Number[],
}
type Set = Group | Run;

type Boneyard = {
    drawPile: Tile[],
  }
type PlayerRack = {
    rows: Tile[],
}
```



# Repository Layout
*At least as of writing*

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE.md
├── README.md
├── resources
│   └── Rummikub_Official_Rules.pdf
├── src
│   ├── domain
│   │   ├── boneyard.rs
│   │   ├── sets.rs
│   │   ├── table.rs
│   │   └── tiles.rs
│   ├── domain.rs
│   └── main.rs
└── tests
    └── integration_tests_only.rs
```