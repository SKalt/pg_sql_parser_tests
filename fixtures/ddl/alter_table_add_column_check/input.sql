ALTER TABLE products ADD COLUMN description text CHECK (description <> '');
