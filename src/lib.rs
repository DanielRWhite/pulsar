/*
Entities: Uniquely identify objects in a game
Components: Datatypes that can be added to entities
Systems: Functions that run for all entities matching a component query
Arch-Types: How data is stored & cached, entities with the same components are stored together as an ArchType for faster lookup
*/
pub mod game;
pub mod entity;