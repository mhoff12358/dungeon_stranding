extends InventoryViz

signal updated_state(inventory: InventoryViz)


func _ready():
    self.game_state().updated_state.connect(self.game_state_updated_state)


func game_state_updated_state():
    self.updated_state.emit(self)
