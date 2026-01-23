ENTRY_POINT := src/main.go
BUILD_DIR := ./build
BIN_NAME := day2part2

build: $(ENTRY_POINT)
	mkdir -p $(BUILD_DIR)
	cp ../input.txt $(BUILD_DIR)
	go build -o $(BUILD_DIR)/$(BIN_NAME) $(ENTRY_POINT)

test: build
	go test ./src

run: build
	./$(BUILD_DIR)/$(BIN_NAME)

clean:
	rm -rf $(BUILD_DIR)
