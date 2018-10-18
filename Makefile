results_dir = results/
runs = 30
threads = 4

ga_names = ga.8.4.2 ga.8.4.3 ga.8.5.2 ga.8.7.2 ga.16.8.2 ga.16.8.3 ga.16.15.2
gp_names = gp.8.4.2 gp.8.4.3 gp.8.5.2 gp.8.7.2 gp.16.8.2 gp.16.8.3 gp.16.15.2 gp.32.16.3

ga_targets= $(addprefix $(results_dir),$(addsuffix .log,$(ga_names)))
gp_targets= $(addprefix $(results_dir),$(addsuffix .log,$(gp_names)))

ga : $(ga_targets)

gp : $(gp_targets)

$(results_dir)ga.%.log: target/release/ga
	@mkdir -p $(results_dir)
	@echo "Making $@..."
	@set -e; \
	FIRST=$$(echo $@ | grep -o '[[:digit:]]\+' | head -1 );\
	TMP=$$(echo $@ | grep -o '[[:digit:]]\+' | tail -2 ); \
	LOG=$$(echo "l($$FIRST)/l(2)"| bc -l | grep -o '[[:digit:]]\+' | head -1);\
	./ga $$LOG $${TMP[@]} --log $@.tmp --runs $(runs) --threads $(threads) $(opts)
	@mv $@.tmp $@

$(results_dir)gp.%.log: target/release/gp
	@mkdir -p $(results_dir)
	@echo "Making $@..."
	@set -e; \
	FIRST=$$(echo $@ | grep -o '[[:digit:]]\+' | head -1 );\
	TMP=$$(echo $@ | grep -o '[[:digit:]]\+' | tail -2 ); \
	LOG=$$(echo "l($$FIRST)/l(2)"| bc -l | grep -o '[[:digit:]]\+' | head -1);\
	./gp $$LOG $${TMP[@]} --log $@.tmp --runs $(runs) --threads $(threads) $(opts)
	@mv $@.tmp $@

target/release/ga target/release/gp: 
	cargo build --release