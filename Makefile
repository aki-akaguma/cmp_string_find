TASKSET = taskset -c 2

BENCH_STR = --bench=bench-find-string
BENCH_REG = --bench=bench-find-regex
BENCH_GLO = --bench=bench-find-glob

TARGET_MUSL = --target=x86_64-unknown-linux-musl

all:

bench-all: bench bench-musl

bench-build-all: bench-build bench-build-musl


bench: bench.en.1 bench.ja.1

bench-musl: bench.en.1-musl bench.ja.1-musl

bench-build:
	cargo bench --no-run

bench-build-musl:
	cargo bench --no-run $(TARGET_MUSL)

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results


bench.en.1:
	@rm -f z.bench.en.1.log
	cargo bench --no-run
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_STR) -- -n | tee -a z.bench.en.1.log
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_REG) -- -n | tee -a z.bench.en.1.log
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_GLO) -- -n | tee -a z.bench.en.1.log

bench.ja.1:
	@rm -f z.bench.ja.1.log
	cargo bench --no-run
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_STR) -- -n | tee -a z.bench.ja.1.log
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_REG) -- -n | tee -a z.bench.ja.1.log
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_GLO) -- -n | tee -a z.bench.ja.1.log

bench.en.1-musl:
	@rm -f z.musl.bench.en.1.log
	cargo bench --no-run $(TARGET_MUSL)
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_REG) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log
	$(TASKSET) env AKI_TEST_DAT=en.1 cargo bench $(BENCH_GLO) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl:
	@rm -f z.musl.bench.ja.1.log
	cargo bench --no-run $(TARGET_MUSL)
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_REG) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
	$(TASKSET) env AKI_TEST_DAT=ja.1 cargo bench $(BENCH_GLO) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
