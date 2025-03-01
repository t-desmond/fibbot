# Hello World Docker Action

This GitHub Action computes Fibonacci numbers for numerical values found in a pull request. It extracts numerical values from the PR and stores them in a vector. It then iterates over each element, mapping numbers less than a defined threshold to a new vector containing their Fibonacci values. Finally, the action posts the result as a comment to the pull request.

### Example Fibonacci Numbers

A list of Fibonacci numbers can be found [here](https://planetmath.org/listoffibonaccinumbers).

## How It Works

1. The action extracts numerical values from the pull request (e.g., from comments or files).
2. It stores these values in a vector.
3. It filters the numbers based on a threshold.
4. Fibonacci values are calculated for numbers less than the defined threshold.
5. The action posts the calculated Fibonacci numbers as a comment on the pull request.

## Example Usage

This example demonstrates how to use the FibBot Action in a GitHub workflow:

```yaml
name: "Run FibBot Action"

on:
  pull_request:
    types: [opened, synchronize, reopened]  # Trigger the action on PR events

# Grants necessary permissions to the event (pull_request)
permissions:
  pull-requests: write  # Allows the action to post comments on PRs
  contents: read        # Allows reading repository contents

jobs:
  Run-FibBot:
    runs-on: ubuntu-latest  # Run the job on the latest Ubuntu environment

    # Grant write permissions for the job
    permissions:
      contents: write       # Allows modifying repository contents
      pull-requests: write  # Allows posting comments on PRs

    steps:
      - name: Checkout code
        uses: actions/checkout@v4  # Checks out the repository code

      - name: Run FibBot
        id: result
        uses: t-desmond/fibbot@main  # Uses the FibBot action (replace with the correct version/tag)
        with:
          enable_fib: true          # Enables Fibonacci calculation
          max_threshold: '10'       # Only numbers less than 10 will be processed
        env:
          pr_number: ${{ github.event.pull_request.number }}  # Passes the PR number as an environment variable
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}  # Provides the GitHub token to post comments
```