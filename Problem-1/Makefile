CC = g++
CFLAGS = -Wall -g

SRC = Solution.cpp
OBJ = $(SRC:.c=.o)
EXECUTABLE = myprogram

all: $(EXECUTABLE)

$(EXECUTABLE): $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(EXECUTABLE)

test:
	./myprogram

# 伪目标：不生成文件，只是执行动作
.PHONY: all clean test
