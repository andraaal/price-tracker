export type Product = {
  id: string;
  name: string;
  brand: string;
  vendor: string;
  price: number;
  base_price: number;
  quantity: string;
  image_url: string;
  shop_url: string;
  reference: PriceReference;
  tags: string[];
};

export type PriceReference = {
  price: number;
  unit: string;
};
