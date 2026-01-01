#!/bin/bash

# ============================================================================
# JSON-Tools-rs Comprehensive Benchmark Runner
# ============================================================================
# This script runs all benchmark suites and generates comparison reports
# Usage:
#   ./scripts/run_benchmarks.sh [OPTIONS]
#
# Options:
#   --all                Run all benchmark suites
#   --isolation          Run isolation benchmarks only
#   --combination        Run combination benchmarks only
#   --realworld          Run real-world benchmarks only
#   --stress             Run stress benchmarks only
#   --comprehensive      Run original comprehensive benchmarks only
#   --compare <branch>   Compare with another branch
#   --baseline           Save results as baseline for future comparisons
#   --quick              Run quick benchmarks (shorter measurement time)
#   --help               Show this help message
# ============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
RUN_ALL=false
RUN_ISOLATION=false
RUN_COMBINATION=false
RUN_REALWORLD=false
RUN_STRESS=false
RUN_COMPREHENSIVE=false
COMPARE_BRANCH=""
SAVE_BASELINE=false
QUICK_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --all)
            RUN_ALL=true
            shift
            ;;
        --isolation)
            RUN_ISOLATION=true
            shift
            ;;
        --combination)
            RUN_COMBINATION=true
            shift
            ;;
        --realworld)
            RUN_REALWORLD=true
            shift
            ;;
        --stress)
            RUN_STRESS=true
            shift
            ;;
        --comprehensive)
            RUN_COMPREHENSIVE=true
            shift
            ;;
        --compare)
            COMPARE_BRANCH="$2"
            shift 2
            ;;
        --baseline)
            SAVE_BASELINE=true
            shift
            ;;
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --help)
            echo "JSON-Tools-rs Comprehensive Benchmark Runner"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --all                Run all benchmark suites"
            echo "  --isolation          Run isolation benchmarks only"
            echo "  --combination        Run combination benchmarks only"
            echo "  --realworld          Run real-world benchmarks only"
            echo "  --stress             Run stress benchmarks only"
            echo "  --comprehensive      Run original comprehensive benchmarks only"
            echo "  --compare <branch>   Compare with another branch"
            echo "  --baseline           Save results as baseline for future comparisons"
            echo "  --quick              Run quick benchmarks (shorter measurement time)"
            echo "  --help               Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --all                    # Run all benchmarks"
            echo "  $0 --isolation --quick      # Quick run of isolation tests"
            echo "  $0 --compare main           # Compare current branch with main"
            echo "  $0 --baseline               # Save current results as baseline"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# If no specific suite selected and not comparing, default to all
if [ "$RUN_ALL" = false ] && \
   [ "$RUN_ISOLATION" = false ] && \
   [ "$RUN_COMBINATION" = false ] && \
   [ "$RUN_REALWORLD" = false ] && \
   [ "$RUN_STRESS" = false ] && \
   [ "$RUN_COMPREHENSIVE" = false ] && \
   [ -z "$COMPARE_BRANCH" ]; then
    RUN_ALL=true
fi

# Create results directory
RESULTS_DIR="target/benchmark_results"
mkdir -p "$RESULTS_DIR"

# Timestamp for this run
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULT_FILE="$RESULTS_DIR/results_$TIMESTAMP.txt"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  JSON-Tools-rs Comprehensive Benchmark Suite              ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Timestamp: $TIMESTAMP${NC}"
echo -e "${BLUE}Results will be saved to: $RESULT_FILE${NC}"
echo ""

# Function to run a benchmark
run_benchmark() {
    local name=$1
    local bench_name=$2

    echo -e "${GREEN}▶ Running $name benchmarks...${NC}"

    if [ "$QUICK_MODE" = true ]; then
        cargo bench --bench "$bench_name" -- --quick 2>&1 | tee -a "$RESULT_FILE"
    else
        cargo bench --bench "$bench_name" 2>&1 | tee -a "$RESULT_FILE"
    fi

    echo "" | tee -a "$RESULT_FILE"
    echo -e "${GREEN}✓ Completed $name benchmarks${NC}"
    echo "" | tee -a "$RESULT_FILE"
}

# Function to compare with baseline
compare_with_baseline() {
    if [ -f "$RESULTS_DIR/baseline.txt" ]; then
        echo -e "${YELLOW}📊 Comparing with baseline...${NC}"
        # TODO: Implement comparison logic using critcmp or similar
        echo -e "${YELLOW}   (Comparison feature coming soon)${NC}"
    else
        echo -e "${YELLOW}ℹ No baseline found. Run with --baseline to create one.${NC}"
    fi
}

# Function to compare with another branch
compare_with_branch() {
    local branch=$1
    local current_branch=$(git branch --show-current)

    echo -e "${YELLOW}📊 Comparing $current_branch with $branch...${NC}"

    # Save current results
    local current_results="$RESULTS_DIR/current_$TIMESTAMP.txt"
    cp "$RESULT_FILE" "$current_results"

    # Checkout comparison branch
    echo -e "${BLUE}Switching to branch: $branch${NC}"
    git stash
    git checkout "$branch"

    # Run benchmarks on comparison branch
    echo -e "${BLUE}Running benchmarks on $branch...${NC}"
    local branch_results="$RESULTS_DIR/${branch}_$TIMESTAMP.txt"

    if [ "$RUN_ALL" = true ]; then
        run_benchmark "Isolation" "isolation_benchmarks" > "$branch_results"
        run_benchmark "Combination" "combination_benchmarks" >> "$branch_results"
        run_benchmark "Real-World" "realworld_benchmarks" >> "$branch_results"
        run_benchmark "Stress" "stress_benchmarks" >> "$branch_results"
        run_benchmark "Comprehensive" "comprehensive_benchmark" >> "$branch_results"
    fi

    # Switch back to original branch
    git checkout "$current_branch"
    git stash pop || true

    echo -e "${GREEN}✓ Comparison complete${NC}"
    echo -e "${BLUE}Current branch results: $current_results${NC}"
    echo -e "${BLUE}Branch '$branch' results: $branch_results${NC}"
}

# Main benchmark execution
if [ -n "$COMPARE_BRANCH" ]; then
    compare_with_branch "$COMPARE_BRANCH"
else
    # Run selected benchmark suites
    if [ "$RUN_ALL" = true ]; then
        run_benchmark "Isolation" "isolation_benchmarks"
        run_benchmark "Combination" "combination_benchmarks"
        run_benchmark "Real-World" "realworld_benchmarks"
        run_benchmark "Stress" "stress_benchmarks"
        run_benchmark "Comprehensive" "comprehensive_benchmark"
    else
        [ "$RUN_ISOLATION" = true ] && run_benchmark "Isolation" "isolation_benchmarks"
        [ "$RUN_COMBINATION" = true ] && run_benchmark "Combination" "combination_benchmarks"
        [ "$RUN_REALWORLD" = true ] && run_benchmark "Real-World" "realworld_benchmarks"
        [ "$RUN_STRESS" = true ] && run_benchmark "Stress" "stress_benchmarks"
        [ "$RUN_COMPREHENSIVE" = true ] && run_benchmark "Comprehensive" "comprehensive_benchmark"
    fi

    # Save as baseline if requested
    if [ "$SAVE_BASELINE" = true ]; then
        cp "$RESULT_FILE" "$RESULTS_DIR/baseline.txt"
        echo -e "${GREEN}✓ Results saved as baseline${NC}"
    fi

    # Compare with baseline
    compare_with_baseline
fi

# Summary
echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  Benchmark Summary                                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✓ All benchmarks completed successfully!${NC}"
echo -e "${BLUE}Results saved to: $RESULT_FILE${NC}"
echo ""
echo -e "${YELLOW}Benchmark Suites Available:${NC}"
echo -e "  • ${BLUE}isolation_benchmarks${NC}    - Individual features in isolation"
echo -e "  • ${BLUE}combination_benchmarks${NC}  - Systematic feature combinations"
echo -e "  • ${BLUE}realworld_benchmarks${NC}    - Real-world API response formats"
echo -e "  • ${BLUE}stress_benchmarks${NC}       - Edge cases and stress tests"
echo -e "  • ${BLUE}comprehensive_benchmark${NC} - Original comprehensive suite"
echo ""
echo -e "${YELLOW}Performance Analysis Tips:${NC}"
echo -e "  • Look for ${RED}regressions${NC} compared to baseline"
echo -e "  • Identify ${GREEN}optimization opportunities${NC} in slow paths"
echo -e "  • Check for ${YELLOW}interaction effects${NC} in combinations"
echo -e "  • Validate ${BLUE}parallel processing${NC} improvements"
echo ""
