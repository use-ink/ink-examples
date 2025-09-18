#!/bin/bash

# This script copies the public integration tests from ink! over 
# and replaces the relatives paths to ink! crates.

# Path to a local copy of the ink! repostiory.
INK_PROJECT_FOLDER="../ink"

# What to substitute the relative `path = "../"` with.
INK_SUBSTITUTION='git = "https://github.com/use-ink/ink", tag = "v6.0.0-alpha.4"'
INK_E2E_SUBSTITUTION='git = "https://github.com/use-ink/ink", tag = "v6.0.0-alpha.4"'

# Clean all examples first, if there are still `target` folders the copy
# will take ages.
fd Cargo.toml $INK_PROJECT_FOLDER/integration-tests/public/ |
	xargs -n1 -I{} bash -c "echo {}; cargo clean --manifest-path={}" 

/bin/cp -rf $INK_PROJECT_FOLDER/integration-tests/public/* .   

# Find all `Cargo.toml` files and replace the `path`
find . -name "Cargo.toml" -type f | while read -r file; do
  # replace `ink`
  perl -pi -e "s|path = \"./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\.\/\.\./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\.\/\.\.\/\.\./\.\./crates/ink\"|$INK_SUBSTITUTION|" "$file"

  # replace `ink_sandbox`
  perl -pi -e "s|path = \"\.\./\.\.\/\.\./\.\.\/\.\./crates/e2e/sandbox\"|$INK_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\./\.\.\/\.\./crates/e2e/sandbox\"|$INK_SUBSTITUTION|" "$file"

  # replace `ink_e2e`
  perl -pi -e "s|path = \"./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\.\/\.\./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
  perl -pi -e "s|path = \"\.\.\/\.\.\/\.\.\/\.\.\/\.\./\.\./crates/e2e\"|$INK_E2E_SUBSTITUTION|" "$file"
done
