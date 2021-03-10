
all:

bench: bench.en.1 bench.ja.1

bench.en.1:
	@rm -f z.bench.en.1.log
	cargo bench --no-run
	env AKI_TEST_DAT=en.1 cargo bench --bench=bench-match-string -- -n | tee -a z.bench.en.1.log
	env AKI_TEST_DAT=en.1 cargo bench --bench=bench-match-regex -- -n | tee -a z.bench.en.1.log
	env AKI_TEST_DAT=en.1 cargo bench --bench=bench-match-glob -- -n | tee -a z.bench.en.1.log

bench.ja.1:
	@rm -f z.bench.ja.1.log
	cargo bench --no-run
	env AKI_TEST_DAT=ja.1 cargo bench --bench=bench-match-string -- -n | tee -a z.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 cargo bench --bench=bench-match-regex -- -n | tee -a z.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 cargo bench --bench=bench-match-glob -- -n | tee -a z.bench.ja.1.log

bench-build:
	cargo bench --no-run

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results
