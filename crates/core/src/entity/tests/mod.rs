use super::EntityManager;

#[test]
pub fn test_entity_manager_create() {
        let entity_manager: EntityManager = EntityManager::new();
        assert_eq!(entity_manager.num_entities(), 0)
}