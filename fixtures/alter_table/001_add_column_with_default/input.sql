ALTER TABLE measurements
  ADD COLUMN mtime timestamp with time zone DEFAULT now();
