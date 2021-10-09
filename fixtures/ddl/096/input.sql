alter table foo ADD CHECK ( x = 1 )
;alter table foo ADD CHECK ( county IN ( 'Oxfordshire', 'Buckinghamshire', 'Warwickshire' ))
;alter table foo ADD CHECK ( outletID >= 100 AND outletID < 200 )
;
