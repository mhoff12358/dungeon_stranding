extends ShopViz


signal updated_state(shop: ShopViz)


func _ready():
	self.out_of_dungeon().updated_state.connect(self.out_of_dungeon_updated_state)


func out_of_dungeon_updated_state():
	self.updated_state.emit(self)
