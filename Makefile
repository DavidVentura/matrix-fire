parts/baffles.dxf: parts/baffles.ps Makefile
	# 72 dpi / 1 inch
	pstoedit -yscale 28.3464566 -xscale 28.3464566 $< -f "dxf: -ctl -mm -polyaslines" $@
parts/baffles.pdf: parts/baffles.ps Makefile
	ps2pdf parts/baffles.ps $@
