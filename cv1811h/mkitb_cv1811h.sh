#!/usr/bin/env bash

mkimage -f arceos-cv1811h.its arceos-cv1811h.itb 
sudo mv  arceos-cv1811h.itb /srv/tftp/
