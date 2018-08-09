EXE_DIR = release
EXE = kontakt-server
# Use 'VERBOSE=1' to echo all commands, for example 'make help VERBOSE=1'.
ifdef VERBOSE
  Q :=
else
  Q := @
endif
defualt:
	$(Q)./$(EXE_DIR)/main
build:
	$(Q)echo "--- Building ---"
	$(Q)cargo build --target x86_64-unknown-linux-musl --release
	$(Q)echo "--- Done ---"
	$(Q)echo "--- Start Copying the file ---"
	$(Q)cp ./target/x86_64-unknown-linux-musl/release/$(EXE) ./$(EXE_DIR)/main
	$(Q)echo "--- Done ---"
	$(Q)echo "--- Start Striping the file ---"
	$(Q)strip ./$(EXE_DIR)/main
	$(Q)echo "--- Done ---"