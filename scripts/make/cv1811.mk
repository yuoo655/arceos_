cv1811: build
	cp $(OUT_BIN) cv1811h/arceos-cv1811.bin
	mkimage -f cv1811h/arceos-cv1811h.its cv1811h/arceos-cv1811.itb
	@cp cv1811h/arceos-cv1811.itb /srv/tftp/
	@echo 'Built the FIT-uImage cv1811h/arceos-cv1811.itb'
