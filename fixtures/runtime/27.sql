# root
openssl req -new -nodes -text -out root.csr \
  -keyout root.key -subj "/CN=