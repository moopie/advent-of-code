ENTRY_POINT := src/main.go
BUILD_DIR := ./bin
BIN_NAME := day1part1

build: $(ENTRY_POINT)
	mkdir -p $(BUILD_DIR)
	cp input.txt $(BUILD_DIR)
	go build -o $(BUILD_DIR)/$(BIN_NAME) $(ENTRY_POINT)

run: build
	./$(BUILD_DIR)/$(BIN_NAME)

clean:
	rm -rf $(BUILD_DIR)