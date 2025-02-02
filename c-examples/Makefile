# Compiler settings
# CC = gcc
# CFLAGS = -Wall

# Find all the C files in the current directory
SOURCES = $(wildcard *.c)

# Define the assembly files
ASM_FILES = $(SOURCES:.c=.s)

# Target: All assembly files
all: $(ASM_FILES)

# Rule to compile each .c file to .s (assembly)
%.s: %.c
	$(CC) $(CFLAGS) -S $< -o $@

# Clean up generated files
clean:
	rm -f $(ASM_FILES)

.PHONY: all clean
