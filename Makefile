all: SSEVectorCalc

SSEVectorCalc: SSEVectorCalc.rs
	rustc SSEVectorCalc.rs -o SSEVectorCalc

clean:
	rm -f SSEVectorCalc

.PHONY: all clean
