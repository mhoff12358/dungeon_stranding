extends Label



func _on_inventory_panel_updated_state(inventory: InventoryViz):
    self.text = ""
    var equipment_array = inventory.combat_equipment()
    for equipment in equipment_array:
        self.text += equipment[0] + " | " + equipment[1] + "\n"

