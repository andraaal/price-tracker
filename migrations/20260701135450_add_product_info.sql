CREATE TYPE vendor AS ENUM ('Spar');
CREATE TYPE tag AS ENUM ('Organic', 'Vegan', 'Vegetarian', 'GlutenFree', 'LactoseFree', 'FairTrade', 'AMASigil', 'Cooled', 'Frozen');

ALTER TABLE products
ADD COLUMN brand text NOT NULL,
Add COLUMN vendor vendor NOT NULL,
ADD COLUMN price smallint NOT NULL,
ADD COLUMN quantity text NOT NULL,
ADD COLUMN image_url text NOT NULL,
ADD COLUMN reference_price smallint NOT NULL,
ADD COLUMN reference_unit text NOT NULL,
ADD COLUMN reference_quantity text NOT NULL,
ADD COLUMN tags tag[] NOT NULL
