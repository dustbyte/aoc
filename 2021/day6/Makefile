CC=clang -W -Wall -ansi
PROGNAME=d6

all: $(PROGNAME)

$(PROGNAME): main.c
	$(CC) $< -o $@

clean:
	rm -f $(PROGNAME)

re: clean all
