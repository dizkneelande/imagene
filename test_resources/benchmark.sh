#!/bin/bash

# Create the output directory if it doesn't exist
mkdir -p out

# Define the font path as a variable
FONT_PATH="./JetBrainsMono-Bold.ttf"

# Function to run a command and measure its execution time
run_benchmark() {
    echo "Running: $1"
    start=$(date +%s.%N)
    eval $1
    end=$(date +%s.%N)
    runtime=$(awk "BEGIN {print $end - $start}")
    echo "Execution time: $runtime seconds"
    echo ""
}

# Ensure the input file exists
if [ ! -f "jigglypuff.jpg" ]; then
    echo "Error: jigglypuff.jpg not found in the current directory."
    exit 1
fi

# Basic resizing
run_benchmark "../target/release/imagene jigglypuff.jpg resize:500,0 out/resized_jigglypuff.png"

# Applying a blur effect
run_benchmark "../target/release/imagene jigglypuff.jpg blur:5.0 out/blurred_jigglypuff.png"

# Adjusting brightness and contrast
run_benchmark "../target/release/imagene jigglypuff.jpg brightness:20 contrast:10 out/bright_contrast_jigglypuff.png"

# Rotating and flipping
run_benchmark "../target/release/imagene jigglypuff.jpg rotate:right flip:h out/rotated_flipped_jigglypuff.png"

# Adding a watermark 
run_benchmark "../target/release/imagene jigglypuff.jpg watermark:\"Pokemon\"\,\(0.5:0.5\)\,\(0.0:0.0:0.0:1.0\)\,\($FONT_PATH:0.5\) out/watermarked_jigglypuff.png"

# Cropping
run_benchmark "../target/release/imagene jigglypuff.jpg crop:100,100,300,300 out/cropped_jigglypuff.png"

# Inverting colors
run_benchmark "../target/release/imagene jigglypuff.jpg invert:true out/inverted_jigglypuff.png"

# Chaining multiple operations
run_benchmark "../target/release/imagene jigglypuff.jpg resize:400,0 blur:2.0 brightness:10 contrast:5 watermark:\"Jigglypuff\"\,\(0.5:0.9\)\,\(1.0:0.0:0.0:0.8\)\,\($FONT_PATH:0.1\) out/final_jigglypuff.png"

# Outputting to stdout (for piping to other programs)
run_benchmark "../target/release/imagene jigglypuff.jpg resize:200,0 stdout > out/small_jigglypuff.png"

# Complex pipeline for benchmarking
echo "Running complex pipeline for benchmarking"
start=$(date +%s.%N)
../target/release/imagene jigglypuff.jpg \
    resize:1000,0 \
    blur:3.0 \
    brightness:15 \
    contrast:10 \
    rotate:right \
    crop:50,50,900,900 \
    watermark:"Benchmark"\,\(0.5:0.5\)\,\(0.0:0.0:0.0:1.0\)\,\($FONT_PATH:0.5\) \
    flip:v \
    unsharpen:3.0,5 \
    invert:true \
    append:jigglypuff.jpg,right \
    resize:500,0 \
    watermark:"Final"\,\(0.5:0.5\)\,\(0.0:0.0:0.0:1.0\)\,\($FONT_PATH:0.5\) \
    out/benchmark_result.png
end=$(date +%s.%N)
runtime=$(awk "BEGIN {print $end - $start}")
echo "Complex pipeline execution time: $runtime seconds"
