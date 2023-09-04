extends Label



func _on_inventory_panel_updated_state(inventory: InventoryViz):
    self.text = "Money: " + str(inventory.money())
    self.text += "\nFood: " + str(inventory.food())
    self.text += "\nWeight: " + str(inventory.weight())
