use game_core::{ids::ItemId, inventory::ItemStack};

#[test]
fn empty_stacks_are_rejected_and_maximum_count_is_preserved() {
    assert_eq!(ItemStack::new(ItemId::new(3), 0), None);
    let stack = ItemStack::new(ItemId::new(0), u32::MAX).unwrap();
    assert_eq!(stack.item(), ItemId::new(0));
    assert_eq!(stack.count().get(), u32::MAX);
    assert_eq!(ItemStack::new(ItemId::new(9), 1).unwrap().count().get(), 1);
}
