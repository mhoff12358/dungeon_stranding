extends Label



func _on_inventory_panel_updated_state(inventory: InventoryViz):
    self.text = ""
    var gear_array = inventory.gear()
    for gear in gear_array:
        self.text += gear[0] + " | " + str(gear[1]) + "\n"

