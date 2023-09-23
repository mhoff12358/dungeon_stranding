extends Button

@export var shop: ShopViz

func _pressed():
	shop.finish_buying()
