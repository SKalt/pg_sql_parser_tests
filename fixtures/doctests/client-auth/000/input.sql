openssl x509 -in myclient.crt -noout --subject -nameopt RFC2253 | sed "s/^subject=//"
