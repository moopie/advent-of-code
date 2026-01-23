ENTRY_POINT := src/main.go
BUILD_DIR := build
BIN_NAME := day9part2
INPUT := ../input.txt

build: $(ENTRY_POINT)
	mkdir -p $(BUILD_DIR)
	cp $(INPUT) $(BUILD_DIR)
	go build -o $(BUILD_DIR)/$(BIN_NAME) $(ENTRY_POINT)

test:
	go test ./src

run: build
	./$(BUILD_DIR)/$(BIN_NAME)

clean:
	rm -rf $(BUILD_DIR)
