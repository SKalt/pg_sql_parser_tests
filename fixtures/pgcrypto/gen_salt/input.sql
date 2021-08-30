UPDATE tbl SET pswhash = crypt('new password', gen_salt('md5'));
