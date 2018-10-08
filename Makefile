results_dir = results/
runs = 2
threads = 2

names = ga.8.4.2 ga.8.4.3 ga.8.5.2 ga.8.7.2 ga.16.8.2 ga.16.8.3 ga.16.15.2
names += gp.8.4.2.2 gp.8.4.2.3 gp.8.4.3.3 gp.8.4.3.3 gp.8.5.2.2 gp.8.5.2.3 gp.8.7.2.2 gp.8.7.2.3 gp.16.8.2.2 gp.16.8.2.3 gp.16.8.2.4 gp.16.8.3.2 gp.16.8.3.3 gp.16.8.3.4


targets= $(addprefix $(results_dir),$(addsuffix .log,$(names)))
test: $(targets)

results/ga.%.log: target/release/ga
	@echo "Making $@..."
	@set -e; \
	FIRST=$$(echo $@ | grep -o '[[:digit:]]\+' | head -1 );\
	TMP=$$(echo $@ | grep -o '[[:digit:]]\+' | tail -2 ); \
	LOG=$$(echo "l($$FIRST)/l(2)"| bc -l | grep -o '[[:digit:]]\+' | head -1);\
	./ga $$LOG $${TMP[@]} --log $@.tmp --runs $(runs) --threads $(threads)
	@mv $@.tmp $@

results/gp.%.log: target/release/gp
	@echo "Making $@..."
	@set -e; \
	FIRST=$$(echo $@ | grep -o '[[:digit:]]\+' | head -1 );\
	TMP=$$(echo $@ | grep -o '[[:digit:]]\+' | tail -3 | head -2 ); \
	LAST=$$(echo $@ | grep -o '[[:digit:]]\+' | tail -1 );\
	LOG=$$(echo "l($$FIRST)/l(2)"| bc -l | grep -o '[[:digit:]]\+' | head -1);\
	./gp $$LOG $${TMP[@]} --log $@.tmp --runs $(runs) --threads $(threads) --max-depth $$LAST
	@mv $@.tmp $@
