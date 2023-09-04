extends Label


func _on_shop_panel_updated_state(shop: ShopViz):
    self.text = shop.selected_text()
