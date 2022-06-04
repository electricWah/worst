
.PHONY: www
www:
	$(MAKE) -C web all

build: bundle
	cp -r $< $@
	$(MAKE) -C $@ setup

lworsti: build
	$(MAKE) -C $< $@
	cp $</$@ $@

rworst: worst.rkt
	@# workaround for bug in racket 7.2
	raco make $<
	raco exe --gui -o $@ $<

.PHONY: tests
tests:
	test/test.sh

.PHONY: deps
deps:
	raco pkg install hyper-literate

.PHONY: literate
literate: worst.rkt literate.css
	scribble --htmls --dest literate ++style literate.css $<

