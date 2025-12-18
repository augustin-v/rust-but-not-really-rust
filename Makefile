main.o: 
	rustc main.rs -C panic="abort" -C opt-level=z -o main.o -C link-arg=-lSystem --emit=obj

executable:
	cc main.o -o main

clean: 
	rm main.o main

all:
	make main.o && make executable