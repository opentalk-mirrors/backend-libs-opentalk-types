# SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
#
# SPDX-License-Identifier: EUPL-1.2
#
# This file can be used with the [`just`](https://just.systems) tool.

[no-exit-message]
_check_cargo_depgraph:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! cargo --list | grep "depgraph" > /dev/null; then
        echo 'cargo-depgraph is not available, please install it with`' >&2
        echo '`cargo install cargo-depgraph`' >&2
        exit 1
    fi

[no-exit-message]
_check_dot:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v dot > /dev/null; then
        echo 'dot is not available, please install Graphviz.`' >&2
        exit 1
    fi
generate-deps-graph: _check_cargo_depgraph _check_dot
    #!/usr/bin/env bash
    set -euo pipefail
    OUT_PATH="target/dep-graph.png"
    cargo depgraph --workspace-only --all-deps | dot -Tpng > $OUT_PATH
    echo "Created dependency graph at $OUT_PATH"

# Create annotated `<crate>-<version>` tags on HEAD for all workspace crates (existing tags are skipped)
[no-exit-message]
tag-release:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo metadata --no-deps --format-version 1 \
        | jq -r '.packages[] | select(.publish != []) | "\(.name)-\(.version)"' \
        | sort \
        | while read -r tag; do
            if git rev-parse -q --verify "refs/tags/$tag" > /dev/null; then
                echo "Tag '$tag' already exists, skipping."
            else
                git tag -a "$tag" -m "$tag"
                echo "Created tag '$tag'."
            fi
        done
