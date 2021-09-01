ECPG = ecpg

%.c: %.pgc
        $(ECPG) $<
